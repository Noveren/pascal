## Pascal 解释器 - Rust 实现
> connect to Thinkpad E490 and Macbook Air M2

```
whitespace			::= c.is_whitespace()
ws					::= whitespace

number			   	::= ws* [0-9]+					只支持十进制整数
```





词元定义

```
<Plus>    ::= "+"
<Minus>   ::= "-"
<Number>  ::= [0-9]+
```

语法定义

```
Expr	  ::=  Number
			 | Expr BinOP Expr
BinOp	  ::=  Plus
```

```
expr   ::= term (addop term)*
term   ::= factor (mulop factor)*
factor ::= expr | number
```

+ 体现了优先级和结合性

## 拾遗

### Display Trait

类型可以实现 Display Trait 以自定义 `{}` 标记输出的样式

```rust
use std
```

## 读取用户输入

```rust
let stdin = std::io::stdin();

std::io::stdio::Stdin
pub fn read_line(&self, buf: &mut String) -> io::Result<usize>
```

### Result

| 函数名                                 | 作用                                                     |
| -------------------------------------- | -------------------------------------------------------- |
| is_ok, is_ok_and, is_err, is_err_and   | 判断结果类型                                             |
| map, map_or, map_or_else, map_err      | 对结果进行变换                                           |
| inspect, inspect_err                   | 若ok则执行闭包，但返回原Ok<br />......<br />不稳定的特性 |
| expect, expect_err, unwrap, unwrap_err | 解包或恐慌                                               |
| unwrap_or_default, unwrap_or           | 解包或默认值                                             |
| into_ok, into_err                      | 一定可以解包                                             |
| and, or                                | 与, 或，从左向右取结果                                   |
| and_then                               | 若ok, 则使用ok值执行，返回一个新结果                     |
| or_else                                | 若err，......                                            |

