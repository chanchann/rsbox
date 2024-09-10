use std::any::Any;

// 定义一个可以存储任何类型的容器
struct AnyContainer {
    data: Box<dyn Any>,
}

impl AnyContainer {
    fn new<T: 'static>(value: T) -> Self {
        AnyContainer {
            data: Box::new(value),
        }
    }

    fn get<T: 'static>(&self) -> Option<&T> {
        self.data.downcast_ref::<T>()
    }
}

fn main() {
    let container = AnyContainer::new(42);
    
    if let Some(value) = container.get::<i32>() {
        println!("值是: {}", value);
    }
    
    if let None = container.get::<String>() {
        println!("不是字符串类型");
    }
}
