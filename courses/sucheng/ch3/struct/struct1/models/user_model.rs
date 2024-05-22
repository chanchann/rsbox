#[derive(Debug)]
pub struct UserModel {
    user_id: i32,
    user_name: String
}

pub fn new_user_model()->UserModel {
    UserModel {
        user_id: 0,
        user_name: String::new()
    }
}