use crate::models::{Category, CommandEntry};
use crate::storage::{load_categories, save_categories};
use crate::utils::copy_to_clipboard;
use inquire::{Select, Text};
use std::error::Error;

pub fn run_select() -> Result<(), Box<dyn Error>> {
    let categories = load_categories()?;
    if categories.is_empty() {
        eprintln!("カテゴリが存在しません。cargo run -- newcat で作成してください。");
        return Ok(());
    }
    let cat_names: Vec<String> = categories.iter().map(|c| c.name.clone()).collect();
    let selected_cat = Select::new("カテゴリを選択してください:", cat_names.clone()).prompt()?;
    run_commands_for(&categories, &selected_cat)
}

pub fn run_select_filtered(category_name: &str) -> Result<(), Box<dyn Error>> {
    let categories = load_categories()?;
    if categories.is_empty() {
        eprintln!("カテゴリが存在しません。cargo run -- newcat で作成してください。");
        return Ok(());
    }
    if !categories.iter().any(|c| c.name == category_name) {
        eprintln!("カテゴリ '{}' が見つかりません。", category_name);
        return Ok(());
    }
    run_commands_for(&categories, category_name)
}

fn run_commands_for(categories: &[Category], category_name: &str) -> Result<(), Box<dyn Error>> {
    let category = categories.iter().find(|c| c.name == category_name).unwrap();

    let choices: Vec<String> = category
        .commands
        .iter()
        .map(|cmd| format!("{} ({})", cmd.name, cmd.description))
        .collect();

    let selected = Select::new("コマンドを選択してください:", choices.clone()).prompt()?;
    let idx = choices.iter().position(|c| c == &selected).unwrap();
    let command = &category.commands[idx];

    copy_to_clipboard(&command.run)?;
    Ok(())
}

pub fn find_commands(query: &str, flags: &[String]) -> Result<(), Box<dyn Error>> {
    let categories = load_categories()?;
    if categories.is_empty() {
        eprintln!("カテゴリが存在しません。cargo run -- newcat で作成してください。");
        return Ok(());
    }

    // フラグ解析
    let search_name = flags.contains(&"--n".into());
    let search_desc = flags.contains(&"--d".into());
    let search_run = flags.contains(&"--c".into());
    // いずれのフラグも指定がなければ、全て検索対象にする
    let (search_name, search_desc, search_run) = if !(search_name || search_desc || search_run) {
        (true, true, true)
    } else {
        (search_name, search_desc, search_run)
    };

    // マッチするコマンドを収集
    let mut matched: Vec<(String, &CommandEntry)> = Vec::new();
    for cat in &categories {
        for cmd in &cat.commands {
            let mut ok = false;
            if search_name && cmd.name.contains(query) {
                ok = true;
            }
            if search_desc && cmd.description.contains(query) {
                ok = true;
            }
            if search_run && cmd.run.contains(query) {
                ok = true;
            }
            if ok {
                matched.push((cat.name.clone(), cmd));
            }
        }
    }

    if matched.is_empty() {
        println!("'{}' にマッチするコマンドはありませんでした。", query);
        return Ok(());
    }

    // 選択肢として表示
    let choices: Vec<String> = matched
        .iter()
        .map(|(cat, cmd)| format!("[{}] {}: {} ({})", cat, cmd.name, cmd.run, cmd.description))
        .collect();
    let selected =
        Select::new("検索結果 - コマンドを選択してください:", choices.clone()).prompt()?;
    let idx = choices.iter().position(|c| c == &selected).unwrap();
    let command = matched[idx].1;

    copy_to_clipboard(&command.run)?;
    Ok(())
}

