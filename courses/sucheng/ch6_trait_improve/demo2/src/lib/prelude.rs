// 放trait

pub trait UserInit<T> {
    fn new(id: T) -> Self;
}

// pub trait CommonInit {
//     fn new() -> Self;
// }