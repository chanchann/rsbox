use sqlx::{mysql::*, Pool};
use sqlx::Row;
use tokio::runtime::Runtime;
use std::env;
use once_cell::sync::OnceCell;
use dotenv::dotenv;

static POOL: OnceCell<Pool<MySql>> = OnceCell::new();

async fn test_query() -> Result<(), sqlx::Error> {
    println!("Test query ....");
    let list = sqlx::query("select id, user_name, user_age from test_user").fetch_all(POOL.get().unwrap()).await?;
    for row in list {
        println!("id : {}, user_name : {}, user_age : {}", row.get::<i32, _>("id"), row.get::<&str, _>("user_name"), row.get::<i32, _>("user_age"));
    }

    Ok(())
}

async fn test_query_one() -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE test_user SET user_name = ? WHERE id = ?")
        .bind("chanchan")
        .bind(1)
        .execute(POOL.get().unwrap())
        .await?;

    println!("Test single query ....");
    let list = sqlx::query("SELECT * from test_user where id = ?")
        .bind(1)
        .fetch_all(POOL.get().unwrap())
        .await?;

    for row in list {
        println!("id : {}, user_name : {}, user_age : {}", row.get::<i32, _>("id"), row.get::<&str, _>("user_name"), row.get::<i32, _>("user_age"));
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let connection_str = env::var("DATABASE_URL")
        .expect("Database url not specified");

    // init global pool first
    POOL.set(MySqlPoolOptions::new().max_connections(10)
                                    .connect(&connection_str).await?).unwrap();

    test_query().await?;
    test_query_one().await?;

    Ok(())
}

// fn main() {
//     dotenv().ok();
//     let connection_str = env::var("DATABASE_URL")
//     // init global pool first
//     POOL.set(MysqlPoolOptions::new().max_connections(10)
//                                     .connect(&connection_str).await?).unwrap();

//     // 如果不加上注解写法
//     let rt = tokio::runtime::Runtime::new().unwrap();
//     let local_set = tokio::task::LocalSet::new();
//     let handler = local_set.spawn(async {
//         test().await.unwrap();
//     });
//     local_set.block_on(&rt, handler).unwrap();
//     Ok(())
// }