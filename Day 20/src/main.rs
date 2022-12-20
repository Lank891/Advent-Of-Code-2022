mod utils;

fn move_vec(buffer: &mut Vec<(i64, i64)>, index: usize) {
    let steps = buffer[index].1;
    let mut s = index as i64 + steps;
    while s < 0 {
        s += (buffer.len() as i64 - 1) * 100_000_000; // 100_000_000 is just a big number to speed up this case - later modulo is used anyway
    }
    s = s % (buffer.len() as i64 - 1);

    if steps < 0 && s == 0 {
        s = buffer.len() as i64 - 1;
    }
    if steps > 0 && s == buffer.len() as i64 - 1 {
        s = 0;
    }

    let item = buffer.remove(index);
    buffer.insert(s as usize, item);
}

fn main() {
    let input_file = utils::input_file_path();
    if let Err(e) = input_file {
        println!("{}", e);
        return;
    }

    let mut buffer: Vec<(i64, i64)> = Vec::new();

    if let Ok(lines) = utils::read_lines(input_file.unwrap()) {
        let mut index = 0;
        for line in lines {
            if let Ok(readed_line) = line {
                
                buffer.push((index, readed_line.parse::<i64>().unwrap()));
                index += 1;
            }
        }
    }

   //println!("{:?}", buffer.iter().map(|x| x.1).collect::<Vec<i64>>());

    let mut part_1_buffer = buffer.clone();
    for i in 0..part_1_buffer.len() {
        let index = part_1_buffer.iter().position(|x| x.0 == i as i64).unwrap();
        move_vec(&mut part_1_buffer, index as usize);
        //println!("{:?}", part_1_buffer.iter().map(|x| x.1).collect::<Vec<i64>>());   
    }

    //println!("Part 1 finished");

    
    let part_1_start_index = part_1_buffer.iter().position(|x| x.1 == 0).unwrap();
    let sum_part_1 = 
        part_1_buffer[(part_1_start_index + 1000) % buffer.len()].1 + 
        part_1_buffer[(part_1_start_index + 2000) % buffer.len()].1 + 
        part_1_buffer[(part_1_start_index + 3000) % buffer.len()].1;

    let decyption_key = 811589153;
    let mut part_2_buffer = buffer.iter().map(|k| (k.0, k.1*decyption_key)).collect::<Vec<(i64, i64)>>();
    for _ in 0..10 {
        for i in 0..part_2_buffer.len() {
            let index = part_2_buffer.iter().position(|x| x.0 == i as i64).unwrap();
            move_vec(&mut part_2_buffer, index as usize);
            //println!("{:?}", buffer.iter().map(|x| x.1).collect::<Vec<i64>>());   
        }
        //println!("Part 2 finished {} times", i+1);
    }

    let part_2_start_index = part_2_buffer.iter().position(|x| x.1 == 0).unwrap();
    let sum_part_2 =
        part_2_buffer[(part_2_start_index + 1000) % buffer.len()].1 + 
        part_2_buffer[(part_2_start_index + 2000) % buffer.len()].1 + 
        part_2_buffer[(part_2_start_index + 3000) % buffer.len()].1;

    println!("Part 1: {}", sum_part_1);
    println!("Part 2: {}", sum_part_2);
}