// 用macro创建自定义函数

macro_rules! func {
    ($fn_name: ident) => {
        fn $fn_name() {
            println!("my func : {}", stringify!($fn_name));
        }
    }
}

func!(php);

fn main() {
    php();
}  