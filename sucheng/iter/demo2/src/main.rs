struct User {
    id: i32,
    age: i32,
    score: i32,
}

impl User {
    fn iter(self)->UserIterator {
        UserIterator{index: 0, item: self}
    }
}

struct UserIterator {
    index: usize,
    item: User,
}

impl Iterator for UserIterator {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.index {
            0 => Some(self.item.id),
            1 => Some(self.item.age),
            2 => Some(self.item.score),
            _ => None,
        };
        self.index += 1;
        ret
    }
}

fn main() {
   let u = User{id: 102, age: 19, score: 100};
   // We know that iter() return &T
   // But here item is i32, so we have to fix it
   // Look at demo3
   u.iter().for_each(|item|{
    println!("{}", item);
   });
}
