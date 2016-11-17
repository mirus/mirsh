extern crate liner;

use liner::Context;
use std::process::{Command, exit};
use std::string::String;
use std::env;
use std::path::Path;

fn exec(proc_name: String) {
    let mut args_iter = proc_name.split_whitespace();
    let to_exec = args_iter.next();
    // println!("{}", proc_name);
    let status = Command::new(to_exec.unwrap())
        .status()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    // println!("process exited with: {}", status);
}

fn build_prompt() -> String {
    String::from("[prompt]$ ")
}

// built in functions
fn cd(new_dir: String) {
    let root = Path::new(&new_dir);
    assert!(env::set_current_dir(&root).is_ok());
}

fn main() {
    let mut con = Context::new();

    loop {
        let res = con.read_line(build_prompt(), &mut |_| {}).unwrap();

        if res.is_empty() {
            break;
        }

        match res.as_ref() {
            "exit" => exit(0),
            "cd" => cd(res.clone()),
            _ => exec(res.clone()),
        }

        con.history.push(res.into());
    }
}
