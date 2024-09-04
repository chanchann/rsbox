/*
使用trait实现"虚类"功能：

Rust中的多态性,通过trait对象,我们可以在运行时处理不同类型的对象,只要它们实现了相同的trait。
*/

trait Animal {
    fn make_sound(&self) -> String;
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn make_sound(&self) -> String {
        "汪汪".to_string()
    }
}

impl Animal for Cat {
    fn make_sound(&self) -> String {
        "喵喵".to_string()
    }
}

fn animal_sound(animal: &dyn Animal) {
    println!("这只动物发出的声音是: {}", animal.make_sound());
}

fn main() {
    let dog = Dog;
    let cat = Cat;

    animal_sound(&dog);
    animal_sound(&cat);

    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)];
    for animal in animals.iter() {
        println!("动物说: {}", animal.make_sound());
    }
}