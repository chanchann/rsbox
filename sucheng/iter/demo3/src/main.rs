struct User {
    id: i32,
    age: i32,
    score: i32,
}

impl User {
    fn iter(&self)->UserIterator {
        UserIterator{index: 0, item: self}
    }
}

struct UserIterator<'a> {
    index: usize,
    item: &'a User,
}

impl<'a> Iterator for UserIterator<'a> {
    type Item = &'a i32;
    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.index {
            0 => Some(&self.item.id),
            1 => Some(&self.item.age),
            2 => Some(&self.item.score),
            _ => None,
        };
        self.index += 1;
        ret
    }
}

impl<'a> IntoIterator for &'a User {
    type Item = &'a i32;
    type IntoIter = UserIterator<'a>;
    fn into_iter(self) -> Self::IntoIter {
        UserIterator {
            index: 0,
            item: self,
        }
    }
}

fn main() {
    let u = User{id: 102, age: 19, score: 100};
    // here item becomes &i32
    u.iter().for_each(|item|{
        println!("{}", item);
    });
    u.into_iter().for_each(|item|{
        println!("{}", item);
    });
}
