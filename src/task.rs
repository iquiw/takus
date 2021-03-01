use std::collections::BTreeMap;
use std::io::{stdout, Write};
use std::path::PathBuf;
use std::process::Command;

use serde::{Deserialize, Serialize};

use super::state::GlobalState;

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
    #[cfg(test)]
    pub fn new(
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

    pub fn execute(&self, gs: &GlobalState) -> anyhow::Result<()> {
        for cmd in &self.cmds {
            let output = Command::new("sh")
                .current_dir(self.dir.as_ref().unwrap_or(gs.working_dir()))
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
