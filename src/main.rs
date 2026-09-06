mod commands;
mod utils;

use std::{env, println};

use crate::utils::get_config_path::get_config_path;

fn main() {
    let args: Vec<String> = env::args().collect();

    // 初期化処理
    utils::init::init();

    // コマンドライン引数の処理
    if args.len() == 1 {
        // 引数がない場合の処理
        println!("引数がありません。");
    } else if args[1] == "init" {
        let config_path = get_config_path();
        std::fs::remove_file(config_path).unwrap();
        utils::init::init();
        println!("config.jsonを初期化しました。");
    } else if args[1] == "set" {
        commands::set_command::set_command();
    } else if args[1] == "del" {
        commands::del_command::del_command(args);
    } else if args[1] == "run" {
        commands::run_command::run_command(args);
    } else {
        println!("不明なコマンドです。");
    }
}
