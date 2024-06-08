#[macro_export]
macro_rules! echo {
    () => {
        println!("Hello, echo!");
    };  // becareful ; here
    ($exp: expr) => (
        println!("{}", $exp);
    )
}

