mod lib {
    pub fn show_me() {
        let age = 13;
        println!("age : {}", age);
    }
}

fn main() {
    lib::show_me();
}