use std::io;

fn main() {
    println!("名前を入力してください");

    let mut line = String::new();
    io::stdin().read_line(&mut line).ok();
    let name = line.trim().to_string();

    println!("{} さん, こんにちは", name);
}
