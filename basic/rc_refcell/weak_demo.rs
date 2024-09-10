/*
使用`Weak`指针来打破循环引用主要在以下情况下是必要的：

1. 父子关系结构：
   当一个数据结构中存在父节点和子节点，并且子节点需要引用父节点时。例如，在树形结构或图结构中。

2. 观察者模式：
   当实现观察者模式时，被观察对象和观察者之间可能形成循环引用。

3. 缓存系统：
   在某些缓存实现中，可能需要保持对缓存项的弱引用，以允许在内存压力下自动清理。

4. 回调函数：
   当对象持有对自身的回调函数时，可能形成循环引用。

5. 双向链表：
   在实现双向链表时，节点之间的相互引用可能导致循环引用。


1. `Parent`持有对`Child`的强引用（`Rc`）。
2. `Child`持有对`Parent`的弱引用（`Weak`）。

使用`Weak`的好处：

1. 防止内存泄漏：
   打破了强引用循环，允许对象在不再被使用时被正确释放。

2. 表达所有权语义：
   `Weak`表示非拥有关系，而`Rc`表示共享所有权。

3. 灵活性：
   允许对象在其引用的对象已经被释放的情况下继续存在。

4. 性能考虑：
   `Weak`指针不增加强引用计数，可能在某些情况下提高性能。

使用`Weak`时需要注意：

1. 访问`Weak`指针指向的数据时，需要先调用`upgrade()`方法将其转换为`Rc`。
2. `upgrade()`可能返回`None`，表示引用的对象已经被释放。

总之，当您发现数据结构中存在潜在的循环引用，并且这些引用之间存在明确的所有权或生命周期关系时，应考虑使用`Weak`指针来打破循环引用。
*/

use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Parent {
    children: RefCell<Vec<Rc<Child>>>,
}

struct Child {
    parent: Weak<Parent>,
}

impl Parent {
    fn new() -> Rc<Self> {
        Rc::new(Parent {
            children: RefCell::new(Vec::new()),
        })
    }

    fn add_child(self: &Rc<Self>, child: Rc<Child>) {
        self.children.borrow_mut().push(child);
    }
}

impl Child {
    fn new(parent: &Rc<Parent>) -> Rc<Self> {
        let child = Rc::new(Child {
            parent: Rc::downgrade(parent),
        });
        parent.add_child(child.clone());
        child
    }
}

fn main() {
    let parent = Parent::new();
    let _child = Child::new(&parent);

    // 父节点和子节点可以正常释放，不会造成内存泄漏
}
```

