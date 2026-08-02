use super::get_config_path::get_config_path;
use super::get_json::get_json;
use crate::tools::command;
use inquire::{Select, error::InquireError};
use std::fs::File;
use std::io::Write;

pub fn del_command(args: Vec<String>) {
    let mut commands = get_json();
    let command_list: Vec<&command::Command>;
    if args.len() > 2 {
        let category_name = &args[2];
        command_list = commands
            .iter()
            .filter(|c| c.category == *category_name)
            .collect();
    } else {
        let mut category_list: Vec<String> = commands
            .iter()
            .map(|c| c.category.clone())
            .collect::<Vec<String>>();
        category_list.sort();
        category_list.dedup();
        let category_selection: Result<String, InquireError> =
            Select::new("カテゴリを選択してください", category_list).prompt();

        match category_selection {
            Ok(selection) => {
                command_list = commands
                    .iter()
                    .filter(|c| c.category == selection)
                    .collect::<Vec<&command::Command>>();
            }
            Err(err) => {
                println!("エラー: {}", err);
                return;
            }
        }
    }

    let command_selection: Result<&command::Command, InquireError> =
        Select::new("コマンドを選択してください", command_list).prompt();

    match command_selection {
        Ok(selection) => {
            let selected_command = selection.clone();

            commands.retain(|c| c != &selected_command);

            let json = serde_json::to_string_pretty(&commands).unwrap();
            let mut file = File::create(get_config_path()).unwrap();
            file.write_all(json.as_bytes()).unwrap();

            println!("コマンドを削除しました: {}", selected_command);
        }
        Err(err) => println!("エラー: {}", err),
    }
}
