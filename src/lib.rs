use std::collections::BTreeSet;
use std::cmp::Ordering;

#[derive(Debug, Default, Clone, Copy)]
struct Task {
    id: u32,
    importance: u32,
    urgency: u32
}
impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Task {}
impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}
impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

#[derive(Debug, Default)]
struct RootTask {
    all_tasks: BTreeSet<Box<Task>>
}
impl RootTask {
    fn add_task(&mut self, task: Task) {
        self.all_tasks.insert(Box::new(task));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_single_task () {
        let mut root = RootTask::default();
        let task = Task::default();
        root.add_task(task);

        assert!(root.all_tasks.contains(&task));
    }

    #[test]
    fn test_add_multiple_task () {
        let mut root = RootTask::default();
        let task_a = Task {
            id: 1,
            ..Default::default()
        };
        let task_b = Task {
            id: 2,
            ..Default::default()
        };
        let task_c = Task {
            id: 3,
            ..Default::default()
        };

        root.add_task(task_a);
        root.add_task(task_b);

        assert!(root.all_tasks.contains(&task_a));
        assert!(root.all_tasks.contains(&task_b));
        assert!(!root.all_tasks.contains(&task_c));
    }

    #[test]
    fn test_multiple_sorted_importance_urgency () {
        let mut root = RootTask::default();
        let task_a = Task {
            id: 1,
            importance: 1,
            urgency: 4,
            ..Default::default()
        };
        let task_b = Task {
            id: 2,
            importance: 2,
            urgency: 3,
            ..Default::default()
        };
        let task_c = Task {
            id: 3,
            importance: 3,
            urgency: 2,
            ..Default::default()
        };
        let task_d = Task {
            id: 4,
            importance: 4,
            urgency: 1,
            ..Default::default()
        };

        root.add_task(task_a);
        root.add_task(task_b);
        root.add_task(task_c);
        root.add_task(task_d);

        let mut task_itr = root.all_tasks.into_iter();
        assert_eq!(Some(Box::new(task_d)), task_itr.next());
        assert_eq!(Some(Box::new(task_b)), task_itr.next());
        assert_eq!(Some(Box::new(task_c)), task_itr.next());
        assert_eq!(Some(Box::new(task_a)), task_itr.next());
        assert_eq!(None, task_itr.next());
    }
}
