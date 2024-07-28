use crate::core::tasks::*;
use std::fmt::Debug;
use std::vec::IntoIter;
use async_trait::async_trait;

#[async_trait]
pub trait DataStore: Debug {
    async fn write(&self, task_itr: IntoIter<&Task>) -> bool;
    async fn read(&self) -> anyhow::Result<IntoIter<Box<Task>>>;
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[derive(Debug, Default)]
    pub struct MockDataStore {
        write_return_val: bool,
    }

    #[async_trait]
    impl DataStore for MockDataStore {
        async fn write(&self, _task_itr: IntoIter<&Task>) -> bool {
            self.write_return_val
        }
        async fn read(&self) -> anyhow::Result<IntoIter<Box<Task>>> {
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
            Ok(test_tasks.into_iter())
        }
    }
}
