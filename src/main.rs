// Copyright (c) Fabian Beskow 2024

use std::env;

fn help() {
    let program_name = env::args().next().unwrap();

    println!(r#"{} <command> [args]
commands:
    init <path>      Initilize an existing directory into an assgen project
    gen [path]       Generate the specified project, otherwise `./`
    help             Show this page
"#, program_name);

}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
        return;
    }

    let command: &str = &args[1];

    match command {
        "init" => {
            if args.len() < 3 { help() }
            assgen::Website::init(&args[2])
                .unwrap_or_else(|e| println!("Error generating project: {:?}", e));
        },
        "gen" => {
            let mut path: String = String::new();
            if args.len() < 3 {
                path.push_str("./")
            } else {
                path.push_str(&args[2])
            }
            assgen::Website::gen(&path).unwrap();
        },
        _ => help(),
    }
}
