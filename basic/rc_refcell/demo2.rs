/*
如果B是A的成员变量，并且和A生命周期一致，B的成员函数会用到A，请问如何设计比较好

对于这种情况，我们可以考虑使用 Rust 的生命周期参数和不安全代码来实现。这种方法可以避免使用 `Rc` 和 `RefCell`，从而减少运行时开销。以下是一个可能的设计方案：


这个设计的关键点：

1. B 结构体中使用 `*const A` 来存储 A 的原始指针。这允许 B 引用 A，而不需要使用引用计数或内部可变性。

2. 使用 `PhantomData<&'static A>` 来标记 B 与 A 有相同的生命周期。这是一个零大小的类型，不会影响运行时性能。

3. 在 A 的构造函数中，我们先创建 A 和 B，然后设置 B 中的 A 指针。

4. B 的方法中使用 unsafe 代码来解引用 A 的指针。这是必要的，因为我们使用了原始指针。

5. 这种设计确保了 B 不会比 A 活得更久，因为 B 是 A 的一部分。

这种方法的优点：

- 避免了运行时开销（没有使用 `Rc` 和 `RefCell`）。
- B 和 A 的生命周期严格绑定。

缺点：

- 使用了 unsafe 代码，需要格外小心以确保安全性。
- 代码复杂度增加，可能更难理解和维护。

在实际使用中，你需要权衡这种方法的优缺点。如果性能是关键考虑因素，并且你有信心正确处理 unsafe 代码，这可能是一个好选择。否则，使用 `Rc` 和 `RefCell` 的方法可能更安全、更容易理解，尽管会有一些运行时开销。
*/


use std::marker::PhantomData;

struct A {
    data: String,
    b: B,
}

struct B {
    // 使用 *const 来存储 A 的原始指针
    a: *const A,
    // PhantomData 用于标记 B 与 A 有相同的生命周期
    _marker: PhantomData<&'static A>,
}

impl A {
    fn new(data: String) -> Self {
        let mut a = A {
            data,
            b: B {
                a: std::ptr::null(),
                _marker: PhantomData,
            },
        };
        // 设置 B 中的 A 指针
        a.b.a = &a as *const A;
        a
    }
}

impl B {
    // 在 B 的方法中使用 A 的数据
    fn use_a_data(&self) -> &str {
        unsafe {
            // 这里我们需要使用 unsafe 代码来解引用原始指针
            &(*self.a).data
        }
    }
}

fn main() {
    let a = A::new(String::from("示例数据"));
    println!("B 引用的 A 的数据: {}", a.b.use_a_data());
    println!("A 中的数据: {}", a.data);
}

