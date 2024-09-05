// 组合 (Composition)
// 你可以在一个结构体中包含其他结构体,然后委托方法调用:

struct Struct1;
impl Struct1 {
    fn method1(&self) {
        println!("Method 1");
    }
}

struct Struct2;
impl Struct2 {
    fn method2(&self) {
        println!("Method 2");
    }
}

struct MyStruct {
    s1: Struct1,
    s2: Struct2,
}

impl MyStruct {
    fn method1(&self) {
        self.s1.method1();
    }

    fn method2(&self) {
        self.s2.method2();
    }
}

fn main() {
    let my_struct = MyStruct {
        s1: Struct1,
        s2: Struct2,
    };
    my_struct.method1();
    my_struct.method2();
}