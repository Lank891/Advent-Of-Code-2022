mod utils;
use std::collections::HashSet;
use std::iter::FromIterator;

fn char_to_points(c: char) -> i32 {
    if c.is_lowercase() {
        return c as i32 - 'a' as i32 + 1;
    } else {
        return c as i32 - 'A' as i32 + 27;
    }
}

fn main() {
    let input_file = utils::input_file_path();
    if let Err(e) = input_file {
        println!("{}", e);
        return;
    }

    let mut errors : Vec<char> = Vec::new();
    let mut badges : Vec<char> = Vec::new();

    if let Ok(lines) = utils::read_lines(input_file.unwrap()) {
        let mut group : Vec<String> = Vec::new();

        for line in lines {
            if let Ok(readed_line) = line {
                let (first, second) = readed_line.split_at(readed_line.len()/2);
                let first_chars : HashSet<char> = HashSet::from_iter(first.chars());
                let second_chars : HashSet<char> = HashSet::from_iter(second.chars());
                first_chars.intersection(&second_chars).for_each(|c| errors.push(*c));

                group.push(readed_line);
            }

            if group.len() == 3  {
                let first_elf_chars : HashSet<char> = HashSet::from_iter(group[0].chars());
                let second_elf_chars : HashSet<char> = HashSet::from_iter(group[1].chars());
                let third_elf_chars : HashSet<char> = HashSet::from_iter(group[2].chars());

                let first_second_common : HashSet<char> = 
                first_elf_chars.intersection(&second_elf_chars).copied().collect();

                first_second_common.intersection(&third_elf_chars).for_each(|c| badges.push(*c));
                group.clear();
            }
        }
    }

    let sum_of_errors = errors.iter().map(|c| char_to_points(*c)).sum::<i32>();
    let sum_of_basdges = badges.iter().map(|c| char_to_points(*c)).sum::<i32>();
    println!("Part 1: {}", sum_of_errors);
    println!("Part 2: {}", sum_of_basdges);
}