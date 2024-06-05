use sqlx::{mysql::*, Pool};
use sqlx::Row;
use tokio::runtime::Runtime;
use std::env;
use once_cell::sync::OnceCell;
use dotenv::dotenv;
use polars::prelude::*;

static POOL: OnceCell<Pool<MySql>> = OnceCell::new();

// async fn test_query() -> Result<(), sqlx::Error> {
//     println!("Test query ....");
//     let list = sqlx::query("select id, user_name, user_age from test_user").fetch_all(POOL.get().unwrap()).await?;
//     for row in list {
//         println!("id : {}, user_name : {}, user_age : {}", row.get::<i32, _>("id"), row.get::<&str, _>("user_name"), row.get::<i32, _>("user_age"));
//     }

//     Ok(())
// }

const CSV_PATH: &str = "./test.csv";
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // dotenv().ok();
    // let connection_str = env::var("DATABASE_URL")
    //     .expect("Database url not specified");

    // init global pool first
    // POOL.set(MySqlPoolOptions::new().max_connections(10)
    //                                 .connect(&connection_str).await?).unwrap();
    let mut df = CsvReadOptions::default()
                                .with_infer_schema_length(None)
                                .with_has_header(true)
                                .try_into_reader_with_file_path(Some(CSV_PATH.into()))?
                                .finish()?;
    df = df.lazy().group_by([col("ts_code")])
                  .agg([
                    col("trade_date").count().alias("count")
                  ]).collect()?;
    println!("{:?}", df);

    // 拼凑sql 入库
    // insert into xxx values (xx, xx), (xx, xx), (xx, xx);
    // let mut query_builder: QueryBuilder<MySql> = QueryBuilder::new("INSERT INTO users(id, username, email, password) ";
    // query_builder.sql()
    // qeury_builder.build().execute(POOL.get().unwrap()).unwrap();
    Ok(())
}