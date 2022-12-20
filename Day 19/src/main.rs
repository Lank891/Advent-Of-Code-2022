mod utils;
use regex::Regex;
use std::collections::HashMap;
use std::time::Instant;
use par_map::ParMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ResourceType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

type Resources = HashMap<ResourceType, i32>;
type Blueprint = HashMap<ResourceType, Resources>;

fn create_cost(ore: i32, clay: i32, obsidian: i32) -> Resources {
    let mut cost: Resources = HashMap::new();
    cost.insert(ResourceType::Ore, ore);
    cost.insert(ResourceType::Clay, clay);
    cost.insert(ResourceType::Obsidian, obsidian);
    cost.insert(ResourceType::Geode, 0);
    return cost;
}

fn get_cost_tuple(robot: ResourceType, blueprint: &Blueprint) -> (i32, i32, i32) {
    let cost = blueprint.get(&robot).unwrap();
    return (*cost.get(&ResourceType::Ore).unwrap(), *cost.get(&ResourceType::Clay).unwrap(), *cost.get(&ResourceType::Obsidian).unwrap());
}

fn can_build_robot(resources: &(i32, i32, i32, i32), blueprint: &(i32, i32, i32)) -> bool {
    resources.0 >= blueprint.0 && resources.1 >= blueprint.1 && resources.2 >= blueprint.2
}

fn max(a: i32, b: i32, c: i32, d: i32) -> i32 {
    return std::cmp::max(a, std::cmp::max(b, std::cmp::max(c, d)));
}

fn max_geodes_for_blueprint(blueprint: &Blueprint, max_time: i32) -> i32 {

    type State = ((i32, i32, i32, i32), (i32, i32, i32, i32), i32); // (robots, resources, time) -> (ore, clay, obsidian, geode)

    let mut stack: Vec<State> = Vec::new();
    stack.push(((1, 0, 0, 0), (0, 0, 0, 0), 0));

    let mut max_geodes = 0;

    let geode_cost = get_cost_tuple(ResourceType::Geode, blueprint);
    let obsidian_cost = get_cost_tuple(ResourceType::Obsidian, blueprint);
    let clay_cost = get_cost_tuple(ResourceType::Clay, blueprint);
    let ore_cost = get_cost_tuple(ResourceType::Ore, blueprint);

    // Calculating max possible geodes given a state - we assume we can build geode robot each future turn
    fn get_max_possible_geodes(actual_geodes: &i32, geode_robots: i32, time: &i32, max_time: &i32) -> i32 {
        let time_left = max_time - time;
        let max_future_geodes = (2*geode_robots+time_left+1)*time_left/2;
        return actual_geodes + max_future_geodes;
    }

    fn process_turn(resources: &(i32, i32, i32, i32), robots: &(i32, i32, i32, i32)) -> (i32, i32, i32, i32) {
        (resources.0 + robots.0, resources.1 + robots.1, resources.2 + robots.2, resources.3 + robots.3)
    }

    fn build_robot(resources: &(i32, i32, i32, i32), cost: &(i32, i32, i32)) -> (i32, i32, i32, i32) {
        (resources.0 - cost.0, resources.1 - cost.1, resources.2 - cost.2, resources.3)
    }

    // For optimalizations - if we have enough robots to produce any robot in one turn, we won't construct new robots
    let max_obsidian_robots = max(
        *blueprint.get(&ResourceType::Geode).unwrap().get(&ResourceType::Obsidian).unwrap(),
        *blueprint.get(&ResourceType::Ore).unwrap().get(&ResourceType::Obsidian).unwrap(),
        *blueprint.get(&ResourceType::Clay).unwrap().get(&ResourceType::Obsidian).unwrap(),
        *blueprint.get(&ResourceType::Obsidian).unwrap().get(&ResourceType::Obsidian).unwrap() 
    );

    let max_clay_robots = max(
        *blueprint.get(&ResourceType::Geode).unwrap().get(&ResourceType::Clay).unwrap(),
        *blueprint.get(&ResourceType::Ore).unwrap().get(&ResourceType::Clay).unwrap(),
        *blueprint.get(&ResourceType::Clay).unwrap().get(&ResourceType::Clay).unwrap(),
        *blueprint.get(&ResourceType::Obsidian).unwrap().get(&ResourceType::Clay).unwrap()
    );

    let max_ore_robots = max(
        *blueprint.get(&ResourceType::Geode).unwrap().get(&ResourceType::Ore).unwrap(),
        *blueprint.get(&ResourceType::Ore).unwrap().get(&ResourceType::Ore).unwrap(),
        *blueprint.get(&ResourceType::Clay).unwrap().get(&ResourceType::Ore).unwrap(),
        *blueprint.get(&ResourceType::Obsidian).unwrap().get(&ResourceType::Ore).unwrap()
    );

    while stack.len() > 0 {
        let (robots, resources, time) = stack.pop().unwrap();

        let new_time = time + 1;     

        // We check building BEFORE processing turn
        let can_build_geode = can_build_robot(&resources, &geode_cost);
        let can_build_obsidian = can_build_robot(&resources, &obsidian_cost);
        let can_build_clay = can_build_robot(&resources, &clay_cost);
        let can_build_ore = can_build_robot(&resources, &ore_cost);

        let new_resources = process_turn(&resources, &robots);

        if new_resources.3 > max_geodes {
            max_geodes = new_resources.3;
        }

        // We stop branching if we've reached time limit
        if time >= max_time {
            continue;
        }

        // We stop branching if we can't possibly get more geodes than we already have
        if get_max_possible_geodes(&new_resources.3, robots.3, &new_time, &max_time) <= max_geodes {
            continue;
        }
        

        if can_build_geode {
            stack.push(((robots.0, robots.1, robots.2, robots.3+1), build_robot(&new_resources, &geode_cost), new_time));
        }

        // We assume we don't build those if we can build geode robot and we don't need more robots than max number of robots needed to produce
        //  materials for any robot in 1 turn

        // Note - for some malicious inputs the "always build geode" won't work - but it should be good enough for the task

        if can_build_obsidian && !can_build_geode && robots.2 < max_obsidian_robots {
            stack.push(((robots.0, robots.1, robots.2+1, robots.3), build_robot(&new_resources, &obsidian_cost), new_time));
        }

        if can_build_clay && !can_build_geode && robots.1 < max_clay_robots {
            stack.push(((robots.0, robots.1+1, robots.2, robots.3), build_robot(&new_resources, &clay_cost), new_time));
        }

        if can_build_ore && !can_build_geode && robots.0 < max_ore_robots {
            stack.push(((robots.0+1, robots.1, robots.2, robots.3), build_robot(&new_resources, &ore_cost), new_time));
        }

        if !can_build_geode {
            stack.push((robots, new_resources, new_time));
        }
        
    }

    return max_geodes;
}

