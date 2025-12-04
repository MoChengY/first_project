use std::io;
fn main() {
    println!("请输入您的名字：");

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("读取输入时发生错误");

    println!("您的名字是：{}", name);
}

