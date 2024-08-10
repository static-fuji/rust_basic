use std::io;

fn main() {
    println!("1つ目の数値を入力してください");
    let mut x = String::new();
    io::stdin().read_line(&mut x).ok();
    let x: i32 = x.trim().parse().ok().unwrap();

    println!("2つ目の数値を入力してください");
    let mut y = String::new();
    io::stdin().read_line(&mut y).ok();
    let y: i32 = y.trim().parse().ok().unwrap();

    println!("{} + {} = {}", x, y, x + y);
}
