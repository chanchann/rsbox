## ref

https://blog.csdn.net/KINGEH/article/details/127164521

## 

PyO3 不仅仅提供了rust binding，也提供了创建Python包的开箱即用的脚手架工具maturin，使用maturin我们可以很方便地创建一个基于Rust开发的Python扩展模块。

```
pip3 install maturin
maturin new rust_mod #项目名称默认会是Python的包名
```

doc: https://pyo3.rs/v0.17.2/

