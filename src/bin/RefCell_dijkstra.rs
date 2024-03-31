use std::cell::RefCell;
use std::rc::Rc;
use std::collections::binary_heap;
use std::ops::DerefMut;

#[derive(Eq, PartialEq)]
struct Node {
    id: u32,
    visited: bool,
    dis: u32,
    next: Vec<(Rc<RefCell<Node>>, u32)>,
}

/// `dis` here is just the key for the heap
#[derive(Eq, PartialEq)]
struct HeapItem {
    node: Rc<RefCell<Node>>,
    dis: u32,
}

impl PartialOrd for HeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.dis.cmp(&self.dis))
    }
}

impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dis.cmp(&self.dis)
    }
}

impl HeapItem {
    fn new(node: Rc<RefCell<Node>>, dis: u32) -> HeapItem {
        HeapItem { node, dis }
    }
    fn node(self) -> Rc<RefCell<Node>> {
        self.node
    }
}

impl Node {
    fn new(id: u32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            id,
            visited: false,
            dis: i32::MAX as u32,
            next: vec![],
        }))
    }
    fn add_edge(&mut self, node: Rc<RefCell<Node>>, dis: u32) {
        self.next.push((node, dis));
    }
}

struct Graph {
    nodes: Vec<Rc<RefCell<Node>>>,
    start: Rc<RefCell<Node>>,
}

impl Graph {
    /// Create a new graph with `n` nodes and `st` as the starting node
    fn new(n: u32, st: u32) -> Graph {
        let mut nodes = Vec::with_capacity(n as usize);
        for i in 0..n {
            nodes.push(Node::new(i));
        }
        let start_node = Rc::clone(&nodes[st as usize]);
        Graph {
            nodes,
            start: start_node,
        }
    }
    fn dijkstra(&self) {
        let mut heap = binary_heap::BinaryHeap::new();
        {
            self.start.borrow_mut().dis = 0;
        }
        heap.push(HeapItem::new(Rc::clone(&self.start), 0));
        while let Some(HeapItem { node, dis }) = heap.pop() {
            let mut node = node.borrow_mut();
            if node.visited {
                continue;
            }
            node.visited = true;
            for (next, next_dis) in &node.next {
                let next = Rc::clone(next);
                let next_dis = *next_dis;
                let next_dis = dis + next_dis;
                let mut next_mut = next.borrow_mut();
                if next_dis < next_mut.dis {
                    next_mut.dis = next_dis;
                    heap.push(HeapItem::new(next.clone(), next_dis));
                }
            }
        }
    }
    fn print_dis(&self) {
        println!(
            "{}",
            self.nodes
                .iter()
                .map(|node| node.borrow().dis.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}

fn read_line() -> Vec<u32> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn main() {
    let data = read_line();
    let n = data[0];
    let m = data[1];
    let st = data[2];
    let graph = Graph::new(n, st);
    for _ in 0..m {
        let data = read_line();
        let u = data[0];
        let v = data[1];
        let dis = data[2];
        if u != v{
            graph.nodes[u as usize]
                .borrow_mut()
                .add_edge(Rc::clone(&graph.nodes[v as usize]), dis);
        }
    }
    graph.dijkstra();
    graph.print_dis()
}