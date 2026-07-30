use super::command::Command;
use serde_json;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn init() {
    // 初期化処理
    let path = Path::new("config.json");

    if path.exists() {
    } else {
        println!("config.jsonを作成します。");
        let command: Vec<Command> = vec![Command {
            name: "npm run dev".to_string(),
            command: "npm run dev".to_string(),
            category: "npm".to_string(),
        }];
        let json = serde_json::to_string_pretty(&command).unwrap();
        let mut file = File::create("config.json").unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }
}
