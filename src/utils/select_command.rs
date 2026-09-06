use crate::utils::command;

use crate::utils::get_json::get_json;
use inquire::{Select, error::InquireError};

pub fn select_command(args: Vec<String>) -> Option<command::Command> {
    let commands = get_json();
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
                return None;
            }
        }
    }

    let command_selection: Result<&command::Command, InquireError> =
        Select::new("コマンドを選択してください", command_list).prompt();

    match command_selection {
        Ok(selection) => Some(selection.clone()),
        Err(err) => {
            println!("エラー: {}", err);
            return None;
        }
    }
}
