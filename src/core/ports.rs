use crate::core::tasks::*;
use std::vec::IntoIter;

pub trait DataStore {
    fn write(&self, task_itr: IntoIter<Box<Task>>) -> bool;
    fn write_task(&self, task: &Task) -> bool;
    fn read(&self) -> IntoIter<Box<Task>>;
    fn read_task(&self, id: u32) -> Box<Task>;
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[derive(Default)]
    pub struct MockDataStore {
        write_return_val: bool,
        write_task_return_val: bool,
        read_return_val: bool,
        reead_task_return_val: bool,
    }

    impl DataStore for MockDataStore {
        fn write(&self, _task_itr: IntoIter<Box<Task>>) -> bool {
            self.write_return_val
        }
        fn write_task(&self, _task: &Task) -> bool {
            self.write_task_return_val
        }
        fn read(&self) -> IntoIter<Box<Task>> {
            let mut task_a = Box::new(Task::default());
            let task_b = Box::new(Task {
                id: 1,
                ..Default::default()
            });
            let mut task_c = Box::new(Task {
                id: 2,
                ..Default::default()
            });

            let subtask_a = Box::new(Task {
                id: 3,
                ..Default::default()
            });
            let subtask_b = Box::new(Task {
                id: 4,
                ..Default::default()
            });
            let subtask_c = Box::new(Task {
                id: 5,
                ..Default::default()
            });

            task_a.add_subtask(subtask_a);

            task_c.add_subtask(subtask_b);
            task_c.add_subtask(subtask_c);

            let test_tasks = vec![task_a, task_b, task_c];
            test_tasks.into_iter()
        }
        fn read_task(&self, id: u32) -> Box<Task> {
            let task = Task {
                id,
                ..Default::default()
            };
            Box::new(task)
        }
    }
}
