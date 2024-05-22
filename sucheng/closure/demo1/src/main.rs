// 闭包类似匿名函数
// 闭包能操作和访问外部环境的变量，使得外部变量和闭包函数共存


fn main() {
    let f = || {
        println!("{}", "abc");
    };

    f();

    let str = String::from("abc");
    let f = || {
        println!("{}", str);
    };
    // &T, 可以执行多次
    f();
    f();
}
