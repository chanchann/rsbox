// In Rust, the types of function parameters and return values must be explicitly defined. 
// If there is no return value, it can be omitted, returning a unit.
fn pi() -> f64 {
  3.1415926
  // no ';' here, so return number
}

fn not_pi() {
  3.1415926;
  // it has ';' so return unit
}

fn main() {
  let is_pi = pi();
  let is_unit1 = not_pi();
  let is_unit2 = {
    pi();
  };
  
  println!("is_pi: {:?}, is_unit1: {:?}, is_unit2: {:?}", is_pi, is_unit1, is_unit2);
}
