mod utils;
use regex::Regex;
use std::collections::HashSet;

type Point = (i32, i32);

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn char_to_direction(dir: char) -> Direction {
    match dir {
        'U' => Direction::Up,
        'D' => Direction::Down,
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => panic!("Invalid direction"),
    }
}

fn chebyshev_distance(a: Point, b: Point) -> i32 {
    let x = (a.0 - b.0).abs();
    let y = (a.1 - b.1).abs();
    x.max(y)
}

fn move_tail_to_head(head: Point, tail: &mut Point) {
    tail.0 += (head.0 - tail.0).signum();
    tail.1 += (head.1 - tail.1).signum();
}

fn make_move(rope: &mut Vec<Point>, dir: Direction) {
    match dir {
        Direction::Up => rope.last_mut().unwrap().1 += 1,
        Direction::Down => rope.last_mut().unwrap().1 -= 1,
        Direction::Left => rope.last_mut().unwrap().0 -= 1,
        Direction::Right => rope.last_mut().unwrap().0 += 1,
    }

    for i in (0..rope.len() - 1).rev() {
        if chebyshev_distance(rope[i], rope[i+1]) > 1 {
            move_tail_to_head(rope[i + 1], &mut rope[i]);
        }
    }
}

fn main() {
    let input_file = utils::input_file_path();
    if let Err(e) = input_file {
        println!("{}", e);
        return;
    }

    let instruction_regex = Regex::new(r"(U|D|R|L) (\d+)").unwrap();

    let mut part_1_rope: Vec<Point> = vec![(0, 0), (0, 0)];
    let mut part_1_visited_by_tail: HashSet<Point> = HashSet::new();
    part_1_visited_by_tail.insert((0, 0));

    let mut part_2_rope: Vec<Point> = vec![(0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)];
    let mut part_2_visited_by_tail: HashSet<Point> = HashSet::new();
    part_2_visited_by_tail.insert((0, 0));

    if let Ok(lines) = utils::read_lines(input_file.unwrap()) {
        for line in lines {
            if let Ok(readed_line) = line {
                
                let instruction = instruction_regex.captures(readed_line.as_str()).unwrap();
                
                let dir = char_to_direction(instruction.get(1).unwrap().as_str().chars().next().unwrap());
                let distance = instruction.get(2).unwrap().as_str().parse::<i32>().unwrap();
                for _ in 0..distance {
                    make_move(&mut part_1_rope, dir.clone());
                    part_1_visited_by_tail.insert(part_1_rope[0].clone());

                    make_move(&mut part_2_rope, dir.clone());
                    part_2_visited_by_tail.insert(part_2_rope[0].clone());
                }
                

            }
        }
    }

    println!("Part 1: {}", part_1_visited_by_tail.len());
    println!("Part 2: {}", part_2_visited_by_tail.len());
}