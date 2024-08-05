use std::rc::Rc;

// 简单的 Rc 使用示例
fn basic_rc() {
    let x = Rc::new(5);
    let y = x.clone();
    let z = x.clone();

    println!("Reference count: {}", Rc::strong_count(&x));
    println!("x: {}, y: {}, z: {}", x, y, z);
}

// 在结构体中使用 Rc
#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Rc<Node>>,
}

fn linked_list() {
    let node3 = Rc::new(Node { value: 3, next: None });
    let node2 = Rc::new(Node { value: 2, next: Some(node3.clone()) });
    let node1 = Rc::new(Node { value: 1, next: Some(node2.clone()) });

    println!("Node 1: {:?}", node1);
    println!("Reference count of node2: {}", Rc::strong_count(&node2));
}

fn main() {
    basic_rc();
    linked_list();
}