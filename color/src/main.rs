use colored::{ColoredString, Colorize};
use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

fn main() {
    let start: u128 = counter();

    let args: Vec<String> = get_args();

    let mut result: ColoredString = "".to_string().black();

    match args[1].as_str() {
        "-green" => {
            result = args[0].bright_green().bold();
        }
        _ => (),
    }

    if args.len() >= 3 {
        match args[2].as_str() {
            "-underline" => result = result.underline(),
            "-italic" => result = result.italic(),
            _ => (),
        }
    }

    println!("{result}");

    let end: u128 = counter();

    if args.contains(&"-et".to_string()) {
        println!("\nColoring was executed in {} microseconds", end - start);
    }
}

fn counter() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros()
}

fn get_args() -> Vec<String> {
    let mut args: Vec<String> = Vec::new();
    for (i, arg) in env::args().enumerate() {
        if i != 0 {
            args.push(arg);
        }
    }
    return args;
}
