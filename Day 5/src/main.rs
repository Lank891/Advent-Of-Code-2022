mod utils;
use std::str;
use regex::Regex;
use std::collections::VecDeque;

#[derive(Clone, Copy)]
struct Instruction {
    count: i32,
    from: i32,
    to: i32,
}

fn main() {
    let input_file = utils::input_file_path();
    if let Err(e) = input_file {
        println!("{}", e);
        return;
    }

    let mut read_instructions : bool = false;

    let container_regex = Regex::new(r"\[([\w])\].*").unwrap();
    let instruction_regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    let mut instructions: Vec<Instruction> = Vec::new();

    if let Ok(lines) = utils::read_lines(input_file.unwrap()) {
        for line in lines {
            if let Ok(readed_line) = line {
                
                if read_instructions  {
                    let instruction = instruction_regex.captures(&readed_line).unwrap();
                    let count = instruction.get(1).unwrap().as_str().parse::<i32>().unwrap();
                    let from = instruction.get(2).unwrap().as_str().parse::<i32>().unwrap();
                    let to = instruction.get(3).unwrap().as_str().parse::<i32>().unwrap();
                    instructions.push(Instruction { count, from, to });
                }
                else { // Build stacks

                    if readed_line.len() > 0 { // Line with stacks
                        let boxes_in_stacks = readed_line
                            .as_bytes()
                            .chunks(4)
                            .map(|container| str::from_utf8(container).unwrap())
                            .map(|container| container_regex.captures(container).map_or_else(|| '0', |gr| gr.get(1).unwrap().as_str().chars().nth(0).unwrap()))
                            .collect::<Vec<char>>();
                        boxes_in_stacks.iter().enumerate().for_each(|(i, box_in_stack)| {
                            if stacks.len() < i + 1 {
                                stacks.push(VecDeque::new());
                            }
                            if box_in_stack != &'0' {
                                stacks[i].push_front(*box_in_stack);
                            }
                        });
                    }

                    if readed_line.trim().is_empty() {
                        read_instructions = true;
                    }
                }
                
                
            }
        }
    }

    let mut part1_stacks: Vec<VecDeque<char>> = Vec::new();
    let mut part2_stacks: Vec<VecDeque<char>> = Vec::new();
    stacks.iter().for_each(|stack| {
        part1_stacks.push(stack.clone());
        part2_stacks.push(stack.clone());
    });

    instructions.iter().for_each(|i| {
        // Part 1
        for _ in 0..i.count {
            let box_to_move = part1_stacks[i.from as usize - 1].pop_back().unwrap();
            part1_stacks[i.to as usize - 1].push_back(box_to_move);
        }

        // Part 2
        let mut boxes_to_move: VecDeque<char> = VecDeque::new();
        for _ in 0..i.count {
            boxes_to_move.push_front(part2_stacks[i.from as usize - 1].pop_back().unwrap());
        }
        for _ in 0..i.count {
            part2_stacks[i.to as usize - 1].push_back(boxes_to_move.pop_front().unwrap());
        }
    });

    let mut part1_string = String::new();
    part1_stacks.iter().for_each(|stack| {
        part1_string.push(stack.back().unwrap().clone());
    });

    let mut part2_string = String::new();
    part2_stacks.iter().for_each(|stack| {
        part2_string.push(stack.back().unwrap().clone());
    });

    println!("Part 1: {}", part1_string);
    println!("Part 2: {}", part2_string);
}