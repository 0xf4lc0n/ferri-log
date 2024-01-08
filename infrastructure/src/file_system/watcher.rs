use anyhow::Result;
use application::prelude::FileSystem;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{error, info};

pub fn async_watcher() -> notify::Result<(
    RecommendedWatcher,
    tokio::sync::mpsc::Receiver<Result<Event, notify::Error>>,
)> {
    let (tx, rx) = mpsc::channel(1);

    let watcher = RecommendedWatcher::new(
        move |res| tx.blocking_send(res).expect("Failed to send event"),
        Config::default(),
    )?;

    Ok((watcher, rx))
}

pub fn watch_dir<T>(path: &str, fs: Arc<T>) -> Result<RecommendedWatcher>
where
    T: FileSystem + Send + Sync + 'static,
{
    let (mut watcher, mut rx) = async_watcher()?;
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    info!("Watching for folder '{path}' started");

    tokio::task::spawn(async move {
        while let Some(res) = rx.recv().await {
            match res {
                Ok(event) => {
                    if let Err(e) = fs.handle_event(event).await {
                        error!("Handle event error: {:?}", e)
                    }
                }
                Err(e) => error!("Watch error: {:?}", e),
            }
        }
    });

    Ok(watcher)
}
