extern crate term;
extern crate termios;

use term::terminfo::TermInfo;
use termios::Termios;
use std::string::String;
use std::io::{stdin, Stdin, stdout, Stdout, stderr, Stderr, Write};

/// Holds the state for the line editor
pub struct Rebar {
    pub i: Stdin,   //stdin handle
    pub o: Stdout,  //stdout handle
    pub e: Stderr,  //stderr handle
    pub termio: Termios,
    pub info: TermInfo,
    pub prompt: String,
    pub buf: String,
    pub cursor_pos: u32,
}

impl Rebar {
    pub fn new() -> Rebar {
        Rebar {
            i: stdin(),
            o: stdout(),
            e: stderr(),
            termio: Termios::from_fd(0).unwrap(),
            info: TermInfo::from_env().unwrap(),
            prompt: String::new(),
            buf: String::new(),
            cursor_pos: 0,
        }
    }
    pub fn from_term(ios: Termios, info: TermInfo) -> Rebar {
        Rebar {
            i: stdin(),
            o: stdout(),
            e: stderr(),
            termio: ios,
            info: info,
            prompt: String::new(),
            buf: String::new(),
            cursor_pos: 0,
        }
    }
    pub fn clear_screen(&mut self) {
        let cl = self.info.strings.get("clear").unwrap();
        match self.o.write_all(cl) {
            Ok(..) => {},
            Err(f) => println!("Failed to clear screen, {}", f),
        };
    }
}
