fn main() {
    // print address
    let name = String::from("abc");
    println!("{:p}", name.as_ptr());
}