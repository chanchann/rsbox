fn main() {
    let mut tags = vec!["11", "223"];
    tags.push("444");
    println!("{:?}", tags);

    let mut tags = Vec::new();
    tags.push("11");
    tags.push("2");
    tags.push("3");
    for i in 0..tags.len() {
        println!("{}", tags[i]);
    }
    let mut tags = Vec::new();
    tags.push("11");
    tags.push("2");
    for i in &tags {
        println!("{}", i);
    }

    let mut tags: Vec<i32> = Vec::new();
    tags.push(1);
    tags.push(5);
    for i in &mut tags {
        *i = *i + 10;
    }
    println!("{:?}", tags);

}