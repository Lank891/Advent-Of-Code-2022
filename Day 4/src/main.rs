mod utils;

type Range = (i32, i32);

fn does_range_contain_range(range1: Range, range2: Range) -> bool {
    range1.0 <= range2.0 && range2.1 <= range1.1
}

fn does_ranges_overlap(range1: Range, range2: Range) -> bool {
    range1.0 <= range2.1 && range2.0 <= range1.1
}

fn string_to_range(s: String) -> Range {
    let mut split = s.split('-');
    let first = split.next().unwrap().parse::<i32>().unwrap();
    let second = split.next().unwrap().parse::<i32>().unwrap();
    (first, second)
}

fn main() {
    let input_file = utils::input_file_path();
    if let Err(e) = input_file {
        println!("{}", e);
        return;
    }

    let mut contained_pairs_num: i32 = 0;
    let mut overlapping_pairs_num: i32 = 0;

    if let Ok(lines) = utils::read_lines(input_file.unwrap()) {

        for line in lines {
            if let Ok(readed_line) = line {
                let mut split = readed_line.split(',');
                let first_range = string_to_range(split.next().unwrap().to_string());
                let second_range = string_to_range(split.next().unwrap().to_string());

                if does_range_contain_range(first_range, second_range) || does_range_contain_range(second_range, first_range) {
                    contained_pairs_num += 1;
                }

                if does_ranges_overlap(first_range, second_range) || does_ranges_overlap(second_range, first_range) {
                    overlapping_pairs_num += 1;
                }
            }
        }
    }

    println!("Part 1: {}", contained_pairs_num);
    println!("Part 2: {}", overlapping_pairs_num);
}