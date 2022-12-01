mod utils;

fn main() {
    let input_file = utils::input_file_path();
    if let Err(e) = input_file {
        println!("{}", e);
        return;
    }

    let mut calories: Vec<i32> = Vec::new();

    if let Ok(lines) = utils::read_lines(input_file.unwrap()) {
        let mut sum: i32 = 0;

        for line in lines {
            if let Ok(readed_line) = line {

                if readed_line.trim().is_empty() {
                    calories.push(sum);
                    sum = 0;
                } else {
                    if let Ok(number) = readed_line.parse::<i32>() {
                        sum += number;
                    }
                }
                
            }

        }
    }

    calories.sort_by(|a, b| b.cmp(a));
    let max_val = calories.first();
    match max_val {
        Some(number) => println!("Part 1: {}", number),
        None => println!("Vector is empty???"),
    }

    let sum_three = calories.iter().take(3).sum::<i32>();
    println!("Part 2: {}", sum_three);
}