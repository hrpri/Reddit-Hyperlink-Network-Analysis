use csv::{self};
use std::{collections::{HashMap, HashSet, BinaryHeap}, usize};
use core::cmp::Reverse;
use rayon::prelude::*;

type Record = HashMap<String, String>; 

pub struct Graph {
    /*
    pub struct Graph: a struct containing data about a graph network
    fields:
        size: an unsigned integer containing the number of nodes in the graph
        adjacency dict: a hashmap with every node in the graph as keys and their 
        corresponding connections in a vector of strings as the values.
        nodes: A hashset containing the name of every node in the graph.
    */
    pub size: usize,
    pub adjacency_dict: HashMap<String, Vec<String>>, 
    pub nodes: HashSet<String>
}

impl Graph {
    pub fn new() -> Graph {
        // Initializes and returns an empty Graph struct
        Graph { size: 0, nodes: HashSet::new(), adjacency_dict: HashMap::new()}
    }

    pub fn init(&mut self, path: &str) {
        /* 
        pub fn init: 
            Populates an empty Graph struct with data from the SNAP Reddit Hyperlinks Dataset.
        Parameters:
            &mut self: mutable reference to the Graph struct on which the method is called.
            path: string literal containing the filepath of the data file.
        */
        
        let mut raw_data_list: Vec<HashMap<String, String>> = Vec::new(); // array of Hashmaps of the raw data read in from the file
        let mut unique_subs: HashSet<String> = HashSet::new(); 

        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b'\t') // data is a .tsv not .csv
            .from_path(path)
            .expect("Error opening file");

        for record in reader.deserialize() {
            let record: Record = record.expect("tsv error");
            raw_data_list.push(record); // Read each line into the raw data vector
        }
        
        for line in &raw_data_list { // Add each node to the set
            unique_subs.insert(line.get("SOURCE_SUBREDDIT").expect("error getting source subreddit").to_string());
            unique_subs.insert(line.get("TARGET_SUBREDDIT").expect("error getting target subreddit").to_string());
        }
        
        self.size = unique_subs.len();
        self.nodes = unique_subs;
        
        
        for node in &self.nodes { 
            self.adjacency_dict.insert(node.to_string(), Vec::new());
        }
        
        for line in &raw_data_list {
            
            let node = line.get("SOURCE_SUBREDDIT").expect("error getting source subreddit");
            let connection = line.get("TARGET_SUBREDDIT").expect("error getting target subreddit");
        
            self.adjacency_dict
                .get_mut(node)
                .expect("error fetching node from adjacency dict")
                .push(connection.to_string());
        }
    }

    fn degree(&self) -> Vec<(String, usize)> {
        /*
        fn degree: 
            calculates the (out) degree (# of outgoing connections) for every node in the graph
        parameters: 
            &self: a reference to the Graph struct on which this method is called
        returns:
            degree: a vector of (String, usize) where String is the node and usize is its 
                    out degree sorted in descending order by out degree. 
        */
        
        let mut degree: Vec<(String, usize)> = Vec::new();
        for node in &self.adjacency_dict {
            degree.push((node.0.clone(), node.1.len()));
        }
        
        degree.sort_by_key(|v| std::cmp::Reverse(v.1));
        degree
    }

    pub fn reverse(&self) -> Graph {
        /*
        pub fn reverse:
            returns the reverse of the graph the method is called on
        parameters:
            &self: reference to the Graph struct that the method is called on
        returns:
            reverse: a Graph struct with the directions of the edges 
                    reversed compared to the original graph
        */
        
        let mut reverse = Graph::new();
        reverse.nodes = self.nodes.clone();
        reverse.size = self.size.clone();

        for node in &self.nodes {
            reverse.adjacency_dict.insert(node.to_string(), Vec::new());
        }

        for node in &self.nodes { // for every node in the graph
            let outedges = self.adjacency_dict.get(node).expect("msg"); // get all outgoing connections

            for outedge in outedges.iter() { // for every outgoing connection
                reverse.adjacency_dict.get_mut(outedge).expect("msg").push(node.to_string()); // push the node pointing to it
            }
        }
        reverse
    }

    pub fn out_degree_centrality(&self) -> Vec<(String, f32)> {
        /*
        pub fn degree_centrality: 
            calculates the out degree centrality for every node in the graph.
            To calculate the in degree centrality, call on the reverse of a graph.
        parameters:
            &self: a reference to the Graph struct the method is called on
        returns:
            centrality: an N dimensional vector of (String, f32) where String 
                        is the node and f32 is its out degree centrality, sorted 
                        in descending order by out degree centrality.
        */

        let mut centrality: Vec<(String, f32)> = Vec::new();
        for node in self.degree() {
            centrality.push((node.0, node.1 as f32 / (self.size - 1) as f32));
        }
        centrality
    }
    
    pub fn in_degree_centrality(&self) -> Vec<(String, f32)> {
        /*
        pub fn in_degree_centrality: 
            calculates the in degree centrality for every node in the graph.
        parameters:
            &self: a reference to the Graph struct the method is called on
        returns:
            centrality: an N dimensional vector of (String, f32) where String 
                        is the node and f32 is its in degree centrality, sorted 
                        in descending order by in degree centrality.
        */

        let mut centrality: Vec<(String, f32)> = Vec::new();
        for node in self.reverse().degree() {
            centrality.push((node.0, node.1 as f32 / (self.size - 1) as f32));
        }
        centrality
    }

    pub fn closeness_centrality(&self) -> Vec<(String, f32)>{
        /*
        pub fn closeness_centrality:
            calculates the Wasserman-Faust adjusted closeness centrality of 
            every node in the graph.
        parameters:
            &self: a reference to the Graph struct that the method is called on
        returns:
            centralities_sorted: An N dimensional vector of (String, f32) where 
            String is the node and f32 is its WF closeness centrality, sorted by 
            closeness centrality
        */
        
        let graph = self.reverse(); // Incoming distance
        let nodes = graph.nodes.clone(); // Clone to avoid borrowing issues in parallel processing
        let big_n = graph.size as f32 - 1.0; // every reachable node
        
        let centralities: Vec<(String, f32)> = nodes.par_iter().map(|node| {
            let shortest_paths = graph.shortest_paths(node.to_string()); 
            
            let sum_dists: usize = shortest_paths.values().sum();
            let n = shortest_paths.len() as f32; // every reachable node
            let closeness_centrality: f32 = if sum_dists != 0 {
                ((n - 1.0) / (big_n - 1.0)) * (n / sum_dists as f32)
            } else {
                0.0
            };
            
            (node.clone(), closeness_centrality)
        }).collect();
    
        let mut centralities_sorted = centralities;
        centralities_sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        centralities_sorted
    }
    
    fn shortest_paths(&self, start: String) -> HashMap<String, usize>{
        /*
            fn shortest_paths: calculates the shortest paths distance between a 
                                node and every reachable node.
            parameters:
                &self: a reference to the graph that the method is called on.
                start: A String containing the node from which to calculate distances from.
            returns:
                distances: A hashmap with all reachable nodes as keys and their shortest
                paths distances from the start node as the corresponding values. 
        */
        let mut distances: HashMap<String, usize> = HashMap::new();
        distances.insert(start.to_string(), 0);

        let mut pq = BinaryHeap::<Reverse<(String,usize)>>::new(); //binary min heap of (String, usize)
        pq.push(Reverse((start.clone(), 0))); // initialaizing the starting node with a distance of 0  
        
        while let Some(Reverse((node, dist))) = pq.pop() {
            let outedges = self.adjacency_dict.get(&node).expect("outedges");
            for outedge in outedges {
                
                let new_dist = dist + 1; // The length of every edge is always 1
                let update = match distances.get(outedge) {
                    None => {true}
                    Some(d) => {new_dist < *d}
                };
                if update {
                    distances.insert(outedge.to_string(), new_dist);
                    pq.push(Reverse((outedge.clone(), new_dist)))
                }
            }
        }
        distances
    }
}

