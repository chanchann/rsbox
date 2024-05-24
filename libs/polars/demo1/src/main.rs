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
fn main() {
    test_series();
}
