use std::io;
fn main() {
    println!("请输入您的名字：");

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("读取输入时发生错误");

    println!("您的名字是：{}", name);
}
// 这是一个简单的 Rust 程序，用于获取用户输入的姓名并打印出来。
