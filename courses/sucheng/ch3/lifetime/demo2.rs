#[derive(Debug)]
struct User<'a> {
    id: &'a i32,
}

fn main() {
    let mut id = 109;  // like this is my car
    let u = User{id: &id};  // my car is borrowed
    println!("{:?}", u);
    id = 25;  // can't use
    // borrowed
    // println!("{:?}", u);  // Cuz this is not give back
    
}