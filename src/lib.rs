extern crate term;

///Clears the screen using terminfo-based clear string
pub fn clear() {
    let info = term::terminfo::TermInfo::from_env();

    let fred = match info {
        Ok(val) => val,
        Err(f) => panic!(f.to_string()),
    };

    let clear = fred.strings.get("clear").unwrap();
    print!("{}", std::str::from_utf8(clear).unwrap());
}

#[test]
fn it_works() {
}
