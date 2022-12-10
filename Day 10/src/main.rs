mod utils;
use regex::Regex;

fn strength_change_part_1(x_register: i32, cycle: i32) -> i32 {
    if [20, 60, 100, 140, 180, 220].contains(&cycle) {
        //println!("{}: {} = {} * {}", cycle, x_register * cycle, cycle, x_register);
        return x_register * cycle;
    }
    return 0;
}

fn add_pixel_to_string_part_2(x_register: i32, cycle: i32) -> String {
    let column = (cycle - 1) % 40;
    let mut ret: String = "".to_string();

    if column == 0 {
        ret += "\n";
    }

    if vec![x_register-1, x_register, x_register+1].contains(&column) {
        ret += "#";
    } else {
        ret += ".";
    }

    return ret;
}

fn main() {
    let input_file = utils::input_file_path();
    if let Err(e) = input_file {
        println!("{}", e);
        return;
    }

    let noop_regex = Regex::new(r"noop").unwrap();
    let addx_regex = Regex::new(r"addx (-?\d+)").unwrap();

    let mut x_register = 1;
    let mut cycle_counter = 1;

    let mut part_1_sum = 0;
    let mut part_2_screen: String = "\n#".to_string();
    

    if let Ok(lines) = utils::read_lines(input_file.unwrap()) {
        for line in lines {
            if let Ok(readed_line) = line {
                
                let noop_match = noop_regex.captures(readed_line.as_str());
                let addx_match = addx_regex.captures(readed_line.as_str());

                if let Some(_noop) = noop_match {
                    cycle_counter += 1;
                    part_1_sum += strength_change_part_1(x_register, cycle_counter);
                    part_2_screen = format!("{}{}", part_2_screen, add_pixel_to_string_part_2(x_register, cycle_counter));
                } else if let Some(addx) = addx_match {
                    let addx_value = addx.get(1).unwrap().as_str().parse::<i32>().unwrap();
                    cycle_counter += 1;
                    part_1_sum += strength_change_part_1(x_register, cycle_counter);
                    part_2_screen = format!("{}{}", part_2_screen, add_pixel_to_string_part_2(x_register, cycle_counter));

                    cycle_counter += 1;
                    x_register += addx_value;
                    part_1_sum += strength_change_part_1(x_register, cycle_counter);
                    part_2_screen = format!("{}{}", part_2_screen, add_pixel_to_string_part_2(x_register, cycle_counter));
                }
            }
        }
    }

    println!("Part 1: {}", part_1_sum);
    println!("Part 2: {}", part_2_screen);
}