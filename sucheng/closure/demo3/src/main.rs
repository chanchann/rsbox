fn myfn<F: FnMut()>(mut f: F) {
    f();
}

fn main() {
    let mut str = String::from("hello world");
    let mut f = || {
        str.push_str("abc");
    };
    f();
    myfn(f);
    println!("{}", str);


    let mut str = String::from("111");
    let mut f = || {
        let a = &mut str;
        a.push_str("222");
    };

    myfn(f);
    println!("{}", str);
}
