#[derive(Debug)]
struct User {
    name: String,
    age: u8
}

fn main() {
    let me = User{name: String::from("name"), age: 19};
    println!("{:?}", me);
}