fn get_user(uid: i32) -> &'static str {
    if uid == 0 {
        return "000";
    } else if uid == 1 {
        return "111";
    } else {
        return "unknown";
    }
}

fn main() {
    // type : &str
    let name = "chanchan";
    let name_complete: &'static str = "chan";
    let uid = 1;
    let name = get_user(uid);
    println!("{}", name);
}