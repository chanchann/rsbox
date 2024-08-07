fn main() {
    let r = local_ref();
    println!("r: {:p}", r);
}

fn local_ref<'a>() -> &'a i32 {
    let a = 42;
    &a
}

/*
在这个上下文中，'a 是一个生命周期参数。生命周期参数是 Rust 中用来确保引用有效性的一种机制。具体来说：

1. 生命周期注解：'a 是一个生命周期注解，用于指定引用的有效范围。

2. 泛型参数：生命周期参数类似于泛型类型参数，但用于描述引用的持续时间。

3. 函数签名：在 fn local_ref<'a>() -> &'a i32 中，'a 表示返回的引用至少在 'a 生命周期内有效。

4. 编译器检查：Rust 编译器使用这些注解来验证所有借用都是有效的，防止悬垂引用。

5. 省略规则：在许多情况下，编译器可以推断生命周期，允许开发者省略显式注解。

生命周期注解帮助 Rust 在编译时确保内存安全，是 Rust 无垃圾回收yet内存安全的关键机制之一。
*/