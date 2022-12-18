mod utils;
use std::collections::HashSet;
use std::collections::HashMap;

static HASH_BOARD_ROWS: usize = 5000;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Jet {
    Left,
    Right,
}

fn letter_to_jet(letter: &char) -> Jet {
    match letter {
        '<' => Jet::Left,
        '>' => Jet::Right,
        _ => panic!("Invalid letter in input"),
    }
}



#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

fn make_horizontal_line_shape(y: usize) -> HashSet<Point> {
    let mut points: HashSet<Point> = HashSet::new();
    for i in 0..4 {
        points.insert(Point::new(2 + i, y));
    }
    return points;
}

fn make_plus_shape(y: usize) -> HashSet<Point> {
    let mut points: HashSet<Point> = HashSet::new();
    for i in 0..3 {
        points.insert(Point::new(2 + i, y + 1));
    }
    for i in 0..3 {
        points.insert(Point::new(3, y + i));
    }
    return points;
}

fn make_j_shape(y: usize) -> HashSet<Point> {
    let mut points: HashSet<Point> = HashSet::new();
    for i in 0..3 {
        points.insert(Point::new(2 + i, y));
    }
    for i in 0..3 {
        points.insert(Point::new(4, y + i));
    }
    return points;
}

fn make_vertical_line_shape(y: usize) -> HashSet<Point> {
    let mut points: HashSet<Point> = HashSet::new();
    for i in 0..4 {
        points.insert(Point::new(2, y+i));
    }
    return points;
}

fn make_square_shape(y: usize) -> HashSet<Point> {
    let mut points: HashSet<Point> = HashSet::new();
    for i in 0..2 {
        points.insert(Point::new(2 + i, y));
    }
    for i in 0..2 {
        points.insert(Point::new(2 + i, y + 1));
    }
    return points;
}


fn main() {
    let input_file = utils::input_file_path();
    if let Err(e) = input_file {
        println!("{}", e);
        return;
    }

    let mut wind: Vec<Jet> = Vec::new();

    if let Ok(lines) = utils::read_lines(input_file.unwrap()) {
        for line in lines {
            if let Ok(readed_line) = line {
                readed_line.chars().for_each(|c| wind.push(letter_to_jet(&c)));
            }
        }
    }

    let mut board: Vec<[bool; 7]> = vec![[true, true, true, true, true, true, true]];
    let mut max_height: usize = 0;

    add_rows_to_board(&mut board, max_height);

    let shapes: Vec<fn(usize) -> HashSet<Point>> = vec![make_horizontal_line_shape, make_plus_shape, make_j_shape, make_vertical_line_shape, make_square_shape];

    // Lookup from (board_hash, shape_index, wind_index) -> (max_height, numer_of_shape)
    let mut cycle_lookup: HashMap<(String, usize, usize), (usize, usize)> = HashMap::new();

    let mut wind_index = 0;

    let mut cycle_height = 0;
    let mut cycle_number_of_shapes = 0;
    let mut index_where_cycle_was_found = 0;

    for i in 0..1_000_000_000_000 {
        max_height = drop_shape(&mut board, &shapes[i % shapes.len()](max_height + 4), &wind, &mut wind_index);
        //println!("max_height: {}; wind_index {}", max_height, wind_index);
        add_rows_to_board(&mut board, max_height);
        //draw_board(&board);

        if i == 2021 {
            println!("Part 1: {}", max_height);
        }

        if board.len() > HASH_BOARD_ROWS {
            let board_hash_p = board_hash(&board);
            if let Some(board_hash) = board_hash_p {
                let key = (board_hash.clone(), i % shapes.len(), wind_index);
                
                if cycle_lookup.contains_key(&key) {
                    let (cycle_max_height, cycle_num_of_shape) = cycle_lookup.get(&key).unwrap();
                    cycle_number_of_shapes = i - cycle_num_of_shape;
                    cycle_height = max_height - cycle_max_height;
                    index_where_cycle_was_found = i;
                    break;
                } else {
                    cycle_lookup.insert(key, (max_height, i));
                }
            }
        }
    }

    println!("CYCLE DETECTED: {} shapes, {} height, index {}", cycle_number_of_shapes, cycle_height, index_where_cycle_was_found);

    let mut max_height_part_1 = max_height.clone();
    let mut max_height_part_2 = max_height.clone();

    if index_where_cycle_was_found < 2022 {
        let mut remainining_shapes_part_1 = 2022 - index_where_cycle_was_found - 1;
        let full_cycles_part_1 = remainining_shapes_part_1 / cycle_number_of_shapes;
        remainining_shapes_part_1 -= full_cycles_part_1 * cycle_number_of_shapes;

        let mut wind_index_part_1 = wind_index;
        let mut board_part_1 = board.clone();

        for i in 0..remainining_shapes_part_1 {
            max_height_part_1 = drop_shape(&mut board_part_1, &shapes[(index_where_cycle_was_found + i + 1) % shapes.len()](max_height_part_1 + 4), &wind, &mut wind_index_part_1);
            add_rows_to_board(&mut board_part_1, max_height_part_1);
        }

        max_height_part_1 += full_cycles_part_1 * cycle_height;

        println!("Part 1: {}", max_height_part_1);
    }

    if index_where_cycle_was_found < 1_000_000_000_000 {
        let mut remainining_shapes_part_2 = 1_000_000_000_000 - index_where_cycle_was_found - 1;
        let full_cycles_part_2 = remainining_shapes_part_2 / cycle_number_of_shapes;
    
        println!("full_cycles_part_2: {}", full_cycles_part_2);
    
        remainining_shapes_part_2 -= full_cycles_part_2 * cycle_number_of_shapes;
    
        let mut wind_index_part_2 = wind_index;
    
        let mut board_part_2 = board.clone();
    
        for i in 0..remainining_shapes_part_2 {
            max_height_part_2 = drop_shape(&mut board_part_2, &shapes[(index_where_cycle_was_found + i + 1) % shapes.len()](max_height_part_2 + 4), &wind, &mut wind_index_part_2);
            add_rows_to_board(&mut board_part_2, max_height_part_2);
        }
        
        max_height_part_2 += full_cycles_part_2 * cycle_height;
    }

    //draw_board(&board);
    
    println!("Part 2: {}", max_height_part_2);
}

