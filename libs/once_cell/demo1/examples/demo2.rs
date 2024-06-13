// Lazy产生全局变量
// 起到类似之前提到过的lazy_static的作用——在第一次使用的时候产生一个动态的全局静态变量
// 好处就是不需要神奇的宏魔法
// 比起lazy_static，我们还支持局部变量
// 使用OnceCell还可以嵌入到结构体中

use std::{sync::Mutex, collections::HashMap};
use once_cell::sync::Lazy;

static GLOBAL_DATA: Lazy<Mutex<HashMap<i32, String>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(13, "Spica".to_string());
    m.insert(74, "Hoyten".to_string());
    Mutex::new(m)
});

fn main() {
    println!("{:?}", GLOBAL_DATA.lock().unwrap());
}