// 4. 作为零大小类型（ZST）：
// Unit structs是零大小类型，不占用内存空间。这在某些情况下可能很有用，比如当你需要满足某些类型系统的要求，但实际上不需要存储任何数据时。

use std::mem::size_of;

struct Empty;

fn main() {
    println!("Size of Empty: {}", size_of::<Empty>()); // 输出：0
}