fn drop_shape(board: &mut Vec<[bool; 7]>, shape: &HashSet<Point>, wind: &Vec<Jet>, next_wind_index: &mut usize) -> usize {
    let mut frame = shape.clone();
    let mut wind_frame: HashSet<Point>;
    let mut drop_frame: HashSet<Point>;
    //println!("frame: {:?}", frame);
    loop {
        let wind_direction = wind[*next_wind_index];
        *next_wind_index = (*next_wind_index + 1) % wind.len();

        wind_frame = frame.clone();
        if wind_direction == Jet::Left {
            if !frame.iter().any(|p| p.x == 0) {
                wind_frame = frame.iter().map(|p| Point::new(p.x - 1 , p.y)).collect();
            }
        } else {
            if !frame.iter().any(|p| p.x == 6) {
                wind_frame = frame.iter().map(|p| Point::new(p.x + 1 , p.y)).collect();
            }
        }

        if !wind_frame.iter().any(|p| board[p.y][p.x]) {
            frame = wind_frame;
        }

        drop_frame = frame.iter().map(|p| Point::new(p.x, p.y - 1)).collect();
        
        if drop_frame.iter().any(|p| board[p.y][p.x]) {
            break;
        }

        frame = drop_frame;
        //println!("frame: {:?}", frame);
    }

    for point in frame {
        board[point.y][point.x] = true;
    }

    for i in (0..board.len()).rev() {
        if board[i].iter().any(|&b| b) {
            return i;
        }
    }
    0
}

fn add_rows_to_board(board: &mut Vec<[bool; 7]>, max_height: usize) {
    while board.len() < max_height + 8 {
        board.push([false, false, false, false, false, false, false]);
    }
}

fn board_hash(board: &Vec<[bool; 7]>) -> Option<String> {
    if board.len() < HASH_BOARD_ROWS { 
        return None;
    }
    let mut hash = "".to_string();
    for row in board.iter().rev().take(HASH_BOARD_ROWS) {
        for cell in row {
            if *cell {
                hash += "1";
            } else {
                hash += "0";
            }
        }
    }
    Option::Some(hash)
}

fn draw_board(board: &Vec<[bool; 7]>) {
    for row in board.iter().rev() {
        for cell in row {
            if *cell {
                print!("██");
            } else {
                print!("  ");
            }
        }
        println!("");
    }
}