mod utils;
use regex::Regex;

#[derive(Debug, Clone)]
struct Monkey {
    number: u64,
    items: Vec<u64>,
    operation: String,
    test_divisor: u64,
    true_throw_monkey: u64,
    false_throw_monkey: u64,
}

fn do_operation(operation: &str, value: u64, modulo: u64, divide_by_three: bool) -> u64 {
    let parts: Vec<&str> = operation.split(' ').collect();
    let left = parts[0].parse::<u64>().or::<u64>(Ok(value)).unwrap();
    let right = parts[2].parse::<u64>().or::<u64>(Ok(value)).unwrap();
    let new_worry_level = match parts[1] {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" => left / right,
        _ => panic!("Unknown operation"),
    };

    let new_worry_level = if divide_by_three {
        new_worry_level / 3
    } else {
        new_worry_level
    };

    new_worry_level % modulo
}

fn construct_monkey(description: String) -> Monkey {
    let lines: Vec<&str> = description.split('\n').map(|line| line.trim()).collect();

    let monkey_id_regex = Regex::new(r"Monkey (\d+):").unwrap();
    let starting_items_regex = Regex::new(r"Starting items: ((?:\d+,?\s?)+)").unwrap();
    let operation_regex = Regex::new(r"Operation: new = (.+)").unwrap();
    let test_regex = Regex::new(r"Test: divisible by (\d+)").unwrap();
    let true_throw_regex = Regex::new(r"If true: throw to monkey (\d+)").unwrap();
    let false_throw_regex = Regex::new(r"If false: throw to monkey (\d+)").unwrap();

    let mut ret = Monkey {
        number: 0,
        items: Vec::new(),
        operation: String::new(),
        test_divisor: 0,
        true_throw_monkey: 0,
        false_throw_monkey: 0,
    };

    for line in lines {
        let monkey_id_match = monkey_id_regex.captures(line);
        let starting_items_match = starting_items_regex.captures(line);
        let operation_match = operation_regex.captures(line);
        let test_match = test_regex.captures(line);
        let true_throw_match = true_throw_regex.captures(line);
        let false_throw_match = false_throw_regex.captures(line);
        
        if let Some(monkey_id) = monkey_id_match {
            ret.number = monkey_id.get(1).unwrap().as_str().parse::<u64>().unwrap();
        } 

        else if let Some(starting_items) = starting_items_match {
            let items_string = starting_items.get(1).unwrap().as_str();
            let items: Vec<&str> = items_string.split(',').map(|item| item.trim()).collect();
            for item in items {
                ret.items.push(item.parse::<u64>().unwrap());
            }
        } 

        else if let Some(operation) = operation_match {
            ret.operation = operation.get(1).unwrap().as_str().to_string();
        } 

        else if let Some(test) = test_match {
            ret.test_divisor = test.get(1).unwrap().as_str().parse::<u64>().unwrap();
        } 
        
        else if let Some(true_throw) = true_throw_match {
            ret.true_throw_monkey = true_throw.get(1).unwrap().as_str().parse::<u64>().unwrap();
        }

        else if let Some(false_throw) = false_throw_match {
            ret.false_throw_monkey = false_throw.get(1).unwrap().as_str().parse::<u64>().unwrap();
        }
    }

    return ret;
}

fn main() {
    let input_file = utils::input_file_path();
    if let Err(e) = input_file {
        println!("{}", e);
        return;
    }

    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut monkey_times_inspecting_item: Vec<u64> = Vec::new();

    if let Ok(lines) = utils::read_lines(input_file.unwrap()) {
        let mut monkey_string = String::new();
        for line in lines {
            if let Ok(readed_line) = line {
                if readed_line.trim() != "" {
                    monkey_string.push_str(&readed_line);
                    monkey_string.push_str("\n");
                }
                else {
                    monkeys.push(construct_monkey(monkey_string.clone()));
                    monkey_times_inspecting_item.push(0);
                    monkey_string = String::new();
                }
            }
        }
        monkeys.push(construct_monkey(monkey_string.clone()));
        monkey_times_inspecting_item.push(0);
    }

    let mut least_common_multiplier_for_divisors: u64 = 1;
    for monkey in &monkeys {
        least_common_multiplier_for_divisors = least_common_multiplier_for_divisors * monkey.test_divisor;
    }

    let mut monkeys_part_2 = monkeys.clone();
    let mut monkey_times_inspecting_item_part_2 = monkey_times_inspecting_item.clone();
    for round in 1..10000 + 1 {
        for id in 0..monkeys.len() {
            unsafe {
                // Part 1
                if round <= 20 {
                    let monkey: *mut Monkey = &mut monkeys[id];
                    while !(*monkey).items.is_empty() {
                        monkey_times_inspecting_item[(*monkey).number as usize] += 1;
                        let next_item = (*monkey).items.pop().unwrap();
        
                        let new_worry_level = do_operation(&(*monkey).operation, next_item, least_common_multiplier_for_divisors, true);
                        if new_worry_level % (*monkey).test_divisor == 0 {
                            monkeys[(*monkey).true_throw_monkey as usize].items.push(new_worry_level);
                        }
                        else {
                            monkeys[(*monkey).false_throw_monkey as usize].items.push(new_worry_level);
                        }
                    }
                }
                
                let monkey: *mut Monkey = &mut monkeys_part_2[id];
                while !(*monkey).items.is_empty() {
                    monkey_times_inspecting_item_part_2[(*monkey).number as usize] += 1;
                    let next_item = (*monkey).items.pop().unwrap();
    
                    let new_worry_level = do_operation(&(*monkey).operation, next_item, least_common_multiplier_for_divisors, false);
                    if new_worry_level % (*monkey).test_divisor == 0 {
                        monkeys_part_2[(*monkey).true_throw_monkey as usize].items.push(new_worry_level);
                    }
                    else {
                        monkeys_part_2[(*monkey).false_throw_monkey as usize].items.push(new_worry_level);
                    }
                }

            }
        }
        //println!("Round {}", i);
        //println!("{:#?}", monkeys);
    }

    monkey_times_inspecting_item.sort_by(|a, b| b.cmp(a));
    monkey_times_inspecting_item_part_2.sort_by(|a, b| b.cmp(a));

    //println!("{:#?}", monkey_times_inspecting_item_part_2);
    
    println!("Part 1: {}", monkey_times_inspecting_item[0] * monkey_times_inspecting_item[1]);
    println!("Part 2: {}", monkey_times_inspecting_item_part_2[0] * monkey_times_inspecting_item_part_2[1]);
}