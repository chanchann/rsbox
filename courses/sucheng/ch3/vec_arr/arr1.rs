fn main() {
    let tags = ["java", "php", "js"];
    // let tags:[&str; 3] = ["java", "php", "js"];
    // let tags: [&str; 10] = ["a"; 10];
    println!("{:?}", tags);
    for i in 0..tags.len() {
        println!("{}", tags[i]);
    }
    for item in tags.iter() {
        println!("{}", item);
    }

    let mut tags: [u8; 10] = [0; 10];
    for i in 0..tags.len() {
        tags[i] = i as u8;
    }
    println!("{:?}", tags);

    for (i, item) in tags.iter().enumerate() {
        println!("{} : {}", i, item);
    }
}