use std::{collections::HashMap, fmt::Display};

use crate::{parser, parser::{Node}};

#[allow(unused)]
pub struct VM {
    stack: Vec<f64>,
    ans: f64,
    env: HashMap<String, fn(&mut VM) -> Result<(), String>>,
}

#[allow(unused)]
pub fn new() -> VM {
    let mut vm = VM {
        stack: Vec::new(),
        ans: 0.,
        env: HashMap::new(),
    };
    vm.env.insert("ADD".to_string(), |vm| {
        let right = vm.stack.pop()
            .ok_or("expect right oprand")?;
        let left  = vm.stack.pop()
            .ok_or("expect left oprand")?;
        vm.stack.push(left + right);
        return Ok(());
    });
    vm.env.insert("SUB".to_string(), |vm| {
        let right = vm.stack.pop()
            .ok_or("expect right oprand")?;
        let left  = vm.stack.pop()
            .ok_or("expect left oprand")?;
        vm.stack.push(left - right);
        return Ok(());
    });
    vm.env.insert("MUL".to_string(), |vm| {
        let right = vm.stack.pop()
            .ok_or("expect right oprand")?;
        let left  = vm.stack.pop()
            .ok_or("expect left oprand")?;
        vm.stack.push(left * right);
        return Ok(());
    });
    vm.env.insert("DIV".to_string(), |vm| {
        let right = vm.stack.pop()
            .ok_or("expect right oprand")?;
        let left  = vm.stack.pop()
            .ok_or("expect left oprand")?;
        let result = left / right;
        if result.is_infinite() {
            return Err("the right oprand is 0".to_string());
        } else {
            vm.stack.push(result);
            return Ok(());
        }
    });
    vm
}

impl VM {
    #[allow(unused)]
    fn exec(&mut self, ast: &Node) -> Result<(), String> {
        match ast {
            Node::Number(num) => {
                let num = num.parse::<f64>()
                    .map_err(|_| format!("Failed to parse number: {}", num))?;
                self.stack.push(num);
            },
            Node::Expr(symbol, nodes) => {
                for n in nodes.iter() {
                    self.exec(n)?;
                }
                let func = self.env.get(&format!("{}", symbol))
                    .ok_or(format!("Undefined Symbol: {}", symbol))?;
                func(self)?;
            }
        }
        return Ok(());
    }
    #[allow(unused)]
    pub fn run(&mut self, src: &str) -> Result<(), String> {
        // 解析源码，生成 AST
        let (_, ast) = parser::parse(&src)
            .map_err(|e| format!("Parser: {} at {}", e.1, e.0))?;
        println!("{}", ast.display());
        // 执行 AST
        self.exec(&ast)?;
        print!("{}", self);
        return Ok(());
    }
}

impl Display for VM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ss = String::new();
        for v in self.stack.iter().rev() {
            ss += &format!("== {:^8.0} ==\n", v);
        }
        write!(f, "{}", ss)
    }
}