use libloading::{Library, library_filename, Symbol};

fn main() {
    unsafe {
        // 加载“hello_world”库
        let lib = Library::new(library_filename("hello_world")).unwrap();
         // 获取函数指针
        let func: Symbol<fn()> = lib.get(b"execute").unwrap();
        func() // 调用函数
    }
}