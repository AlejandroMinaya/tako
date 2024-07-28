pub mod ports {
    use async_trait::async_trait;
    use std::vec::IntoIter;
    use super::Task;
    use std::fmt::Debug;

    #[async_trait]
    pub trait DataStore: Debug {
        async fn write(&self, task_itr: IntoIter<&Task>) -> bool;
        async fn read(&self) -> anyhow::Result<IntoIter<Box<Task>>>;
    }

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
            Ok(test_tasks.into_iter())
        }
    }
}


use std::cmp::Ordering;
use std::collections::HashMap;

/* TASK STATUS ============================================================= */
#[derive(
    Debug,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
)]
#[repr(u8)]
pub enum TaskStatus {
    #[default]
    Open = 1,
    Blocked = 253,
    Archived = 254,
    Done = 255,
}
impl Into<TaskStatus> for i32 {
    fn into(self) -> TaskStatus {
        match self {
            253 => TaskStatus::Blocked,
            254 => TaskStatus::Archived,
            255 => TaskStatus::Done,
            _ => TaskStatus::Open,

        }
    }
}

/* TASK ==================================================================== */
#[derive(Debug, Default, Clone)]
pub struct Task {
    id: u32,
    importance: f32,
    urgency: f32,
    status: TaskStatus,
    subtasks_map: HashMap<u32, Box<Self>>,
}

impl Task {
    pub fn new(id: u32, importance: f32, urgency: f32, status: TaskStatus) -> Self {
        Task {
            id,
            importance,
            urgency,
            status,
            subtasks_map: HashMap::new()
        }
    }
    pub fn new_with_id(id: u32) -> Self {
        Task {
            id,
            ..Default::default()
        }
    }
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
        if self.subtasks_map.is_empty() {
            return 1;
        };

        let sub_itr = self.subtasks_map.values();
        return 1 + sub_itr.fold(0_u32, |result, subtask| result + subtask.get_complexity());
    }

    pub fn add_subtask(&mut self, subtask: Box<Self>) {
        self.subtasks_map.insert(subtask.id, subtask);
    }

    pub fn get_subtasks(&self) -> Vec<&Self> {
        // TODO (maybe): Cache vector and only sort after insertion/deletion to the map, instead of each time
        let mut collected_subtasks: Vec<&Self> = self
            .subtasks_map
            .values()
            .map(|boxed_task| boxed_task.as_ref())
            .collect();
        collected_subtasks.sort();
        return collected_subtasks;
    }
    pub fn get_all_subtasks(&self) -> Vec<&Self> {
        let mut all_subtasks: Vec<&Self> = vec![];
        if self.subtasks_map.len() == 0 {
            return all_subtasks;
        }
        self.subtasks_map.values().for_each(|subtask| {
            let mut microtasks = subtask.get_all_subtasks();
            all_subtasks.append(&mut microtasks);
        });
        self.subtasks_map.values().for_each(|subtask| {
            all_subtasks.push(&subtask);
        });
        all_subtasks.sort();
        return all_subtasks
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
impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

#[cfg(test)]
mod task_tests {
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

        assert!(root.subtasks_map.contains_key(&1));
        assert!(root.subtasks_map.contains_key(&2));
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

        let mut task_itr = root.get_subtasks().into_iter();
        assert_eq!(task_itr.next().expect("Expected Task A").id, 1);
        assert_eq!(task_itr.next().expect("Expected Task D").id, 4);
        assert_eq!(task_itr.next().expect("Expected Task B").id, 2);
        assert_eq!(task_itr.next().expect("Expected Task C").id, 3);
        assert_eq!(None, task_itr.next());
    }

    #[test]
    fn test_same_importance_different_complexity_sort() {
        let mut root = Task::default();
        let mut task_b = Box::new(Task::new_with_id(2));
        let subtask_a = Box::new(Task::new_with_id(4));
        let subtask_b = Box::new(Task::new_with_id(5));
        let task_a = Box::new(Task::new_with_id(1));
        let task_c = Box::new(Task::new_with_id(3));

        task_b.add_subtask(subtask_a);
        task_b.add_subtask(subtask_b);

        root.add_subtask(task_a);
        root.add_subtask(task_b);
        root.add_subtask(task_c);

        let mut task_itr = root.get_subtasks().into_iter();
        assert_eq!(task_itr.next().expect("Expected Task A").id, 1);
        assert_eq!(task_itr.next().expect("Expected Task C").id, 3);
        assert_eq!(task_itr.next().expect("Expected Task B").id, 2);
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

        let mut itr = root.get_subtasks().into_iter();
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

        let mut itr = root.get_subtasks().into_iter();
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

        let mut itr = task.get_subtasks().into_iter();
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

        assert!(!task_a.get_subtasks().contains(&subtask), "Subtask found in Task A");
        assert!(task_b.get_subtasks().contains(&subtask), "Subtask not found in Task B");
    }
    */

    #[test]
    fn test_get_task_complexity_single_level() {
        /*
         *           (t)
         *          / | \
         *      (sA)(sB)(sC)
         */
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

        assert_eq!(task.get_complexity(), 4);
    }

    #[test]
    fn test_get_task_complexity_multilevel() {
        /*
         *           (t)
         *          /   \
         *       (sA)   (sB)
         *             /   \
         *           (sC) (sD)
         *                 |
         *                (sE)
         */
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

        assert_eq!(task.get_complexity(), 6);
    }

    #[test]
    fn test_get_task_complexity_multilevel_single_leaf() {
        /*
         * (t) - (sA) - (sB) - (sC) - (sD) - (sE)
         */
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

        assert_eq!(task.get_complexity(), 6);
    }

    #[test]
    fn test_collect_all_tasks() {
        /*      
         *            (r)
         *           /   \
         *       (tA)    (tB)
         *        |      /  \
         *      (sB)   (sA)(sC)
         *      /  \
         *    (mA)(mB)
         */
        let mut root = Box::new(Task::default());
        
        let mut task_a = Box::new(Task::new_with_id(1));
        let mut task_b = Box::new(Task::new_with_id(2));

        let subtasks_a = Box::new(Task::new_with_id(3));
        let mut subtasks_b = Box::new(Task::new_with_id(4));
        let subtasks_c = Box::new(Task::new_with_id(5));

        let microtask_a = Box::new(Task::new_with_id(6));
        let microtask_b = Box::new(Task::new_with_id(7));

        subtasks_b.add_subtask(microtask_a);
        subtasks_b.add_subtask(microtask_b);

        task_a.add_subtask(subtasks_b);

        task_b.add_subtask(subtasks_a);
        task_b.add_subtask(subtasks_c);

        root.add_subtask(task_a);
        root.add_subtask(task_b);

        let mut itr = root.get_all_subtasks().into_iter();
        assert_eq!(itr.next().expect("Expected Task #3").id, 3);
        assert_eq!(itr.next().expect("Expected Task #5").id, 5);
        assert_eq!(itr.next().expect("Expected Task #6").id, 6);
        assert_eq!(itr.next().expect("Expected Task #7").id, 7);
        assert_eq!(itr.next().expect("Expected Task #2").id, 2);
        assert_eq!(itr.next().expect("Expected Task #4").id, 4);
        assert_eq!(itr.next().expect("Expected Task #1").id, 1);
        assert_eq!(itr.next(), None);

    }
}

