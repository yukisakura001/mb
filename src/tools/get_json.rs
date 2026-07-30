use super::command::Command;
use std::fs;

pub fn get_json() -> Vec<Command> {
    let json_str = match fs::read_to_string("config.json") {
        Ok(content) => content,
        Err(_) => {
            println!("config.jsonが見つかりません。初期化処理を実行してください。");
            return Vec::new();
        }
    };
    let commands: Vec<Command> = match serde_json::from_str(&json_str) {
        Ok(cmds) => cmds,
        Err(_) => {
            println!("config.jsonの形式が正しくありません。");
            return Vec::new();
        }
    };
    return commands;
}
