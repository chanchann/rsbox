## ref

https://course.rs/basic-practice/base-features.html

## 接收命令行参数

usage:

cargo run -- searchstring example-filename.txt

-- 告诉 cargo 后面的参数是给我们的程序使用的，而不是给 cargo 自己使用，例如 -- 前的 run 就是给它用的。

## 不可信的输入

用户会输入什么你根本就不知道，例如他输入了一个非 Unicode 字符，你能阻止吗？显然不能，但是这种输入会直接让我们的程序崩溃！

原因是当传入的命令行参数包含非 Unicode 字符时， std::env::args 会直接崩溃，如果有这种特殊需求，建议大家使用 std::env::args_os，该方法产生的数组将包含 OsString 类型，而不是之前的 String 类型，前者对于非 Unicode 字符会有更好的处理。