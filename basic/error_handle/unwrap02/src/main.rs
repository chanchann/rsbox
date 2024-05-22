use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn typed_example() -> Result<()> {
    //age2 is error on purpose
    let data = r#"
        {
            "name": "John Doe",
            "age2": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    let p:Person = serde_json::from_str(data).unwrap();

    println!("Please call {} at the number {}", p.name, p.phones[0]);

    Ok(())
}

fn main() {
    match typed_example() {
        Ok(_) => println!("program ran ok"),
        Err(_) => println!("program ran with error"),
    }
}
