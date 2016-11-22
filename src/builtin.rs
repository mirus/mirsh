use std::env;
use std::path::Path;
use std::string::String;

// built in functions
pub fn cd(new_dir: String) {
    let root = Path::new(&new_dir);
    assert!(env::set_current_dir(&root).is_ok());
}

pub fn echo(echo_str: String) {
    let mut args_iter = echo_str.split_whitespace();
    args_iter.next();

    let mut args = args_iter.collect::<Vec<&str>>();

    println!("{}", &args.join(""));
}
