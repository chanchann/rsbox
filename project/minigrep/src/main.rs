use std::env;

fn main() {
    // collect 方法其实并不是std::env包提供的，而是迭代器自带的方法(env::args() 会返回一个迭代器)，它会将迭代器消费后转换成我们想要的集合类型
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}