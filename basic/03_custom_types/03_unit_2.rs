// 2. 实现特征：
// Dog和Cat是unit structs，它们实现了Animal特征。

trait Animal {
    fn make_sound(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }
}

impl Animal for Cat {
    fn make_sound(&self) {
        println!("Meow!");
    }
}

fn main() {
    let dog = Dog;
    let cat = Cat;
    
    dog.make_sound();
    cat.make_sound();
}