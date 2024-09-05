/*
组合和委托来模拟继承：

使用泛型和trait bounds：
*/

struct Engine;

impl Engine {
    fn start(&self) {
        println!("引擎启动");
    }
}

struct Car {
    engine: Engine,
}

impl Vehicle for Car {
    fn start(&self) {
        self.engine.start();
        println!("汽车启动");
    }
}

fn start_vehicle<T: Vehicle>(vehicle: &T) {
    vehicle.start();
}

fn main() {
    let car = Car;
    let bicycle = Bicycle;
    
    start_vehicle(&car);
    start_vehicle(&bicycle);
}