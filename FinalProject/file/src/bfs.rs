use std::collections::{VecDeque};
pub fn bfs(graph:Vec<Vec<usize>>)-> f64 {
    // Start of calculation of shortest distance between each pair of vertices
    let num_vertices = graph.len();
    let mut sum_shortest_distance = 0;
    let mut num_shortest_distances = 0;
    for i in 0..num_vertices {
        let mut dist = vec![-1; num_vertices];
        dist[i] = 0;

        let mut queue = VecDeque::new();
        queue.push_back(i);

        while !queue.is_empty() {
            let curr = queue.pop_front().unwrap();
            for &neighbor in &graph[curr] {
                if dist[neighbor] == -1 {
                    dist[neighbor] = dist[curr] + 1;
                    queue.push_back(neighbor);
                }
            }
        }

        for j in (i + 1)..num_vertices {
            if dist[j] != -1 {
                sum_shortest_distance += dist[j];
                num_shortest_distances += 1;
            }
        }
    }
    // Calculate average shortest distance
    if num_shortest_distances >0{
        let avg_shortest_distance = sum_shortest_distance as f64 / num_shortest_distances as f64;
        return avg_shortest_distance;
    }
    else{
        return 0.0
    }
    // End of calculation of shortest distance between each pair of vertices
}

use crate::file_read::{read_graph_facebook, read_graph_twitch};
pub fn main(){
let graph_facebook = read_graph_facebook("facebook_combined.txt");
let graph_twitch: Vec<Vec<usize>> = read_graph_twitch("musae_PTBR_edges.csv");

let degrees_sep_facebook = bfs(graph_facebook); 
println!("For Facebook: Average shortest distance, aka average degrees of separation between users, is {}",degrees_sep_facebook);

let degrees_sep_twitch = bfs(graph_twitch); 
println!("For Twitch: Average shortest distance, aka average degrees of separation between users, is {}",degrees_sep_twitch);

if degrees_sep_facebook != degrees_sep_twitch{
    let diff = degrees_sep_twitch - degrees_sep_facebook;
    let percent_diff = 100.0*diff/((degrees_sep_twitch + degrees_sep_facebook)/2.0);
    if percent_diff < 0.0{
        let percent_diff = -1.0 * percent_diff;
        println!("Difference of {}% exists in degrees of separation for the networks", percent_diff);
    }
    else{
        println!("Difference of {}% exists in degrees of separation for the networks", percent_diff);
    }
}
else{
    println!{"No difference in degrees of separation for the two different networks"};
}
}