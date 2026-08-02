use super::command::Command;
use super::get_config_path::get_config_path;
use serde_json;
use std::fs::File;
use std::io::Write;

pub fn init() {
    // 初期化処理
    let get_config_path = get_config_path();

    if get_config_path.exists() {
    } else {
        println!("config.jsonを作成します。");
        let command: Vec<Command> = vec![Command {
            name: "npm run dev".to_string(),
            command: "npm run dev".to_string(),
            category: "npm".to_string(),
        }];
        let json = serde_json::to_string_pretty(&command).unwrap();
        let mut file = File::create(&get_config_path).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }
}
