## Arc 和 Mutex 经常一起使用?

1. `Arc`（原子引用计数）:
   - 用于在多个所有者之间共享数据的所有权。
   - 允许多个线程同时读取数据。
   - 不提供数据修改的同步机制。

2. `Mutex`（互斥锁）:
   - 用于保护共享数据，确保在任何时刻只有一个线程可以访问数据。
   - 提供了数据修改的同步机制。
   - 不处理所有权共享问题。

`Arc` 和 `Mutex` 经常一起使用的原因：

1. 线程安全的可变共享：
   当多个线程需要共享可变数据时，`Arc<Mutex<T>>` 是一个常见的模式。
   - `Arc` 允许多个线程拥有数据的所有权。
   - `Mutex` 确保在任何时刻只有一个线程可以修改数据。

2. 所有权和同步的结合：
   `Arc` 解决了所有权共享问题，而 `Mutex` 解决了数据访问同步问题。

3. 避免数据竞争：
   这种组合可以有效防止数据竞争，这是 Rust 的主要设计目标之一。

4. 灵活性：
   允许在需要时进行细粒度的锁定和解锁操作。

在您的特定情况下：

- 如果 `Library` 是线程安全的，并且您不需要在多个线程中修改它，那么只使用 `Arc` 就足够了。
- 如果您需要在多个线程中修改 `Library` 或其中的数据，或者 `Library` 本身不是线程安全的，那么使用 `Arc<Mutex<Library>>` 会更安全。

例如：

```rust
let lib = Arc::new(Mutex::new(unsafe { Library::new(strategy_path)? }));

// 在闭包中使用
let creator_arc = {
    let lib_clone = Arc::clone(&lib);
    Arc::new(move || -> *mut dyn StrategyFactory {
        let lib = lib_clone.lock().unwrap();
        let creator: Symbol<FuncCreateStraFact> = unsafe {
            lib.get(b"create_strategy_fact").unwrap()
        };
        unsafe { creator() }
    })
};
```

这种方式确保了在访问 `Library` 时的线程安全性。但是，如果您确定 `Library` 不会在多线程环境中被修改，或者其操作本身就是线程安全的，那么可以只使用 `Arc` 而不需要 `Mutex`。

选择使用 `Arc` 还是 `Arc<Mutex<T>>` 取决于您的具体使用场景和线程安全需求。

