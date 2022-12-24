mod utils;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;

type Point = (i32, i32);

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl From<char> for Direction {
    fn from(item: char) -> Self {
        match item {
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '^' => Direction::Up,
            _ => panic!("Invalid direction"),
        }
    }
}

type Blizzard = (Point, Direction);

fn next_blizzard(blizzards: &HashSet<Blizzard>, width: i32, height: i32) -> HashSet<Blizzard> {
    let mut new_blizzards: HashSet<Blizzard> = HashSet::new();

    for blizzard in blizzards {
        let (point, direction) = blizzard;
        let (x, y) = *point;

        let mut new_point = match direction {
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Up => (x, y - 1),
        };

        if new_point.0 == 0 && *direction == Direction::Left {
            new_point.0 = width - 2;
        }
        if new_point.0 == width - 1 && *direction == Direction::Right {
            new_point.0 = 1;
        }
        if new_point.1 == 0 && *direction == Direction::Up {
            new_point.1 = height - 2;
        }
        if new_point.1 == height - 1 && *direction == Direction::Down {
            new_point.1 = 1;
        }

        new_blizzards.insert((new_point, *direction));
    }

    return new_blizzards;
}

fn simulate_all_blizzards(blizzards: &HashSet<Blizzard>, width: i32, height: i32) -> HashMap<i32, HashSet<Blizzard>> {
    let mut possible_blizzards: HashMap<i32, HashSet<Blizzard>> = HashMap::new();

    let mut minute = 0;
    possible_blizzards.insert(minute, blizzards.clone());

    loop {
        let new_blizzards = next_blizzard(&possible_blizzards.get(&minute).unwrap(), width, height);
        minute += 1;

        if new_blizzards == *blizzards {
            break;
        }

        possible_blizzards.insert(minute, new_blizzards);
    }

    return possible_blizzards;
}

fn is_blizzard_at_point(point: Point, blizzards: &HashSet<Blizzard>) -> bool {
    let possible_points = vec![
        (point, Direction::Right),
        (point, Direction::Down),
        (point, Direction::Left),
        (point, Direction::Up),
    ];

    if possible_points.iter().any(|p| blizzards.contains(p)) {
        return true;
    }

    return false;
}

// Assume starting point is (1, 0), end point is (width - 2, height - 1), and walls are at the edges except those 2 points
fn get_neighbours(point: Point, width: i32, height: i32) -> Vec<Point> {
    let (x, y) = point;
    let neighbours: Vec<Point> = vec![(x-1, y), (x+1, y), (x, y-1), (x, y+1)];
    
    neighbours.iter().filter(|p| {
        let (x, y) = **p;
        (x > 0 && x < width - 1 && y > 0 && y < height - 1) || (**p == (1, 0) || **p == (width - 2, height - 1))
    }).map(|p| *p).collect::<Vec<Point>>()
}

fn bfs(all_blizzards: &HashMap<i32, HashSet<Blizzard>>, width: i32, height: i32, start: Point, end: Point, starting_time: i32) -> i32 {
    let no_of_blizzards = all_blizzards.len();

    let mut queue: VecDeque<(Point, i32)> = VecDeque::new();
    queue.push_back((start, starting_time));
    let mut visited: HashSet<(Point, i32)> = HashSet::new();
    visited.insert((end, starting_time));

    loop {
        let (point, minute) = queue.pop_front().unwrap();
        //println!("Visiting point {:?} at minute {}", point, minute);
        let new_minute = minute + 1;
        if point == end {
            return  minute;
        }

        let mut neighbours = get_neighbours(point, width, height);
        neighbours.push(point); // We can stay in the same place
        let blizzards = all_blizzards.get(&((new_minute % no_of_blizzards as i32))).unwrap();
        for neighbour in neighbours {
            if is_blizzard_at_point(neighbour, blizzards) || visited.contains(&(neighbour, new_minute)) {
                continue;
            }

            queue.push_back((neighbour, new_minute));
            visited.insert((neighbour, new_minute));
        }
    }
}

fn main() {
    let input_file = utils::input_file_path();
    if let Err(e) = input_file {
        println!("{}", e);
        return;
    }

    let mut width = 0;
    let mut height = 0;

    let mut initial_blizzards: HashSet<Blizzard> = HashSet::new();

    if let Ok(lines) = utils::read_lines(input_file.unwrap()) {
        for line in lines {
            if let Ok(readed_line) = line {
                width = readed_line.len();

                for (i, c) in readed_line.chars().enumerate() {
                    if c != '.' && c != '#' {
                        initial_blizzards.insert(((i as i32, height), Direction::from(c)));
                    }
                }
                
            }
            height += 1;
        }
    }

    let all_blizzards = simulate_all_blizzards(&initial_blizzards, width as i32, height as i32);
    //println!("{:?}", all_blizzards);

    let start_point = (1, 0);
    let end_point = (width as i32 - 2, height as i32 - 1);

    let start_to_end = bfs(&all_blizzards, width as i32, height as i32, start_point, end_point, 0);
    let end_to_start = bfs(&all_blizzards, width as i32, height as i32, end_point, start_point, start_to_end);
    let start_to_end_again = bfs(&all_blizzards, width as i32, height as i32, start_point, end_point, end_to_start);

    println!("Part 1: {}", start_to_end);
    println!("Part 2: {}", start_to_end_again);
}