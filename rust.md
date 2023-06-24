# Rust 笔记

总结于《Rust 权威指南》2018 版本

## Rust 入门指南

### 安装指南

- `$ curl https://sh.rustup.rs -sSf | sh` 下载并安装 rustup 工具
- `$ rustc --version` 检测是否安装完成
- `$ rustup update` 更新 Rust 版本
- `$ rustup self uninstall` 卸载 rustup 及 Rust 工具链
- `$ rustup doc` 在浏览器中打开离线文档

ps: 为了正常地编译执行 Rust 程序，你需要一个链接器（linker）

### 第一个 Rust 程序

- `$ mkdir hello_world` 新建文件夹
- `$ cd hello_world` 打开文件夹
- `$ code main.rs` 使用 vscode 创建并打开文件
- `$ rustc main.rs` 编译文件
- `$ ./main` 执行编译后的文件

ps: 记得在 main.rs 中输入以下代码

```rs
fn main(){
    println!("Hello, world!");
}
```

### 使用 Cargo

cargo : Rust 工具链中内置的构建系统及包管理器

- `cargo --version` 检测是否已被安装
- `cargo new hello_cargo` 创建项目
- `cd hello_cargo` 打开项目
- `cargo build` 编译项目
- `./target/debug/hello_cargo` 运行编译后的代码
- `cargo run` 编译并运行项目

其他 cargo 命令

- `cargo check` 检查是否可以通过编译
- `cargo build --release` 在优化模式下构建用于发布的可执行程序，并置于 target/release 目录下

## 编写一个猜数游戏

```rs
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

```

ps: 别忘了安装依赖 rand `cargo add rand@0.3.14`

## 通用编程概念

### 定义不可变变量

```rs
let x = 5;
x = 6; // 报错
```

### 定义可变变量

```rs
let mut x = 5;
x = 6; // 不报错
```

### 定义常量

- 常量必须显式地标注值的类型
- 常量绑定的值必须是编译时可确定的

```rs
const TRUE: i32 = 1;
const FALSE: i32 = 0;
const OTHER: i32 = {
    let a = 0;
    let b = 1;
    a + b
}; // 这也能编译时确定？
```

```rs
fn test() -> i32 { -1 }
const OTHER: i32 = test(); // 报错
```

### 隐藏（shadow）

- Rust 支持重复使用 let 关键字覆盖掉同名变量
- 与 mut 的区别在于，可以覆盖不同的变量类型
- 甚至能用 mut 覆盖

```rs
let x = 5;
let x = 6;
let x = "hello";
let x = x.len();
let mut x = 1;
x = 101;
// 请问，let 默认不可变的意义是什么?
```

### 数据类型

- 大部分情况编译器可自行推导出变量类型
- 部分情况需要显式标注类型

```rs
let x = 5; // 自动推导出 x 的类型为 i32
let x = "42".parse().expect("Not a number!"); // 不可自动推导
let x: i32 = "42".parse().expect("Not a number!"); // 显式标注类型
```

### 标量类型

- Rust 内建四种基础的标量类型：整数、浮点、布尔、字符
- 对应的常用的类型：i32, f64, bool, char

```rs
let x: i32 = 1;
let x: f64 = 1.1;
let x: bool = false;
let x: char = 'x'; // 注：char 类型使用单引号
```

### 复合类型

Rust 内置两种基础的复合类型：元祖（tuple）和数组（array）

```rs
let x: (i32, f64, char) = (1, 1.1, '1'); // 元组的声明，类型标注不是必须的
let (i, f, c) = x; // 元组的解构
let x = x.0; // 元组的访问
```

```rs
let x: [i32; 3] = [1, 2, 3]; // 数组的声明，类型标注不是必须的
let x = [1; 5]; // 等价于 let x = [1, 1, 1, 1, 1];
let x = x[0]; // 数组的访问
```

### 函数

- 函数默认返回空的元组类型
- 函数最后一行不带分号则表示返回值，在最后一行之前若要返回需要显式使用 return

```rs
fn demo_function(name: &str) { // -> ()
    println!("Hello, {}!", name);
}

fn plus_one(a: i32) -> i32 {
    a + 1
}

fn big_one(a: i32, b: i32) -> i32 {
    if a > b {
        return a;
    }

    b
}
```
