extern crate termion;

use std::io::{stdin, stdout, Write};
use std::process;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn run_command(command: &mut String) {
    println!("\r");
    match command.as_str() {
        "exit" => process::exit(0),
        _ => println!("Unknown command: {}", command)
    }
    command.clear();
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    writeln!(
        stdout,
        "{}{}CoRuSH - ^c to exit.\r",
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
            Key::Ctrl('c') => break,
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
