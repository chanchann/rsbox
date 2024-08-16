// 4. 在 impl 块中使用生命周期：

struct MyStruct<'a> {
    x: &'a i32,
}

impl<'a> MyStruct<'a> {
    fn get_x(&self) -> &'a i32 {
        self.x
    }
}