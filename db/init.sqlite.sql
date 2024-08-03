CREATE TABLE IF NOT EXISTS tasks (
    id INTEGER PRIMARY KEY,
    importance FLOAT NOT NULL,
    urgency FLOAT NOT NULL,
    status INTEGER NOT NULL,
    parent_task_id INTEGER NULL,
    FOREIGN KEY (parent_task_id)
        REFERENCES tasks(id)
);
