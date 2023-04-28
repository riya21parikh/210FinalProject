
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
pub fn read_graph_facebook(filename: &str) -> Vec<Vec<usize>> {
    // Read the graph from file
    let file = File::open(filename).expect("Failed to open input file");
    let reader = BufReader::new(file);
    let  lines = reader.lines();

    // Start of creating edges & nodes
    let mut edges = Vec::new();
    let mut vertices = HashSet::new();

    for line in lines {
        let parts1 = line.unwrap();
        let parts = parts1.split_whitespace().collect::<Vec<_>>();
        let u = parts[0].parse::<usize>().unwrap();
        let v = parts[1].parse::<usize>().unwrap();
        edges.push((u, v));
        vertices.insert(u);
        vertices.insert(v);
    }

    let num_vertices = vertices.len();
    let mut graph = vec![vec![]; num_vertices];
    for &(u, v) in &edges {
        graph[u].push(v);
        graph[v].push(u);
    }

    graph
}

pub fn read_graph_twitch(filename: &str) -> Vec<Vec<usize>> {
    // Read the graph from file
    let file = File::open(filename).expect("Failed to open input file");
    let reader = BufReader::new(file);
    let  lines = reader.lines();

    // Start of creating edges & nodes
    let mut edges = Vec::new();
    let mut vertices = HashSet::new();

    for line in lines {
        let parts1 = line.unwrap();
        let parts = parts1.split(',').collect::<Vec<_>>();
        let u = parts[0].parse::<usize>().unwrap();
        let v = parts[1].parse::<usize>().unwrap();
        edges.push((u, v));
        vertices.insert(u);
        vertices.insert(v);
    }

    let num_vertices = vertices.len();
    let mut graph = vec![vec![]; num_vertices];
    for &(u, v) in &edges {
        graph[u].push(v);
        graph[v].push(u);
    }

    graph
}

pub fn main(){
let graph_facebook = read_graph_facebook("facebook_combined.txt");
let graph_twitch: Vec<Vec<usize>> = read_graph_twitch("musae_PTBR_edges.csv");
}