use std::env;
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
    } else if resp == "no output" {
        command_prompt();
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
    // let parts = input.split_whitespace().collect::<Vec<&str>>();
    let parts = shlex::split(input).unwrap_or_default();
    let command = &parts[0];
    match command.as_str() {
        "exit" => {
            builtin_command_flag = true;
            return (String::from("exit"), builtin_command_flag);
        }
        "echo" => {
            // add env var echo if $ prefix
            // return the rest of the input
            builtin_command_flag = true;
            let joined = parts[1..].join(" ");
            let trimmed = joined.trim_matches('\'');
            // return (parts[1..].join(" "), builtin_command_flag);
            return (trimmed.to_string(), builtin_command_flag);
        }
        "pwd" => {
            builtin_command_flag = true;
            let output = var("PWD").unwrap_or_default();
            return (output.trim().to_string(), builtin_command_flag);
        }
        "cd" => {
            builtin_command_flag = true;
            // if just cd , go to home directory
            if parts.len() < 2 {
                let home_dir = var("HOME").unwrap_or_default();
                env::set_var("PWD", &home_dir);
                env::set_current_dir(home_dir).unwrap();
                return ("no output".to_string(), builtin_command_flag);
            }
            let dir = &parts[1];
            let exists = directory_exists(&dir);
            if !exists {
                return (
                    format!("{}: No such file or directory", dir),
                    builtin_command_flag,
                );
            } else {
                let _test = move_dir(&dir);

                return ("no output".to_string(), builtin_command_flag);
            }
        }
        "type" => {
            builtin_command_flag = true;
            // let query = input_iter.next().unwrap_or("");
            let query = &parts[1];
            // don't infinite loop on type type
            if query.contains("type") {
                return (
                    format!("{} is a shell builtin", query),
                    builtin_command_flag,
                );
            }
            // check if command is a shell builtin
            let builtin = is_builtin(&query);
            if builtin {
                // check if command is a shell builtin
                return (
                    format!("{} is a shell builtin", query),
                    builtin_command_flag,
                );
            }
            // check if file exists in PATH
            let file_exist = find_file_in_path(&query);
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
            if find_file_in_path(&command).is_some() {
                // let output = Command::new(command).args(&parts[1..]).output();
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

fn directory_exists(dir: &str) -> bool {
    // TODO : handle ~
    let dir_parts = dir.split('/').collect::<Vec<&str>>();
    if dir_parts.len() == 0 {
        return false;
    }
    if dir_parts[0] == "~" {
        let home_dir = var("HOME").unwrap_or_default();
        let new_dir = format!("{}/{}", home_dir, dir_parts[1..].join("/"));
        let path = Path::new(&new_dir);
        return path.exists();
    }
    let path = Path::new(dir);
    return path.exists();
}

fn move_dir(dir: &str) {
    let dest_parts = dir.split('/').collect::<Vec<&str>>();
    let current_dir = env::current_dir().unwrap();
    let current_parts = current_dir
        .to_str()
        .unwrap()
        .split('/')
        .collect::<Vec<&str>>();
    let mut new_dir_path = <Vec<&str>>::new();

    let home_dir = var("HOME").unwrap_or_default();

    match dest_parts[0] {
        "." => {
            new_dir_path.push(current_dir.to_str().unwrap());
        }
        "~" => {
            new_dir_path.push(home_dir.as_str());
        }
        "" => {}
        _ => {
            for part in current_parts.iter() {
                new_dir_path.push(part);
            }
        }
    }

    for part in dest_parts.iter() {
        match part {
            &"" => {
                new_dir_path.push(part);
            }
            &"." => {}
            &"~" => {}
            &".." => {
                new_dir_path.pop();
            }
            _ => {
                new_dir_path.push(part);
            }
        }
    }
    if new_dir_path.is_empty() {
        return;
    }
    let ending_dir = new_dir_path.join("/");
    env::set_current_dir(&ending_dir).unwrap();
    env::set_var("PWD", env::current_dir().unwrap());
}
