extern crate termion;

use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    writeln!(
        stdout,
        "{}{}^c to exit. Type stuff, use alt, and so on.",
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

    for k in stdin.keys() {
/*         write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::CurrentLine
        )
        .unwrap();
*/
        match k.as_ref().unwrap() {
            Key::Ctrl('c') => break,
            Key::Char(c) => print!("{}", c),
            Key::Alt(c) => println!("*{}", c),
            Key::Ctrl(c) => println!("^{}", c),
            Key::Esc => println!("ESC"),
            Key::Left => println!("←"),
            Key::Right => println!("→"),
            Key::Up => println!("↑"),
            Key::Down => println!("↓"),
            Key::Backspace => println!("×"),
            _ => {
                println!("{:?}", k)
            }
        }
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
