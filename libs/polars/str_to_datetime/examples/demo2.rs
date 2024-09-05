/*
Expr.str.strptime(
    dtype: PolarsTemporalType,
    format: str | None = None,
    *,
    strict: bool = True,
    exact: bool = True,
    cache: bool = True,
    ambiguous: Ambiguous | Expr = 'raise',
) → Expr

- Format to use for conversion. Refer to the chrono crate documentation for the full specification. Example: "%Y-%m-%d %H:%M:%S". 
If set to None (default), the format is inferred from the data.


ambiguous
Determine how to deal with ambiguous datetimes:

'raise' (default): raise

'earliest': use the earliest datetime

'latest': use the latest datetime

'null': set to null

*/
use polars::prelude::*;

fn print_str_to_date(df: DataFrame) -> PolarsResult<()> {
    let df = df.lazy()
        .with_column(
            col("时间")
                .str()
                .strptime(
                    DataType::Datetime(TimeUnit::Microseconds, None),
                    StrptimeOptions {
                        format: None,  // If set to None (default), the format is inferred from the data.
                        strict: true,  // Raise an error if any conversion fails.
                        exact: true,
                        cache: true,
                    },
                    lit("raise") // ambiguous: Ambiguous | Expr = 'raise',
                )
                .alias("时间_datetime")
        )
        .collect()?;
    println!("步骤2 - 转换为日期时间后的 schema: {:?}", df.schema());
    println!("步骤2 - 时间列示例: {:?}", df.column("时间_datetime")?.head(Some(5)));

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