struct TreeNode {
    val : i32,
    depth : i32,
    left : Option<Box<TreeNode>>,
    right : Option<Box<TreeNode>>,
}
impl TreeNode {
    /// create a new tree node with the given value and depth
    pub fn new(val: i32, depth: i32) -> Self {
        TreeNode {
            val,
            depth,
            left: None,
            right: None,
        }
    }

    /// insert a value into the tree with the following rules:
    /// - if the value is less than the current node, insert it into the left subtree
    /// - if the value is greater than or equal to the current node, insert it into the right subtree
    /// - if the left or right subtree is empty, create a new node with the value and insert it
    pub fn insert(&mut self, val: i32) {
        if val < self.val{
            match &mut self.left {
                Some(node) => node.insert(val),
                None => self.left = Some(Box::new(TreeNode::new(val, self.depth + 1))),
            }
        } else {
            match &mut self.right {
                Some(node) => node.insert(val),
                None => self.right = Some(Box::new(TreeNode::new(val, self.depth + 1))),
            }
        }
    }

    /// tranverse the tree and find the node with maximum product of depth and value
    pub fn find_max(&self) -> i32 {
        let mut max = self.val * self.depth;
        if let Some(node) = &self.left {
            max = max.max(node.find_max());
        }
        if let Some(node) = &self.right {
            max = max.max(node.find_max());
        }
        max
    }

    /// create a tree from a vector of values
    /// the first value is the root, and the rest are inserted into the tree
    /// the depth of the root should be 1
    pub fn from_vec(v: Vec<i32>) -> Self {
        let (first, rest) = v.split_first().unwrap();
        let mut root = TreeNode::new(*first, 1);
        for &val in rest {
            root.insert(val);
        }
        root
    }
}
fn read_input() -> Vec<i32> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn quiz() {
    assert_eq!(24, TreeNode::from_vec(vec![3, 1, 2, 7, 5, 4, 6]).find_max());
    let v = read_input();
    let root = TreeNode::from_vec(v);
    println!("{}", root.find_max());
}

fn main() {
    quiz();
}