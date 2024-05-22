#[derive(Debug)]
struct User {
    id: u32,
    sex: Option<String>,
}

fn check(u: User) {
    if let Some(s) = u.sex {
        println!("{}", s);
    }
}

fn main() {
    let u = User {
        id: 10,
        sex: Some(String::from("man")),
    };
    check(u);
}