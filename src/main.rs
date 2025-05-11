use std::env::var;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

fn main() {
    command_prompt();
    return;
}

fn command_prompt() {
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // return anything as invalid command
    let (resp, _) = switch_command(input.trim());
    if resp == "exit" {
        return;
    } else {
        // println!("u doin sum");
        // println!("{}: {}", resp, input.trim());
        println!("{}", resp.trim());
        // Call the command prompt again
        command_prompt();
    }
}

fn switch_command(input: &str) -> (String, bool) {
    let mut builtin_command_flag = false;
    if input.is_empty() {
        return (String::from(""), builtin_command_flag);
    }
    // let mut input_iter = input.split_whitespace();
    let parts = input.split_whitespace().collect::<Vec<&str>>();
    // let command = input_iter.next().unwrap_or("");
    let command = parts[0];
    match command {
        "exit" => {
            builtin_command_flag = true;
            return (String::from("exit"), builtin_command_flag);
        }
        "echo" => {
            // return the rest of the input
            // return input_iter.collect::<Vec<&str>>().join(" ");
            builtin_command_flag = true;
            return (parts[1..].join(" "), builtin_command_flag);
        }
        "type" => {
            builtin_command_flag = true;
            // let query = input_iter.next().unwrap_or("");
            let query = parts[1];
            // don't infinite loop on type type
            if query.contains("type") {
                return (
                    format!("{} is a shell builtin", query),
                    builtin_command_flag,
                );
            }
            // check if command is a shell builtin
            let builtin = is_builtin(query);
            if builtin {
                // check if command is a shell builtin
                return (
                    format!("{} is a shell builtin", query),
                    builtin_command_flag,
                );
            }
            // check if file exists in PATH
            let file_exist = find_file_in_path(query);
            if file_exist.is_some() {
                return (
                    format!("{} is {}", query, file_exist.unwrap()),
                    builtin_command_flag,
                );
            }
            // return invalid if command is not found
            return (format!("{}: not found", query), builtin_command_flag);
        }
        // "help" => {
        //     println!("Available commands: help, exit 0");
        // }
        _ => {
            if find_file_in_path(command).is_some() {
                // trim input
                let output = Command::new(command).args(&parts[1..]).output();
                // not my solution
                // match std::process::Command::new(command)
                //     .args(&parts[1..])
                //     .output()
                // {
                //     Ok(output) => {
                //         // io::stdout().write_all(&output.stdout).unwrap();
                //         // io::stderr().write_all(&output.stderr).unwrap();
                //         return String::from_utf8_lossy(&output.stdout).to_string();
                //     }
                //     Err(_) => {
                //         // eprintln!("{}: command not found", command);
                //         return format!("{}: command not found", command);
                //     }
                // }
                let output_stdout = output
                    .unwrap()
                    .stdout
                    .iter()
                    .map(|&b| b as char)
                    .collect::<String>();
                // // println!("Stdout: {}", output_stdout);
                // // return executable output
                return (output_stdout.trim().to_string(), builtin_command_flag);
            } else {
                return (
                    format!("{}: command not found", command),
                    builtin_command_flag,
                );
            }
        }
    }
}

fn is_builtin(command: &str) -> bool {
    return switch_command(command).1;
    // try running command, if it fails, return invalid
    // let resp = switch_command(command);
    // // if not found or is in PATH, return false
    // if resp.contains("not found") || resp.contains(&format!("{} is", command)) || resp.is_empty() {
    //     return false;
    // }
    // return true;

    // let builtins = ["echo", "type", "exit"];
    // builtins.contains(&command)
}

fn find_file_in_path(file_name: &str) -> Option<String> {
    let path = var("PATH").unwrap_or_default();
    let paths = path.split(':');
    for p in paths {
        // file_path = path / query
        let file_path = format!("{}/{}", p, file_name);
        if Path::new(&file_path).exists() {
            return Some(file_path);
        }
    }
    None
}
