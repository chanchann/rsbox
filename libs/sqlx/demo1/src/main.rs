use sqlx::{mysql::*, pool};
use sqlx::Row;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = MysqlPoolOptions::new().max_connections(10)
                                      .connect("mysql://root:123123@localhost:3306/rustdb").await?;
    let ret = sqlx::query("select id, user_name, user_age from users").fetch_all(&pool).await?;
    ret.iter().for_each(|row| {
        row.get("id");
    });
}
