use std::io;

fn main() {
    println!("整数を入力してください");
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    let mut x: i32 = s.trim().parse().ok().unwrap();

    let mut a: i32 = 1;
    while x > 0{
        a = a*x;
        x = x-1;
    }

    println!("階乗は{}", a);
}
