use crate::core::tasks::*;
use std::fmt::Debug;
use std::vec::IntoIter;

pub trait DataStore: Debug {
    fn write(&self, task_itr: IntoIter<&Task>) -> bool;
    fn write_task(&self, task: &Task) -> bool;
    fn read(&self) -> IntoIter<Box<Task>>;
    fn read_task(&self, id: u32) -> Box<Task>;
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[derive(Debug, Default)]
    pub struct MockDataStore {
        write_return_val: bool,
        write_task_return_val: bool,
    }

    impl DataStore for MockDataStore {
        fn write(&self, _task_itr: IntoIter<&Task>) -> bool {
            self.write_return_val
        }
        fn write_task(&self, _task: &Task) -> bool {
            self.write_task_return_val
        }
        fn read(&self) -> IntoIter<Box<Task>> {
            let mut task_a = Box::new(Task::new_with_id(0));
            let task_b = Box::new(Task::new_with_id(1));
            let mut task_c = Box::new(Task::new_with_id(2));
            let subtask_a = Box::new(Task::new_with_id(3));
            let subtask_b = Box::new(Task::new_with_id(4));
            let subtask_c = Box::new(Task::new_with_id(5));
            task_a.add_subtask(subtask_a);

            task_c.add_subtask(subtask_b);
            task_c.add_subtask(subtask_c);

            let test_tasks = vec![task_a, task_b, task_c];
            test_tasks.into_iter()
        }
        fn read_task(&self, id: u32) -> Box<Task> {
            Box::new(Task::new_with_id(id))
        }
    }
}
