use polars::prelude::*;

const CSV_PATH: &str = "./test.csv";

fn test_lazy() -> anyhow::Result<()>{
    let df = CsvReadOptions::default()
                                .with_infer_schema_length(None)
                                .with_has_header(true)
                                .try_into_reader_with_file_path(Some(CSV_PATH.into()))?
                                .finish()?;
    let g = df.lazy().group_by(["ts_code"]).agg([
        col("open").alias("number").count()
    ]).collect()?;

    println!("{:?}", g);
    Ok(())
}

fn test_filter() -> anyhow::Result<()>{
    let mut df = CsvReadOptions::default()
                                .with_infer_schema_length(None)
                                .with_has_header(true)
                                .try_into_reader_with_file_path(Some(CSV_PATH.into()))?
                                .finish()?;
    let df1 = df.clone().lazy().filter(
        col("ts_code").eq(lit("GOOG"))
    ).filter(
        col("open").gt_eq(2010.0)
    ).collect()?;

    println!("{:?}", df1);

    // todo
    // let df2 = df.lazy().filter(
    //     col("ts_code").str().contains("^OOG")
    // ).collect()?;

    // println!("{:?}", df2);

    let print_exr = [
        col("ts_code").alias("code_alias"),
        col("open").alias("open_alias"),
    ];
    let ret = df.lazy().select(
        print_exr
    ).limit(2).collect();
    println!("{:?}", ret);
    Ok(())
}

fn main() {
    // let _ = test_lazy();

    let _ = test_filter();
}
