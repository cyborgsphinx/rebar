extern crate term;
extern crate termios;

use term::terminfo::TermInfo;
use termios::Termios;
use std::string::String;
use std::io::{stdin, Stdin, stdout, Stdout, stderr, Stderr};

/// Holds the state for the line editor
pub struct Rebar {
    i: Stdin,   //stdin handle
    o: Stdout,  //stdout handle
    e: Stderr,  //stderr handle
    termio: Termios,
    terms: TermInfo,
    prompt: String,
    buf: String,
    cursor_pos: usize,
}

/// Initialisation function
pub fn rebar() -> Rebar {
    let info = match TermInfo::from_env() {
        Ok(s) => s,
        Err(f) => panic!(f),
    };
    let ios = match Termios::from_fd(0) {
        Ok(s) => s,
        Err(f) => panic!(f),
    };
    let res = Rebar {
        i: stdin(),
        o: stdout(),
        e: stderr(),
        termio: ios,
        terms: info,
        prompt: String::new(),
        buf: String::new(),
        cursor_pos: 0
    };
    res
}

///Clears the screen using terminfo-based clear string
pub fn clear() {
    let info = term::terminfo::TermInfo::from_env();

    let fred = match info {
        Ok(val) => val,
        Err(f) => panic!(f.to_string()),
    };

    let clear = fred.strings.get("clear").unwrap();
    let mut derf = term::stdout().unwrap();
    match derf.write_all(clear) {
        Ok(..) => {},
        Err(f) => println!("Failed to clear: {}", f),
    };
}

///Refresh the line, so that you can move cursors and shit
pub fn refresh_screen(inline: &str) {
    let mut line = String::new();
    line.push_str("\r");
    line.push_str(inline);
    print!("{}", line);
}

#[test]
fn clear_screen() {
    clear();
}
