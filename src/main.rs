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

    let (resp, _) = switch_command(input.trim());
    if resp == "exit" {
        return;
    } else {
        println!("{}", resp.trim());
        // Keep command prompt up until exit
        command_prompt();
    }
}

fn switch_command(input: &str) -> (String, bool) {
    let mut builtin_command_flag = false;
    if input.is_empty() {
        return (String::from(""), builtin_command_flag);
    }
    let parts = input.split_whitespace().collect::<Vec<&str>>();
    let command = parts[0];
    match command {
        "exit" => {
            builtin_command_flag = true;
            return (String::from("exit"), builtin_command_flag);
        }
        "echo" => {
            // return the rest of the input
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
        _ => {
            if find_file_in_path(command).is_some() {
                // trim input
                let output = Command::new(command).args(&parts[1..]).output();
                let output_stdout = output
                    .unwrap()
                    .stdout
                    .iter()
                    .map(|&b| b as char)
                    .collect::<String>();
                // return executable output
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
    // return the is_builtin flag for the queried command, inefficient but should be reliable
    return switch_command(command).1;
}

fn find_file_in_path(file_name: &str) -> Option<String> {
    let path = var("PATH").unwrap_or_default();
    let paths = path.split(':');
    for p in paths {
        let file_path = format!("{}/{}", p, file_name);
        if Path::new(&file_path).exists() {
            return Some(file_path);
        }
    }
    None
}
