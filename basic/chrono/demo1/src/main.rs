use chrono::prelude::*;

fn main() {
    let now = Local::now();
    println!("{:?}", now);
    // https://docs.rs/chrono/latest/chrono/format/strftime/index.html#specifiers
    println!("{}", now.format("%Y-%m-%d"));

    let t = NaiveDate::parse_from_str("20200228", "%Y%m%d").unwrap();
    println!("{}", t);
}
