use std::io::{self, Write};

struct Task {
    name: String,
    complete: bool,
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        show_menu();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(_) => {
                println!("Error reading input!");
                continue;
            }
        }

        let choice: i32 = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid Input!");
                continue;
            }
        };

        match choice {
            1 => show_todo(&tasks),
            2 => add_task(&mut tasks),
            3 => mark_as_done(&mut tasks),
            4 => delete_task(&mut tasks),
            5 => break,
            _ => {
                println!("Invalid Input!");
                continue;
            }
        }
    }
}

fn show_todo(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("No tasks added yet!")
    } else {
        println!("\n Tasks:")
    }

    for (i, task) in tasks.iter().enumerate() {
        println!(
            "{}. [{}] {}",
            i + 1,
            if task.complete { "*" } else { " " },
            task.name
        );
    }
    println!("");
}

fn add_task(tasks: &mut Vec<Task>) {
    let task_name = input_with_msg("Enter task: ");
    let task = Task {
        name: task_name,
        complete: false,
    };
    tasks.push(task);
}

fn mark_as_done(tasks: &mut Vec<Task>) {
    let task_id: usize = match input_with_msg("Enter task id: ").parse() {
        Ok(id) => id,
        Err(_) => {
            println!("Invalid task id!");
            return;
        }
    };

    if let Some(task) = tasks.get_mut(task_id - 1) {
        task.complete = true;
    } else {
        println!("Invalid task id!")
    }
}

fn delete_task(tasks: &mut Vec<Task>) {
    let task_id: usize = match input_with_msg("Enter task id: ").parse() {
        Ok(id) => id,
        Err(_) => {
            println!("Invalid task id!");
            return;
        }
    };

    if tasks.get(task_id - 1).is_none() {
        println!("Invalid task id!");
        return;
    }
    tasks.remove(task_id - 1);
}

fn input_with_msg(message: &str) -> String {
    let mut buffer = String::new();
    print!("{}", message);
    io::stdout().flush().unwrap();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => buffer.trim().to_string(),
        Err(_) => {
            println!("Error reading line!");
            String::new()
        }
    }
}

fn show_menu() {
    println!("============= TODO List =============");
    println!("[1] Show TODO List");
    println!("[2] Add task");
    println!("[3] Mark as done");
    println!("[4] Delete a task");
    println!("[5] Quit");
    print!("Choose option: ");
    io::stdout().flush().unwrap();
}
