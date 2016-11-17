extern crate liner;

use liner::Context;
use std::process::{Command, exit};
use std::string::String;

fn exec(proc_name: String) {
    // println!("{}", proc_name);
    let status = Command::new(proc_name)
        .status()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    // println!("process exited with: {}", status);
}

fn main() {
    let mut con = Context::new();

    loop {
        let res = con.read_line("[prompt]$ ", &mut |_| {}).unwrap();

        if res.is_empty() {
            break;
        }

        match res.as_ref() {
            "exit" => exit(0),
            _ => exec(res.clone()),
        }

        con.history.push(res.into());
    }
}
