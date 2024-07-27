use crate::core::ports::*;
use crate::core::tasks::*;
use async_trait::async_trait;
use sqlx::{
    query_as,
    sqlite::{
        Sqlite,
        SqlitePoolOptions,
        SqliteRow,
        SqliteTypeInfo,
    },
    Database,
    Type,
    Decode,
    FromRow,
    Row,
    Error
};
use std::vec::IntoIter;

#[derive(Debug)]
struct SQLiteStore {
    conn: String,
}

impl SQLiteStore {
    fn new(conn: String) -> Self {
        SQLiteStore { conn }
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
    ) -> Result<TaskStatus, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let raw_value = <i32 as Decode<DB>>::decode(value)?;
        Ok(raw_value.into())
    }
}

impl<'r> Type<Sqlite> for TaskStatus {
    fn type_info() -> SqliteTypeInfo {
        <i32 as Type<Sqlite>>::type_info()
    }
}

#[async_trait]
impl DataStore for SQLiteStore {
    async fn read(&self) -> anyhow::Result<IntoIter<Box<Task>>> {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(&self.conn)
            .await?;
        let tasks: Vec<Box<Task>> = query_as("SELECT * FROM tasks;")
            .fetch_all(&pool)
            .await?
            .into_iter()
            .map(|raw_task| Box::new(raw_task))
            .collect();

        println!("DB Tasks: {:?}", tasks);
        Ok(tasks.into_iter())
    }

    async fn write(&self, task_itr: IntoIter<&Task>) -> bool {
        todo!();
    }
}
