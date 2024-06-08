// 宏不是函数，参数分为两个部分
// 1. 参数前缀 ($ 开头)
// 2. 指示符，有多种，比较常用的有 expr(用于表达式)，stmt(语句) ...

use macro_study::echo;

macro_rules! me {
    // 展开代码进行执行
    () => (
        println!("Hello, me!");
    );
    ($exp: expr) => {
        println!("{}", stringify!($exp));
    };
    // 可变参数
    // * / +
    ($($exp: expr), *) => {
        $(
            println!("{}", stringify!($exp));
        )+
    };
}

fn main() {
    me!();
    echo!();
    echo!("echo param!"); // 这里字符串也是一个表达式
    let a = 3;
    echo!(a == 3);
    echo!(a);
    me!(a == 3);
}