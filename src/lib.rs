#![feature(old_io)]
extern crate term;

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

#[test]
fn clear_screen() {
    clear();
}
