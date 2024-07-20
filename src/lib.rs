use std::collections::BTreeSet;

#[derive(Debug, Default)]
struct Task {}

#[derive(Debug, Default)]
struct RootTask {
    all_tasks: BTreeSet<Box<Task>>
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_add_task () {
        let root = RootTask::default();
        let task = Box::new(Task::default());
        root.add_task(task);

        assert!(root.all_tasks.contains(&task));


    }
}
