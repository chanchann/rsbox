use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
struct User {
    #[serde(default="User::default_age")]
    id: i32,
    #[serde(rename(serialize = "user_name", deserialize = "user_name"))]
    name: String,
}

impl User {
    fn default_age() -> i32 {
        28
    }
}

fn main() {
    let user = User {
        id: 1,
        name: String::from("hello"),
    };

    let json = serde_json::to_string(&user).unwrap();
    println!("{}", json);

    let user: User = serde_json::from_str(&json).unwrap();
    println!("{:?}", user);

    let user_str = r#"
    {
        "user_name": "hello"
    }"#;
    let user: User = serde_json::from_str(user_str).unwrap();
    println!("{:?}", user);
}