// There are 3 ways to create iterator
// 1. iter() -> &T  || only watch
// 2. iter_mut() -> &mut T   || modify
// 3. into_iter() -> T (value), it can transfer ownership 

fn test_iter() {
    let list = [1, 2, 3];
    list.iter().for_each(|item| {
        println!("{}", item);
    });
    println!("{:?}", list);
}

fn test_iter_mut() {
    let mut list = [String::from("u1"), String::from("u2")];
    list.iter_mut().for_each(|item| {
        item.push_str("--user");
    });
    println!("{:?}", list);
}

fn test_into_iter() {
    let list = [String::from("u1"), String::from("u2")];
    let new_list = list.into_iter().map(|mut item| {
        item.push_str("--user");
    });
    // Map { iter: IntoIter(["u1", "u2"]) }
    println!("{:?}", new_list);
}

fn test_into_iter2() {
    let list = [String::from("u1"), String::from("u2")];
    // Hafve to know the type
    let new_list: Vec<String> = list.into_iter().map(|mut item| {
        item.push_str("--user");
        item
    }).collect();
    println!("{:?}", new_list);
}

fn test_into_iter3() {
    let list = [String::from("u1"), String::from("u2")];
    let new_list = list.into_iter().map(|mut item| {
        item.push_str("--user");
        item
    }).collect::<Vec<String>>();
    println!("{:?}", new_list);
}

fn main() {
    test_iter();
    test_iter_mut();
    test_into_iter();
    test_into_iter2();
    test_into_iter3();
}