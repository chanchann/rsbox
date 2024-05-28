## ref

https://rustcc.cn/article?id=e347d7ca-60d6-4e57-bfb6-2076009692f3

##

once_cell提供了unsync::OnceCell和sync::OnceCell这两种Cell（字面意思，前者用于单线程，后者用于多线程），用来存储堆上的信息，并且具有最多只能赋值一次的特性。 API大概是：

```rust
impl<T> OnceCell<T> {
    fn new() -> OnceCell<T> { ... }
    fn set(&self, value: T) -> Result<(), T> { ... }
    fn get(&self) -> Option<&T> { ... }
}
```

