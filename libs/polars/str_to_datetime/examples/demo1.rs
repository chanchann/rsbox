/*
这个例子演示了如何将str转化为datetime
之前再这报错卡了很久

改 lit(true) => lit("raise") 就好了

这里最重要的就是strptime

https://docs.pola.rs/api/python/stable/reference/expressions/api/polars.Expr.str.strptime.html

https://docs.pola.rs/api/rust/dev/polars/prelude/struct.StrptimeOptions.html

Available on crate feature lazy only.

https://docs.pola.rs/api/python/stable/reference/expressions/api/polars.Expr.str.strptime.html

*/

use polars::prelude::*;

fn print_str_to_date(df: DataFrame) -> PolarsResult<()> {
    let df = df.lazy()
    .with_column(
        col("时间").alias("时间_str")
    )
    .collect()?;
    // println!("步骤1 - 复制时间列后的 schema: {:?}", df.schema());
    // println!("步骤1 - 时间列示例: {:?}", df.column("时间_str")?.head(Some(5)));
    // println!("所有列的数据类型：");
    // for col in df.get_columns() {
    //     println!("{}: {:?}", col.name(), col.dtype());
    // }

    let df = df.lazy()
        .with_column(
            col("时间_str")
                .str()
                .strptime(
                    DataType::Datetime(TimeUnit::Microseconds, None),
                    StrptimeOptions {
                        format: Some("%Y-%m-%d %H:%M".to_string()),
                        strict: true,
                        exact: true,
                        cache: true,
                    },
                    lit("raise")
                )
                .alias("时间_datetime")
        )
        .collect()?;
    println!("步骤2 - 转换为日期时间后的 schema: {:?}", df.schema());
    println!("步骤2 - 时间列示例: {:?}", df.column("时间_datetime")?.head(Some(5)));

    let df = df.lazy()
        .select([
            col("时间_datetime").alias("时间"),
            col("开盘"),
            col("收盘"),
            col("最高"),
            col("最低"),
            col("成交量"),
            col("成交额"),
            col("振幅"),
            col("涨跌幅"),
            col("涨跌额"),
            col("换手率"),
        ])
        .collect()?;
    // println!("步骤3 - 最终 schema: {:?}", df.schema());
    // println!("步骤3 - 时间列示例: {:?}", df.column("时间")?.head(Some(5)));

    Ok(())
}

fn main() -> PolarsResult<()> {
    // 创建示例数据
    let df = df![
        "时间" => ["2023-05-01 09:30", "2023-05-01 10:00", "2023-05-01 10:30", "2023-05-01 11:00", "2023-05-01 11:30"],
        "开盘" => [100.0, 101.0, 102.0, 103.0, 104.0],
        "收盘" => [101.0, 102.0, 103.0, 104.0, 105.0],
        "最高" => [102.0, 103.0, 104.0, 105.0, 106.0],
        "最低" => [99.0, 100.0, 101.0, 102.0, 103.0],
        "成交量" => [1000, 1100, 1200, 1300, 1400],
        "成交额" => [100000.0, 110000.0, 120000.0, 130000.0, 140000.0],
        "振幅" => [0.01, 0.02, 0.03, 0.04, 0.05],
        "涨跌幅" => [0.01, 0.01, 0.01, 0.01, 0.01],
        "涨跌额" => [1.0, 1.0, 1.0, 1.0, 1.0],
        "换手率" => [0.001, 0.002, 0.003, 0.004, 0.005]
    ]?;

    print_str_to_date(df)?;

    Ok(())
}