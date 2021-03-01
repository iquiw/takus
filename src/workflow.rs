use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use anyhow::bail;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use yaml_merge_keys::merge_keys_serde;

use super::state::GlobalState;
use super::task::Task;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Workflow {
    version: String,
    templates: Option<Value>,
    tasks: BTreeMap<String, Task>,
}

impl Workflow {
    pub fn load<P>(path: P) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        let mut f = File::open(path)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        let yaml: Value = serde_yaml::from_str(&s)?;
        let merged = merge_keys_serde(yaml)?;
        Ok(serde_yaml::from_value(merged)?)
    }

    pub fn run(&mut self, gs: &GlobalState, selected_names: &Vec<String>) -> anyhow::Result<()> {
        for selected_name in selected_names {
            let selected = Workflow::select(&self.tasks, selected_name)?;
            let ordered = Workflow::order(&selected);
            for task_name in ordered {
                if let Some(task) = self.tasks.get(&task_name) {
                    println!("Task {}", task_name);
                    task.execute(gs)?;
                }
            }
        }
        Ok(())
    }

    fn select<'a>(
        map: &'a BTreeMap<String, Task>,
        task_name: &str,
    ) -> anyhow::Result<BTreeMap<String, &'a Task>> {
        let mut selected = BTreeMap::<String, &Task>::new();
        if let Some(task) = map.get(task_name) {
            let mut names: Vec<String> = task.deps().clone();
            selected.insert(task_name.to_string(), &task);
            loop {
                let mut nexts = vec![];
                for name in names {
                    if let Some(task) = map.get(&name) {
                        nexts.append(&mut task.deps().clone());
                        selected.insert(name.to_string(), &task);
                    }
                }
                if nexts.is_empty() {
                    break;
                } else {
                    names = vec![];
                    names.append(&mut nexts);
                }
            }
            Ok(selected)
        } else {
            bail!("Task not found: {}", task_name)
        }
    }

    fn order(map: &BTreeMap<String, &Task>) -> Vec<String> {
        let mut task_names: Vec<&String> = map.keys().collect();
        let mut dmap = BTreeMap::<String, u32>::new();
        let mut index = 0;
        while task_names.len() > 0 {
            let mut removed = vec![];
            for i in 0..task_names.len() {
                let task_name = task_names[i];
                let task = map.get(task_name).unwrap();
                let dep_len = task
                    .deps()
                    .iter()
                    .filter(|d| !dmap.contains_key(*d))
                    .count();
                if dep_len == 0 {
                    dmap.insert(task_name.to_string(), index);
                    index += 1;
                    removed.push(i);
                }
            }
            removed.reverse();
            for i in removed {
                task_names.remove(i);
            }
        }
        let mut dlist: Vec<(&String, &u32)> = dmap.iter().collect();
        dlist.sort_by(|x, y| x.1.cmp(y.1));
        dlist.iter().map(|x| x.0.to_string()).collect()
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
    fn test_select() {
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
        let result = Workflow::select(&mut m, "C");
        assert!(result.is_ok());
        let map = result.unwrap();
        assert_eq!(map.len(), 3);
        assert!(map.contains_key("C"));
        assert!(map.contains_key("D"));
        assert!(map.contains_key("E"));
    }

    #[test]
    fn test_order() {
        let t_e = new_task(vec![]);
        let t_d = new_task(vec!["E".to_string()]);
        let t_c = new_task(vec!["D".to_string()]);
        let t_b = new_task(vec!["E".to_string(), "C".to_string()]);
        let t_a = new_task(vec!["B".to_string()]);

        let mut m = BTreeMap::<String, &Task>::new();
        m.insert("A".to_string(), &t_a);
        m.insert("B".to_string(), &t_b);
        m.insert("C".to_string(), &t_c);
        m.insert("D".to_string(), &t_d);
        m.insert("E".to_string(), &t_e);
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
