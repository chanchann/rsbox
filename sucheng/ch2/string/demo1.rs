fn main() {
    let mut name = String::new();
    name.push_str("chanchan");
    println!("{}", name);
    
    let s = "chan";
    let name = String::from(s);
    println!("{}", name);

    let first_name = String::from("chan");
    let last_name = String::from("yi");
    let name = first_name + &last_name;  // be careful of this &
    println!("{}", name);

    let first_name = String::from("chan");
    let last_name = String::from("yi");
    let name = format!("{} {}", first_name, last_name);
    println!("{}", name);
}