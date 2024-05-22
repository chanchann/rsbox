#[derive(Debug, Default)]
struct User {
    id: i32,
    name: String,
}

// 自己实现clone
impl Clone for User {
    fn clone(&self) -> Self {
        User {
            id: self.id,
            name: self.name.clone() + "_copy",
        }
    }
}

fn main() {
    let a = User { id:1, ..Default::default() };
    let b = a.clone();
    println!("{:?}, {:?}", a, b);

    // 可变引用排他性
    // EROOR :
    // let user = User { id:1, ..Default::default() };
    // let user1 = &mut user;
    // let user2 = &mut user;
    // println!("{:?}, {:?}", user1, user2);
}