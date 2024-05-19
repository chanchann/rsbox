use crate::model::book_model::Book;
use crate::model::phone_model::Phone;

pub trait Prods {
    fn new(id: i32, price: f32) -> Self;
    fn get_price(&self)->f32;
}

impl Prods for Book {
    fn new(id: i32, price: f32) -> Book {
        Book{id, price}
    }
    fn get_price(&self)->f32 {
        &self.price + 10.0
    }
}

impl Prods for Phone {
    fn new(id: i32, price: f32) -> Phone {
        Phone{id, price}
    }
    fn get_price(&self)->f32 {
        &self.price + 20.0
    }
}