mod utils;

fn snafu_to_dec(n: String) -> i64 {
    let mut mult = 1;
    let mut dec = 0;

    for c in n.chars().rev() {
        match c {
            '=' => dec -= mult * 2,
            '-' => dec -= mult,
            '0' => (),
            '1' => dec += mult,
            '2' => dec += mult * 2,
            _ => panic!("Invalid character in SNAFU number!"),
        }
        mult *= 5;
    }

    dec
}

fn dec_to_snafu(n: i64) -> String {
    let mut snafu = String::new();
    let mut dec = n;

    while dec > 0 {
        let mut add = 0;
        match dec % 5 {
            0 => snafu = format!("0{}", snafu),
            1 => snafu = format!("1{}", snafu),
            2 => snafu = format!("2{}", snafu),
            3 => {
                snafu = format!("={}", snafu);
                add = 1;
            }
            4 => {
                snafu = format!("-{}", snafu);
                add = 1;
            }
            _ => panic!("Invalid decimal number!"),
        }
        dec /= 5;
        dec += add;
    }

    snafu
}

fn main() {
    let input_file = utils::input_file_path();
    if let Err(e) = input_file {
        println!("{}", e);
        return;
    }

    let mut input: Vec<String> = Vec::new();

    if let Ok(lines) = utils::read_lines(input_file.unwrap()) {
        for line in lines {
            if let Ok(readed_line) = line {
                
                input.push(readed_line);
                
            }
        }
    }

    let decimals = input.iter().map(|x| snafu_to_dec(x.to_string())).collect::<Vec<i64>>();
    let sum = decimals.iter().sum::<i64>();

    let snafu = dec_to_snafu(sum);

    println!("Part 1: {}", snafu);
    println!("Part 2: {}", "Not today :)");
}