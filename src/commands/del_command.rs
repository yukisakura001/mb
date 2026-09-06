use crate::utils::command;
use crate::utils::get_config_path::get_config_path;
use crate::utils::select_command::select_command;
use std::fs::File;
use std::io::Write;

pub fn del_command(args: Vec<String>) {
    let command_selection: Option<command::Command> = select_command(args);
    let mut commands: Vec<command::Command> = crate::utils::get_json::get_json();

    match command_selection {
        Some(selection) => {
            let selected_command = selection.clone();

            commands.retain(|c| c != &selected_command);

            let json = serde_json::to_string_pretty(&commands).unwrap();
            let mut file = File::create(get_config_path()).unwrap();
            file.write_all(json.as_bytes()).unwrap();

            println!("コマンドを削除しました: {}", selected_command);
        }
        None => println!("コマンドが選択されませんでした"),
    }
}
