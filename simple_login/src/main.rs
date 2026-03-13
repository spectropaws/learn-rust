use rpassword;
use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let mut creds: HashMap<String, String> = HashMap::new();
    creds.insert("johnsmith".to_string(), "password123".to_string());
    creds.insert("anna".to_string(), "@password".to_string());
    creds.insert("naruto".to_string(), "hinata123".to_string());

    println!("Please login with your credentials!");

    let username = match input_msg("username: ") {
        Ok(u) => u,
        Err(_) => {
            println!("Error reading input!");
            return;
        }
    };

    // Check if username exists and store password
    let stored_pass: &String = match creds.get(&username) {
        Some(s) => s,
        None => {
            println!("Username does not exist!");
            return;
        }
    };

    let password = match rpassword::prompt_password("password: ") {
        Ok(p) => p,
        Err(e) => {
            println!("Error reading password! E: {}", e);
            return;
        }
    };

    // check password
    if password == *stored_pass {
        println!("Login successful! Welcome {}.", username);
    } else {
        println!("Invalid password!");
    }
}

fn input_msg(message: &str) -> Result<String, io::Error> {
    let mut input = String::new();
    print!("{}", message);
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
