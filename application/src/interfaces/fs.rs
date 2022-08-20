use anyhow::Result;

pub trait FileSystem {
    fn watch_file(&self, path: &str) -> Result<()>;
}
