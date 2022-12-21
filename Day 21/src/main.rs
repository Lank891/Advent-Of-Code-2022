mod utils;
use std::collections::HashMap;

fn eval_monkey(monkey: String, monkeys: &mut HashMap<String, Result<i128, String>>) -> i128 {
    if let Ok(v) = monkeys.get(&monkey).unwrap() {
        return *v;
    }

    let expr = monkeys.get(&monkey).unwrap().as_ref().unwrap_err().clone();
    let parts = expr.split(" ").collect::<Vec<&str>>();

    let left = eval_monkey(parts[0].to_string(), monkeys);
    let right = eval_monkey(parts[2].to_string(), monkeys);

    let result = match parts[1] {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" => left / right,
        _ => panic!("Unknown operator {}", parts[1])
    };

    monkeys.insert(monkey, Ok(result));
    return result;
}

fn can_monkey_be_evaluated(monkey: String, monkeys: & HashMap<String, Result<i128, String>>) -> bool {
    if monkey == "humn" {
        return false;
    }

    if let Ok(_) = monkeys.get(&monkey).unwrap() {
        return true;
    }
    
    let expr = monkeys.get(&monkey).unwrap().as_ref().unwrap_err().clone();
    let parts = expr.split(" ").collect::<Vec<&str>>();

    let left = can_monkey_be_evaluated(parts[0].to_string(), monkeys);
    let right = can_monkey_be_evaluated(parts[2].to_string(), monkeys);

    return left && right;
} 

// Assumption - it is a tree => no cycle => human is needed only in one part
fn find_humn_in_subtree(monkey: String, monkeys: &mut HashMap<String, Result<i128, String>>, expected_value: i128) -> i128 {
    if monkey == "humn" {
        monkeys.insert("humn".to_string(), Ok(expected_value));
        return expected_value;
    }

    // If we are here we cannot be in value node, always expression
    let expr = monkeys.get(&monkey).unwrap().as_ref().unwrap_err().clone();
    let parts = expr.split(" ").collect::<Vec<&str>>();

    let can_left = can_monkey_be_evaluated(parts[0].to_string(), &monkeys);
    //let can_right = can_monkey_be_evaluated(parts[2].to_string(), &monkeys);  // Not needed since it is always opposite of left

    let other_value = 
        if can_left 
            { eval_monkey(parts[0].to_string(), monkeys) } 
        else 
            { eval_monkey(parts[2].to_string(), monkeys) };

    let to_find: i128;
    if parts[1] == "+" {
        to_find = expected_value - other_value;
    } 
    else if parts[1] == "-" && can_left {
        to_find = other_value - expected_value ;
    } 
    else if parts[1] == "-" && !can_left {
        to_find = other_value + expected_value ;
    } 
    else if parts[1] == "*" {
        to_find = expected_value / other_value;
    }
    else if parts[1] == "/" && can_left {
        to_find = other_value / expected_value;
    } 
    else if parts[1] == "/" && !can_left {
        to_find = other_value * expected_value;
    } 
    else {
        panic!("Unknown operator {}", parts[1])
    }

    let tree_to_find_human_in = 
        if can_left 
            { parts[2].to_string() } 
        else 
            { parts[0].to_string() };

    let human = find_humn_in_subtree(tree_to_find_human_in, monkeys, to_find);
    return human;
}

fn main() {
    let input_file = utils::input_file_path();
    if let Err(e) = input_file {
        println!("{}", e);
        return;
    }

    // Hash map from monkey to their value - the value can be either a number or an expression. Sadly no "Either" 
    // monad without library so Result is used as a replacement with worse names.
    let mut monkeys: HashMap<String, Result<i128, String>> = HashMap::new();

    if let Ok(lines) = utils::read_lines(input_file.unwrap()) {
        for line in lines {
            if let Ok(readed_line) = line {
                
                let split = readed_line.split(": ").collect::<Vec<&str>>();
                let name = split[0];
                let value = split[1].parse::<i128>();
                if let Ok(v) = value {
                    monkeys.insert(name.to_string(), Ok(v));
                } else {
                    monkeys.insert(name.to_string(), Err(split[1].to_string()));
                }
                
            }
        }
    }

    //println!("{:?}", monkeys);

    let part_1 = eval_monkey("root".to_string(), &mut monkeys.clone());

    // Assumption - it is a tree => no cycle => human is needed only in one part
    let mut monkeys_part_2 = monkeys.clone();
    let expr_root = monkeys.get("root").unwrap().as_ref().unwrap_err().clone();
    let parts_root = expr_root.split(" ").collect::<Vec<&str>>();

    let can_left = can_monkey_be_evaluated(parts_root[0].to_string(), &monkeys_part_2);
    //let can_right = can_monkey_be_evaluated(parts_root[2].to_string(), &monkeys_part_2); // Not needed since it is always opposite of left

    let value_to_get = 
        if can_left 
            { eval_monkey(parts_root[0].to_string(), &mut monkeys_part_2) } 
        else 
            { eval_monkey(parts_root[2].to_string(), &mut monkeys_part_2) };

    let tree_to_find_human_in = 
        if can_left 
            { parts_root[2].to_string() } 
        else 
            { parts_root[0].to_string() };

    let human = find_humn_in_subtree(tree_to_find_human_in, &mut monkeys_part_2, value_to_get);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", human);
}