#![allow(unused_imports)]
#![allow(unused_variables)]

use std::{
    fmt,
    io::{self, stdin, stdout, Stdin, Stdout, Write},
    sync::BarrierWaitResult,
};

use termion::{
    clear,
    cursor::{self, DetectCursorPos},
    event::Key,
    input::TermRead,
    raw::{self, IntoRawMode, RawTerminal},
    screen::AlternateScreen,
    style,
};

use clap::{Arg, ArgAction, Command};
pub mod buffer;
pub mod hackshell;
pub mod hcursor;
pub mod history;
use buffer::Buffer;
use hackshell::Hackshell;
use hcursor::Cursor;
fn calculate_lines(string_len: u16) -> u16 {
    let (width, _) = termion::terminal_size().unwrap();
    if string_len == 0 {
        return 0;
    } else {
        let lines = string_len / width;
        if string_len % width > 0 {
            lines + 1
        } else {
            lines
        }
        
    }
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut hackshell = Hackshell::new(&mut stdout);

    write!(
        stdout,
        "{}{}",
        hackshell.buffer.prefix,
        hackshell
            .cursor
            .move_cursor_to(hackshell.buffer.buffer_size, hackshell.cursor.y)
    )
    .unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('\n') => {
                let mut results: &str = "";
                println!();
                if hackshell.buffer.data.is_empty(){
                    hackshell.write_results(&mut stdout, results, 1);
                    stdout.flush().unwrap();
                    continue;
                }
                hackshell.history.add_buffer(hackshell.buffer.clone());
                hackshell.history.search_index = hackshell.history.size;
                match hackshell.buffer.data.as_str() {
                    "clear" => {
                        hackshell.clear(&mut stdout);
                        continue
                    }
                    "ls" => {}
                    "exit" => {
                        break;
                    }
                    _ => results = "unknown command"
                }
                hackshell.write_results(&mut stdout, results, calculate_lines(hackshell.buffer.buffer_size - 1));
                hackshell.buffer.clear_data();
                stdout.flush().unwrap();
            }
            Key::Char('\t') => match hackshell.autocomplete() {
                Some(s) => {
                    hackshell
                        .cursor
                        .move_position(hackshell.buffer.buffer_size, hackshell.cursor.y);
                    for i in s.chars() {
                        hackshell.buffer.write_data(i, hackshell.cursor.x);
                        hackshell.cursor.move_forward();
                    }
                }
                None => continue,
            },
            Key::Ctrl('c') => break,
            Key::Char(c) => {
                hackshell.buffer.write_data(c, hackshell.cursor.x);
                hackshell.cursor.move_forward();
            }
            Key::Left => {
                if hackshell.cursor.x == hackshell.buffer.prefix.len() as u16 {
                    continue;
                } else {
                    
                    hackshell.cursor.move_backward();
                }
            }
            Key::Right => {
                if hackshell.cursor.x
                    == (hackshell.buffer.prefix.len() + hackshell.buffer.data.len()) as u16
                {
                    continue;
                } else {
                    hackshell.cursor.move_forward();
                }
            }
            Key::Up => {
                if hackshell.history.search_index == 0 {
                    continue;
                } else {
                    hackshell.history.search_index -= 1;
                    hackshell.buffer =
                        hackshell.history.data[hackshell.history.search_index].clone();
                }
            }
            Key::Down => {
                if hackshell.history.search_index + 1 >= hackshell.history.size {
                    continue;
                } else {
                    hackshell.history.search_index += 1;
                    hackshell.buffer =
                        hackshell.history.data[hackshell.history.search_index].clone();
                }
            }
            Key::Backspace => {
                if hackshell.cursor.x == hackshell.buffer.prefix.len() as u16 {
                    continue;
                } else {
                    hackshell.buffer.remove_char(hackshell.cursor.x);
                    hackshell.cursor.move_backward()
                }
            }
            _ => println!("Unknow command"),
        }
        hackshell.write_buffer(&mut stdout);
        stdout.flush().unwrap();
    }
}
