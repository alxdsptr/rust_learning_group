// ! REMEMBER TO RENAME YOUR `RefCell::new` and `Option::map_or_else`

use std::cell::RefCell;

fn stack_interface(init: Vec<i32>) -> impl FnMut(Option<i32>) -> Option<i32> {
    // let mut stack = init;
    let stack = RefCell::my_new(init);
    move |op| {
        op.my_map_or_else(||stack.borrow_mut().pop(), |x|
            {stack.borrow_mut().push(x); None})
    }
}

/// Test your function.
fn test_func()
{
    let stdin = std::io::stdin();
    let mut stack_op = stack_interface(vec![]);
    let mut buffer = String::new();
    while let Ok(count) = stdin.read_line(&mut buffer) {
        if count == 0 {
            break;
        }

        let op = buffer.trim().parse::<i32>().ok();
        if let Some(out) = stack_op(op) {
            println!("{}", out);
        } else if op.is_none() {
            println!("None");
        }

        buffer.clear();
    }
}

fn main() {
    test_func();
}