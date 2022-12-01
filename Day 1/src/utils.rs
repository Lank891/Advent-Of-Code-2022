use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn input_file_path() -> Result<String, String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("No input file specified, specify it as the first argument.".into());
    }
    return Ok(args[1].clone());  
}