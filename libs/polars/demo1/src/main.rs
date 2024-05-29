use polars::prelude::*;

fn test_series() -> Result<(), PolarsError> {
    let series = Series::new("code", &["001", "002"]);
    println!("{}", series);

    let open = Series::new("open", &[1.0, 2.0]);
    println!("{}", open);

    let df = DataFrame::new(vec![series, open]);
    println!("{:?}", df);
    Ok(())
}

const CSV_PATH: &str = "./test.csv";
const NAMES: &[&'static str] = &["代码","开盘"];

fn test_csv() -> Result<(), PolarsError> {
    let mut df = CsvReadOptions::default()
                                .with_infer_schema_length(None)
                                .with_has_header(true)
                                .try_into_reader_with_file_path(Some(CSV_PATH.into()))?
                                .finish()?;
    df = df.head(Some(10));
    println!("{:?}", df);
    df = df.select(&["ts_code", "open"])?.head(Some(5));
    println!("{:?}", df);
    df.set_column_names(NAMES)?;
    println!("{:?}", df);
    Ok(())
}

fn test_series_cal() -> Result<(), PolarsError> {
    let mut df = CsvReadOptions::default()
                                .with_infer_schema_length(None)
                                .with_has_header(true)
                                .try_into_reader_with_file_path(Some(CSV_PATH.into()))?
                                .finish()?;
    let open = df.column("open")?;
    println!("{:?}", open);

    let open = df.column("open")? + 100;
    println!("{:?}", open);

    let high = df.column("high")?;
    let low = df.column("low")?;
    let ret = high - low;
    println!("{:?}", ret);

    let pre_close = df.column("pre_close")?;
    let amp = (high - low).divide(pre_close)? * 100.0;
    println!("{:?}", amp);
    let amp: Series = amp.f64()?.into_iter().map(|item| {
        format!("{:.2}%", item.unwrap_or(0.0))
    }).collect();
    let amp = Series::new("amp", &amp);
    // df插入amp列
    df.with_column(amp)?;
    println!("{:?}", df);
    Ok(())
}

fn test_group_by() -> anyhow::Result<()>{
    let mut df = CsvReadOptions::default()
    .with_infer_schema_length(None)
    .with_has_header(true)
    .try_into_reader_with_file_path(Some(CSV_PATH.into()))?
    .finish()?;
    println!("{:?}", df);

    let g = df.group_by(["ts_code"])?.count()?;
    println!("{:?}", g);

    let g = df.group_by(["ts_code"])?.select(["open"]).count()?;

    println!("{:?}", g);
    Ok(())
}

fn main() {
    // test_series();
    // test_csv();
    // test_series_cal();
    let _ = test_group_by();
}
