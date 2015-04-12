extern crate rebar;

use std::io;
use std::io::Write;
//use std::io::Read;
use std::string::String;

pub fn main() {
    let mut sin = io::stdin();
    let mut fred = String::new();
    fred.push_str("Hello there");
    let mut sout = io::stdout();
    print!("{}", fred);
    match sout.flush() {
        Ok(..) => {},
        Err(..) => {},
    };
    match sin.read_line(&mut fred) {
        Ok(..) => {},
        Err(..) => {},
    };
    rebar::refresh_screen(fred.as_ref());
}
