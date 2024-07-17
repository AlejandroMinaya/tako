CREATE TYPE task_status AS ENUM ('OPEN', 'BLOCKED', 'ARCHIVED', 'DONE');
CREATE TABLE tasks (
    id              SERIAL PRIMARY KEY,                     -- Unique Identifier
    title           VARCHAR(80) NOT NULL ,                  -- Task Title
    importance      SERIAL,                                 -- Importance Score
    urgency         SERIAL,                                 -- Urgency Score
    status          task_status NOT NULL DEFAULT 'OPEN',    -- Task Status
    parent_task_id  iNT REFERENCES tasks(id)                -- Parent Task Reference (if needed)
);
