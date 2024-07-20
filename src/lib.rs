use std::collections::BTreeSet;
use std::cmp::Ordering;

#[derive(Debug, Default, Clone, Copy)]
struct Task {
    id: u32,
    importance: u32,
    urgency: u32
}
impl Task {
    fn get_distance_to_max (&self) -> u32 {
        todo!();
    }
}
impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Task {}
impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        // TODO: This may overflow eventually, but I understand that it is not a concern right now
        // since we would need to reach around 2^16 tasks
        match self.urgency.cmp(&other.urgency) {
            Ordering::Equal => match self.importance.cmp(&other.importance) {
                Ordering::Equal => self.id.cmp(&other.id),
                other => other
            },
            other => other
        }
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
    fn test_distance_to_max_task() {
        let task_a = Task {
            importance: u32::MAX,
            urgency: u32::MAX,
            ..Default::default()
        };
        let task_b = Task {
            importance: 2_u32.pow(32) - 5,
            urgency: 2_u32.pow(32) - 13,
            ..Default::default()
        };

        assert_eq!(task_a.get_distance_to_max(), 0);
        assert_eq!(task_b.get_distance_to_max(), 4);


    }

    #[test]
    fn test_multiple_sorted_importance_urgency () {
        let mut root = RootTask::default();
        let task_a = Task {
            id: 1,
            importance: 4,
            urgency: 1,
            ..Default::default()
        };
        let task_b = Task {
            id: 2,
            importance: 3,
            urgency: 2,
            ..Default::default()
        };
        let task_c = Task {
            id: 3,
            importance: 2,
            urgency: 3,
            ..Default::default()
        };
        let task_d = Task {
            id: 4,
            importance: 1,
            urgency: 4,
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