pub fn create_category() -> Result<(), Box<dyn Error>> {
    let name = Text::new("新しいカテゴリ名を入力してください:").prompt()?;
    let mut categories = load_categories()?;
    if categories.iter().any(|c| c.name == name) {
        eprintln!("カテゴリ '{}' は既に存在します。", name);
        return Ok(());
    }
    println!(
        "カテゴリ '{}' を作成します。最初のコマンドを追加してください。",
        name
    );
    let cmd_name = Text::new("コマンド名:").prompt()?;
    let mut cmd_run = Text::new("実行文字列:").prompt()?;
    if cmd_run.is_empty() {
        cmd_run.push_str(&cmd_name);
    }
    let cmd_desc = Text::new("説明:").prompt()?;

    categories.push(Category {
        name: name.clone(),
        commands: vec![CommandEntry {
            name: cmd_name,
            run: cmd_run,
            description: cmd_desc,
        }],
    });
    save_categories(&categories)?;
    println!("カテゴリ '{}' を作成し、コマンドを登録しました。", name);
    Ok(())
}

pub fn set_command() -> Result<(), Box<dyn Error>> {
    let mut categories = load_categories()?;
    if categories.is_empty() {
        eprintln!("カテゴリが存在しません。cargo run -- newcat で作成してください。");
        return Ok(());
    }
    let names: Vec<String> = categories.iter().map(|c| c.name.clone()).collect();
    let selected_name = Select::new(
        "コマンドを追加するカテゴリを選択してください:",
        names.clone(),
    )
    .prompt()?;
    let idx = categories
        .iter()
        .position(|c| c.name == selected_name)
        .unwrap();

    println!(
        "カテゴリ '{}' に追加するコマンドを入力してください。",
        selected_name
    );
    let cmd_name = Text::new("コマンド名:").prompt()?;
    let mut cmd_run = Text::new("実行文字列:").prompt()?;
    if cmd_run.is_empty() {
        cmd_run.push_str(&cmd_name);
    }
    let cmd_desc = Text::new("説明:").prompt()?;

    categories[idx].commands.push(CommandEntry {
        name: cmd_name.clone(),
        run: cmd_run,
        description: cmd_desc,
    });
    save_categories(&categories)?;
    println!(
        "カテゴリ '{}' にコマンド '{}' を追加しました。",
        selected_name, cmd_name
    );
    Ok(())
}

pub fn delete_category() -> Result<(), Box<dyn Error>> {
    let mut categories = load_categories()?;
    if categories.is_empty() {
        eprintln!("削除可能なカテゴリが存在しません。");
        return Ok(());
    }
    let names: Vec<String> = categories.iter().map(|c| c.name.clone()).collect();
    let selected_name =
        Select::new("削除するカテゴリを選択してください:", names.clone()).prompt()?;
    categories.retain(|c| c.name != selected_name);
    save_categories(&categories)?;
    println!("カテゴリ '{}' を削除しました。", selected_name);
    Ok(())
}

pub fn delete_command() -> Result<(), Box<dyn Error>> {
    let mut categories = load_categories()?;
    if categories.is_empty() {
        eprintln!("カテゴリが存在しません。");
        return Ok(());
    }
    let names: Vec<String> = categories.iter().map(|c| c.name.clone()).collect();
    let selected_cat = Select::new(
        "コマンドを削除するカテゴリを選択してください:",
        names.clone(),
    )
    .prompt()?;
    let idx = categories
        .iter()
        .position(|c| c.name == selected_cat)
        .unwrap();
    if categories[idx].commands.is_empty() {
        eprintln!(
            "カテゴリ '{}' に削除可能なコマンドがありません。",
            selected_cat
        );
        return Ok(());
    }
    let cmd_names: Vec<String> = categories[idx]
        .commands
        .iter()
        .map(|cmd| cmd.name.clone())
        .collect();
    let selected_cmd =
        Select::new("削除するコマンドを選択してください:", cmd_names.clone()).prompt()?;
    categories[idx]
        .commands
        .retain(|cmd| cmd.name != selected_cmd);
    save_categories(&categories)?;
    println!(
        "カテゴリ '{}' からコマンド '{}' を削除しました。",
        selected_cat, selected_cmd
    );
    Ok(())
}