fn main() {
    let input_file = utils::input_file_path();
    if let Err(e) = input_file {
        println!("{}", e);
        return;
    }

    let blueprint_regex = Regex::new(r"Blueprint \d+: Each ore robot costs (\d+) ore\. Each clay robot costs (\d+) ore\. Each obsidian robot costs (\d+) ore and (\d+) clay\. Each geode robot costs (\d+) ore and (\d+) obsidian\.").unwrap();
    let mut blueprints: Vec<Blueprint> = Vec::new();

    if let Ok(lines) = utils::read_lines(input_file.unwrap()) {
        for line in lines {
            if let Ok(readed_line) = line {
                
                let captures = blueprint_regex.captures(&readed_line).unwrap();
                let mut blueprint: Blueprint = HashMap::new();
                blueprint.insert(ResourceType::Ore, create_cost(captures.get(1).unwrap().as_str().parse::<i32>().unwrap(), 0, 0));
                blueprint.insert(ResourceType::Clay, create_cost(captures.get(2).unwrap().as_str().parse::<i32>().unwrap(), 0, 0));
                blueprint.insert(ResourceType::Obsidian, create_cost(captures.get(3).unwrap().as_str().parse::<i32>().unwrap(), captures.get(4).unwrap().as_str().parse::<i32>().unwrap(), 0));
                blueprint.insert(ResourceType::Geode, create_cost(captures.get(5).unwrap().as_str().parse::<i32>().unwrap(), 0, captures.get(6).unwrap().as_str().parse::<i32>().unwrap()));

                blueprints.push(blueprint);
            }
        }
    }

    
    let p1_timer = Instant::now();
    let geodes = blueprints.iter().cloned().par_map(|blueprint| max_geodes_for_blueprint(&blueprint, 24)).collect::<Vec<i32>>();
    let mut quality_level = 0;
    for i in 0..geodes.len() {
        quality_level += (i as i32 + 1) * geodes[i];
    }
    println!("Elapsed time (part 1): {:.2?}", p1_timer.elapsed());
    
    let p2_timer = Instant::now();
    let geodes_2 = blueprints[0..3].iter().cloned().par_map(|blueprint| max_geodes_for_blueprint(&blueprint, 32)).collect::<Vec<i32>>();
    let reult_2 = geodes_2[0] * geodes_2[1] * geodes_2[2];
    println!("Elapsed time (part 2): {:.2?}", p2_timer.elapsed());

    println!("Part 1: {}", quality_level);
    println!("Part 2: {}", reult_2);
}