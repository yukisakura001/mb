use crate::models::{Category, CommandEntry};
use crate::utils::get_config_path;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

pub fn load_categories() -> Result<Vec<Category>, Box<dyn Error>> {
    let config_path = get_config_path()?;
    match File::open(&config_path) {
        Ok(mut file) => {
            let mut buf = String::new();
            file.read_to_string(&mut buf)?;
            let cats = serde_json::from_str(&buf)?;
            Ok(cats)
        }
        Err(_) => Ok(Vec::new()),
    }
}

pub fn save_categories(cats: &[Category]) -> Result<(), Box<dyn Error>> {
    let config_path = get_config_path()?;
    let json = serde_json::to_string_pretty(cats)?;
    let mut file = File::create(&config_path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn init_demo() -> Result<(), Box<dyn Error>> {
    let demo_categories = vec![Category {
        name: "mb".into(),
        commands: vec![
            CommandEntry {
                name: "init".into(),
                run: "mb init".into(),
                description: "初期化".into(),
            },
            CommandEntry {
                name: "run".into(),
                run: "mb run".into(),
                description: "カテゴリを選択".into(),
            },
            CommandEntry {
                name: "cat".into(),
                run: "mb cat <カテゴリ>".into(),
                description: "指定カテゴリで実行".into(),
            },
            CommandEntry {
                name: "newcat".into(),
                run: "mb newcat".into(),
                description: "新しいカテゴリを作成".into(),
            },
            CommandEntry {
                name: "newcmd".into(),
                run: "mb newcmd".into(),
                description: "カテゴリにコマンドを追加".into(),
            },
            CommandEntry {
                name: "edit".into(),
                run: "mb edit".into(),
                description: "コマンドを編集".into(),
            },
            CommandEntry {
                name: "delcat".into(),
                run: "mb delcat".into(),
                description: "カテゴリを削除".into(),
            },
            CommandEntry {
                name: "delcmd".into(),
                run: "mb delcmd".into(),
                description: "カテゴリからコマンドを削除".into(),
            },
            CommandEntry {
                name: "find".into(),
                run: "mb find <検索文字列> [--n] [--d] [--c]".into(),
                description: "コマンドを検索[--name] [--description] [--command]".into(),
            },
            CommandEntry {
                name: "open".into(),
                run: "mb open".into(),
                description: "config.json を開く".into(),
            },
            CommandEntry {
                name: "ver".into(),
                run: "mb ver".into(),
                description: "バージョンを確認する".into(),
            },
        ],
    }];
    save_categories(&demo_categories)?;
    println!("デモの初期化が完了しました。config.json を作成しました。");
    Ok(())
}
