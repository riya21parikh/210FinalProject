#[cfg(test)]
    use super::*;

    #[test]
    fn test_calculate_shortest_distance() {
        // Test case 1: Empty graph
        let graph1: Vec<Vec<usize>> = vec![];
        assert_eq!(bfs::bfs(graph1), 0.0);

        // Test case 2: Graph w 1 vertex
        let graph2: Vec<Vec<usize>> = vec![vec![0]];
        assert_eq!(bfs::bfs(graph2), 0.0);

        // Test case 3: Graph w 3 disconnected vertices
        let graph4: Vec<Vec<usize>> = vec![vec![], vec![], vec![]];
        assert_eq!(bfs::bfs(graph4), 0.0);

        // Test case 4: Graph w 3 vertices and 2 edges
        let graph5: Vec<Vec<usize>> = vec![vec![1], vec![0, 2], vec![1]];
        assert_eq!(bfs::bfs(graph5), 1.3333333333333333);

        // Test case 5: Graph w 4 vertices and 1 disconnected vertex
        let graph6: Vec<Vec<usize>> = vec![vec![1], vec![0, 2], vec![1], vec![]];
        assert_eq!(bfs::bfs(graph6),1.3333333333333333);

        // Test case 6: Graph w 5 vertices and lots of interconnected edges
        let graph7: Vec<Vec<usize>> = vec![vec![1, 4], vec![0, 2, 4], vec![1, 3], vec![2], vec![0, 1]];
        assert_eq!(bfs::bfs(graph7), 1.7);
    }


#[cfg(test)]
    use super::*;
    use crate::dfs::connected_components;
    use std::collections::{HashSet};

    #[test]
    fn test_calculate_connect() {
    #[macro_export]
    macro_rules! hashset {
        ($( $x:expr ),*) => {
            {
                let mut set = ::std::collections::HashSet::new();
                $(
                    set.insert($x);
                )*
                set
            }
        };
    }
        // Test case 1: Empty graph
        let graph1: Vec<Vec<usize>> = vec![];
        let expected_output1: Vec<HashSet<usize>> = vec![];
        assert_eq!(connected_components(&graph1), expected_output1);

        // Test case 2: Graph w 2 disconnected vertices
        let graph3: Vec<Vec<usize>> = vec![vec![], vec![]];
        let expected_output2: Vec<HashSet<usize>> = vec![hashset!{0}, hashset!{1}];
        assert_eq!(connected_components(&graph3), expected_output2);

        // Test case 3: Graph w 3 vertices and 2 edges
        let graph3: Vec<Vec<usize>> = vec![vec![1], vec![0, 2], vec![1]];
        let expected_output3: Vec<HashSet<usize>> = vec![hashset!{0, 1, 2}];
        assert_eq!(connected_components(&graph3), expected_output3);

        // Test case 4: Graph w 4 vertices and 1 disconnected vertex
        let graph4: Vec<Vec<usize>> = vec![vec![1], vec![0, 2], vec![1], vec![]];
        let expected_output4: Vec<HashSet<usize>> = vec![hashset!{0, 1, 2}, hashset!{3}];
        assert_eq!(connected_components(&graph4), expected_output4);
        
        // Test case 5: Graph w 5 vertices and 1 disconnected vertex
        let graph5: Vec<Vec<usize>> = vec![vec![1, 2, 3], vec![0, 2], vec![1, 0], vec![0,2], vec![]];
        let expected_output5: Vec<HashSet<usize>> = vec![hashset!{0, 1, 2, 3}, hashset!{4}];
        assert_eq!(connected_components(&graph5), expected_output5);

    }





