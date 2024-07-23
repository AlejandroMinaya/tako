use crate::core::*;

mod core;

fn main() {
    let hello_task = tasks::Task::default();
    println!("tako task: {:?}", hello_task);
}
