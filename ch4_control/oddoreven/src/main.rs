use std::io;

fn main() {
    println!("整数を入力してください");
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    let x: i32 = s.trim().parse().ok().unwrap();

    let x = x%2;
    match x{
        0 => println!("偶数です"),
        1 => println!("奇数です"),
        _ => println!("偶数でも奇数でもないです"),
    }
}
