## 三种闭包类型

Fn(&self)  引用(&T)  不改变和释放变量，可运行多次

FnMut(&mut self) 可变引用 (&mut T)

FnOnce(self) 值(T)