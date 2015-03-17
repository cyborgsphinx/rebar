extern crate rebar;

use std::io;
use std::io::Write;
//use std::io::Read;
use std::string::String;

pub fn main() {
    let mut sin = io::stdin();
    let mut fred = String::new();
    let mut sout = io::stdout();
    sout.flush();
    print!("Hello there");
    sin.read_line(&mut fred);
    rebar::refresh_screen(fred.as_slice());
}
