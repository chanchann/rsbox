mod model;
mod api;

use api::prods::Prods;
use api::stock::Stock;
use model::book_model::*;
use model::phone_model::*;

fn show_prod(p: impl Prods) {
    println!("price : {}", p.get_price());
}

// A more normal way is to use generics type
fn show_prod_generic<T: Prods>(p : T) {
    println!("price : {}", p.get_price());
}

fn sum<T: Prods, U: Prods>(p1: T, p2: U) {
    println!("Sum : {}", p1.get_price() + p2.get_price());
}

fn test_two_trait() {
    let book: Book = Prods::new(101, 20.6);
    let phone: Phone = Prods::new(102, 20.1);
    sum(book, phone);
}

fn show_detail<T: Prods + Stock>(p: T) {
    println!("{} : {}", p.get_price(), p.get_stock());
}

fn show_detail_two<T>(p: T) 
where T:Prods + Stock {
    println!("{} : {}", p.get_price(), p.get_stock());
}

fn add_trait() {
    let book1: Book = Prods::new(101, 20.6);
    let book2: Book = Prods::new(101, 20.6);
    println!("{}", book1+book2);
}

fn main() {
    // Must specify type here
    let book: Book = Prods::new(12, 25.0);
    println!("{:?}", book);
    println!("{:?}", book.get_price());
    show_prod(book);

    let book: Book = Prods::new(24, 25.0);
    let stock = book.get_stock();
    println!("stock : {}", stock);
    show_prod_generic(book);

    test_two_trait();

    let book: Book = Prods::new(24, 25.0);
    show_detail(book);

    let book: Book = Prods::new(24, 25.0);
    show_detail_two(book);

    add_trait();
}