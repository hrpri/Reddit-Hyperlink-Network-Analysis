mod project;

fn main() {
    let mut graph = project::Graph::new(); 
    graph.init("soc-redditHyperlinks-body.tsv"); // Populate Graph with data
    let reverse = graph.reverse(); // For calculating in-centrality

    let mut num_edges = 0;
    for node in graph.adjacency_dict.values() {
        num_edges += node.len();
    }
    print!("The network has {:?} nodes and {:?} edges\n", graph.nodes.len(), num_edges);
    // Degree centrality

    let out_deg = graph.out_degree_centrality();
    let in_deg = graph.in_degree_centrality();

    print!("The top 5 subreddits with the highest out degree centrality are:\n{:?}\n", &out_deg[0..5]);
    print!("The top 5 subreddits with the highest in degree centrality are:\n{:?}\n", &in_deg[0..5]);

    // Closeness centrality

    let in_closeness = graph.closeness_centrality();
    let out_closeness = reverse.closeness_centrality();

    print!("The top 5 subreddits with the highest out closeness centrality are:\n{:?}\n", &out_closeness[0..5]);
    print!("The top 5 subreddits with the highest in closeness centrality are:\n{:?}\n", &in_closeness[0..5]);
}