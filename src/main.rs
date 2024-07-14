extern crate termion;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::ErrorKind;
use std::io::{stdin, stdout, Write};
use std::process;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn cmd_ls() {
    let path_result = env::current_dir();
    match path_result {
        Ok(path) => {
            // let paths = fs::read_dir(path.display()).unwrap();
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => println!("Error: path not found"),
            _ => println!("Error reading path: {error:?}")
        }
    };
}

fn cmd_help() {
    let mut commands = HashMap::new();
    commands.insert("ls", "lists directory entries");
    commands.insert("exit","exits the shell");
    commands.insert("help","shows this help");
    println!("\rAvailable builtins:");
    for (command, description) in commands.iter() {
        println!("\r{}\t{}", command, description);
    }
    println!("\r");
}

fn run_command(command: &mut String) {
    print!("\r");
    match command.as_str() {
        "ls" => cmd_ls(),
        "exit" => process::exit(0),
        "help" => cmd_help(),
        _ => println!("Unknown command: {}", command)
    }
    command.clear();
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    writeln!(
        stdout,
        "{}{}Welcome to CoRuSH.\r",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();
    stdout.flush().unwrap();

    write!(
        stdout,
        "{} > ",
        termion::cursor::BlinkingUnderline
    )
    .unwrap();
    stdout.flush().unwrap();

    let mut current_command = String::from("");

    for k in stdin.keys() {
        match k.as_ref().unwrap() {
            Key::Ctrl('d') => break,
            Key::Char('\n') => {
                println!();
                run_command(&mut current_command);
                print!("\r > ");
            },
            Key::Char(c) => {
                current_command.push(*c);
                print!("{}", c);
            },
            _ => {
                println!("{:?}", k)
            }
        }
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
