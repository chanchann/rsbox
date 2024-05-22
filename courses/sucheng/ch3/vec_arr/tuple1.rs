fn main() {
    let my: (&str, u8) = ("chanchan", 19);
    println!("{:?}", my);
    println!("{} - {}", my.0, my.1)
}