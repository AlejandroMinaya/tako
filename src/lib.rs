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

/* TASK DEFINITION */
#[derive(Debug, Default, Clone)]
pub struct Task {
    id: u16,
    children: Rc<BTreeSet<Rc<Self>>>,
    title: String,
    importance: u16,
    urgency: u16,
    status: TaskStatus
}
impl Task {
    pub fn add_subtask(&self, subtask: Rc<Self>) {
        (*self.children).insert(subtask);
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
        let mut task = Task::default();
        let subtask = Rc::new(Task {
            id: 1,
            ..Default::default()
        });

        task.add_subtask(subtask.clone());

        assert!(
            task.children.contains(&subtask),
            "Expected subtask to be included as subtask child"
        );
    }

    fn test_adding_subtask_with_children_doesnt_append_grandchildren () {
        let mut grand_task = Task::default();
        let mut task = Rc::new(Task { id: 1, ..Default::default() });
        let child_task = Rc::new(Task { id: 2, ..Default::default() });

        (*task).add_subtask(child_task.clone());
        grand_task.add_subtask(task.clone());
    }
}
