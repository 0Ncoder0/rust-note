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
