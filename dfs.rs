
use std::collections::{HashSet};

pub fn dfs(graph: &Vec<Vec<usize>>, visited: &mut HashSet<usize>, current_vertex: usize, connected_component: &mut HashSet<usize>) {
    visited.insert(current_vertex);
    connected_component.insert(current_vertex);
    for neighbor in &graph[current_vertex] {
        if !visited.contains(neighbor) {
            dfs(graph, visited, *neighbor, connected_component);
        }
    }
}

pub fn connected_components(graph: &Vec<Vec<usize>>) -> Vec<HashSet<usize>> {
    let mut visited = HashSet::new();
    let mut connected_components = Vec::new();
    for current_vertex in 0..graph.len() {
        if !visited.contains(&current_vertex) {
            let mut connected_component = HashSet::new();
            dfs(graph, &mut visited, current_vertex, &mut connected_component);
            connected_components.push(connected_component);
        }
    }
    connected_components
}

use crate::file_read::{read_graph_facebook, read_graph_twitch};
pub fn main(){
let graph_facebook = read_graph_facebook("facebook_combined.txt");
let graph_twitch: Vec<Vec<usize>> = read_graph_twitch("musae_PTBR_edges.csv");

let _connected_components_facebook = connected_components(&graph_facebook);
println!("Found {} connected component(s) for Facebook", _connected_components_facebook.len());
// Turns out eveything is connected and the data set site told me that so I did all that for nothing

let _connected_components_twitch = connected_components(&graph_twitch);
println!("Found {} connected component(s) for Twitch", _connected_components_twitch.len());
}