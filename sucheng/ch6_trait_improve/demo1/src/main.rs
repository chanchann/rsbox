use std::fmt;

#[derive(Debug, Default)]
struct User {
    id: i32,
    name: String,
}

impl User {
    fn new() -> Self {
        User { ..Default::default() }
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(id: {}, name: {})", self.id, self.name)
    }
}

fn main() {
    let u = User::new();
    println!("{}", u);
}