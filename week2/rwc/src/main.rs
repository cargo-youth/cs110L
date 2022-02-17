use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    // Your code here :)
    let file = File::open(filename);
    let mut lines = Vec::new();
    for line in io::BufReader::new(file).lines() {
        let line_str = line;
    }
}
