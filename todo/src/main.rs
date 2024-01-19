use std::io;
#[derive(Debug)]  
struct Task {
    id: u32,
    name: String,
    completed: bool
}

fn update_task(task: &mut Task) {
    task.completed = true;
}

fn display_task(task_list: &Vec<Task>) {
    for task in task_list {
        println!("{} - {} ({})", task.id, task.name, task.completed);
    }
}

fn main() {
    let mut task_list: Vec<Task> = Vec::new();

    let task_1: Task = Task {
        id: 1,
        name: String::from("Buy Coffee"),
        completed: false
    };
    
    let task_2: Task = Task {
        id: 2,
        name: String::from("Buy Gums"),
        completed: false
    };

    task_list.push(task_1);
    task_list.push(task_2);

    println!("{:?}", task_list);

    loop {
        println!("What would you like to do?");
        println!("1. Add a task");
        println!("2. Complete a task");
        println!("3. Display tasks");
        println!("4. Quit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        let choice = choice.trim().parse::<u32>().expect("Invalid input");

        match choice {
            1 => {
                println!("Enter the name of the task:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read line");
                let name = name.trim().to_string();

                let id = task_list.len() as u32 + 1;

                let task = Task {
                    id,
                    name,
                    completed: false,
                };

                task_list.push(task);
            },
            2 => {
                println!("Enter the ID of the task you want to complete:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id = id.trim().parse::<u32>().expect("Invalid input");

                if let Some(task) = task_list.iter_mut().find(|t| t.id == id) {
                    update_task(task);
                } else {
                    println!("Task with ID {} not found", id);
                }
            },
            3 => {
                display_task(&task_list);
            },
            4 => {
                println!("Goodbye!");
                break;
            },
            _ => {
                println!("Invalid choice");
            },
        }
    }
}
