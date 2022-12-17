mod utils;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp;
use itertools::Itertools;
use std::time::Instant;
use std::collections::VecDeque;
//use once_cell::sync::Lazy;

//static mut MEMORIZED_PATHS: Lazy<HashMap<(Vec<String>, i32, i32), i32>> = Lazy::new(|| HashMap::new());

fn dfs(start_node: &String, max_time: &i32, distances: &HashMap<(String, String), i32>, nodes_to_consider: &HashSet<String>, original_graph: &HashMap<String, (i32, Vec<String>)>) -> i32 {
    let mut stack: VecDeque<(String, i32, i32, HashSet<String>)> = VecDeque::new();
    let mut max_flow = 0;
    
    stack.push_back((start_node.clone(), 0, 0, HashSet::new()));

    while stack.len() > 0  {
        let actual_node_all = stack.pop_back().unwrap();

        let actual_node = actual_node_all.0; // Node we are in now
        let actual_time = actual_node_all.1; // Time of arrival + opening to the node
        let actual_flow = actual_node_all.2; // Actual flow before opening
        let mut opened_nodes = actual_node_all.3; // Already opened nodes

        opened_nodes.insert(actual_node.clone());
        
        let flow_gain = original_graph.get(&actual_node).unwrap().0 * cmp::max(*max_time - actual_time, 0);
        let new_flow = actual_flow + flow_gain;

        if new_flow > max_flow {
            max_flow = new_flow;
        }

        if actual_time < *max_time {
            let left_nodes = nodes_to_consider.difference(&opened_nodes);

            for node in left_nodes {
                let distance_to_new = distances.get(&(actual_node.clone(), node.clone())).unwrap();
                let time_after_travel_and_open = actual_time + distance_to_new + 1;
                if time_after_travel_and_open > *max_time {
                    continue;
                }
                stack.push_back((node.clone(), time_after_travel_and_open, new_flow, opened_nodes.clone()));
            }
        }
    }

    max_flow
}

fn main() {
    let input_file = utils::input_file_path();
    if let Err(e) = input_file {
        println!("{}", e);
        return;
    }

    let line_regex = Regex::new(r"Valve (\w{2}) has flow rate=(\d+); tunnel(?:s)? lead(?:s)? to valve(?:s)? ([\w, ]*)").unwrap();
    let mut original_graph : HashMap<String, (i32, Vec<String>)> = HashMap::new();
    let start: String = "AA".to_string();
    let max_time_part_1 = 30;
    let max_time_part_2 = 26;

    if let Ok(lines) = utils::read_lines(input_file.unwrap()) {
        for line in lines {
            if let Ok(readed_line) = line {
                
                let captures = line_regex.captures(&readed_line).unwrap();
                let from = captures.get(1).unwrap().as_str();
                let flow_rate = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
                let to = captures.get(3).unwrap().as_str();
                let to_vec : Vec<String> = to.split(", ").map(|s| s.to_string()).collect();
                original_graph.insert(from.to_string(), (flow_rate, to_vec));
                
            }
        }
    }

    let mut distances : HashMap<(String, String), i32> = HashMap::new();
    for a in original_graph.keys() {
        for b in original_graph.keys() {
            if a == b {
                distances.insert((a.to_string(), b.to_string()), 0);
            } else {
                distances.insert((a.to_string(), b.to_string()), 999999);
            }
        }
    }

    for (from, (_, to_vec)) in original_graph.iter() {
        for to in to_vec.iter() {
            distances.insert((from.to_string(), to.to_string()), 1);
        }
    }

    for k in original_graph.keys() {
        for i in original_graph.keys() {
            for j in original_graph.keys() {
                let d_ik = distances.get(&(i.to_string(), k.to_string())).unwrap();
                let d_kj = distances.get(&(k.to_string(), j.to_string())).unwrap();
                let d_ij = distances.get(&(i.to_string(), j.to_string())).unwrap();
                if d_ik + d_kj < *d_ij {
                    distances.insert((i.to_string(), j.to_string()), d_ik + d_kj);
                }
            }
        }
    }

    let mut filtered_distances : HashMap<(String, String), i32> = distances.clone();
    for (key, _) in distances.iter() {
        if key.0 != start && original_graph.get(&key.0).unwrap().0 == 0 {
            filtered_distances.remove(key);
        }
        if key.1 != start && original_graph.get(&key.1).unwrap().0 == 0 {
            filtered_distances.remove(key);
        }
    }

    let non_zero_nodes : Vec<String> = original_graph.keys().filter(|k| original_graph.get(*k).unwrap().0 != 0).map(|s| s.to_string()).collect();
    let nodes_to_consider: HashSet<String> = non_zero_nodes.iter().map(|s| s.clone()).collect();

    let p1_timer = Instant::now();
    let p1 = dfs(&start, &max_time_part_1, &filtered_distances, &nodes_to_consider, &original_graph);
    println!("Elapsed time (part 1): {:.2?}", p1_timer.elapsed());

    let p2_timer = Instant::now();
    let mut p2 = 0;
    for i in 2..nodes_to_consider.len()/2+1 {
        nodes_to_consider.iter().combinations(i).for_each(|c| {
            let human_nodes: HashSet<String> = c.iter().map(|s| s.to_string()).collect();
            let elephant_nodes: HashSet<String> = nodes_to_consider.difference(&human_nodes).map(|s| s.to_string()).collect();

            let human_flow = dfs(&start, &max_time_part_2, &filtered_distances, &human_nodes, &original_graph);
            let elephant_flow = dfs(&start, &max_time_part_2, &filtered_distances, &elephant_nodes, &original_graph);

            let total_flow = human_flow + elephant_flow;
            if total_flow > p2 {
                p2 = total_flow;
            }
        })
    }
    println!("Elapsed time (part 2): {:.2?}", p2_timer.elapsed());
    

    println!("Part 1: {:?}", p1);
    println!("Part 2: {:?}", p2);
}

