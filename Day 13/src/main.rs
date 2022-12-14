mod utils;

fn next_token(chars: &Vec<char>, index: usize) -> (String, usize) {
    let mut actual = index;
    let mut token = "".to_string();

    if chars[actual] == '[' {
        token = "[".to_string();
        return (token, actual + 1);
    }

    if chars[actual] == ']' {
        token = "]".to_string();
        return (token, actual + 1);
    }

    if chars[actual] == ',' {
        return next_token(chars, actual + 1); // Skip , and search further
    }

    while chars[actual].is_digit(10) {
        token += chars[actual].to_string().as_str();
        actual += 1;
    }
    return (token, actual);
}

fn tokenize(a: &String) -> Vec<String> {
    let mut index = 0;
    let mut tokens: Vec<String> = Vec::new();
    while index < a.len() {
        let (token, i) = next_token(&a.chars().collect(), index);
        index = i;
        tokens.push(token);
    }
    return tokens;
}
// negative if a < b; 0 if a == b; positive if a > b
fn compare_lists(a_: &Vec<String>, b_: &Vec<String>) -> i32 {
    let mut a = a_.clone();
    let mut b = b_.clone();

    let mut a_index: usize = 0;
    let mut b_index: usize = 0;

    while a_index < a.len() && b_index < b.len() {

        // Case 1: left array is shorter, left < right
        if a[a_index] == "]" && b[b_index] != "]" {
            return -1;
        }

        // Case 2: right array is shorter, left > right
        else if a[a_index] != "]" && b[b_index] == "]" {
            return 1;
        }

        // Case 3: left we begin next array, but right is a number
        else if a[a_index] == "[" && b[b_index] != "[" {
            a_index += 1;
            b.insert(b_index + 1, "]".to_string());
            continue;
        }

        // Case 4: right we begin next array, but left is a number
        else if a[a_index] != "[" && b[b_index] == "[" {
            b_index += 1;
            a.insert(a_index + 1, "]".to_string());
            continue;
        }

        // Case 5: both are beginning of the array
        else if a[a_index] == "]" && b[b_index] == "]" {
            a_index += 1;
            b_index += 1;
            continue;
        }

        // Case 6: both are ending of the array
        else if a[a_index] == "[" && b[b_index] == "[" {
            a_index += 1;
            b_index += 1;
            continue;
        }

        // Case 7: both are numbers
        else {
            let a_num = a[a_index].parse::<i32>().unwrap();
            let b_num = b[b_index].parse::<i32>().unwrap();
            if a_num < b_num {
                return -1;
            }
            else if a_num > b_num {
                return 1;
            }
            else {
                a_index += 1;
                b_index += 1;
            }
        }
    }

    return 0;
}

fn main() {
    let input_file = utils::input_file_path();
    if let Err(e) = input_file {
        println!("{}", e);
        return;
    }

    let mut tokenized_arrays: Vec<Vec<String>> = Vec::new();

    if let Ok(lines) = utils::read_lines(input_file.unwrap()) {
        for line in lines {
            if let Ok(readed_line) = line {
                if readed_line.trim() != "" {
                    tokenized_arrays.push(tokenize(&readed_line));
                }
            }
        }
    }

    let mut part_1_sum = 0;
    for i in 0..tokenized_arrays.len()/2 {
        let a = &tokenized_arrays[i*2];
        let b = &tokenized_arrays[i*2+1];
       
        let comparison_result = compare_lists(a, b);
        //println!("{} {} {}", a.join(""), if comparison_result < 0 { "<" } else if comparison_result == 0 { "=" } else { ">" }, b.join(""));
        if comparison_result < 0 {
            part_1_sum += i + 1;
            
        }
    }

    println!("Part 1: {}", part_1_sum);

    tokenized_arrays.push(tokenize(&"[[2]]".to_string()));
    tokenized_arrays.push(tokenize(&"[[6]]".to_string()));

    tokenized_arrays.sort_by(|a, b| compare_lists(a, b).cmp(&0));
    
    let marker_2_index = tokenized_arrays.iter().position(|x| x == &tokenize(&"[[2]]".to_string())).unwrap() + 1;
    let marker_6_index = tokenized_arrays.iter().position(|x| x == &tokenize(&"[[6]]".to_string())).unwrap() + 1;

    println!("Part 2: {}", marker_2_index * marker_6_index);
}