use uuid::Uuid;

fn main() {
    // 生成一个随机的UUID (v4)
    let id = Uuid::new_v4();
    println!("{}", id);

    // 从字符串解析UUID
    let uuid = Uuid::parse_str("936DA01F-9ABD-4d9d-80C7-02AF85C822A8").expect("Invalid UUID");
    println!("{}", uuid);

    // 生成一个简单的UUID字符串（没有破折号）
    let simple = Uuid::new_v4().simple().to_string();
    println!("{}", simple);
}