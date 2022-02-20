use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    // Your code here :)
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut ans = 0  ;
    for  line in reader.lines() {
        let   line = line.unwrap();
        
        ans += line.split_ascii_whitespace().map(|_| 1).sum::<i32>();
       
        
    }
    println!("{}", ans);
}
