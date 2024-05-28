use sqlx::{mysql::*, pool};
use sqlx::Row;
use tokio::runtime::Runtime;

use once_cell::sync::OnceCell;
static POOL: OnceCell<Pool<Mysql>> = OnceCell::new();

async fn test() -> Result<(), sqlx::Error> {
    // let pool = MysqlPoolOptions::new().max_connections(10)
    //                                   .connect("mysql://root:123123@localhost:3306/rustdb").await?;
    let ret = sqlx::query("select id, user_name, user_age from users").fetch_all(POOL.get().unwrap()).await?;
    ret.iter().for_each(|row| {
        row.get<i32, &str>("id");
        row.get::<&str, _>(1);
    });

    let ret = sqlx::query("insert into users (user_name, user_age) values (?,?)").bind("abc").bind(10).execute(POOL.get().unwrap()).await?;
    println!("{:?}", ret.last_insert_id());
    Ok(())
}

fn main() {
    // init global pool first
    POOL.set(MysqlPoolOptions::new().max_connections(10)
                                    .connect("mysql://root:123123@localhost:3306/rustdb").await?).unwrap();

    // 如果不加上注解写法
    let rt = tokio::runtime::Runtime::new().unwrap();
    let local_set = tokio::task::LocalSet::new();
    let handler = local_set.spawn(async {
        test().await.unwrap();
    });
    local_set.block_on(&rt, handler).unwrap();
    Ok(())
}