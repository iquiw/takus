use std::collections::{BTreeMap, HashSet};

use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use super::task::Task;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Workflow {
    version: String,
    templates: Option<Value>,
    tasks: BTreeMap<String, Task>,
}

struct TaskDependant {
    name: String,
    dependants: Vec<String>,
}

impl Workflow {
    pub fn run(&self) -> anyhow::Result<()> {
        for (task_name, task) in self.tasks.iter() {
            println!("Task {}", task_name);
            task.execute()?;
        }
        Ok(())
    }

    fn order(map: &BTreeMap<String, Task>) -> Vec<String> {
        let mut dmap = BTreeMap::<String, u32>::new();
        let mut index = 0;
        for (task_name, task) in map.iter() {
            let dep_len = task.deps().iter().filter(|d| dmap.contains_key(*d)).count();
            if dep_len == 0 {
                dmap.insert(task_name.to_string(), index);
                index += 1;
            }
        }
        println!("{:?}", dmap);
        vec![]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::task::*;
    use std::collections::BTreeMap;

    fn new_task(deps: Vec<String>) -> Task {
        Task::new(vec![], deps, None, BTreeMap::new())
    }

    #[test]
    fn test_order() {
        let t_e = new_task(vec![]);
        let t_d = new_task(vec!["E".to_string()]);
        let t_c = new_task(vec!["D".to_string()]);
        let t_b = new_task(vec!["E".to_string(), "C".to_string()]);
        let t_a = new_task(vec!["B".to_string()]);

        let mut m = BTreeMap::<String, Task>::new();
        m.insert("A".to_string(), t_a);
        m.insert("B".to_string(), t_b);
        m.insert("C".to_string(), t_c);
        m.insert("D".to_string(), t_d);
        m.insert("E".to_string(), t_e);
        let ord = Workflow::order(&m);
        assert_eq!(
            ord,
            vec![
                "E".to_string(),
                "D".to_string(),
                "C".to_string(),
                "B".to_string(),
                "A".to_string()
            ]
        );
    }
}
