use std::env;
use std::path::PathBuf;

pub struct GlobalState {
    working_dir: PathBuf,
}

impl GlobalState {
    pub fn new() -> anyhow::Result<Self> {
        Ok(GlobalState {
            working_dir: env::current_dir()?,
        })
    }

    pub fn working_dir(&self) -> &PathBuf {
        &self.working_dir
    }
}
