// trait 继承
// 一个 trait 可以继承其他 trait 的功能:

trait Trait1 {
    fn method1(&self);
}

trait Trait2: Trait1 {
    fn method2(&self);
}

struct MyStruct;

impl Trait1 for MyStruct {
    fn method1(&self) {
        println!("Method 1");
    }
}

impl Trait2 for MyStruct {
    fn method2(&self) {
        println!("Method 2");
    }
}

fn main() {
    let my_struct = MyStruct;
    my_struct.method1();
    my_struct.method2();
}