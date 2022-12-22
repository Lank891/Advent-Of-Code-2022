mod utils;
use regex::Regex;
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Place {
    Void,
    Empty,
    Wall,
}

impl From<char> for Place {
    fn from(item: char) -> Self {
        match item {
            '#' => Place::Wall,
            '.' => Place::Empty,
            ' ' => Place::Void,
            _ => panic!("Invalid place"),
        }
    }
}

type Position = (i32, i32);

#[derive(Copy, Clone, PartialEq, Eq)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl From<i32> for Direction {
    fn from(item: i32) -> Self {
        match item {
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Up,
            _ => panic!("Invalid direction"),
        }
    }
}

type PlayerState = (Position, Direction);

// Right, Down, Left, Up; each option is a new position and a new direction
type NeighboursWithDirectionChange = (Option<(Position, Direction)>, Option<(Position, Direction)>, Option<(Position, Direction)>, Option<(Position, Direction)>);

fn get_neighbours_with_wraparoud_p1(position: Position, map: &Vec<Vec<Place>>, width: usize, height: usize) -> NeighboursWithDirectionChange {
    let (x, y) = position;
    let mut neighbours: NeighboursWithDirectionChange = (None, None, None, None);

    let mut left = (x - 1, y);
    if left.0 < 0 {
        left.0 = width as i32 - 1;
    }
    while map[left.1 as usize][left.0 as usize] == Place::Void {
        left.0 -= 1;
        if left.0 < 0 {
            left.0 = width as i32 - 1;
        }
    }
    if map[left.1 as usize][left.0 as usize] == Place::Empty {
        neighbours.2 = Some((left, Direction::Left));
    } else {
        neighbours.2 = None;
    }

    let mut right = (x + 1, y);
    if right.0 >= width as i32 {
        right.0 = 0;
    }
    while map[right.1 as usize][right.0 as usize] == Place::Void {
        right.0 += 1;
        if right.0 >= width as i32 {
            right.0 = 0;
        }
    }
    if map[right.1 as usize][right.0 as usize] == Place::Empty {
        neighbours.0 = Some((right, Direction::Right));
    } else {
        neighbours.0 = None;
    }

    let mut up = (x, y - 1);
    if up.1 < 0 {
        up.1 = height as i32 - 1;
    }

    while map[up.1 as usize][up.0 as usize] == Place::Void {
        up.1 -= 1;
        if up.1 < 0 {
            up.1 = height as i32 - 1;
        }
    }
    if map[up.1 as usize][up.0 as usize] == Place::Empty {
        neighbours.3 = Some((up, Direction::Up));
    } else {
        neighbours.3 = None;
    }

    let mut down = (x, y + 1);
    if down.1 >= height as i32 {
        down.1 = 0;
    }
    while map[down.1 as usize][down.0 as usize] == Place::Void {
        down.1 += 1;
        if down.1 >= height as i32 {
            down.1 = 0;
        }
    }
    if map[down.1 as usize][down.0 as usize] == Place::Empty {
        neighbours.1 = Some((down, Direction::Down));
    } else {
        neighbours.1 = None;
    }

    return neighbours;
}

