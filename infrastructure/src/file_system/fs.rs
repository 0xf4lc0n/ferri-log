use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::{Path, PathBuf},
};

use anyhow::{anyhow, bail, Result};
use application::prelude::{Cache, DiskLogEntryDto, FileSystem, LogRepository};
use async_trait::async_trait;
use notify::{Event, EventKind};
use skytable::{error::Error::SkyError, error::SkyhashError, RespCode};
use tracing::{debug, info, instrument};

pub struct LinuxFS<T: Cache, L: LogRepository> {
    cache: T,
    log_repo: L,
}

impl<T, L> LinuxFS<T, L>
where
    T: Cache,
    L: LogRepository,
{
    pub fn new(cache: T, repo: L) -> Self {
        LinuxFS {
            cache,
            log_repo: repo,
        }
    }

    async fn on_files_modification(&self, paths: Vec<PathBuf>) -> Result<()> {
        for path in paths {
            self.handle_file_change(path).await?;
        }

        Ok(())
    }

    fn on_files_delete(&self, paths: Vec<PathBuf>) -> Result<()> {
        for path in paths {
            let path = path_buff_to_string(&path)?;
            self.cache.del(&path)?;
        }

        Ok(())
    }

    #[instrument(skip_all)]
    async fn handle_file_change(&self, path: PathBuf) -> Result<()> {
        if !path.is_file() {
            debug!("{} isn't a file", path.to_str().unwrap());
            return Ok(());
        }

        let path = path_buff_to_string(&path)?;
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

        self.process_log_entry(buffer).await?;
        self.cache.update(&path, (to_skip + n).to_string())?;

        Ok(())
    }

    async fn process_log_entry(&self, buff: String) -> Result<()> {
        for line in buff.lines() {
            debug!("Processing the following file contetn: {line}");
            let log_entry = serde_json::from_str::<DiskLogEntryDto>(line)?;
            self.log_repo.create_log(log_entry).await?;
        }
        Ok(())
    }
}

#[async_trait]
impl<T, L> FileSystem for LinuxFS<T, L>
where
    T: Cache + Send + Sync,
    L: LogRepository + Send + Sync,
{
    async fn handle_event(&self, event: Event) -> Result<()> {
        info!("Handling evet: {event:?}");

        match event.kind {
            EventKind::Modify(_) => self.on_files_modification(event.paths).await,
            EventKind::Remove(_) => self.on_files_delete(event.paths),
            _ => Ok(()),
        }
    }
}

fn path_buff_to_string(path: &Path) -> Result<String> {
    match path.to_str() {
        Some(s) => Ok(s.to_owned()),
        None => bail!("Cannot obtain &str from PathBuf"),
    }
}

#[allow(dead_code)]
fn get_file_name(path: &Path) -> Result<&str> {
    match path.file_name() {
        Some(name) => match name.to_str() {
            Some(name) => Ok(name),
            None => bail!("File name failed UTF-8 validity check"),
        },
        None => bail!("Cannot extract file name because path is invalid"),
    }
}
