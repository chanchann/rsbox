#[derive(Debug)]
struct User {
    name: String,
    age: u8
}

impl User {
    fn version(&self) {
        println!("1.0");
    }

    fn to_string(&self)->String {
        format!("{}-{}", &self.name, &self.age)
    }
}

fn main() {
    let me = User{name: String::from("name"), age: 19};
    println!("{:?}", me);
    println!("{:#?}", me);
    me.version();
    println!("{}", me.to_string());
}