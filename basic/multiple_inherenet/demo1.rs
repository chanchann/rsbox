// 多继承替代方案1：
// 使用多个trait

trait Trait1 {
    fn method1(&self);
}

trait Trait2 {
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