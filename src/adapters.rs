use crate::core::tasks::{
    Task,
    TaskStatus,
    BoxTaskVec,
    ports::DataStore
};
use async_trait::async_trait;
use sqlx::{
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
use std::vec::IntoIter;


#[derive(Debug)]
pub struct SQLiteStore {
    conn: String,
}

impl SQLiteStore {
    pub fn new(conn: String) -> Self {
        SQLiteStore { conn }
    }
    async fn fill_subtasks(&self, task: &Task, pool: &SqlitePool) -> anyhow::Result<BoxTaskVec> {
        let raw_subtasks: Vec<Task> = 
            query_as("SELECT * FROM tasks WHERE parent_task_id = ?;")
            .bind(task.get_id())
            .fetch_all(pool)
            .await?;
        let mut results: BoxTaskVec = vec![];
        for mut raw_subtask in raw_subtasks.into_iter() {
            let microtasks = Box::pin(self.fill_subtasks(&raw_subtask, pool)).await?;
            raw_subtask.add_subtasks_vec(microtasks);
            results.push(Box::new(raw_subtask));
        }
        Ok(results)
    }
    async fn read_orphans(&self, pool: &SqlitePool) -> anyhow::Result<BoxTaskVec> {
        let orphans: Vec<Box<Task>> = query_as("SELECT * FROM tasks WHERE parent_task_id ISNULL;")
            .fetch_all(pool)
            .await?
            .into_iter()
            .map(|raw_task| Box::new(raw_task))
            .collect();
        Ok(orphans)
    }

}
impl<'r> FromRow<'r, SqliteRow> for Task {
    fn from_row(row: &'r SqliteRow) -> Result<Self, Error> {
        let task = Task::new(
            row.try_get("id")?,
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
            let subtasks = Box::pin(self.fill_subtasks(&orphan, &pool)).await?;
            orphan.add_subtasks_vec(subtasks);
            loaded_orphans.push(orphan);
        }
        Ok(loaded_orphans)
    }

    async fn write(&self, task_itr: IntoIter<&Task>) -> bool {
        todo!();
    }
}
