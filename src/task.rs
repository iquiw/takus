use std::collections::BTreeMap;
use std::io::{stdout, Write};
use std::path::PathBuf;
use std::process::Command;

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
    pub(super) fn new(
        cmds: Vec<String>,
        deps: Vec<String>,
        dir: Option<PathBuf>,
        envs: BTreeMap<String, String>,
    ) -> Self {
        Task {
            cmds,
            deps,
            dir,
            envs,
        }
    }

    pub fn execute(&self) -> anyhow::Result<()> {
        for cmd in &self.cmds {
            let output = Command::new("sh")
                .envs(&self.envs)
                .arg("-c")
                .arg(cmd)
                .output()?;
            stdout().write_all(&output.stdout)?;
        }
        Ok(())
    }

    pub(super) fn deps(&self) -> &Vec<String> {
        &self.deps
    }
}
