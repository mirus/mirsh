extern crate liner;
extern crate colored;

use colored::*;
use liner::Context;
use std::process::{Command, exit};
use std::string::String;
use std::io::ErrorKind;
use std::env;
use std::path::Path;

mod builtin;
mod shell;

fn file_not_found(to_exec: &str) {
    println!("mirsh: command not found: {}", to_exec);
}

fn exec(proc_name: String) {
    let mut args_iter = proc_name.split_whitespace();
    let to_exec = args_iter.next();
    let mut args = args_iter.collect::<Vec<&str>>();

    let status = Command::new(to_exec.unwrap())
        .args(&args)
        .status();

    if let Some(err) = status.err() {
        if err.kind() == ErrorKind::NotFound {
            file_not_found(&to_exec.unwrap());
        }
    }

    println!("");
}

fn build_prompt() -> String {
    let current_directory = env::current_dir();
    String::from(format!("{}$ ", current_directory.unwrap().display()))
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
            "cd" => builtin::cd(res.clone()),
            "echo" => builtin::echo(res.clone()),
            _ => exec(res.clone()),
        }

        con.history.push(res.into());
    }
}