fn get_neighbours_for_p2_specific_map_layout(position: Position, map: &Vec<Vec<Place>>, size: usize) -> NeighboursWithDirectionChange {
    // We are assuming the map is a unfolded cube size x size x size
    // Very input specific, expected layout is
    //
    // ░░░ ▓B▓ ▓C▓
    // ░░░ ▓A▓ ░░░
    // ▓D▓ ▓E▓ ░░░
    // ▓F▓ ░░░ ░░░

    let width = 3 * size;
    let height = 4 * size;

    let (x, y) = position;
    let mut neighbours: NeighboursWithDirectionChange = (None, None, None, None);

    let mut left = (x - 1, y);
    let mut left_direction = Direction::Left;
    if left.0 < 0 {
        left.0 = width as i32 - 1;
    }
    if map[left.1 as usize][left.0 as usize] == Place::Void {
        // Case 1: We are in wall B -> we swap to left of wall D and direction is Right
        if y < size as i32 {
            let ydiff = y;
            left = (0, 3 * size as i32 - 1 - ydiff);
            left_direction = Direction::Right;
        }
        // Case 2: We are in wall A -> we swap to up of wall D and direction is Down
        if y >= size as i32 && y < 2 * size as i32 {
            let ydiff = y - size as i32;
            left = (ydiff, 2 * size as i32);
            left_direction = Direction::Down;
        }
        // Case 3: We are in wall D -> we swap to left of wall B and direction is Right
        if y >= 2 * size as i32 && y < 3 * size as i32 {
            let ydiff = y - 2 * size as i32;
            left = (size as i32, size as i32 - 1 - ydiff);
            left_direction = Direction::Right;
        }
        // Case 4: We are in wall F -> we swap to up of wall B and direction is Down
        if y >= 3 * size as i32 {
            let ydiff = y - 3 * size as i32;
            left = (size as i32 + ydiff, 0);
            left_direction = Direction::Down;
        }
    }
    if map[left.1 as usize][left.0 as usize] == Place::Empty {
        neighbours.2 = Some((left, left_direction));
    } else {
        neighbours.2 = None;
    }

    let mut right = (x + 1, y);
    let mut right_direction = Direction::Right;
    if right.0 >= width as i32 {
        right.0 = 0;
    }
    if map[right.1 as usize][right.0 as usize] == Place::Void {
        // Case 1: We are in C -> We swap to right of wall E and direction is Left
        if y < size as i32 {
            let ydiff = y;
            right = (2 * size as i32 - 1, 3 * size as i32 - 1 - ydiff);
            right_direction = Direction::Left;
        }
        // Case 2: We are in A -> We swap to down of wall C and direction is Up
        if y >= size as i32 && y < 2 * size as i32 {
            let ydiff = y- size as i32;
            right = (2 * size as i32 + ydiff, size as i32 - 1);
            right_direction = Direction::Up;
        }
        // Case 3: We are in E -> We swap to right of wall C and direction is Left
        if y >= 2 * size as i32 && y < 3 * size as i32 {
            let ydiff = y - 2 * size as i32;
            right = (3 * size as i32 - 1, size as i32 - 1 - ydiff);
            right_direction = Direction::Left;
        }
        // Case 4: We are in F -> We swap to down of wall E and direction is Up
        if y >= 3 * size as i32 {
            let ydiff = y - 3 * size as i32;
            right = (size as i32 + ydiff, 3 * size as i32 - 1);
            right_direction = Direction::Up;
        }
    }
    if map[right.1 as usize][right.0 as usize] == Place::Empty {
        neighbours.0 = Some((right, right_direction));
    } else {
        neighbours.0 = None;
    }

    let mut up = (x, y - 1);
    let mut up_direction = Direction::Up;
    if up.1 < 0 {
        up.1 = height as i32 - 1;
    }
    if map[up.1 as usize][up.0 as usize] == Place::Void {
        // Case 1: We are in wall D -> we swap to left of wall A and direction is Right
        if x < size as i32 {
            let xdiff = x;
            up = (size as i32, size as i32 + xdiff);
            up_direction = Direction::Right;
        }
        // Case 2: We are in wall B -> we swap to left of wall F and direction is Right
        if x >= size as i32 && x < 2 * size as i32 {
            let xdiff = x - size as i32;
            up = (0, 3 * size as i32 + xdiff);
            up_direction = Direction::Right;
        }
        // Case 3: We are in wall C -> we swap to down of wall F and direction is Up
        if x >= 2 * size as i32 && x < 3 * size as i32 {
            let xdiff = x - 2 * size as i32;
            up = (xdiff, 4 * size as i32 - 1);
            up_direction = Direction::Up;
        }
    }
    if map[up.1 as usize][up.0 as usize] == Place::Empty {
        neighbours.3 = Some((up, up_direction));
    } else {
        neighbours.3 = None;
    }

    let mut down = (x, y + 1);
    let mut down_direction = Direction::Down;
    if down.1 >= height as i32 {
        down.1 = 0;
    }
    if map[down.1 as usize][down.0 as usize] == Place::Void {
        // Case 1: We are in wall F -> we swap to up of wall C and direction is Down
        if x < size as i32 {
            let xdiff = x;
            down = (2 * size as i32 + xdiff, 0);
            down_direction = Direction::Down;
        }
        // Case 2: We are in wall E -> we swap to right of wall F and direction is Left
        if x >= size as i32 && x < 2 * size as i32 {
            let xdiff = x - size as i32;
            down = (size as i32 - 1, 3 * size as i32 + xdiff);
            down_direction = Direction::Left;
        }
        // Case 3: We are in wall C -> we swap to right of wall A and direction is Left
        if x >= 2 * size as i32 && x < 3 * size as i32 {
            let xdiff = x - 2 * size as i32;
            down = (2 * size as i32 - 1, size as i32 + xdiff);
            down_direction = Direction::Left;
        }
    }
    if map[down.1 as usize][down.0 as usize] == Place::Empty {
        neighbours.1 = Some((down, down_direction));
    } else {
        neighbours.1 = None;
    }

    return neighbours;
}

fn print_map(map: &Vec<Vec<Place>>, player: Option<PlayerState>) {
    for (y, line) in map.iter().enumerate() {
        for (x, place) in line.iter().enumerate() {
            if let Some(((px, py), dir)) = player {
                if px == x as i32 && py == y as i32 {
                    match dir {
                        Direction::Right => print!(" ►"),
                        Direction::Down => print!(" ▼"),
                        Direction::Left => print!("◄ "),
                        Direction::Up => print!("▲ "),
                    }
                    continue;
                }
            }
            match place {
                Place::Void => print!(".."),
                Place::Empty => print!("░░"),
                Place::Wall => print!("▓▓"),
            }
        }
        println!();
    }
}

