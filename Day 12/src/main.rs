mod utils;
use queues::*;

fn position_to_index(x: i32, y: i32, width: i32) -> i32 {
    y * width + x
}

fn index_to_position(index: i32, width: i32) -> (i32, i32) {
    (index % width, index / width)
}

fn ord(c: char) -> i32 {
    c as i32 - 'a' as i32
}

fn main() {
    let input_file = utils::input_file_path();
    if let Err(e) = input_file {
        println!("{}", e);
        return;
    }

    let mut start_index: i32 = 0;
    let mut end_index: i32 = 0;
    let mut map: Vec<i32> = Vec::new();
    let mut width: i32 = 0;
    let mut height: i32 = 0;

    if let Ok(lines) = utils::read_lines(input_file.unwrap()) {
        for line in lines {
            if let Ok(readed_line) = line {
                width = readed_line.len() as i32;

                for (i, c) in readed_line.chars().enumerate() {
                    if c.is_lowercase() {
                        map.push(ord(c));
                    }
                    else if c == 'S' {
                        map.push(ord('a')); // 0
                        start_index = position_to_index(i as i32, height, width);
                    }
                    else if c == 'E' {
                        map.push(ord('z'));
                        end_index = position_to_index(i as i32, height, width);}
                }
                
            }
            height += 1;
        }
    }

    let mut graph: Vec<Vec<i32>> = Vec::new();
    for i in 0..map.len() {
        graph.push(Vec::new());
        let (x, y) = index_to_position(i as i32, width);
        if x-1 >= 0 && map[position_to_index(x-1, y, width) as usize] <= map[i] + 1 {
            graph[i].push(position_to_index(x-1, y, width));
        }
        if x+1 < width && map[position_to_index(x+1, y, width) as usize] <= map[i] + 1 {
            graph[i].push(position_to_index(x+1, y, width));
        }
        if y-1 >= 0 && map[position_to_index(x, y-1, width) as usize] <= map[i] + 1 {
            graph[i].push(position_to_index(x, y-1, width));
        }
        if y+1 < height && map[position_to_index(x, y+1, width) as usize] <= map[i] + 1 {
            graph[i].push(position_to_index(x, y+1, width));
        }
    }

    let mut visited_part_1: Vec<bool> = vec![false; map.len()];
    let mut next_in_path_part_1: Vec<i32> = vec![0; map.len()];
    let mut queue_part_1: Queue<i32> = Queue::new();

    visited_part_1[start_index as usize] = true;
    let _ = queue_part_1.add(start_index);
    while queue_part_1.size() > 0 {
        let current = queue_part_1.remove().unwrap();
        if current == end_index {
            break;
        }
        for next in graph[current as usize].iter() {
            if !visited_part_1[*next as usize] {
                visited_part_1[*next as usize] = true;
                next_in_path_part_1[*next as usize] = current;
                let _ = queue_part_1.add(*next);
            }
        }
    }

    let mut part_1_path_length = 0;
    let mut current = end_index;
    while current != start_index {
        current = next_in_path_part_1[current as usize];
        part_1_path_length += 1;
    }



    let mut visited_part_2: Vec<bool> = vec![false; map.len()];
    let mut next_in_path_part_2: Vec<i32> = vec![0; map.len()];
    let mut queue_part_2: Queue<i32> = Queue::new();

    map.iter().enumerate().filter(|(_, h)| **h == ord('a')).for_each(|(i, _)| {
        visited_part_2[i] = true;
        let _ = queue_part_2.add(i as i32);
    });
    while queue_part_2.size() > 0 {
        let current = queue_part_2.remove().unwrap();
        if current == end_index {
            break;
        }
        for next in graph[current as usize].iter() {
            if !visited_part_2[*next as usize] {
                visited_part_2[*next as usize] = true;
                next_in_path_part_2[*next as usize] = current;
                let _ = queue_part_2.add(*next);
            }
        }
    }

    let mut part_2_path_length = 0;
    let mut current = end_index;
    while map[current as usize] != ord('a') {
        current = next_in_path_part_2[current as usize];
        part_2_path_length += 1;
    }

    //println!("{:?}", graph);

    println!("Part 1: {}", part_1_path_length);
    println!("Part 2: {}", part_2_path_length);
}