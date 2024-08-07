use crate::core::tasks::{
    Task,
    TaskStatus,
    BoxTaskVec,
    
};
use crate::ports::DataStore;
use async_trait::async_trait;
use sqlx::{
    query,
    query_as,
    sqlite::{
        Sqlite,
        SqlitePoolOptions,
        SqlitePool,
        SqliteRow,
        SqliteTypeInfo,
        SqliteArgumentValue
    },
    encode::IsNull,
    TypeInfo,
    Database,
    Type,
    Decode,
    Encode,
    FromRow,
    Row,
    error::BoxDynError,
    Error
};
use async_recursion::async_recursion;


#[derive(Debug)]
pub struct SQLiteStore {
    conn: String,
}

impl SQLiteStore {
    pub fn new(conn: String) -> Self {
        SQLiteStore { conn }
    }
    #[async_recursion]
    async fn fill_subtasks<'a>(&'a self, task: &'a Task, pool: &'a SqlitePool) -> BoxTaskVec {
        let mut results: BoxTaskVec = vec![];
        let raw_subtasks_query = query_as("SELECT * FROM tasks WHERE parent_task_id = ?;")
            .bind(task.id)
            .fetch_all(pool);
        match raw_subtasks_query.await {
            Ok(raw_subtasks) => {
                for mut raw_subtask in raw_subtasks.into_iter() {
                    let microtasks = Box::pin(self.fill_subtasks(&raw_subtask, pool)).await;
                    raw_subtask.add_subtasks_vec(microtasks);
                    results.push(Box::new(raw_subtask));
                }
            },
            Err(_) => { println!("Couldn't retrieve subtasks for task #{}", task.id) }
        };
        results
    }
    async fn read_orphans(&self, pool: &SqlitePool) -> anyhow::Result<BoxTaskVec> {
        let orphans: Vec<Box<Task>> = query_as("SELECT * FROM tasks WHERE parent_task_id ISNULL;")
            .fetch_all(pool)
            .await?
            .into_iter()
            .map(Box::new)
            .collect();
        Ok(orphans)
    }
    #[async_recursion]
    async fn write_tasks_helper(&self, pool: &SqlitePool, tasks: Vec<&Task>, parent_id: Option<u32>) -> anyhow::Result<()> {
        for task in tasks {
            dbg!(task);
            let _ = query("REPLACE INTO tasks VALUES (?,?,?,?,?);")
                .bind(task.id)
                .bind(task.importance)
                .bind(task.urgency)
                .bind(task.status as u8)
                .bind(parent_id)
                .execute(pool).await;
            let _ = self.write_tasks_helper(pool, task.get_subtasks(), Some(task.id)).await;
        }
        Ok(())
    }

}
impl<'r> FromRow<'r, SqliteRow> for Task {
    fn from_row(row: &'r SqliteRow) -> Result<Self, Error> {
        let task = Task::new(
            row.try_get("id")?,
            row.try_get("desc")?,
            row.try_get("importance")?,
            row.try_get("urgency")?,
            row.try_get("status")?,
        );
        Ok(task)
    }
}

impl<'r, DB: Database> Decode<'r, DB> for TaskStatus
where
    i32: Decode<'r, DB>,
{
    fn decode(
        value: <DB as Database>::ValueRef<'r>,
    ) -> Result<TaskStatus, BoxDynError> {
        let raw_value = <i32 as Decode<DB>>::decode(value)?;
        Ok(raw_value.into())
    }
}
impl<'q> Encode<'q, Sqlite> for TaskStatus {
    fn encode_by_ref(
        &self,
        args: &mut Vec<SqliteArgumentValue<'q>>
    ) -> Result<IsNull, BoxDynError>{
        args.push(SqliteArgumentValue::Int(*self as i32));

        Ok(IsNull::No)
    }
}

impl<'r> Type<Sqlite> for TaskStatus {
    fn type_info() -> SqliteTypeInfo {
        <i32 as Type<Sqlite>>::type_info()
    }
    fn compatible(ty: &SqliteTypeInfo) -> bool {
        ty.name() == "INTEGER"
    }
}

const MAX_CONNECTIONS: u32 = 5;

#[async_trait]
impl DataStore for SQLiteStore {
    async fn read(&self) -> anyhow::Result<BoxTaskVec> {
        let pool = SqlitePoolOptions::new()
            .max_connections(MAX_CONNECTIONS)
            .connect(&self.conn)
            .await?;

        let mut loaded_orphans: BoxTaskVec = vec![];

        let orphans = self.read_orphans(&pool).await?;
        for mut orphan in orphans.into_iter() {
            let subtasks = Box::pin(self.fill_subtasks(&orphan, &pool)).await;
            orphan.add_subtasks_vec(subtasks);
            loaded_orphans.push(orphan);
        }
        Ok(loaded_orphans)
    }

    async fn write(&self, tasks: Vec<&Task>) -> anyhow::Result<()> {
        let pool = SqlitePoolOptions::new()
            .max_connections(MAX_CONNECTIONS)
            .connect(&self.conn)
            .await?;
            let _ = self.write_tasks_helper(&pool, tasks, None).await;

        Ok(())
    }
}
