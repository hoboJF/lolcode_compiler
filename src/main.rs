use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2{
        eprintln!("usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];
    let lolspeak_string = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("error reading file '{}': {}", filename, err);
        std::process::exit(1);
    });
    println!("{}", lolspeak_string);
}
