use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use super::task::Task;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Workflow {
    version: String,
    templates: Option<Value>,
    tasks: BTreeMap<String, Task>,
}
