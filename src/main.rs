use crate::ihm::TaskManager;
use crate::task::Tasks;

mod task;
mod ihm;


fn main() {
    let tasks: Tasks = Vec::new();

    TaskManager::new(tasks).run();
}
