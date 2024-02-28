use crate::ihm::TaskManager;
use crate::task::Tasks;

mod task;
mod ihm;


fn main() {
    // possible to use a file to store tasks
    let tasks: Tasks = Vec::new();

    TaskManager::new(tasks).run();
}
