mod configuration;

use application::interfaces::fs::FileSystem;
use infrastructure::cache;
use infrastructure::fs;
use infrastructure::telemetry;
use std::sync::Arc;
use std::thread;
use tracing::{error, info};
fn main() {
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "debug".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let config = configuration::get_configuration().unwrap();

    let cache = Arc::new(cache::SkyTableCache::new("127.0.0.1", 2003));
    let file_system = Arc::new(fs::LinuxFS::new(cache));

    let fs_handle = {
        let fs = file_system.clone();
        thread::spawn(move || fs.watch_file(&config.application.folder_to_watch))
    };

    match fs_handle.join().unwrap() {
        Ok(_) => info!("File system closed gracefully"),
        Err(e) => error!("Error occured when joining watch_file: {e:?}"),
    }
}
