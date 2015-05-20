extern crate term;
extern crate termios;

use termios::Termios;
use std::string::String;

pub struct Rebar {
    termio: Termios,
    clear: String,
    buf: String,
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
