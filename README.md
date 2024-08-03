# tako 🐙
_Life Management System_

> **TL;DR:** A system to automatically sort tasks based on their importance, urgency, and complexity. 

## How does it work?
The idea is based on Eisenhower's Matrix where you have two axes to represent importance & urgency. By arranging tasks relative to one another on this matrix, you can sort them based on the most relevant task you should be doing next.

## How do you manage tasks with different complexities?
This will be different for everybody in the same way that importance and urgency are not the same. The idea is that you can add subtasks to a task and based on the amount of subtasks, you can get a complexity value. After using it you should get better at breaking down tasks into their most basic components.

## How do I run it?
[PENDING]


## Roadmap
### Core
#### Task
- ✅ Add/update task
- ✅ Add/update nested task
- ✅ Add/update nested task using list
- ✅ Get task complexity
- ✅ Get task importance & urgency rank, a.k.a distance
- ✅ Get top-level subtasks
- ✅ Get all subtasks
- ✅ Get subtask parent
- ⬛ Delete task
- ⬛ Delete subtask
#### Oswald (manager)
- ✅ Add/update task
- ✅ Get top-level subtasks
- ✅ Get all subtasks
- ✅ Load data from datastore
- ✅ Save data to datastore
### Ports
#### SQLite
- ✅ Read data
- ✅ Write data
### Clients/Services
#### API (axum)
- ✅ Start service
- ✅ Get all tasks
- ✅ Add/update task

### Pending Decisions
- How to save/load the data from **any** store into using **any** client/service.