fn rotate(direction: Direction, turn: char) -> Direction {
    let mut new_direction = direction as i32;
    if turn == 'L' {
        new_direction -= 1;
    }
    else {
        new_direction += 1;
    }

    if new_direction < 0 {
        new_direction = 3;
    }
    else if new_direction > 3 {
        new_direction = 0;
    }

    return Direction::from(new_direction);
}

fn travel(player: &mut PlayerState, map: &HashMap<Position, NeighboursWithDirectionChange>, instructions: &Vec<Result<i32, char>>, map_for_drawing: &Vec<Vec<Place>>, print: bool) {
    for instruction in instructions {
        if let Ok(n) = instruction {
            for _ in 0..*n {
                let neighbours = map.get(&player.0).unwrap();
                match player.1 {
                    Direction::Right => {
                        if let Some(((x, y), dir)) = neighbours.0 {
                            player.0 = (x, y);
                            player.1 = dir;
                        }
                    },
                    Direction::Down => {
                        if let Some(((x, y), dir)) = neighbours.1 {
                            player.0 = (x, y);
                            player.1 = dir;
                        }
                    },
                    Direction::Left => {
                        if let Some(((x, y), dir)) = neighbours.2 {
                            player.0 = (x, y);
                            player.1 = dir;
                        }
                    },
                    Direction::Up => {
                        if let Some(((x, y), dir)) = neighbours.3 {
                            player.0 = (x, y);
                            player.1 = dir;
                        }
                    },
                }
                if print {
                    println!("\n");
                    print_map(map_for_drawing, Some(*player));
                }
                
            }
        }
        else {
            player.1 = rotate(player.1, instruction.unwrap_err());
            if print {
                println!("\n");
                print_map(map_for_drawing, Some(*player));
            }
        }

        
    }
}

fn get_score(player: &PlayerState) -> i32 {
    let ((x, y), dir) = player;
    return 1000 * (y + 1) + 4 * (x + 1) + *dir as i32;
}

fn main() {
    let input_file = utils::input_file_path();
    if let Err(e) = input_file {
        println!("{}", e);
        return;
    }

    let mut map: Vec<Vec<Place>> = Vec::new();
    let mut width = 0;
    let mut height = 0;

    let instruction_regex = Regex::new(r"(\d+)?(R|L)?").unwrap();
    let mut instructions: Vec<Result<i32, char>> = Vec::new();

    let mut reading_cube = true;
    if let Ok(lines) = utils::read_lines(input_file.unwrap()) {
        for line in lines {
            if let Ok(readed_line) = line {

                if readed_line.trim().is_empty() {
                    reading_cube = false;
                    continue;
                }
                
                if reading_cube {
                    height += 1;
                    let mut map_line: Vec<Place> = Vec::new();
                    readed_line.chars().for_each(|c| map_line.push(Place::from(c)));
                    width = std::cmp::max(width, map_line.len());
                    map.push(map_line);
                }
                else {
                    instruction_regex.captures_iter(&readed_line).for_each(|cap| {
                        if cap.get(1).is_some() {
                            instructions.push(Ok(cap[1].parse::<i32>().unwrap()));
                        }
                        if cap.get(2).is_some() {
                            instructions.push(Err(cap[2].to_string().chars().next().unwrap()));
                        }
                    })
                }
            }
        }
    }

    for line in &mut map {
        while line.len() < width {
            line.push(Place::Void);
        }
    }

    //print_map(&map, None);
    let player: PlayerState = ((map[0].iter().position(|p| p == &Place::Empty).unwrap() as i32, 0), Direction::Right);
    let mut player_part_1 = player.clone();
    let mut player_part_2 = player.clone();
    //print_map(&map, Some(player));

    let mut map_part_1 = HashMap::<Position, NeighboursWithDirectionChange>::new();
    let mut map_part_2 = HashMap::<Position, NeighboursWithDirectionChange>::new();

    for y in 0..height {
        for x in 0..width {
            if map[y][x] == Place::Empty {
                map_part_1.insert((x as i32, y as i32), get_neighbours_with_wraparoud_p1((x as i32, y as i32), &map, width, height));
                map_part_2.insert((x as i32, y as i32), get_neighbours_for_p2_specific_map_layout((x as i32, y as i32), &map, 50));
            }
        }
    }
    
    travel(&mut player_part_1, &map_part_1, &instructions, &map, false);
    travel(&mut player_part_2, &map_part_2, &instructions, &map, false);
    
    //println!("\n\n\n");

    //print_map(&map, Some(player));
    //println!("{:?}", map_part_1);

    println!("Part 1: {}", get_score(&player_part_1));
    println!("Part 2: {}", get_score(&player_part_2));
}