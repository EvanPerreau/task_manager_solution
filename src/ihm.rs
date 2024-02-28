use crate::task::{Task, TaskImportance, Tasks, TaskStatus};

pub struct TaskManager {
    tasks: Tasks,
    current_choice: String,
}

impl TaskManager {
    pub fn new(tasks: Tasks) -> TaskManager {
        TaskManager { tasks: tasks, current_choice: String::new() }
    }

    fn get_input(&self) -> String {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        return input.trim().to_string().to_lowercase().replace(" ", "");
    }

    fn display_menu(&self) {
        println!("| TASK MANAGER |");
        println!();
        println!("1. List tasks");
        println!("2. Add a task");
        println!("3. Remove a task");
        println!("4. Modify a task");
        println!("5. Exit");
        println!();
    }

    fn display_edit_menu(&self) {
        println!("1. Change the title");
        println!("2. Change the description");
        println!("3. Change the status");
        println!("4. Change the importance");
        println!("5. Go back");
        println!();
    }

    fn modify_task(&mut self) {
        self.display_tasks();
        println!("Please enter the index of the task you want to modify :");
        let index = self.get_input().parse::<usize>().unwrap();
        println!();

        if index < self.tasks.len() {
            self.display_edit_menu();
            let choice = self.get_input();
            match choice.as_str() {
                "1" => {
                    println!("Please enter the new title of the task:");
                    let title = self.get_input();
                    self.tasks[index].title = title;
                    println!();
                    println!("Task modified")
                }
                "2" => {
                    println!("Please enter the new description of the task:");
                    let description = self.get_input();
                    self.tasks[index].description = description;
                    println!();
                    println!("Task modified")
                }
                "3" => {
                    println!("Please enter the new status of the task (Todo, InProgress, Done):");
                    let status = match self.get_input().as_str() {
                        "todo" => TaskStatus::Todo,
                        "inprogress" => TaskStatus::InProgress,
                        "done" => TaskStatus::Done,
                        _ => TaskStatus::Todo,
                    };
                    self.tasks[index].status = status;
                    println!();
                    println!("Task modified")
                }
                "4" => {
                    println!("Please enter the new importance of the task (Low, Medium, High):");
                    let importance = match self.get_input().as_str() {
                        "low" => TaskImportance::Low,
                        "medium" => TaskImportance::Medium,
                        "high" => TaskImportance::High,
                        _ => TaskImportance::Low
                    };
                    self.tasks[index].importance = importance;
                    println!();
                    println!("Task modified")
                }
                "5" => return,
                _ => println!("Invalid choice"),
            }
        } else {
            println!("Invalid index");
        }
        println!();
    }

    fn display_tasks(&self) {
        println!("Tasks list:");
        println!();

        for (index, task) in self.tasks.iter().enumerate() {
            println!("{} - {}", index, task.title);
            println!("    Description: {}", task.description);
            println!("    Status: {}", task.status);
            println!("    Importance: {}", task.importance);
        }
        println!();
    }

    fn add_task(&mut self) {

        println!("Please enter the title of the task:");
        let title = self.get_input();
        println!();

        println!("Please enter the description of the task:");
        let description = self.get_input();
        println!();

        println!("Please enter the status of the task (Todo, InProgress, Done):");
        let status = match self.get_input().as_str() {
            "todo" => TaskStatus::Todo,
            "inprogress" => TaskStatus::InProgress,
            "done" => TaskStatus::Done,
            _ => TaskStatus::Todo,
        };
        println!();

        println!("Please enter the importance of the task (Low, Medium, High):");
        let importance = match self.get_input().as_str() {
            "low" => TaskImportance::Low,
            "medium" => TaskImportance::Medium,
            "high" => TaskImportance::High,
            _ => TaskImportance::Low
        };
        println!();

        self.tasks.push(Task { title, description, status, importance });
        println!("Task added");
        println!();
    }

    fn remove_task(&mut self) {
        self.display_tasks();
        println!();

        println!("Please enter the index of the task you want to remove :");
        let index = self.get_input().parse::<usize>().unwrap();
        if index < self.tasks.len() {
            self.tasks.remove(index);
            println!("Task {index} removed")
        } else {
            println!("Invalid index");
        }
        println!();
    }

    pub fn run(&mut self) {
        loop {
            self.display_menu();
            self.current_choice = self.get_input();
            match self.current_choice.as_str() {
                "1" => self.display_tasks(),
                "2" => self.add_task(),
                "3" => self.remove_task(),
                "4" => self.modify_task(),
                "5" => {
                    println!("Goodbye !");
                    break;
                },
                _ => println!("Invalid choice"),
            }
        }
    }
}