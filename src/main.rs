/*
* cli program to run various commands related to updating my macos system
*
* program flow:
*   1. parse command from cli arg
*   2. execute correct function based on command
*
* commands:
*   1. help: display help message containing commands and simple definition
*   2. last: write to stdout last update time defined in <UPDATEME_HOME>/meta.json
*   3. meta: read <UPDATEME_HOME>/meta.json and write to stdout
*   4. update: run system commands defined by a file written in json
*/

use std::env;
use std::process::exit;

const COMMANDS: [&str; 4] = ["help", "last", "meta", "update"];

enum Command {
    Help,
    Last,
    Meta,
    Update,
    Unknown,
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        match s {
            "help" => Command::Help,
            "last" => Command::Last,
            "meta" => Command::Meta,
            "update" => Command::Update,
            &_ => Command::Unknown,
        }
    }
}

fn help(program: &str) {
    println!("usage: {} <command>\n", program);
    println!("valid commands:");
    for c in COMMANDS.iter() {
        println!("  - {}", c)
    }
}

fn last() {
    assert!(1 == 0)
}

fn meta() {
    assert!(1 == 0)
}

fn update() {
    assert!(1 == 0)
}

fn main() {
    // collect cli args
    let args: Vec<String> = env::args().collect();

    // make sure we got a value back
    let program = match args.get(0) {
        Some(p) => p,
        None => panic!("Unable to extract executable name from command line"),
    };

    let input = match args.get(1) {
        Some(i) => i,
        None => "unknown",
    };

    let command: Command = input.into();

    match command {
        Command::Last => last(),
        Command::Meta => meta(),
        Command::Update => update(),
        Command::Help | Command::Unknown => {
            help(program);
            exit(1)
        }
    }
}
