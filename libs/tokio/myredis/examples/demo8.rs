// Send bound
// 由 tokio::spawn产生的任务必须实现 Send。这允许Tokio运行时在线程之间 move 任务，而这些任务在一个 .await 中被暂停。
// 当 .await 被调用时，任务就回到了调度器中。下一次任务被执行时，它将从最后的让出点恢复。
// 为了使其正常工作，所有在 .await 之后使用的状态都必须由任务保存。如果这个状态是 Send，即可以跨线程移动，那么任务本身也可以跨线程移动。
// 反过来说，如果这个状态不是 Send，那么任务也不是。

use tokio::task::yield_now;
use std::rc::Rc;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        // The scope forces `rc` to drop before `.await`.
        {
            let rc = Rc::new("hello");
            println!("{}", rc);
        }

        // `rc` is no longer used. It is **not** persisted when
        // the task yields to the scheduler
        yield_now().await;
    });
}
