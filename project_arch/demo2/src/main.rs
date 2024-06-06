mod user_info;
use user_info::user::User;

fn main() {
    let user = User::new("123".to_string(), 12);
    println!("{:?}", user.name());
}
