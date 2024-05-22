fn exec<F>(f: F)
where
    F: Fn(),
{
    f();
}

struct User<F>
where F:Fn() {
    id: i32,
    info: F
}

struct Student<'a> {
    id: i32,
    info: &'a dyn Fn()  // 默认是static生命周期
}

fn main() {
    let str = String::from("abc");
    let f = || {
        println!("{}", str);
    };
    exec(f);
    exec(f);

    let u = User {id: 101, info: f};
    (u.info)();  // important here

    let info = u.info;
    info();

    let u = User {id: 101, info: &f};
    (u.info)(); 
    let info = u.info;
    info();
}
