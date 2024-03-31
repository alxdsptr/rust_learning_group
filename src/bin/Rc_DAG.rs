use std::rc::Rc;

/// it's ok to modify the struct definition
struct GraphNode {
    val: u32,
    next: Vec<Rc<GraphNode>>,
}

impl GraphNode {
    /// you should modify the function signature and implement the function body
    // fn new(val : i32, vec : Vec<Rc<GraphNode>>) -> Self {
    //
    // }


    /// feel free if you want to use other method to traverse the graph
    fn dfs(&self) -> u32 {
        let mut max_val = 0;
        for node in self.next.iter() {
            max_val = max_val.max(node.dfs());
        }
        self.val + max_val
    }
}

/// Used to read the first line of input - the number of nodes
fn read_nodes_num() -> u32 {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}

/// Used to read the next n lines of input.
/// Each time it reads one line and returns a vector of the numbers in the line,
/// The first number is the value of the node,
/// and the rest are the indices of the nodes it points to
fn read_one_node_info() -> Vec<u32> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

/// you can change the function signature if you want
fn create_graph(n: u32) -> Rc<GraphNode> {
    let mut nodes: Vec<Rc<GraphNode>> = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let info = read_one_node_info();
        let val = info[0];
        let mut next = Vec::with_capacity(info.len() - 1);
        for &idx in &info[1..] {
            next.push(nodes[idx as usize].clone());
        }
        nodes.push(Rc::new(GraphNode { val, next }));
    }
    nodes[(n-1) as usize].clone()
}

/// you can also use other method to solve the problem
/// feel free to change the template code
fn quiz() {
    let n = read_nodes_num();
    let start_node = create_graph(n);
    println!("{}", start_node.dfs());
}
fn main() {
    quiz()
}