// if writing style

fn get_user(uid: i32) -> &'static str {
    let ret = if uid == 0 {
        "000"
    } else if uid == 1 {
        "111"
    } else {
        "unknown"
    };
    ret
}


fn main() {
    let a = 1;
    let b = if a == 1 {
        5
    } else {
        10
    };
    println!("{}", b);
    println!("{}", get_user(a));
}