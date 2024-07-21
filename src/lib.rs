use std::collections::BTreeSet;
use std::cmp::Ordering;

#[derive(Debug, Default, Clone, Copy)]
struct Task {
    id: u32,
    importance: f32,
    urgency: f32
}
impl Task {
    fn get_distance (&self) -> f32 {
        let importance_comp = self.importance.powf(2.0);
        let urgency_comp = self.urgency.powf(2.0);
        let result = (importance_comp + urgency_comp).sqrt();
        return if result != f32::INFINITY { result } else { f32::MAX }
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
        let dist = self.get_distance();
        let other_dist = other.get_distance();
        let ordering;

        if dist > other_dist {
            ordering = Ordering::Greater;
        } else if dist < other_dist {
            ordering = Ordering::Less;
        } else {
            ordering = self.id.cmp(&other.id)
        }

        return ordering;
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
    fn test_distance_to_max_task_overflow () {
        let task = Task {
            importance: f32::MAX,
            urgency: f32::MAX,
            ..Default::default()
        };
        assert_eq!(task.get_distance(), f32::MAX);
    }

    #[test]
    fn test_distance_to_max_task () {
        let task = Task {
            importance: 4.0,
            urgency: 3.0,
            ..Default::default()
        };

        assert_eq!(task.get_distance(), 5.0);
    }


    #[test]
    fn test_multiple_sorted_importance_urgency () {
        let mut root = RootTask::default();
        let task_a = Task {
            id: 1,
            importance: 4.0,
            urgency: 1.0,
            ..Default::default()
        };
        let task_b = Task {
            id: 2,
            importance: 3.0,
            urgency: 2.0,
            ..Default::default()
        };
        let task_c = Task {
            id: 3,
            importance: 2.0,
            urgency: 3.0,
            ..Default::default()
        };
        let task_d = Task {
            id: 4,
            importance: 1.0,
            urgency: 4.0,
            ..Default::default()
        };

        root.all_tasks.insert(Box::new(task_a));
        root.all_tasks.insert(Box::new(task_b));
        root.all_tasks.insert(Box::new(task_c));
        root.all_tasks.insert(Box::new(task_d));

        let mut task_itr = root.all_tasks.into_iter();
        assert_eq!(Some(Box::new(task_b)), task_itr.next());
        assert_eq!(Some(Box::new(task_c)), task_itr.next());
        assert_eq!(Some(Box::new(task_a)), task_itr.next());
        assert_eq!(Some(Box::new(task_d)), task_itr.next());
        assert_eq!(None, task_itr.next());
    }
}
