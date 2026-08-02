use super::command::Command;
use super::get_config_path::get_config_path;
use super::get_json::get_json;
use inquire::Text;
use std::fs::File;
use std::io::Write;

pub fn set_command() {
    let category = match Text::new("カテゴリ名を入力してください").prompt() {
        Ok(input) => input,
        Err(_) => {
            println!("カテゴリ名の入力に失敗しました。");
            return;
        }
    };
    let name = match Text::new("コマンド名を入力してください").prompt() {
        Ok(input) => input,
        Err(_) => {
            println!("コマンド名の入力に失敗しました。");
            return;
        }
    };
    let command = match Text::new("コマンドを入力してください").prompt() {
        Ok(input) => input,
        Err(_) => {
            println!("コマンドの入力に失敗しました。");
            return;
        }
    };
    // 実行処理

    let commands: Vec<Command> = get_json();
    let new_command = Command {
        name,
        command,
        category,
    };
    let updated_commands = commands
        .into_iter()
        .chain(std::iter::once(new_command))
        .collect::<Vec<Command>>();
    let json = serde_json::to_string_pretty(&updated_commands).unwrap();
    let mut file = File::create(get_config_path()).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    println!("コマンドを登録しました。");
}
