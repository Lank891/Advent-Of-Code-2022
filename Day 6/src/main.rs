mod utils;
use substring::Substring;
use std::collections::HashSet;

fn main() {
    let input_file = utils::input_file_path();
    if let Err(e) = input_file {
        println!("{}", e);
        return;
    }

    if let Ok(lines) = utils::read_lines(input_file.unwrap()) {
        for line in lines {
            if let Ok(readed_line) = line {
                
                for n in 0..readed_line.len()-4 {
                    let chars : HashSet<char> = HashSet::from_iter(readed_line.substring(n, n+4).chars());
                    if chars.len() == 4 {
                        println!("Part 1: {}", n+4);
                        break;
                    }
                }

                for n in 0..readed_line.len()-14 {
                    let chars : HashSet<char> = HashSet::from_iter(readed_line.substring(n, n+14).chars());
                    if chars.len() == 14 {
                        println!("Part 2: {}", n+14);
                        break;
                    }
                }
                
            }
        }
    }
}