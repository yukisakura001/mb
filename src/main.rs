mod tools;

use std::{env, println};

fn main() {
    let args: Vec<String> = env::args().collect();

    // 初期化処理
    tools::init::init();

    // コマンドライン引数の処理
    if args.len() == 1 {
        // 引数がない場合の処理
        println!("引数がありません。");
    } else if args[1] == "init" {
        std::fs::remove_file("config.json").unwrap();
        tools::init::init();
        println!("config.jsonを初期化しました。");
    } else if args[1] == "set" {
        tools::set_command::set_command();
    } else if args[1] == "del" {
        tools::del_command::del_command(args);
    } else if args[1] == "run" {
        tools::run_command::run_command(args);
    } else {
        println!("不明なコマンドです。");
    }
}
