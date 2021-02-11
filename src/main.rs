use std::fs::File;
use std::io::Read;

use serde_yaml::Value;
use yaml_merge_keys::merge_keys_serde;

use takus::workflow::Workflow;

fn main() -> anyhow::Result<()> {
    let mut f = File::open("takus.yml")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    let yaml: Value = serde_yaml::from_str(&s)?;
    let merged = merge_keys_serde(yaml)?;
    let wf: Workflow = serde_yaml::from_value(merged)?;
    wf.run()
}
