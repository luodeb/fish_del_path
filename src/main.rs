use std::process::Command;
use std::io::{self, Write};

fn main() {
    // 读取 fish_user_paths 环境变量
    let output = Command::new("fish")
        .args(&["-c", "echo $fish_user_paths"])
        .output()
        .expect("Failed to execute fish command");

    let paths_str = String::from_utf8_lossy(&output.stdout);
    let paths: Vec<&str> = paths_str.trim().split_whitespace().collect();

    if paths.is_empty() || (paths.len() == 1 && paths[0].is_empty()) {
        println!("fish_user_paths is empty");
        return;
    }

    // Display path list (numbers start from 1)
    println!("Paths in fish_user_paths:");
    for (index, path) in paths.iter().enumerate() {
        println!("{}: {}", index + 1, path);
    }

    // Wait for user input
    print!("\nEnter the path number to delete (1-{}): ", paths.len());
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Parse the input number
    let index: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, please enter a number");
            return;
        }
    };

    // Validate the number (numbers start from 1)
    if index < 1 || index > paths.len() {
        println!("Invalid number, please enter a number between 1 and {}", paths.len());
        return;
    }

    // Execute delete command using fish (fish array index starts from 1)
    let delete_cmd = format!("set -e fish_user_paths[{}]", index);
    println!("\nExecuting command: {}", delete_cmd);

    let result = Command::new("fish")
        .args(&["-c", &delete_cmd])
        .status()
        .expect("Failed to execute delete command");

    if result.success() {
        println!("Successfully deleted path: {}", paths[index - 1]);
    } else {
        println!("Deletion failed");
    }
}
