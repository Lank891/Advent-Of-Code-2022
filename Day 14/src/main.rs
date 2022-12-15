mod utils;
use std::collections::HashSet;

type Point = (i32, i32);

fn generate_line(start: &Point, end: &Point) -> Vec<Point> {
    let mut line = Vec::new();
    let (x1, y1) = start;
    let (x2, y2) = end;

    if x1 == x2 {
        let mut y = *y1.min(y2);
        let y_max = *y1.max(y2);
        while y <= y_max {
            line.push((*x1, y));
            y += 1;
        }
    } else if y1 == y2 {
        let mut x = *x1.min(x2);
        let x_max = *x1.max(x2);
        while x <= x_max {
            line.push((x, *y1));
            x += 1;
        }
    }

    line
}

fn simulate_falling_sand(map: &HashSet<Point>, floor_level: Option<i32>) -> Option<Point> {
    let mut sand: Point = (500, 0);

    let steps_threshold = 1000; // If we fall for this many steps, we assume it is void :c
    for _ in 0..steps_threshold {
        // We are on the floor (if applicable)
        if floor_level.is_some() && sand.1 == floor_level.unwrap() - 1 {
            return Option::Some(sand);
        }

        let down: Point = (sand.0, sand.1 + 1);
        let left: Point = (sand.0 - 1, sand.1 + 1);
        let right: Point = (sand.0 + 1, sand.1 + 1);

        if !map.contains(&down) {
            sand = down;
        } else if !map.contains(&left) {
            sand = left;
        } else if !map.contains(&right) {
            sand = right;
        } else {
            return Option::Some(sand);
        }
    }

    return Option::None;
}

fn main() {
    let input_file = utils::input_file_path();
    if let Err(e) = input_file {
        println!("{}", e);
        return;
    }

    let mut map : HashSet<Point> = HashSet::new();
    let mut max_y = 0;

    if let Ok(lines) = utils::read_lines(input_file.unwrap()) {
        for line in lines {
            if let Ok(readed_line) = line {
                let mut points: Vec<Point> = Vec::new();
                readed_line.split(" -> ").for_each(|s| {
                    let mut p = s.split(",");
                    let point = (p.next().unwrap().parse::<i32>().unwrap(), p.next().unwrap().parse::<i32>().unwrap());
                    points.push(point);

                    if point.1 > max_y {
                        max_y = point.1;
                    }
                });

                let mut all_points_from_line: Vec<Point> = Vec::new();
                for i in 0..points.len() - 1 {
                    let line = generate_line(&points[i], &points[i + 1]);
                    all_points_from_line.extend(line);
                }
                
                for p in all_points_from_line {
                        map.insert(p);
                }
            }
        }
    }

    let mut map_part_1 = map.clone();
    let mut simulated_blocks_part_1 = 0;
    loop {
        let sand = simulate_falling_sand(&map_part_1, Option::None);
        if sand.is_none() {
            break;
        } else {
            map_part_1.insert(sand.unwrap());
            simulated_blocks_part_1 += 1;
        }
    }


    let mut map_part_2 = map.clone();
    let mut simulated_blocks_part_2 = 0;
    loop {
        let sand = simulate_falling_sand(&map_part_2, Option::Some(max_y + 2));
        simulated_blocks_part_2 += 1;
        if sand == Some((500, 0)) {
            break;
        } else {
            map_part_2.insert(sand.unwrap());
        }
    }

    println!("Part 1: {}", simulated_blocks_part_1);
    println!("Part 2: {}", simulated_blocks_part_2);
}