fn max<'a>(a: &'a i32, b: &'a i32) ->&'a i32 {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    let a = 32;
    let b = 12;
    let c = max(&a, &b);
    println!("{}", c);
}