use std::io;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;

fn main() {
    // initialise a dummy graph
    let mut graph = Graph::new();

    // define some nodes
    let a: usize = graph.add_node("a");
    let b = graph.add_node("b");
    let c = graph.add_node("c");
    let d = graph.add_node("d");

    // add some edges
    graph.add_edge(a, b);
    graph.add_edge(c, d);
    graph.add_edge(d, a);

    // print it
    println!("{:?}", graph);
}

// alias the reference counted node pointer (makes things cleaner)
type NodeRef<'a> = Rc<RefCell<Node<'a>>>;

// each node contains a vector of references to the adjacent nodes
struct Node<'a> {
    name: &'a str,
    adjacent: Vec<NodeRef<'a>>,
}

impl<'a> Node<'a> {
    // return a reference counted pointer to a new node
    pub fn new(name: &'a str) -> NodeRef<'a> {
        Rc::new(RefCell::new(Node {
            name: name,
            adjacent: Vec::new(),
        }))
    }

    // add an adjacent node
    pub fn add_adjacent(&mut self, neighbor: &NodeRef<'a>) {
        self.adjacent.push(neighbor.clone());
    }
}

// each edge contains a weight and references to the two end nodes
// #[derive(Debug)]
// struct Edge<'a, T: 'a> {
//    weight: T,
//    endpoints: (NodeRef<'a>, NodeRef<'a>),
// }

// a collection of nodes and edges
#[derive(Debug)]
struct Graph<'a> {
    nodes: Vec<NodeRef<'a>>, // edges: Vec<Edge<'a, T>>,
}

impl<'a> Graph<'a> {
    pub fn new() -> Graph<'a> {
        Graph {
            nodes: vec![],
            //edges: vec![],
        }
    }

    // add a new empty node with the specified data, returning the node index
    pub fn add_node(&mut self, datum: &'a str) -> usize {
        self.nodes.push(Node::new(datum));
        self.nodes.len() - 1
    }

    // add an edge between two specified nodes
    pub fn add_edge(&mut self, start: usize, end: usize) {
        (self.nodes[start].borrow_mut()).add_adjacent(&self.nodes[end]);
        (self.nodes[end].borrow_mut()).add_adjacent(&self.nodes[start]);
    }
}

// cannot rely on derive macro for cyclic data structures, because displaying the adjacent nodes
// causes a stack overflow
impl<'a> fmt::Debug for Node<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let edge_names = self.adjacent.iter().map(|x| x.borrow_mut().name).collect::<Vec<&str>>();
        write!(f,
               "Node {{ name: {}, adjacent: {:?} }}",
               self.name,
               edge_names)
    }
}
