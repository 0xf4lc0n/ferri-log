use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::PathBuf,
    sync::{mpsc::channel, Arc},
    time::Duration,
};

use anyhow::{anyhow, bail, Result};
use application::prelude::{Cache, DiskLogEntryDto, FileSystem};
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
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

    fn handle_event(&self, event: DebouncedEvent) -> Result<()> {
        info!("{event:?}");

        match event {
            DebouncedEvent::Write(path) => self.handle_file_change(path),
            DebouncedEvent::Remove(path) => self
                .cache
                .del(&LinuxFS::<T>::path_buff_to_str(path)?)
                .map(|_| ())
                .map_err(|e| anyhow!(e)),
            _ => Ok(()),
        }
    }

    #[instrument(skip_all)]
    fn handle_file_change(&self, path: PathBuf) -> Result<()> {
        let path = LinuxFS::<T>::path_buff_to_str(path)?;
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

    fn path_buff_to_str(path: PathBuf) -> Result<String> {
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

impl<T> FileSystem for LinuxFS<T>
where
    T: Cache,
{
    fn watch_file(&self, path: &str) -> Result<()> {
        let (tx, rx) = channel();
        let mut watcher = watcher(tx, Duration::from_secs(1))?;

        watcher.watch(path, RecursiveMode::Recursive)?;

        loop {
            match rx.recv() {
                Ok(event) => self.handle_event(event)?,
                Err(e) => error!("Cannot watch '{path}': {e}"),
            }
        }
    }
}
