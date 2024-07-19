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
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Task {
    id: u16,
    title: String,
    importance: u16,
    urgency: u16,
    status: TaskStatus
}

/* PORTS & ADAPTERS */
/* -- TASK PORTS & ADAPTERS */
pub trait TaskPort {
    fn create(task: Task) -> Result<Box<Task>, Error>;
}

/* -- DATA PORTS & ADAPTERS */
pub trait DataPort {
    fn write() -> Result<Box<Task>, Error>;
    fn save() -> Result<Box<Task>, Error>;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_task_equality_based_on_i () {
        let a = Task {
            id: 0,
            title: "Dummy Test".to_owned(),
            importance: 0,
            urgency: 0,
            status: TaskStatus::Open
        };
        let b = Task {
            id: 1,
            title: "Dummy Test".to_owned(),
            importance: 0,
            urgency: 0,
            status: TaskStatus::Open
        };
        let c = Task {
            id: 0,
            title: "Smart Test".to_owned(),
            importance: 42,
            urgency: 1337,
            status: TaskStatus::Done
        };

        assert_eq!(a, c, "Expected same ID tasks to be the be equal");
        assert_ne!(b, a, "Expected same content but ID tasks to not be equal");
    }
}
