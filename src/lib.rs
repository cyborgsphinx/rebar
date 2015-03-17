#![feature(old_io)]
extern crate term;

use std::string::String;

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
    let mut line = String::from_str("\r");
    line.push_str(inline);
    print!("{}", line);
}

#[test]
fn clear_screen() {
    clear();
}
