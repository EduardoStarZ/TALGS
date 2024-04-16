use colored::{ColoredString, Colorize};
use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

fn main() {
    let start: u128 = counter();

    let mut args: Vec<String> = get_args();

    let mut result: ColoredString = match args[1].as_str() {
        "-blue" => args[0].bright_cyan(),
        "-yellow" => args[0].bright_yellow(),
        "-red" => args[0].bright_red(),
        "-green" => args[0].bright_green(),
        _ => args[0].normal(),
    };

    args.remove(0);

    if args.len() > 1 {
        for x in args.iter() {
            result = match x.as_str() {
                "-bold" => result.bold(),
                "-italic" => result.italic(),
                "-underline" => result.underline(),
                _ => result,
            }
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
