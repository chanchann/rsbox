// 'static bound
// 当你在Tokio运行时中催生一个任务时，它的类型必须是 'static 的。
// 这意味着生成的任务不能包含对任务之外拥有的数据的任何引用。
// 本例无法正常编译
/*
error[E0373]: async block may outlive the current function, but it borrows `v`, which is owned by the current function
  --> examples/demo6.rs:12:17
   |
12 |       task::spawn(async {
   |  _________________^
13 | |         println!("Here's a vec: {:?}", v);
   | |                                        - `v` is borrowed here
14 | |     });
   | |_____^ may outlive borrowed value `v`
   |
   = note: async blocks are not executed immediately and must either take a reference or ownership of outside variables they use
help: to force the async block to take ownership of `v` (and any other referenced variables), use the `move` keyword
   |
12 |     task::spawn(async move {
   |                       ++++

For more information about this error, try `rustc --explain E0373`.
*/

use tokio::task;

#[tokio::main]
async fn main() {
    let v = vec![1, 2, 3];

    task::spawn(async {
        println!("Here's a vec: {:?}", v);
    });
}

