use std::fmt;
use super::prelude::*;

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
}

// impl User {
    // pub fn new() -> Self {
    //     User { ..Default::default() }
    // }
    // 这里无法重载
    // fn new(id: i32) -> Self {
    //     User {id:id, ..Default::default()}
    // }
// }

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(id: {}, name: {})", self.id, self.name)
    }
}

impl UserInit<i32> for User {
    fn new(id: i32) -> Self {
        User { id,..Default::default() }
    }
}

impl UserInit<&str> for User {
    fn new(name: &str) -> Self {
        User { name:name.to_string(),..Default::default() }
    }
}

// 和上面是不能被同时引用的
// impl CommonInit for User {
//     fn new() -> Self {
//         User {..Default::default() }
//     }
// }

impl Default for User {
    fn default() -> Self {
        User {
            id: 1,
            name: String::from("default"),
        }
    }
}
