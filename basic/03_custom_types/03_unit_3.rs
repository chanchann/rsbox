// 作为状态机的状态：

struct On;
struct Off;

struct LightBulb<State> {
    state: std::marker::PhantomData<State>,
}

impl LightBulb<Off> {
    fn turn_on(self) -> LightBulb<On> {
        println!("Turning light on!");
        LightBulb { state: std::marker::PhantomData }
    }
}

impl LightBulb<On> {
    fn turn_off(self) -> LightBulb<Off> {
        println!("Turning light off!");
        LightBulb { state: std::marker::PhantomData }
    }
}

fn main() {
    let light: LightBulb<Off> = LightBulb { state: std::marker::PhantomData };
    let light = light.turn_on();
    let light = light.turn_off();
}