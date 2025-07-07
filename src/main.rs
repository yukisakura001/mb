use mb::*;
use std::env;
use std::path::Path;
use std::process;

fn main() {
    let config_path = Path::new("config.json");
    if !config_path.exists() {
        if let Err(e) = init_demo() {
            eprintln!("初回起動: デモ設定の作成に失敗しました: {}", e);
            process::exit(1);
        }
        println!("初回起動: デモ設定を作成しました。");
    }

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("使い方: cargo run -- <init|run|cat|find|open|newcat|newcmd|delcat|delcmd>");
        return;
    }

    let cmd = args[1].as_str();
    let result = match cmd {
        "init" => init_demo(),
        "run" => run_select(),
        "cat" => {
            if args.len() < 3 {
                eprintln!("使い方: cargo run -- cat <カテゴリー名>");
                Ok(())
            } else {
                run_select_filtered(&args[2])
            }
        }
        "find" => {
            if args.len() < 3 {
                eprintln!("使い方: cargo run -- find <検索文字列> [--n] [--d] [--c]");
                Ok(())
            } else {
                let query = &args[2];
                let flags = &args[3..];
                find_commands(query, flags)
            }
        }
        "open" => open_config(),
        "newcat" => create_category(),
        "newcmd" => set_command(),
        "delcat" => delete_category(),
        "delcmd" => delete_command(),
        _ => {
            eprintln!("不明なコマンド: {}", cmd);
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("エラー: {}", e);
    }
}
