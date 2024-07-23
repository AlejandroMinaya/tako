use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum TaskStatus {
    #[default]
    Open,
    Blocked,
    Archived,
    Done,
}

#[derive(Debug, Default, Clone)]
pub struct Task<'a> {
    pub id: u32,
    pub importance: f32,
    pub urgency: f32,
    pub status: TaskStatus,
    subtasks_tree: BTreeSet<&'a Task<'a>>,
    subtasks_tree_map: HashMap<u32, Box<Task<'a>>>,
}

impl<'a> Task<'a> {
    fn get_distance(&self) -> f32 {
        let importance_comp = self.importance.powf(2.0);
        let urgency_comp = self.urgency.powf(2.0);
        let result = (importance_comp + urgency_comp).sqrt();
        return if result != f32::INFINITY {
            result
        } else {
            f32::MAX
        };
    }

    fn get_complexity(&self) -> u32 {
        if self.subtasks_tree.is_empty() {
            return 1;
        };

        let sub_itr = self.subtasks_tree.iter();
        return sub_itr.fold(0_u32, |result, subtask| result + subtask.get_complexity());
    }

    pub fn add_subtask(&'a mut self, subtask: Box<Task<'a>>) {
        let subtask_id = subtask.id;
        match self.subtasks_tree_map.insert(subtask_id, subtask) {
            Some(old_subtask) => {
                self.subtasks_tree.remove(old_subtask.as_ref());
            }
            None => (),
        };
        let new_subtask_ref = self
            .subtasks_tree_map
            .get(&subtask_id)
            .expect("Expected Task #{subtask_id}");
        self.subtasks_tree.insert(new_subtask_ref);
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
        if self.id == other.id {
            return Ordering::Equal;
        }

        // Compare task status
        if self.status != other.status {
            return self.status.cmp(&other.status);
        }

        // Compare (urgency, importance)
        let dist = self.get_distance();
        let other_dist = other.get_distance();
        if dist > other_dist {
            return Ordering::Less;
        }
        if dist < other_dist {
            return Ordering::Greater;
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
    fn test_add_multiple_task() {
        let mut root = Task::default();
        let task_a = Box::new(Task {
            id: 1,
            ..Default::default()
        });
        let task_b = Box::new(Task {
            id: 2,
            ..Default::default()
        });

        root.add_subtask(task_a.clone());
        root.add_subtask(task_b.clone());

        assert!(root.subtasks_tree.contains(task_a.as_ref()));
        assert!(root.subtasks_tree.contains(task_b.as_ref()));
    }

    #[test]
    fn test_distance_to_max_task_overflow() {
        let task = Task {
            importance: f32::MAX,
            urgency: f32::MAX,
            ..Default::default()
        };
        assert_eq!(task.get_distance(), f32::MAX);
    }

    #[test]
    fn test_distance_to_max_task() {
        let task = Task {
            importance: 4.0,
            urgency: 3.0,
            ..Default::default()
        };

        assert_eq!(task.get_distance(), 5.0);
    }

    #[test]
    fn test_multiple_sorted_importance_urgency() {
        let mut root = Task::default();
        let task_a = Box::new(Task {
            id: 1,
            importance: 4.0,
            urgency: 1.0,
            ..Default::default()
        });
        let task_b = Box::new(Task {
            id: 2,
            importance: 3.0,
            urgency: 2.0,
            ..Default::default()
        });
        let task_c = Box::new(Task {
            id: 3,
            importance: 2.0,
            urgency: 3.0,
            ..Default::default()
        });
        let task_d = Box::new(Task {
            id: 4,
            importance: 1.0,
            urgency: 4.0,
            ..Default::default()
        });

        root.add_subtask(task_a);
        root.add_subtask(task_b);
        root.add_subtask(task_c);
        root.add_subtask(task_d);

        let mut task_itr = root.subtasks_tree.into_iter();
        assert_eq!(task_itr.next().expect("Expected Task A").id, 1);
        assert_eq!(task_itr.next().expect("Expected Task D").id, 4);
        assert_eq!(task_itr.next().expect("Expected Task B").id, 2);
        assert_eq!(task_itr.next().expect("Expected Task C").id, 3);
        assert_eq!(None, task_itr.next());
    }

    #[test]
    fn test_same_importance_different_complexity_sort() {
        let mut root = Task::default();
        let mut task_b = Box::new(Task {
            id: 1,
            ..Default::default()
        });
        let subtask_a = Box::new(Task::default());
        let subtask_b = Box::new(Task {
            id: 2,
            ..Default::default()
        });
        let task_a = Box::new(Task::default());
        let task_c = Box::new(Task {
            id: 2,
            ..Default::default()
        });

        task_b.add_subtask(subtask_a);
        task_b.add_subtask(subtask_b);

        root.add_subtask(task_a);
        root.add_subtask(task_b);
        root.add_subtask(task_c);

        let mut task_itr = root.subtasks_tree.into_iter();
        assert_eq!(task_itr.next().expect("Expected Task A").id, 0);
        assert_eq!(task_itr.next().expect("Expected Task C").id, 2);
        assert_eq!(task_itr.next().expect("Expected Task B").id, 1);
        assert_eq!(None, task_itr.next());
    }

    #[test]
    fn test_different_status_sort() {
        let mut root = Task::default();
        let task_a = Box::new(Task {
            id: 4,
            ..Default::default()
        });
        let task_b = Box::new(Task {
            id: 3,
            status: TaskStatus::Done,
            ..Default::default()
        });
        let task_c = Box::new(Task {
            id: 2,
            status: TaskStatus::Blocked,
            ..Default::default()
        });
        let task_d = Box::new(Task {
            id: 1,
            status: TaskStatus::Archived,
            ..Default::default()
        });

        root.add_subtask(task_a);
        root.add_subtask(task_b);
        root.add_subtask(task_c);
        root.add_subtask(task_d);

        let mut itr = root.subtasks_tree.into_iter();
        assert_eq!(itr.next().expect("Expected Task A (Open)").id, 4);
        assert_eq!(itr.next().expect("Expected Task C (Blocked)").id, 2);
        assert_eq!(itr.next().expect("Expected Task D (Archived)").id, 1);
        assert_eq!(itr.next().expect("Expected Task B (Done)").id, 3);
        assert_eq!(None, itr.next(), "Expected None");
    }

    #[test]
    fn test_reinsert_into_correct_position_after_update() {
        let mut root = Task::default();
        let task_a = Box::new(Task::default());
        let task_b = Box::new(Task {
            id: 1,
            ..Default::default()
        });
        let task_c = Box::new(Task {
            id: 2,
            ..Default::default()
        });

        let updated_task_b = Box::new(Task {
            id: 1,
            importance: 10.0,
            urgency: 10.0,
            ..Default::default()
        });

        root.add_subtask(task_a);
        root.add_subtask(task_b);
        root.add_subtask(task_c);
        root.add_subtask(updated_task_b);

        let mut itr = root.subtasks_tree.into_iter();
        assert_eq!(itr.next().expect("Expected Task B").id, 1);
        assert_eq!(itr.next().expect("Expected Task A").id, 0);
        assert_eq!(itr.next().expect("Expected Task C").id, 2);
        assert_eq!(None, itr.next(), "Expected None");
    }

    #[test]
    fn test_add_same_id_subtask_updates_it() {
        let mut task = Task::default();
        let subtask = Box::new(Task::default());
        let other_subtask = Box::new(Task {
            importance: 42.0,
            ..Default::default()
        });

        task.add_subtask(subtask);
        task.add_subtask(other_subtask.clone());

        let mut itr = task.subtasks_tree.into_iter();
        let retrieved_subtask = itr.next();
        assert_eq!(
            retrieved_subtask
                .expect("expected task with id = 0")
                .importance,
            42.0
        );
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

        assert!(!task_a.subtasks_tree.contains(&subtask), "Subtask found in Task A");
        assert!(task_b.subtasks_tree.contains(&subtask), "Subtask not found in Task B");
    }
    */

    #[test]
    fn test_get_task_complexity_single_level() {
        let mut task = Task::default();
        let subtask_a = Box::new(Task {
            id: 1,
            ..Default::default()
        });
        let subtask_b = Box::new(Task {
            id: 2,
            ..Default::default()
        });
        let subtask_c = Box::new(Task {
            id: 3,
            ..Default::default()
        });

        task.add_subtask(subtask_a);
        task.add_subtask(subtask_b);
        task.add_subtask(subtask_c);

        assert_eq!(task.get_complexity(), 3);
    }

    #[test]
    fn test_get_task_complexity_multilevel() {
        let mut task = Task::default();
        // Level 1
        let subtask_a = Box::new(Task {
            id: 1,
            ..Default::default()
        });
        let mut subtask_b = Box::new(Task {
            id: 2,
            ..Default::default()
        });
        // Level 2
        let subtask_c = Box::new(Task {
            id: 3,
            ..Default::default()
        });
        let mut subtask_d = Box::new(Task {
            id: 4,
            ..Default::default()
        });
        // Level 3
        let subtask_e = Box::new(Task {
            id: 3,
            ..Default::default()
        });

        subtask_d.add_subtask(subtask_e);

        subtask_b.add_subtask(subtask_d);
        subtask_b.add_subtask(subtask_c);

        task.add_subtask(subtask_b);
        task.add_subtask(subtask_a);

        assert_eq!(task.get_complexity(), 3);
    }

    #[test]
    fn test_get_task_complexity_multilevel_single_leaf() {
        let mut task = Task::default();
        // Level 1
        let mut subtask_a = Box::new(Task {
            id: 1,
            ..Default::default()
        });
        // Level 2
        let mut subtask_b = Box::new(Task {
            id: 2,
            ..Default::default()
        });
        // Level 3
        let mut subtask_c = Box::new(Task {
            id: 3,
            ..Default::default()
        });
        // Level 4
        let mut subtask_d = Box::new(Task {
            id: 4,
            ..Default::default()
        });
        // Level 5
        let subtask_e = Box::new(Task {
            id: 3,
            ..Default::default()
        });

        subtask_d.add_subtask(subtask_e);
        subtask_c.add_subtask(subtask_d);
        subtask_b.add_subtask(subtask_c);
        subtask_a.add_subtask(subtask_b);
        task.add_subtask(subtask_a);

        assert_eq!(task.get_complexity(), 1);
    }
}