#[test]
fn test_degree(){
    let mut testgraph = Graph::new();
    let mut target_degree: Vec<(String, usize)> = vec![("0".to_string(), 0)];
    
    testgraph.adjacency_dict.insert(0.to_string(), Vec::new());
    
    for i in 1..5 {
        testgraph.adjacency_dict.insert(i.to_string(), Vec::new());
        for j in 0..i {
            testgraph.adjacency_dict.get_mut(&i.to_string()).expect("deg test error").push(j.to_string());
        }
        target_degree.push((i.to_string(), i));
    }
    target_degree.sort_by_key(|k| std::cmp::Reverse(k.1));
    
    let deg = testgraph.degree();
    assert_eq!(deg, target_degree);
}

#[test]
fn test_reverse() {
    let mut graph = Graph::new();

    graph.nodes.insert("A".to_string());
    graph.nodes.insert("B".to_string());
    graph.nodes.insert("C".to_string());
    graph.size = 3;

    graph.adjacency_dict.insert("A".to_string(), vec!["B".to_string()]);
    graph.adjacency_dict.insert("B".to_string(), vec!["C".to_string()]);
    graph.adjacency_dict.insert("C".to_string(), vec![]);
    
    let reversed_graph = graph.reverse();

    assert_eq!(reversed_graph.adjacency_dict.get("A").unwrap(), &Vec::<String>::new());
    assert_eq!(reversed_graph.adjacency_dict.get("B").unwrap(), &vec!["A".to_string()]);
    assert_eq!(reversed_graph.adjacency_dict.get("C").unwrap(), &vec!["B".to_string()]);
}

#[test]
fn test_shortest_paths() {
    let mut graph = Graph::new();

    graph.nodes.insert("A".to_string());
    graph.nodes.insert("B".to_string());
    graph.nodes.insert("C".to_string());
    graph.size = 3;

    graph.adjacency_dict.insert("A".to_string(), vec!["B".to_string()]);
    graph.adjacency_dict.insert("B".to_string(), vec!["C".to_string()]);
    graph.adjacency_dict.insert("C".to_string(), vec![]);

    let distances = graph.shortest_paths("A".to_string());

    let mut expected_distances = HashMap::new();
    expected_distances.insert("A".to_string(), 0);
    expected_distances.insert("B".to_string(), 1);
    expected_distances.insert("C".to_string(), 2);

    assert_eq!(distances, expected_distances);
}