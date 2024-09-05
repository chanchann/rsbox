trait Vehicle {
    fn start(&self) {
        println!("车辆启动");
    }
    
    fn stop(&self) {
        println!("车辆停止");
    }
}

struct Car;
struct Bicycle;

impl Vehicle for Car {
    // 重写start方法
    fn start(&self) {
        println!("汽车发动引擎");
    }
}

impl Vehicle for Bicycle {
    // 使用默认实现
}

fn main() {
    let car = Car;
    let bicycle = Bicycle;

    println!("汽车：");
    car.start();
    car.stop();

    println!("\n自行车：");
    bicycle.start();
    bicycle.stop();

    // 使用trait对象演示多态
    let vehicles: Vec<Box<dyn Vehicle>> = vec![Box::new(Car), Box::new(Bicycle)];
    
    println!("\n所有车辆：");
    for (index, vehicle) in vehicles.iter().enumerate() {
        println!("车辆 {}:", index + 1);
        vehicle.start();
        vehicle.stop();
        println!();
    }
}