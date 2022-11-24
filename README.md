## Pascal 解释器 - Rust 实现
> connect to Thinkpad E490 and Macbook Air M2
词元定义

```
<Plus>    ::= "+"
<Integer> ::= [0-9]
```

语法定义

```
ExprAdd   ::= <Integer><Plus><Integer>
```





## 拾遗

### Display Trait

类型可以实现 Display Trait 以自定义 `{}` 标记输出的样式

```rust
use std
```

### 读取用户输入

```rust
let stdin = std::io::stdin();

std::io::stdio::Stdin
pub fn read_line(&self, buf: &mut String) -> io::Result<usize>
```

