// 作为标记类型：
// Kilometers和Miles作为标记类型，帮助我们区分不同的距离单位。

struct Kilometers;
struct Miles;

fn distance_in_km(value: f64, unit: Kilometers) -> f64 {
    value
}

fn distance_in_miles(value: f64, unit: Miles) -> f64 {
    value * 1.60934
}

fn main() {
    let km_distance = distance_in_km(100.0, Kilometers);
    let mile_distance = distance_in_miles(100.0, Miles);
}