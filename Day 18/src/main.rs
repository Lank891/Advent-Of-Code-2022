mod utils;
use std::collections::HashSet;
use std::collections::VecDeque;

type Point = (i32, i32, i32);

fn generate_neighbours_for_point(point: &Point) -> Vec<Point> {
    let mut neighbours = Vec::new();
    
    neighbours.push((point.0 - 1, point.1, point.2));
    neighbours.push((point.0 + 1, point.1, point.2));
    neighbours.push((point.0, point.1 - 1, point.2));
    neighbours.push((point.0, point.1 + 1, point.2));
    neighbours.push((point.0, point.1, point.2 - 1));
    neighbours.push((point.0, point.1, point.2 + 1));

    neighbours
}

fn generate_void_fill(cubes: &HashSet<Point>) -> HashSet<Point> {
    let mut void_fill = HashSet::new();

    let mut queue: VecDeque<Point> = VecDeque::new();

    // Enough for my input
    let max_coord = 22;
    let min_coord = -2;

    let starting_point = (min_coord, min_coord, min_coord);
    queue.push_back(starting_point);
    void_fill.insert(starting_point);

    while !queue.is_empty() {

        let next_element = queue.pop_front().unwrap();
        let neighbours = generate_neighbours_for_point(&next_element);
        
        for neighbour in neighbours {
            if neighbour.0 < min_coord || neighbour.0 > max_coord || neighbour.1 < min_coord || neighbour.1 > max_coord || neighbour.2 < min_coord || neighbour.2 > max_coord {
                continue;
            }

            if cubes.contains(&neighbour) {
                continue;
            }

            if void_fill.contains(&neighbour) {
                continue;
            }

            void_fill.insert(neighbour);
            queue.push_back(neighbour);
        }
    }

    return void_fill;
}


fn main() {
    let input_file = utils::input_file_path();
    if let Err(e) = input_file {
        println!("{}", e);
        return;
    }

    let mut cubes: HashSet<Point> = HashSet::new();

    if let Ok(lines) = utils::read_lines(input_file.unwrap()) {
        for line in lines {
            if let Ok(readed_line) = line {
                let points: Vec<&str> = readed_line.split(',').collect();
                cubes.insert((points[0].parse::<i32>().unwrap(), points[1].parse::<i32>().unwrap(), points[2].parse::<i32>().unwrap()));
                
            }
        }
    }

    let possible_walls: Vec<Vec<Point>> = cubes.iter().map(|point| generate_neighbours_for_point(point)).collect();
    let all_possible_neighbours = possible_walls.iter().flatten().collect::<Vec<&Point>>();
    let neigbours_not_overlapping_with_existing_cubes = all_possible_neighbours.iter().filter(|point| !cubes.contains(point)).map(|point| *point.clone()).collect::<Vec<Point>>();

    let area_part_1 = neigbours_not_overlapping_with_existing_cubes.len();

    let void_blocks = generate_void_fill(&cubes);

    let neighbours_touching_void = neigbours_not_overlapping_with_existing_cubes.iter().filter(|point| void_blocks.contains(point)).map(|point| point.clone()).collect::<Vec<Point>>();

    let area_part_2 = neighbours_touching_void.len();

    println!("Part 1: {}", area_part_1);
    println!("Part 2: {}", area_part_2);
}