use std::io::{self, Write};

fn main() {
    loop {
        // input two numbers
        let num1 = match input("Enter first number: ").trim().parse() {
            Ok(num) => num,
            Err(_) => { 
                println!("Invalid number!");
                continue
            }
        };

        let num2 = match input("Enter second number: ").trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number!");
                continue;
            }
        };

        // input operator
        let op: char = match input("Enter operator: ").trim().chars().next() {
            Some(o) => o,
            None => {
                println!("Invalid operator!");
                continue
            }
        };

        let result: Option<i32> = calculate(num1, num2, op);
        match result {
            Some(value) => println!("Result: {}", value),
            None => println!("Invalid operation"),
        }

        match input("Continue (y/n)? ").trim().chars().next() {
            Some('y') => continue,
            _ => break
        }
    }
}

fn calculate(num1: i32, num2: i32, op: char) -> Option<i32> {
    match op {
        '+' => Some(num1 + num2),
        '-' => Some(num1 - num2),
        '*' => Some(num1 * num2),
        '/' if num2 != 0 => Some(num1 / num2),
        _ => None,
    }
}

fn input(message: &str) -> String {
    let mut buffer = String::new();
    print!("{}", message);
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut buffer).unwrap();
    buffer
}
