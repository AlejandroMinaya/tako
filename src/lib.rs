use std::rc::Rc;
use std::cmp::{
    Ord, Ordering
};
use std::collections::BTreeSet;

use thiserror::Error as DefaultError;


#[derive(Debug, PartialEq, Clone, Default)]
pub enum TaskStatus {
    #[default]
    Open,
    Blocked,
    Archived,
    Done
}

/* ERRORS */
#[derive(DefaultError, Debug)]
pub enum Error {
    #[error("Unknown error")]
    UnknownError
}

/* STRUCTS */
#[derive(Debug, Default, Clone)]
pub struct Task {
    id: u16,
    children: BTreeSet<Task>,
    title: String,
    importance: u16,
    urgency: u16,
    status: TaskStatus
}
impl Task {
    pub fn add_subtask(&self, subtask: &Self) {
        todo!();
    }
}
impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}
impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id
    }
}
impl Eq for Task { }

/* PORTS & ADAPTERS */
/* -- TASK PORTS & ADAPTERS */

/* -- DATA PORTS & ADAPTERS */
pub trait DataPort {
    fn save() -> Result<Rc<Task>, Error>;
    fn get_by_id(id: u16) -> Result<Rc<Task>, Error>;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_task_equality_based_on_i () {
        let a = Task {
            id: 0,
            ..Default::default()
        };
        let b = Task {
            id: 1,
            ..Default::default()
        };
        let c = Task {
            id: 0,
            title: "C".to_owned(),
            ..Default::default()
        };

        assert_eq!(
            a, c,
            "Expected same ID tasks to be the be equal"
        );
        assert_ne!(
            a, b,
            "Expected same content but ID tasks to not be equal"
        );
    }


    #[test]
    fn test_adding_subtask_to_task () {
        let task = Task::default();
        let subtask = Rc::new(Task {
            id: 1,
            ..Default::default()
        });

        task.add_subtask(&subtask);

        assert!(
            task.children.contains(&subtask),
            "Expected subtask to be included as subtask child"
        );
    }
}
