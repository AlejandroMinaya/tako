use std::collections::BTreeSet;
use std::cmp::Ordering;

#[derive(Debug, Default)]
struct Task<'a> {
    id: u32,
    importance: f32,
    urgency: f32,
    subtasks: BTreeSet<&'a Task<'a>>
}

impl<'a> Task<'a> {
    fn get_distance (&self) -> f32 {
        let importance_comp = self.importance.powf(2.0);
        let urgency_comp = self.urgency.powf(2.0);
        let result = (importance_comp + urgency_comp).sqrt();
        return if result != f32::INFINITY { result } else { f32::MAX }
    }

    fn get_complexity (&self) -> u32 {
        todo!();
    }

    fn add_subtask (&mut self, subtask: &'a Task<'a>) {
        self.subtasks.insert(subtask);
    }
}
impl PartialEq for Task<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Task<'_> {}
impl Ord for Task<'_> {
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
impl PartialOrd for Task<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

#[derive(Debug, Default)]
struct RootTask<'a> {
    all_tasks: BTreeSet<&'a Task<'a>>
}
impl<'a> RootTask<'a> {
    fn add_task(&mut self, task: &'a Task) {
        self.all_tasks.insert(task);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_single_task () {
        let mut root = RootTask::default();
        let task = Task::default();
        root.add_task(&task);

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

        root.add_task(&task_a);
        root.add_task(&task_b);

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

        root.all_tasks.insert(&task_a);
        root.all_tasks.insert(&task_b);
        root.all_tasks.insert(&task_c);
        root.all_tasks.insert(&task_d);

        let mut task_itr = root.all_tasks.into_iter();
        assert_eq!(Some(&task_b), task_itr.next());
        assert_eq!(Some(&task_c), task_itr.next());
        assert_eq!(Some(&task_a), task_itr.next());
        assert_eq!(Some(&task_d), task_itr.next());
        assert_eq!(None, task_itr.next());
    }

    #[test]
    fn test_add_subtask_to_task () {
        let mut task = Task::default();
        let subtask = Task {
            id: 1,
            ..Default::default()
        };

        task.add_subtask(&subtask);

        task.subtasks.contains(&subtask);
    }

    /*
     * TODO: Would be too cumbersome to enforce right now
    #[test]
    fn test_add_subtask_with_parent_moves_subtask () {
        let mut task_a = Task::default();
        let mut task_b = Task::default();
        let subtask = Task {
            id: 1,
            ..Default::default()
        };

        task_a.add_subtask(&subtask);
        task_b.add_subtask(&subtask);

        assert!(!task_a.subtasks.contains(&subtask), "Subtask found in Task A");
        assert!(task_b.subtasks.contains(&subtask), "Subtask not found in Task B");
    }
    */

    #[test]
    fn test_get_task_complexity_single_level() {
        let mut task = Task::default();
        let subtask_a = Task {
            id: 1,
            ..Default::default()
        };
        let subtask_b = Task {
            id: 2,
            ..Default::default()
        };
        let subtask_c = Task {
            id: 3,
            ..Default::default()
        };

        task.add_subtask(&subtask_a);
        task.add_subtask(&subtask_b);
        task.add_subtask(&subtask_c);

        assert_eq!(task.get_complexity(), 3);
    }

    #[test]
    fn test_get_task_complexity_multilevel() {
        let mut task = Task::default();
        // Level 1
        let subtask_a = Task {
            id: 1,
            ..Default::default()
        };
        let mut subtask_b = Task {
            id: 2,
            ..Default::default()
        };
        // Level 2
        let subtask_c = Task {
            id: 3,
            ..Default::default()
        };
        let mut subtask_d = Task {
            id: 4,
            ..Default::default()
        };
        // Level 3
        let subtask_e = Task {
            id: 3,
            ..Default::default()
        };

        subtask_d.add_subtask(&subtask_e);

        subtask_b.add_subtask(&subtask_d);
        subtask_b.add_subtask(&subtask_c);

        task.add_subtask(&subtask_b);
        task.add_subtask(&subtask_a);


        assert_eq!(task.get_complexity(), 3);
    }
}
