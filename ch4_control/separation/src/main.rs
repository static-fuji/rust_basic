use std::io;

fn main() {
    println!("整数を入力してください");
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    let x: i32 = s.trim().parse().ok().unwrap();

    match x{
        0 => println!("0です"),
        1..=9 => println!("10未満の数です"),
        10..=i32::MAX => println!("10以上の数です"),
        i32::MIN..=-1 => println!("負の数です"),
    }
}
