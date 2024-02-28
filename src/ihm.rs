use crate::task::{Task, TaskImportance, Tasks, TaskStatus};

pub struct TaskManager {
    tasks: Tasks,
    current_choice: String,
}

impl TaskManager {
    pub fn new(tasks: Tasks) -> TaskManager {
        TaskManager { tasks: tasks, current_choice: String::new() }
    }

    fn get_input_with_prompt(&self, prompt: &str) -> String {
        self.print_message(prompt);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        return input.trim().to_string().to_lowercase().replace(" ", "");
    }

    fn print_message(&self, message: &str) {
        println!("{}", message);
        println!();
    }

    fn display_menu(&self) {
        self.print_message("| TASK MANAGER |\n1. List tasks\n2. Add a task\n3. Remove a task\n4. Modify a task\n5. Exit");
    }

    fn display_tasks(&self) {
        self.print_message("Tasks list:");
        for (index, task) in self.tasks.iter().enumerate() {
            self.print_message(&format!("{} - {}\n    Description: {}\n    Status: {}\n    Importance: {}", index, task.title, task.description, task.status, task.importance));
        }
    }

    fn display_edit_menu(&self) {
        self.print_message("1. Change the title\n2. Change the description\n3. Change the status\n4. Change the importance\n5. Go back");
    }

    fn add_task(&mut self) {
        let title = self.get_input_with_prompt("Please enter the title of the task:");
        let description = self.get_input_with_prompt("Please enter the description of the task:");
        let status = TaskStatus::from_str(self.get_input_with_prompt("Please enter the status of the task (Todo, InProgress, Done):").as_str());
        let importance = TaskImportance::from_str(self.get_input_with_prompt("Please enter the importance of the task (Low, Medium, High):").as_str());
        self.tasks.push(Task { title, description, status, importance });
        self.print_message("Task added");
    }

    fn modify_task(&mut self) {
        self.display_tasks();
        let index = self.get_input_with_prompt("Please enter the index of the task you want to modify :").parse::<usize>().unwrap();
        if index < self.tasks.len() {
            self.display_edit_menu();
            let choice = self.get_input_with_prompt("");
            match choice.as_str() {
                "1" => {
                    let title = self.get_input_with_prompt("Please enter the new title of the task:");
                    self.tasks[index].title = title;
                    self.print_message("Task modified");
                }
                "2" => {
                    let description = self.get_input_with_prompt("Please enter the new description of the task:");
                    self.tasks[index].description = description;
                    self.print_message("Task modified");
                }
                "3" => {
                    let status = TaskStatus::from_str(self.get_input_with_prompt("Please enter the new status of the task (Todo, InProgress, Done):").as_str());
                    self.tasks[index].status = status;
                    self.print_message("Task modified");
                }
                "4" => {
                    let importance = TaskImportance::from_str(self.get_input_with_prompt("Please enter the new importance of the task (Low, Medium, High):").as_str());
                    self.tasks[index].importance = importance;
                    self.print_message("Task modified");
                }
                "5" => return,
                _ => self.print_message("Invalid choice"),
            }
        } else {
            self.print_message("Invalid index");
        }
    }

    fn remove_task(&mut self) {
        self.display_tasks();
        let index = self.get_input_with_prompt("Please enter the index of the task you want to remove :").parse::<usize>().unwrap();
        if index < self.tasks.len() {
            self.tasks.remove(index);
            self.print_message(&format!("Task {} removed", index));
        } else {
            self.print_message("Invalid index");
        }
    }

    pub fn run(&mut self) {
        loop {
            self.display_menu();
            self.current_choice = self.get_input_with_prompt("");
            match self.current_choice.as_str() {
                "1" => self.display_tasks(),
                "2" => self.add_task(),
                "3" => self.remove_task(),
                "4" => self.modify_task(),
                "5" => {
                    self.print_message("Goodbye !");
                    break;
                },
                _ => self.print_message("Invalid choice"),
            }
        }
    }
}