use serde::{Serialize};
use serde::ser::{Serializer, SerializeStruct};

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
}

impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer {
        let mut s = serializer.serialize_struct("User", 2)?;
        s.serialize_field("user_id", &self.id)?;
        s.serialize_field("user_name", &self.name)?;
        s.end()
    }
}

fn main() {
    let user = User {
        id: 1,
        name: String::from("hello"),
    };

    let json = serde_json::to_string(&user).unwrap();
    println!("{}", json);
}
