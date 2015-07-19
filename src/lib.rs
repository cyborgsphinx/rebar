extern crate term;
extern crate termios;

use term::terminfo::TermInfo;
use termios::Termios;
use std::string::String;
//use std::io::{stdin, Stdin, stdout, Stdout, stderr, Stderr};

pub mod rebar;
use rebar::Rebar;

/// Initialisation function
pub fn init() -> Rebar {
    let info = match TermInfo::from_env() {
        Ok(s) => s,
        Err(f) => panic!(f),
    };
    let ios = match Termios::from_fd(0) {
        Ok(s) => s,
        Err(f) => panic!(f),
    };
    Rebar::from_term(ios, info)
}

///Clears the screen using terminfo-based clear string
pub fn clear(reb: &mut Rebar) {
    reb.clear_screen();
}

///Refresh the line, so that you can move cursors and shit
pub fn refresh_screen(inline: &str) {
    let mut line = String::new();
    line.push_str("\r");
    line.push_str(inline);
    print!("{}", line);
}


#[test]
fn init_rebar() {
    let mut reb = init();
    reb.prompt.push('$');
    reb.cursor_pos += 1;
}

#[test]
fn clear_screen() {
    let mut reb = Rebar::new();
    clear(&mut reb);
}

#[test]
fn refresh() {
    print!("Hello");
    refresh_screen("J");
}
