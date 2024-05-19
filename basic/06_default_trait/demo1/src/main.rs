#[derive(Debug)]
struct TraitTest {
    enable: bool,
    type_id: i32,
}

impl Default for TraitTest {
    fn default() -> Self {
        TraitTest {
            enable: true,
            type_id: 1,
        }
    }
}

#[derive(Default, Debug)]
struct TraitTest2 {
    a: i32,
    b: bool,
}

fn main() {
    let td = TraitTest::default();

    println!("{:?}", td);

    let td2 = TraitTest2::default();
    println!("{:?}", td2);

    let td3 = TraitTest {
        enable: false,
        ..TraitTest::default()
    };

    println!("{:?}", td3);
}