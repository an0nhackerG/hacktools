use super::buffer::Buffer;
use super::calculate_lines;
use super::hcursor::Cursor;
use super::history::History;
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

pub struct Hackshell {
    pub buffer: Buffer,
    pub cursor: Cursor,
    pub commands: Vec<String>,
    pub history: History,
}
impl Hackshell {
    pub fn new(stdout_in: &mut raw::RawTerminal<Stdout>) -> Hackshell {
        let prefix = "hackshell-> ".to_string();

        let (_x, y) = stdout_in.cursor_pos().unwrap();
        Hackshell {
            buffer: Buffer::new(prefix.clone(), prefix.len() as u16),
            cursor: Cursor::new(prefix.len() as u16, y),
            commands: vec![
                "clear".to_string(),
                "ntwscan".to_string(),
                "ls".to_string(),
                "exit".to_string(),
            ],
            history: History::new(),
        }
    }

    //Here we  acess directly cursor::Goto becouse we want to move the cursor but not update the cursor info in the structure Cursor.
    // Write buffer differs from writing results on the screen, this module is about writing the input of the user on the stdout
    pub fn write_buffer(&mut self, stdout: &mut RawTerminal<Stdout>) {
        write!(
            stdout,
            "{}{}{}{}",
            cursor::Goto(self.buffer.prefix_size, self.cursor.y),
            clear::AfterCursor,
            self.buffer.data,
            cursor::Goto(self.cursor.x, self.cursor.y),
        )
        .unwrap();
    }

    // Write_results differs from write_buffer, this module is about dropping the results in the stdout
    // we actualy drop all the results in the result variable in the stdout, and in the final make the buffer
    // ready to be writing in write_buffer module.
    pub fn write_results(&mut self, stdout: &mut RawTerminal<Stdout>, results: &str, lines_for_buffer: u16) {

        write!(
            stdout,
            "{}{}{}{}{}{}{}",
            self.cursor
                .move_cursor_to(self.buffer.prefix_size, self.cursor.y),
            self.buffer.data,
            self.cursor.move_cursor_to(0, self.cursor.y + lines_for_buffer),
            results,
            self.cursor
                .move_cursor_to(0, self.cursor.y + calculate_lines(results.len() as u16)),
            self.buffer.prefix,
            self.cursor
                .move_cursor_to(self.buffer.prefix_size, self.cursor.y),
        )
        .unwrap();
    }

    // the clear command itself, just clear all the screen
    pub fn clear(&mut self, stdout: &mut RawTerminal<Stdout>) {
        self.cursor.x = self.buffer.prefix_size;
        self.cursor.y = 1;
        self.buffer.clear_data();
        write!(
            stdout,
            "{}{}{}{}",
            clear::All,
            self.cursor.move_cursor_to(0, self.cursor.y),
            self.buffer.prefix,
            self.cursor.move_cursor_to(self.buffer.prefix.len() as u16, self.cursor.y),
        )
        .unwrap_or_else(|f| println!("Unable to clear: {:?}", f));
        stdout.flush().unwrap();
    }

    // Compare the input of the user with the commands available in the shell, if the input contains part of the commands, autocomplete it
    pub fn autocomplete(&self) -> Option<String> {
        let parsing: Vec<&str> = self.buffer.data.split_whitespace().collect();
        if parsing.is_empty() {
            return None;
        }
        let to_compare = parsing.last().unwrap();
        for command in &self.commands {
            if command.starts_with(to_compare) {
                return Some(command[to_compare.len()..].to_string());
            }
        }
        None
    }
}
