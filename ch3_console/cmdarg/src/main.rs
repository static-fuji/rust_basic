use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect(); //コマンドラインの引数
    let argc = args.len(); //引数の個数

    if argc !=3 {
        println!("引数の個数を確認してください");
        process::exit(1);
    }

    let x:f32 = args[1].parse().unwrap();
    let y:f32 = args[2].parse().unwrap();

    println!("{} + {} = {}", x, y, x + y);
}
