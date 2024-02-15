use std::{self, process::exit};

use nekuda::{check_necessary_dependencies, help, install, upgrade};

fn main() {
    
    check_necessary_dependencies();

    let mut args  = std::env::args();

    if args.len() <= 1 {
        help();
        exit(0);
    }
    let command = args.nth(1).expect("No command given");

    println!("[CLI] command is `{}`", &command);

    match command.as_str() {
        "instaLL" => install(),
        "upgrade" => upgrade(),
        "help" | "" => help(),
        _ => eprintln!("Unknown command"),
     };
}

