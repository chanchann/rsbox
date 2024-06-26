#[derive(Debug)]
pub struct User {
    name: String,
    age: i32
}

impl User {
    pub fn new(name: String, age: i32) -> User {
        User{
            name,
            age
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}