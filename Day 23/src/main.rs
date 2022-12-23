mod utils;
use std::collections::HashSet;
use std::collections::HashMap;

type Point = (i32, i32);

fn move_north(elf: Point, elves: &HashSet<Point>) -> Option<Point> {
    let (x, y) = elf;
    let north = (x, y - 1);
    let north_east = (x + 1, y - 1);
    let north_west = (x - 1, y - 1);

    if elves.contains(&north) || elves.contains(&north_east) || elves.contains(&north_west) {
        None
    } else {
        Some(north)
    }
}

fn move_south(elf: Point, elves: &HashSet<Point>) -> Option<Point> {
    let (x, y) = elf;
    let south = (x, y + 1);
    let south_east = (x + 1, y + 1);
    let south_west = (x - 1, y + 1);

    if elves.contains(&south) || elves.contains(&south_east) || elves.contains(&south_west) {
        None
    } else {
        Some(south)
    }
}

fn move_west(elf: Point, elves: &HashSet<Point>) -> Option<Point> {
    let (x, y) = elf;
    let west = (x - 1, y);
    let north_west = (x - 1, y - 1);
    let south_west = (x - 1, y + 1);

    if elves.contains(&west) || elves.contains(&north_west) || elves.contains(&south_west) {
        None
    } else {
        Some(west)
    }
}

fn move_east(elf: Point, elves: &HashSet<Point>) -> Option<Point> {
    let (x, y) = elf;
    let east = (x + 1, y);
    let north_east = (x + 1, y - 1);
    let south_east = (x + 1, y + 1);

    if elves.contains(&east) || elves.contains(&north_east) || elves.contains(&south_east) {
        None
    } else {
        Some(east)
    }
}

fn should_elf_move(elf: Point, elves: &HashSet<Point>) -> bool {
    let (x, y) = elf;
    let north = (x, y - 1);
    let south = (x, y + 1);
    let west = (x - 1, y);
    let east = (x + 1, y);

    if elves.contains(&north) || elves.contains(&south) || elves.contains(&west) || elves.contains(&east) {
        return true;
    }

    let north_east = (x + 1, y - 1);
    let north_west = (x - 1, y - 1);
    let south_east = (x + 1, y + 1);
    let south_west = (x - 1, y + 1);

    if elves.contains(&north_east) || elves.contains(&north_west) || elves.contains(&south_east) || elves.contains(&south_west) {
        return true;
    }

    false
}

fn get_area(elves: &HashSet<Point>) -> i32 {
    let mut min_x = i32::max_value();
    let mut max_x = i32::min_value();
    let mut min_y = i32::max_value();
    let mut max_y = i32::min_value();

    for elf in elves {
        let (x, y) = *elf;
        if x < min_x {
            min_x = x;
        }
        if x > max_x {
            max_x = x;
        }
        if y < min_y {
            min_y = y;
        }
        if y > max_y {
            max_y = y;
        }
    }

    (max_x - min_x + 1) * (max_y - min_y + 1)
}

fn main() {
    let input_file = utils::input_file_path();
    if let Err(e) = input_file {
        println!("{}", e);
        return;
    }

    let mut elves: HashSet<Point> = HashSet::new();
    let mut movements: Vec<fn(Point, &HashSet<Point>) -> Option<Point>> = vec![move_north, move_south, move_west, move_east];

    let mut y = 0;
    if let Ok(lines) = utils::read_lines(input_file.unwrap()) {
        for line in lines {
            if let Ok(readed_line) = line {
                
                readed_line.chars().enumerate().for_each(|(x, c)| {
                    if c == '#' {
                        elves.insert((x as i32, y));
                    }
                });
                
            }
            y += 1;
        }
    }

    //println!("{:?}", elves);

    let mut part_1_score = 0;
    let mut round_where_elves_stopped_moving = 0;

    for i in 0..10000 {
        let mut elves_moves: HashMap<Point, Point> = HashMap::new();
        let mut destination_points: HashSet<Point> = HashSet::new();
        let mut doubled_destination_points: HashSet<Point> = HashSet::new();

        for elf in &elves {
            if should_elf_move(*elf, &elves) {

                let mut moved = false;
                for movement in &movements {
                    if let Some(new_elf) = movement(*elf, &elves) {
                        elves_moves.insert(*elf, new_elf);

                        if destination_points.contains(&new_elf)  {
                            doubled_destination_points.insert(new_elf);
                        } else {
                            destination_points.insert(new_elf);
                        }

                        moved = true;
                        break;
                    }
                }

                if !moved {
                    elves_moves.insert(*elf, *elf);
                    if destination_points.contains(elf)  {
                        doubled_destination_points.insert(*elf);
                    } else {
                        destination_points.insert(*elf);
                    }
                }

            } else {
                elves_moves.insert(*elf, *elf);
                if destination_points.contains(elf)  {
                    doubled_destination_points.insert(*elf);
                } else {
                    destination_points.insert(*elf);
                }
            }
        }

        let mut elves_copy = elves.clone();
        for elf in &elves {
            if let Some(new_elf) = elves_moves.get(elf) {
                if !doubled_destination_points.contains(new_elf) {
                    elves_copy.remove(elf);
                    elves_copy.insert(*new_elf);
                }
            }
        }
        if i == 10 {
            part_1_score = get_area(&elves_copy) - elves.len() as i32;
        }

        if elves == elves_copy  {
            round_where_elves_stopped_moving = i + 1;
            break;
        }

        elves = elves_copy;

        movements.rotate_left(1);
    }

    println!("Part 1: {}", part_1_score);
    println!("Part 2: {}", round_where_elves_stopped_moving);
}