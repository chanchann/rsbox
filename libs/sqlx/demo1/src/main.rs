use sqlx::{mysql::*, pool};
use sqlx::Row;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    for (key, value) in env::vars() {
        println!("Env variables: {}: {}", key, value);
    }
    let connection_str = env::var("DATABASE_URL")
        .expect("Database url not specified");
    println!("Connection string: {}", connection_str);


    let pool = MySqlPoolOptions::new().max_connections(10)
                                      .connect(&connection_str).await?;
    let ret = sqlx::query("select id, user_name, user_age from test_user").fetch_all(&pool).await?;
    ret.iter().for_each(|row| {
        println!("id : {}, name : {}", row.get::<i32, _>("id"),
                                       row.get::<&str, _>(1));
    });

    let ret = sqlx::query("insert into test_user (user_name, user_age) values (?,?)").bind("abc").bind(10).execute(&pool).await?;
    println!("{:?}", ret.last_insert_id());
    Ok(())
}
