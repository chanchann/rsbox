use crate::model::book_model::Book;

pub trait Stock {
    fn get_stock(&self) -> i32;
}

impl Stock for Book {
    fn get_stock(&self) -> i32 {
        2
    }
}