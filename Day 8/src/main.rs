mod utils;


fn get_index(x: usize, y: usize, width: usize) -> usize {
    (y * width + x) as usize
}

fn main() {
    let input_file = utils::input_file_path();
    if let Err(e) = input_file {
        println!("{}", e);
        return;
    }

    let mut tree_sizes : Vec<i32> = Vec::new();
    let mut width : usize = 0;
    let mut height : usize = 0;

    let mut part1_visible : Vec<bool> = Vec::new();
    let mut part2_score : Vec<i32> = Vec::new();

    if let Ok(lines) = utils::read_lines(input_file.unwrap()) {
        for line in lines {
            height += 1;
            if let Ok(readed_line) = line {
                width = readed_line.len();
                for c in readed_line.chars() {
                    tree_sizes.push(c.to_digit(10).unwrap() as i32);
                    part1_visible.push(false);
                    part2_score.push(0);
                }
            }
        }
    }

    // Check edges
    for y in 0..height {
        let index_left = get_index(0, y, width);
        let index_right = get_index(width - 1, y, width);

        part1_visible[index_left] = true;
        part1_visible[index_right] = true;
    }

    // Check left to right
    for y in 0..height {
        let mut max_left: i32 = 0;
        let mut max_right: i32 = 0;
        for x in 0..width {
            let index_left = get_index(x, y, width);
            let index_right = get_index(width - x - 1, y, width);

            if tree_sizes[index_left] > max_left {
                part1_visible[index_left] = true;
                max_left = tree_sizes[index_left];
            }

            if tree_sizes[index_right] > max_right {
                part1_visible[index_right] = true;
                max_right = tree_sizes[index_right];
            }
        }
    }

    for x in 0..width {
        let index_top = get_index(x, 0, width);
        let index_bottom = get_index(x, height - 1, width);

        part1_visible[index_top] = true;
        part1_visible[index_bottom] = true;
    }

    // Check top top bottom
    for x in 0..width {
        let mut max_top: i32 = 0;
        let mut max_bottom: i32 = 0;

        for y in 0..height {
            let index_top = get_index(x, y, width);
            let index_bottom = get_index(x, height - y - 1, width);

            if tree_sizes[index_top] > max_top {
                part1_visible[index_top] = true;
                max_top = tree_sizes[index_top];
            }

            if tree_sizes[index_bottom] > max_bottom {
                part1_visible[index_bottom] = true;
                max_bottom = tree_sizes[index_bottom];
            }
        }
    }

    // Part 2
    for y in 0..height {
        for x in 0..width {
            let index = get_index(x, y, width);

            // Top
            let mut top_score = 0;
            if y > 0 {
                for y1 in (0..y).rev() {
                    let index1 = get_index(x, y1, width);
                    if tree_sizes[index1] < tree_sizes[index] {
                        top_score += 1;
                    }
                    else {
                        top_score += 1;
                        break;
                    }
                }
            }

            // Bottom
            let mut bottom_score = 0;
            if y < height - 1 {
                for y1 in y+1..height {
                    let index1 = get_index(x, y1, width);
                    if tree_sizes[index1] < tree_sizes[index] {
                        bottom_score += 1;
                    }
                    else {
                        bottom_score += 1;
                        break;
                    }
                }
            }

            // Left
            let mut left_score = 0;
            if x > 0 {
                for x1 in (0..x).rev() {
                    let index1 = get_index(x1, y, width);
                    if tree_sizes[index1] < tree_sizes[index] {
                        left_score += 1;
                    }
                    else {
                        left_score += 1;
                        break;
                    }
                }
            }

            // Right
            let mut right_score = 0;
            if x < width - 1 {
                for x1 in x+1..width {
                    let index1 = get_index(x1, y, width);
                    if tree_sizes[index1] < tree_sizes[index] {
                        right_score += 1;
                    }
                    else {
                        right_score += 1;
                        break;
                    }
                }
            }
            //println!("({}, {}): {} {} {} {} = {}", x, y, top_score, bottom_score, left_score, right_score, top_score * bottom_score * left_score * right_score);
            part2_score[index] = top_score * bottom_score * left_score * right_score;
        }
    }

    /*
    for y in 0..height {
        for x in 0..width {
            print!("{} ", part2_score[get_index(x, y, width)] as i32);
        }
        println!("");
    }
    */
    

    println!("Part 1: {}", part1_visible.iter().filter(|&&x| x).count());
    println!("Part 2: {}", part2_score.iter().max().unwrap());
}