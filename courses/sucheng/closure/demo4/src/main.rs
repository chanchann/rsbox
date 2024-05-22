fn myfnOnce<F: FnOnce()>(f: F) {
    f();
}

fn main() {
    let str = String::from("hello world");
    let f = || {
        let a = str;  // transfer ownership
        // println!("{}", str);
        println!("{}", a);
    };
    myfnOnce(f);
}
