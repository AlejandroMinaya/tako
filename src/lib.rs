use std::collections::BTreeSet;
use std::cmp::Ordering;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
enum TaskStatus {
    #[default]
    Open,
    Blocked,
    Archived,
    Done
}

#[derive(Debug, Default)]
pub struct Task<'a> {
    id: u32,
    importance: f32,
    urgency: f32,
    status: TaskStatus,
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
        if self.subtasks.is_empty() { return 1 };

        let sub_itr = self.subtasks.iter();
        return sub_itr.fold(
            0_u32,
            |result, subtask| { result + subtask.get_complexity() }
        )
    }

    pub fn add_subtask (&mut self, subtask: &'a Task<'a>) {
        self.subtasks.replace(subtask);
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
        // Compare if it is the same task
        if self == other { return Ordering::Equal }

        // Compare task status
        if self.status != other.status {
            return self.status.cmp(&other.status);
        }

        // Compare (urgency, importance)
        let dist = self.get_distance();
        let other_dist = other.get_distance();
        if dist > other_dist {
            return Ordering::Greater;
        }
        if dist < other_dist {
            return Ordering::Less;
        }

        // Compare complexity
        let self_complexity = self.get_complexity();
        let other_complexity = other.get_complexity();
        if self_complexity != other_complexity {
            return self_complexity.cmp(&other_complexity);
        }

        // Compare IDs
        return self.id.cmp(&other.id);
    }
}
impl PartialOrd for Task<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_multiple_task () {
        let mut root = Task::default();
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

        root.add_subtask(&task_a);
        root.add_subtask(&task_b);

        assert!(root.subtasks.contains(&task_a));
        assert!(root.subtasks.contains(&task_b));
        assert!(!root.subtasks.contains(&task_c));
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
        let root = Task {
            subtasks: BTreeSet::from([&task_a, &task_b, &task_c, &task_d]),
            ..Default::default()
        };


        let mut task_itr = root.subtasks.into_iter();
        assert_eq!(Some(&task_b), task_itr.next());
        assert_eq!(Some(&task_c), task_itr.next());
        assert_eq!(Some(&task_a), task_itr.next());
        assert_eq!(Some(&task_d), task_itr.next());
        assert_eq!(None, task_itr.next());
    }

    #[test]
    fn test_same_importance_different_complexity_sort () {
        let subtask_a = Task::default();
        let subtask_b = Task { id: 2, ..Default::default() };
        let task_a = Task::default();
        let task_b = Task {
            id: 1, 
            subtasks: BTreeSet::from([&subtask_a, &subtask_b]),
            ..Default::default()
        };
        let task_c = Task {
            id: 2,
            ..Default::default()
        };
        let root = Task {
            subtasks: BTreeSet::from([&task_a, &task_b, &task_c]),
            ..Default::default()
        };

        let mut task_itr = root.subtasks.into_iter();
        assert_eq!(Some(&task_a), task_itr.next(), "Expected Task A");
        assert_eq!(Some(&task_c), task_itr.next(), "Expected Task C");
        assert_eq!(Some(&task_b), task_itr.next(), "Expected Task B");
        assert_eq!(None, task_itr.next());

    }

    #[test]
    fn test_different_status_sort () {
        let task_a = Task {
            id: 4,
            ..Default::default()
        };
        let task_b = Task {
            id: 3,
            status: TaskStatus::Done,
            ..Default::default()
        };
        let task_c = Task {
            id: 2,
            status: TaskStatus::Blocked,
            ..Default::default()
        };
        let task_d = Task {
            id: 1,
            status: TaskStatus::Archived,
            ..Default::default()
        };

        let root = Task {
            subtasks: BTreeSet::from([&task_a, &task_b, &task_c, &task_d]),
            ..Default::default()
        };

        let mut itr = root.subtasks.into_iter();
        assert_eq!(Some(&task_a), itr.next(), "Expected Task A (Open)");
        assert_eq!(Some(&task_c), itr.next(), "Expected Task C (Blocked)");
        assert_eq!(Some(&task_d), itr.next(), "Expected Task D (Archived)");
        assert_eq!(Some(&task_b), itr.next(), "Expected Task B (Done)");
        assert_eq!(None, itr.next(), "Expected None");
    }

    /* TODO
    #[test]
    fn test_reinsert_into_correct_position_after_update() {
        
    }
    */


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

    #[test]
    fn test_add_same_id_subtask_updates_it() {
        let mut task = Task::default();
        let subtask = Task::default();
        let other_subtask = Task {
            importance: 42.0,
            ..Default::default()
        };

        task.add_subtask(&subtask);
        task.add_subtask(&other_subtask);

        let retrieved_subtask = task.subtasks.get(&subtask);
        assert_eq!(retrieved_subtask.expect("expected task with id = 0").importance, 42.0);
    }

    /*
     * TODO: Would be too cumbersome to enforce right now
     * 2024-07-21
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

    #[test]
    fn test_get_task_complexity_multilevel_single_leaf() {
        let mut task = Task::default();
        // Level 1
        let mut subtask_a = Task {
            id: 1,
            ..Default::default()
        };
        // Level 2
        let mut subtask_b = Task {
            id: 2,
            ..Default::default()
        };
        // Level 3
        let mut subtask_c = Task {
            id: 3,
            ..Default::default()
        };
        // Level 4
        let mut subtask_d = Task {
            id: 4,
            ..Default::default()
        };
        // Level 5
        let subtask_e = Task {
            id: 3,
            ..Default::default()
        };

        subtask_d.add_subtask(&subtask_e);
        subtask_c.add_subtask(&subtask_d);
        subtask_b.add_subtask(&subtask_c);
        subtask_a.add_subtask(&subtask_b);
        task.add_subtask(&subtask_a);


        assert_eq!(task.get_complexity(), 1);
    }

}
