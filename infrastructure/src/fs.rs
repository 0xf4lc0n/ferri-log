use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::PathBuf,
    sync::Arc,
};

use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

use anyhow::{anyhow, bail, Result};
use application::prelude::{Cache, DiskLogEntryDto, FileSystem};
use async_trait::async_trait;
use skytable::{error::Error::SkyError, error::SkyhashError, RespCode};
use tracing::{debug, error, info, instrument};

pub struct LinuxFS<T: Cache> {
    cache: Arc<T>,
}

impl<T> LinuxFS<T>
where
    T: Cache,
{
    pub fn new(cache: Arc<T>) -> Self {
        LinuxFS { cache }
    }

    fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
        let (mut tx, rx) = channel(1);

        let watcher = RecommendedWatcher::new(
            move |res| {
                futures::executor::block_on(async {
                    tx.send(res).await.unwrap();
                })
            },
            Config::default(),
        )?;

        Ok((watcher, rx))
    }

    fn handle_event(&self, event: Event) -> Result<()> {
        info!("{event:?}");

        match event.kind {
            EventKind::Modify(_) => self.on_files_modification(event.paths),
            EventKind::Remove(_) => self.on_files_delete(event.paths),
            _ => Ok(()),
        }
    }

    fn on_files_modification(&self, paths: Vec<PathBuf>) -> Result<()> {
        for path in paths {
            self.handle_file_change(path)?;
        }

        Ok(())
    }

    fn on_files_delete(&self, paths: Vec<PathBuf>) -> Result<()> {
        for path in paths {
            let path = Self::path_buff_to_string(path)?;
            self.cache.del(&path)?;
        }

        Ok(())
    }

    #[instrument(skip_all)]
    fn handle_file_change(&self, path: PathBuf) -> Result<()> {
        let path = LinuxFS::<T>::path_buff_to_string(path)?;
        let mut file = File::open(&path)?;
        let mut buffer = String::new();

        let to_skip = match self.cache.get::<u64>(&path) {
            Ok(v) => v,
            Err(SkyError(SkyhashError::Code(RespCode::NotFound))) => {
                self.cache.set(&path, "0")?;
                u64::default()
            }
            Err(e) => return Err(anyhow!(e)),
        };

        debug!("To skip: {to_skip}");

        file.seek(SeekFrom::Start(to_skip))?;
        let n = file.read_to_string(&mut buffer)? as u64;
        debug!("Readed {n}: {buffer:?}");
        LinuxFS::<T>::process_log_entry(buffer);
        self.cache.update(&path, (to_skip + n).to_string())?;

        Ok(())
    }

    fn path_buff_to_string(path: PathBuf) -> Result<String> {
        match path.to_str() {
            Some(s) => Ok(s.to_owned()),
            None => bail!("Cannot obtain &str from PathBuf"),
        }
    }

    fn process_log_entry(buff: String) {
        for line in buff.lines() {
            debug!("Current line: {line}");

            let log_entry = serde_json::from_str::<DiskLogEntryDto>(line).unwrap();

            debug!("LogEntry: {log_entry:?}");
        }
    }
}

#[async_trait]
impl<T> FileSystem for LinuxFS<T>
where
    T: Cache + Send + Sync,
{
    async fn watch_file(&self, path: &str) -> Result<()> {
        let (mut watcher, mut rx) = Self::async_watcher()?;

        watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

        info!("Watching for folder '{path}' started");

        while let Some(res) = rx.next().await {
            match res {
                Ok(event) => self.handle_event(event)?,
                Err(e) => error!("Watch error: {:?}", e),
            }
        }

        Ok(())
    }
}
