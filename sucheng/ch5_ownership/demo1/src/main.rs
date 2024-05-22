// use std::marker::Copy;
// 1. Copy 和 Move 很类似。不过copy后，不会销毁原有变量
// 2. 一个类型所有属性都实现了Copy, 它本身也可以实现Copy
// 3. 常见数字类型，bool类型都实现了Copy
// 4. 凡是实现drop trait的不能再实现Copy(String)
// 5. Copy好比是一种浅拷贝(Clone是深拷贝)
// 6. Copy告诉编译器这个类型默认采用copy，而不是move

#[derive(Debug, Copy, Clone)]
struct User {
    id: i32
}

#[derive(Debug)]
struct Person {
    id: i32
}

// Just tell compiler this type can be copy
impl Copy for Person {}

impl Clone for User {
    fn clone(&self) -> Self {
        &self
    }
}

// wrong, what can we do?
// #[derive(Debug, Copy, Clone)]
// struct User {
//     id: i32,
//     name: String  // 没法实现copy了
// }

fn main() {
    let a = String::from("Hello");
    // a 转移给了b
    // let b = a;
    // println!("{}", a);
    let b = a.clone();  // deep copy
    println!("{}, {}", a, b);

    let a = 1;
    let b = a;
    println!("{}, {}", a, b);
    // 为什么这里可以，因为String没有实现copy trait

    let a = User { id: 1 };
    let b = a;
    println!("{:?}", a);

    let a = Person { id: 1 };
    let b = a;
    println!("{:?}", a);

}
