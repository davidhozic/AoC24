use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::print_map;


pub fn part_one() {
    parse_input();
}


pub fn part_two() {

}

pub fn parse_input() {
    let input: String = read_to_string("./inputs/day16.txt").unwrap();
    let mut map: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let graph = graph_from_map(&map);
    let (path, score) = get_shortest_path(
        &graph,
        (map.len() - 2, 1usize),
        (1, map[0].len() - 2)
    );

    for (y, x) in &path {
        map[*y][*x] = 'X';
    }

    print_map(&map);
    println!("{score} {}", path.len());
}


/// Finds the best path using Dijkstra's algorithm. Returns the path and the score using this path.
fn get_shortest_path(
    graph: &HashMap<(usize, usize), Vec<(usize, usize, usize)>>,
    start_node: (usize, usize),
    goal_node: (usize, usize)
) -> (Vec<(usize, usize)>, usize) {
    let mut ol = HashMap::new();
    let mut cl = HashMap::new();
    ol.insert(start_node, (0, (0, 0)));
    while !ol.is_empty() {
        // Select current best
        let (node, (cost, parent)) = {  // Gotta love Rust's lifetimes :)
            let n: (&(usize, usize), &(usize, (usize, usize))) = ol.iter().min_by_key(|(_, v)| v.0).unwrap();
            (*n.0, *n.1)
        };

        // Update neigbors
        // if let None = graph.get(&node) {
        //     panic!("{node:?}");
        // }

        for (ny, nx, edge_price) in graph.get(&node).unwrap_or(&Vec::with_capacity(0)) {
            if cl.contains_key(&(*ny, *nx)) {
                continue;
            }

            // Add to open list if the new cost is better
            let new_cost = edge_price + cost;
            if let Some((neighbor_cost, _)) = ol.get(&(*ny, *nx)) {
                if new_cost < *neighbor_cost {
                    ol.insert((*ny, *nx), (new_cost, node));    
                }
            }
            else {
                ol.insert((*ny, *nx), (new_cost, node));
            }
        }

        cl.insert(node, (cost, parent));
        ol.remove(&node);
        if node == goal_node {  // Found best path to the end
            break;
        }
    }


    // Create the path
    let mut path = vec![goal_node];
    let (best_cost, mut parent) = cl[&goal_node];
    while parent != start_node {
        path.push(parent);
        parent = cl[&parent].1
    }

    path.push(start_node);
    (path.iter().rev().map(|x| *x).collect(), best_cost)
}


fn graph_from_map(map: &Vec<Vec<char>>) -> HashMap<(usize, usize), Vec<(usize, usize, usize)>> {
    let mut positions= vec![(map.len() - 2, 1usize, Direction::EAST)];  // Tracks current position in the graph
    let mut nodes = HashMap::new();

    let mut visited = HashSet::new();
    // Find nodes and edges of the map. This basically builds a graph.
    while positions.len() > 0 {  // Iterate until all cursors leave the map
        // Check neigbors
        let mut new_positions = Vec::new();
        for (y, x, direction) in &positions {
            for new_direction in Direction::iter() {
                let (yn, xn) = match new_direction {
                    Direction::NORTH => (*y - 1, *x),
                    Direction::SOUTH => (*y + 1, *x),
                    Direction::WEST => (*y, *x - 1),
                    Direction::EAST => (*y, *x + 1),
                };

                let delta_direction = (new_direction as usize).abs_diff(*direction as usize) % 2;
                if map[yn][xn] == '#' || visited.contains(&(yn, xn, new_direction)) {  // Reached a wall || Already visisted
                    continue;
                }

                visited.insert((yn, xn, new_direction));
                let edge_cost = delta_direction * 1000 + 1;  // Rotation = 1000 points, move = 1 point
                new_positions.push((yn, xn, new_direction));
                let edge_list = nodes.entry((*y, *x)).or_insert(Vec::new());
                edge_list.push((yn, xn, edge_cost));
            }
        }
        positions = new_positions;
    }

    nodes
}


#[derive(Debug, EnumIter, PartialEq, Clone, Copy, Eq, Hash)]
enum Direction {
    NORTH = 0,
    WEST,
    SOUTH,
    EAST
}
