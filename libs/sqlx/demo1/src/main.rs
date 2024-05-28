use sqlx::{mysql::*, pool};
use sqlx::Row;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = MysqlPoolOptions::new().max_connections(10)
                                      .connect("mysql://root:123123@localhost:3306/rustdb").await?;
    let ret = sqlx::query("select id, user_name, user_age from users").fetch_all(&pool).await?;
    ret.iter().for_each(|row| {
        row.get<i32, &str>("id");
        row.get::<&str, _>(1);
    });

    let ret = sqlx::query("insert into users (user_name, user_age) values (?,?)").bind("abc").bind(10).execute(&pool).await?;
    println!("{:?}", ret.last_insert_id());
    Ok(())
}
