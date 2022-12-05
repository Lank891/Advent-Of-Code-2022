mod utils;


fn main() {
    let input_file = utils::input_file_path();
    if let Err(e) = input_file {
        println!("{}", e);
        return;
    }

    if let Ok(lines) = utils::read_lines(input_file.unwrap()) {
        for line in lines {
            if let Ok(readed_line) = line {
                

                
            }
        }
    }

    println!("Part 1: {}", 1);
    println!("Part 2: {}", 1);
}