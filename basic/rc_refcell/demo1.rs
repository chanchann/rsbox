/*
rust如何实现这个功能，有个struct A， 一个struct B，
B和A是组合模式，B是A的成员变量，他们之间的生命周期一致，
但是B的成员变量会引用A

使用了Rc和RefCell来处理这种循环引用：

这个实现的关键点：

1. 使用`Rc<RefCell<>>`来包装A和B，允许多个所有权和内部可变性。
2. A结构体中的B字段是`Option<Rc<RefCell<B>>>`，因为在A创建时B还不存在。
3. B结构体持有对A的引用，同时存储A的data的副本。
4. 在创建B时，我们先创建B，然后更新A使其包含B。
5. 使用`clone()`方法来创建`Rc`的新引用。

这种方法允许A和B相互引用，同时保持它们的生命周期一致。
但是要注意，这种方法会引入运行时开销，并且可能导致循环引用，需要小心处理以避免内存泄漏。

在实际应用中，你可能需要根据具体需求调整这个结构，可能的话尽量避免这种循环引用的设计。
*/
use std::rc::Rc;
use std::cell::RefCell;

struct A {
    data: String,
    b: Option<Rc<RefCell<B>>>,
}

struct B {
    a: Rc<RefCell<A>>,
    reference_to_a_data: String,
}

impl A {
    fn new(data: String) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(A {
            data,
            b: None,
        }))
    }

    fn set_b(&mut self, b: Rc<RefCell<B>>) {
        self.b = Some(b);
    }
}

impl B {
    fn new(a: Rc<RefCell<A>>) -> Rc<RefCell<Self>> {
        let reference_to_a_data = a.borrow().data.clone();
        let b = Rc::new(RefCell::new(B {
            a: a.clone(),
            reference_to_a_data,
        }));
        a.borrow_mut().set_b(b.clone());
        b
    }
}

fn main() {
    let a = A::new(String::from("示例数据"));
    let b = B::new(a.clone());
    
    println!("B引用的A的数据: {}", b.borrow().reference_to_a_data);
    println!("A中的数据: {}", a.borrow().data);
}