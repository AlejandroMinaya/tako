use crate::core::tasks::{
    Task,
    TaskStatus,
    ports::DataStore
};
use async_trait::async_trait;
use sqlx::{
    query_as,
    sqlite::{
        Sqlite,
        SqlitePoolOptions,
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
        Ok(tasks.into_iter())
    }

    async fn write(&self, task_itr: IntoIter<&Task>) -> bool {
        todo!();
    }
}