/* OSWALD (TASK SERVICE) =================================================== */
// https://www.imdb.com/title/tt0293734/
#[derive(Debug)]
pub struct Oswald {
    data_store: Box<dyn ports::DataStore>,
    root: Task,
}
impl Oswald {
    pub fn new(data_store: Box<dyn ports::DataStore>) -> Self {
        Oswald {
            data_store,
            root: Task::default(),
        }
    }
    pub fn add_task(&mut self, task: Box<Task>) {
        self.root.add_subtask(task)
    }

    pub fn get_all_tasks(&self) -> Vec<&Task> {
        self.root.get_all_subtasks()
    }

    async fn load(&mut self) -> anyhow::Result<()> {
        self.data_store
            .read()
            .await?
            .for_each(|boxed_task| self.root.add_subtask(boxed_task));
        Ok(())
    }
}

/* TESTS =================================================================== */
#[cfg(test)]
mod oswald_tests {
    use super::{
        Oswald,
        Task
    };
    use super::ports::MockDataStore;

    #[sqlx::test]
    async fn test_load_all_tasks_from_data_store() {
        let mut oswald = Oswald::new(Box::new(MockDataStore::default()));

        let _ = oswald.load().await;

        assert!(oswald.root.subtasks_map.contains_key(&0));
        assert!(oswald.root.subtasks_map.contains_key(&1));
        assert!(oswald.root.subtasks_map.contains_key(&2));

        assert!(oswald
            .root
            .subtasks_map
            .get(&0)
            .unwrap()
            .subtasks_map
            .contains_key(&3));
        assert!(oswald
            .root
            .subtasks_map
            .get(&2)
            .unwrap()
            .subtasks_map
            .contains_key(&4));
        assert!(oswald
            .root
            .subtasks_map
            .get(&2)
            .unwrap()
            .subtasks_map
            .contains_key(&5));
    }

    #[test]
    fn test_add_task() {
        let mut oswald = Oswald::new(Box::new(MockDataStore::default()));
        let task = Box::new(Task::new_with_id(1));

        oswald.add_task(task);

        assert!(oswald.root.subtasks_map.contains_key(&1));
    }

    #[sqlx::test]
    async fn test_get_loaded_tasks() {
        let mut oswald = Oswald::new(Box::new(MockDataStore::default()));

        assert!(oswald.load().await.is_ok(), "Expected MockDataStore to load");

        let mut itr = oswald.get_all_tasks().into_iter();

        assert_eq!(itr.next().expect("Expected  Task #1").id, 1);
        assert_eq!(itr.next().expect("Expected  Task #3").id, 3);
        assert_eq!(itr.next().expect("Expected  Task #4").id, 4);
        assert_eq!(itr.next().expect("Expected  Task #5").id, 5);
        assert_eq!(itr.next().expect("Expected  Task #0").id, 0);
        assert_eq!(itr.next().expect("Expected  Task #2").id, 2);
        assert_eq!(itr.next(), None);
    }
}

/* ========================================================================= */
