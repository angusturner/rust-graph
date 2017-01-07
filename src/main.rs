fn main() {}


// each node is a vector of references to the adjacent nodes
struct Node<'a> {
    neighbors: Vec<&'a Node<'a>>,
}

impl<'a> Node<'a> {
    pub fn add_neighbor(&'a mut self, node: &'a Node<'a>) {
        self.neighbors.push(node);
    }
}

// each edge contains a weight and references to the two end nodes
struct Edge<'a, T: 'a> {
    weight: T,
    endpoints: (&'a Node<'a>, &'a Node<'a>),
}

// a collection of nodes and edges
struct Graph<'a, T: 'a> {
    nodes: Vec<Node<'a>>,
    edges: Vec<Edge<'a, T>>,
}

//
impl<'a, T> Graph<'a, T> {
    // add a node
    pub fn add_node(&'a mut self) {
        self.nodes.push(Node {
            neighbors: vec![]
        });
    }

    pub fn add_edge(&'a mut self, weight: T, endpoints: (usize, usize)) {
        // destructure endpoints
        let (s, e) = endpoints;

        // add the edge
        self.edges.push(Edge {
            weight: weight,
            endpoints: (&self.nodes[s], &self.nodes[e], )
        });

        // update the neighbor lists for the two end nodes
        self.nodes[s].add_neighbor(&self.nodes[e]);
    }
}
