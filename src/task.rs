use std::collections::BTreeMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Task {
    cmds: Vec<String>,
    #[serde(default)]
    deps: Vec<String>,
    dir: Option<PathBuf>,
    #[serde(default)]
    envs: BTreeMap<String, String>,
}

impl Task {
    pub fn execute() {
    }
}
