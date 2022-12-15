mod utils;
use regex::Regex;
use std::cmp;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Range {
    left: i64,
    right: i64,
}

impl Range {
    fn new(a: i64, b: i64) -> Range {
        Range { left: cmp::min(a, b), right: cmp::max(a, b) }
    }
}

#[derive(Debug, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }
}

fn manhattan_distance(a: &Point, b: &Point) -> i64 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

#[derive(Debug, Clone)]
struct Sensor {
    position: Point,
    closest_beacon: Point,
    distance: i64,
}

fn is_point_detected_by_any_sensor(point: &Point, sensors: &Vec<Sensor>, min_coords:i64, max_coords: i64) -> bool {
    // We don't care about points outside of the range - so we can just return that they are detected
    if point.x < min_coords || point.x > max_coords || point.y < min_coords || point.y > max_coords {
        return true;
    }
    sensors.iter().any(|sensor| sensor.detects(point))
}

impl Sensor {
    fn new(position: Point, closest_beacon: Point) -> Sensor {
        Sensor {
            distance: manhattan_distance(&position, &closest_beacon),
            position,
            closest_beacon,
        }
    }

    fn detects(&self, beacon: &Point) -> bool {
        manhattan_distance(&self.position, beacon) <= self.distance
    }

    fn check_positions_around(&self, all_sensors: &Vec<Sensor>, min_coords:i64, max_coords: i64) -> Option<Point> {
        let mut checked_position = Point::new(self.position.x, self.position.y - self.distance - 1); // Start from the top

        let mut direction = (1, 1); //Go bottom right
        while checked_position.x < self.position.x + self.distance + 1 {
            if !is_point_detected_by_any_sensor(&checked_position, all_sensors, min_coords, max_coords) {
                return Option::Some(checked_position);
            }
            checked_position = Point::new(checked_position.x + direction.0, checked_position.y + direction.1);
        }
        // Now we have point at the right

        direction = (-1, 1); // Go bottom left
        while checked_position.y < self.position.y + self.distance + 1 {
            if !is_point_detected_by_any_sensor(&checked_position, all_sensors, min_coords, max_coords) {
                return Option::Some(checked_position);
            }
            checked_position = Point::new(checked_position.x + direction.0, checked_position.y + direction.1);
        }
        // Now we have point at the bottom

        direction = (-1, -1); // Go top left
        while checked_position.x > self.position.x - self.distance - 1 {
            if !is_point_detected_by_any_sensor(&checked_position, all_sensors, min_coords, max_coords) {
                return Option::Some(checked_position);
            }
            checked_position = Point::new(checked_position.x + direction.0, checked_position.y + direction.1);
        }
        // Now we have point at the left

        direction = (1, -1); // Go top right
        while checked_position.y > self.position.y - self.distance - 1 {
            if !is_point_detected_by_any_sensor(&checked_position, all_sensors, min_coords, max_coords) {
                return Option::Some(checked_position);
            }
            checked_position = Point::new(checked_position.x + direction.0, checked_position.y + direction.1);
        }
        // Now we are at the top again - and no point around this sensor is not detected by any other sensor
        Option::None
    }
}

fn main() {
    let input_file = utils::input_file_path();
    if let Err(e) = input_file {
        println!("{}", e);
        return;
    }

    let line_regex = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();

    let mut sensors: Vec<Sensor> = Vec::new();

    if let Ok(lines) = utils::read_lines(input_file.unwrap()) {
        for line in lines {
            if let Ok(readed_line) = line {
                
                let capture = line_regex.captures(readed_line.as_str()).unwrap();
                sensors.push(Sensor::new(
                    Point::new(capture.get(1).unwrap().as_str().parse::<i64>().unwrap(), capture.get(2).unwrap().as_str().parse::<i64>().unwrap()),
                    Point::new(capture.get(3).unwrap().as_str().parse::<i64>().unwrap(), capture.get(4).unwrap().as_str().parse::<i64>().unwrap())
                ));
            }
        }
    }

    let row_to_find_coverage = 2_000_000;
    let covered_columns: HashSet<i64> = sensors.iter()
        .filter(|sensor| sensor.distance >= (sensor.position.y - row_to_find_coverage).abs())
        .map(|sensor| {
            let columns_span_size = sensor.distance - (sensor.position.y - row_to_find_coverage).abs();
            let a = sensor.position.x - columns_span_size;
            let b = sensor.position.x + columns_span_size;
            Range::new(a, b)
        }).flat_map(|range| {
            (range.left..=range.right).collect::<Vec<i64>>()
        }).filter(|x| sensors.iter().any(|sensor| sensor.closest_beacon.y == row_to_find_coverage && sensor.closest_beacon.x != *x))
        .collect();
    
    let min_coords_to_check = 0;
    let max_coords_to_check = 4_000_000;

    let mut undetected_beacon: Point = Point::new(0, 0);
    for i in 0..sensors.len() {
        let possible_undetected_beacon = sensors[i].check_positions_around(&sensors, min_coords_to_check, max_coords_to_check);
        if let Some(position) = possible_undetected_beacon {
            undetected_beacon = position;
            break;
        }
    }

    println!("Part 1: {}", covered_columns.len());
    println!("Part 2: {}", 4_000_000 * undetected_beacon.x + undetected_beacon.y);
}