mod models;

fn main() {
    let user = models::user_model::new_user_model();
    println!("{:?}", user);
}