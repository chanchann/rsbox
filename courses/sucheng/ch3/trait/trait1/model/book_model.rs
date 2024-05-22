#[derive(Debug)]
pub struct Book {
    pub id: i32,
    pub price: f32,
}

// pub fn new_book(id: i32, price: f32)->Book {
//     Book {
//         id,
//         price
//     }
// }

// impl Book {
//     pub fn get_price(&self)->&f32 {
//         &self.price
//     }
// }