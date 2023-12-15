# Exploring Node Centrality in Reddit's Hyperlink Network
## Introduction
The goal of this project is to explore the structure of Reddit's hyperlink network using different measures of node centrality. This analysis was conducted using the Rust programming language along with data from the Stanford Network Analysis Project (SNAP). 
### Data set Information
This project is designed to be ran on the SNAP Reddit Hyperlink Network `soc-redditHyperlinks-body.tsv` data set which contains a "Network of subreddit-to-subreddit hyperlinks extracted from hyperlinks in the body of the [Reddit] post." This data was collected from Reddit between January 2014 and April 2017. 
## Degree Centrality
The first measure of centrality is degree centrality. A node's degree centrality is the fraction of the total nodes in the graph that it is connected with. For directed graphs, this can be calculated using either the outgoing connections (out degree centrality) or the incoming connections (in degree centrality). Empirically, this is extremely simple to measure; simply count a given node's connections, and devide the result by the size of the graph. Since the network in this project is represented in a hashmap with nodes as keys and lists of outgoing connections as values, the out degree centrality can be calculated using the formula $\frac{\text{length(adj\_dict[node])}}{N}$. Conversly, the in degree centrality can be calculated the same exact way only on the reverse of the graph, where the reverse is the same network with the directions of all edges inverted.
## Closeness Centrality
Closeness centrality measures a node's centrality based on its distance from other nodes in the graph. The issue with this metric is that if a graph has disconnected subgraphs, it only measures centrality within the parent subgraph, not the entire network. Thankfully, this can be addjusted for, resulting in a value relative to the entire network. The formula for this measure is $C_{WF}(u) = \frac{n-1}{N-1} \frac{n - 1}{\sum_{v=1}^{n-1} d(v, u)}$ where d(v, u) is the shortest paths distance between v and u, n-1 is the number of nodes reachable from u, and N-1 is the number of nodes in the graph. This adjusted metric was proposed by Wasserman and Faust in their 1994 book, *Social Network Analysis: Methods and Applications*. Similar to degree centrality, both the in and out closeness centrality of a node can be calculated using the reverse of a graph.
## Running the Project
1. Navigate to [this](https://snap.stanford.edu/data/soc-RedditHyperlinks.html) link and download the file labeled `soc-redditHyperlinks-body.tsv`.
2. Clone this github repo.
3. Make sure the data file is in the outermost directory.
4. Run the project with `cargo run --release`.

Please note that this project will take a considerable amount of time to run. This is because of the extreme computational complexity of computing closeness centrality (for each node in the graph, its shortest paths distnace to every reachable node must be computed). I have implemented multithreading for the complex part and it takes about 4 minutes for the entire project to complete running on 10 cpu cores. 

## Output
```
The network has 35776 nodes and 286561 edges
The top 5 subreddits with the highest out degree centrality are:
[("subredditdrama", 0.13039832), ("circlebroke", 0.06591195), ("shitliberalssay", 0.055010483), ("outoftheloop", 0.054730956), ("copypasta", 0.050985325)]
The top 5 subreddits with the highest in degree centrality are:
[("askreddit", 0.20486373), ("iama", 0.103256464), ("pics", 0.07767995), ("writingprompts", 0.06960168), ("videos", 0.068371765)]
The top 5 subreddits with the highest out closeness centrality are:
[("subredditdrama", 0.21867381), ("outoftheloop", 0.1968073), ("copypasta", 0.19576512), ("drama", 0.19551788), ("circlejerkcopypasta", 0.19333176)]
The top 5 subreddits with the highest in closeness centrality are:
[("askreddit", 0.3139648), ("iama", 0.3045849), ("videos", 0.2878546), ("pics", 0.2828598), ("todayilearned", 0.2819347)]

```

### Sources
- https://neo4j.com/developer/graph-data-science/centrality-graph-algorithms/ 
- https://networkx.org/documentation/stable/reference/algorithms/generated/networkx.algorithms.centrality.closeness_centrality.html
- https://networkx.org/documentation/stable/reference/algorithms/generated/networkx.algorithms.centrality.degree_centrality.html

