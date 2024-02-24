#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::{ptr::null, thread};
use termion::{
    clear,
    cursor::{self, DetectCursorPos},
    event::Key,
    input::TermRead,
    raw::{self, IntoRawMode, RawTerminal},
    screen::AlternateScreen,
    style,
};

use std::{
    fmt,
    io::{self, stdin, stdout, Stdin, Stdout, Write},
    sync::BarrierWaitResult,
};

struct HCursor {
    x: u16,
    y: u16,
}

struct Buffer {
    input: String,
    prefix: String,
    size: u16,
}

struct History{
    commands: Box<Vec<String>>,
    size: u32
}

struct Terminal {
    hcursor: HCursor,
    buffer: Buffer,
    history: History
}

impl Terminal {
    pub fn new(stdout_in: &mut raw::RawTerminal<Stdout>) -> Terminal {
        let (x_in,y_in) = stdout_in.cursor_pos().unwrap();
        Terminal {
            buffer: Buffer{input: "".to_string(), prefix: "Hackshell-> ".to_string(), size: 0},
            hcursor: HCursor{x: x_in, y: y_in},
            history: History{commands: Box::new(Vec::new()), size: 0}
        }
    }  
}

fn main() {
    println!("Hello, world!");
}
