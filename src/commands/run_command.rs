use crate::utils::command;

use crate::utils::select_command::select_command;
use arboard::Clipboard;

pub fn run_command(args: Vec<String>) {
    // 実行処理

    let command_selection: Option<command::Command> = select_command(args);

    match command_selection {
        Some(selection) => {
            let mut clipboard = Clipboard::new().unwrap();
            clipboard.set_text(selection.command.clone()).unwrap();
            println!(
                "コマンドをクリップボードにコピーしました: {}",
                selection.command
            );
        }
        None => println!("コマンドが選択されませんでした"),
    }
}
