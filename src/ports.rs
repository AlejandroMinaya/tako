use async_trait::async_trait;
use crate::core::tasks::{
    Task,
    BoxTaskVec
};
use std::fmt::Debug;

#[async_trait]
pub trait DataStore: Debug {
    async fn write(&self, _tasks: Vec<&Task>) -> anyhow::Result<()>;
    async fn read(&self) -> anyhow::Result<BoxTaskVec>;
}

#[derive(Debug, Default)]
pub struct MockDataStore;

#[async_trait]
impl DataStore for MockDataStore {
    async fn write(&self, _tasks: Vec<&Task>) -> anyhow::Result<()> {
        Ok(())
    }
    async fn read(&self) -> anyhow::Result<BoxTaskVec> {
        /*
         *                (r)
         *              /  |  \
         *             /   |   \
         *          (tA) (tB) (tC)
         *           |       /   \
         *         (sA)    (sB) (sC)
         */
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
        Ok(test_tasks)
    }
}

#[derive(Debug, Default)]
pub struct DummyStore;

#[async_trait]
impl DataStore for DummyStore {
    async fn write(&self, _tasks: Vec<&Task>) -> anyhow::Result<()> {
        Ok(())
    }
    async fn read(&self) -> anyhow::Result<BoxTaskVec> {
        Ok(vec![])
    }
}
