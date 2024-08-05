/*
error[E0597]: `x` does not live long enough
 --> demo3.rs:5:13
  |
4 |         let x = 5;
  |             - binding `x` declared here
5 |         r = &x;
  |             ^^ borrowed value does not live long enough
6 |     }
  |     - `x` dropped here while still borrowed
7 |     println!("r: {}", r);
  |                       - borrow later used here

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0597`.
*/

fn main() {
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("r: {}", r);
}

