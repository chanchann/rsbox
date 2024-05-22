mod lib;
use lib::*;

// TODO : 这里还有问题，无参和有参构造无法同时存在

fn main() {
    // let u = User::new();
    // println!("{}", u);

    let u = User::default();
    println!("{}", u);

    let u = User::new(10);
    println!("{}", u);

    let u = User::new("hello");
    println!("{}", u);
}
