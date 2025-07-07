use arboard::Clipboard;
use inquire::{Select, Text};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::{self, Command};
use std::{
    env,
    error::Error,
    fs::File,
    io::{Read, Write},
};

#[derive(Serialize, Deserialize)]
struct Category {
    name: String,
    commands: Vec<CommandEntry>,
}

#[derive(Serialize, Deserialize)]
struct CommandEntry {
    name: String,
    run: String,
    description: String,
}

fn main() {
    let config_path = Path::new("config.json");
    if !config_path.exists() {
        if let Err(e) = init_demo() {
            eprintln!("初回起動: デモ設定の作成に失敗しました: {}", e);
            process::exit(1);
        }
        println!("初回起動: デモ設定を作成しました。");
    }

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("使い方: cargo run -- <init|run|cat|find|open|newcat|newcmd|delcat|delcmd>");
        return;
    }

    let cmd = args[1].as_str();
    let result = match cmd {
        "init" => init_demo(),
        "run" => run_select(),
        "cat" => {
            if args.len() < 3 {
                eprintln!("使い方: cargo run -- cat <カテゴリー名>");
                Ok(())
            } else {
                run_select_filtered(&args[2])
            }
        }
        "find" => {
            if args.len() < 3 {
                eprintln!("使い方: cargo run -- find <検索文字列> [--n] [--d] [--c]");
                Ok(())
            } else {
                let query = &args[2];
                let flags = &args[3..];
                find_commands(query, flags)
            }
        }
        "open" => open_config(),
        "newcat" => create_category(),
        "newcmd" => set_command(),
        "delcat" => delete_category(),
        "delcmd" => delete_command(),
        _ => {
            eprintln!("不明なコマンド: {}", cmd);
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("エラー: {}", e);
    }
}

fn load_categories() -> Result<Vec<Category>, Box<dyn Error>> {
    match File::open("config.json") {
        Ok(mut file) => {
            let mut buf = String::new();
            file.read_to_string(&mut buf)?;
            let cats = serde_json::from_str(&buf)?;
            Ok(cats)
        }
        Err(_) => Ok(Vec::new()),
    }
}

fn save_categories(cats: &[Category]) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string_pretty(cats)?;
    let mut file = File::create("config.json")?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

fn init_demo() -> Result<(), Box<dyn Error>> {
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
        ],
    }];
    save_categories(&demo_categories)?;
    println!("デモの初期化が完了しました。config.json を作成しました。");
    Ok(())
}

fn run_select() -> Result<(), Box<dyn Error>> {
    let categories = load_categories()?;
    if categories.is_empty() {
        eprintln!("カテゴリが存在しません。cargo run -- newcat で作成してください。");
        return Ok(());
    }
    let cat_names: Vec<String> = categories.iter().map(|c| c.name.clone()).collect();
    let selected_cat = Select::new("カテゴリを選択してください:", cat_names.clone()).prompt()?;
    run_commands_for(&categories, &selected_cat)
}

fn run_select_filtered(category_name: &str) -> Result<(), Box<dyn Error>> {
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

    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(command.run.clone())?;
    println!("`{}` をコピーしました。", command.run);
    Ok(())
}

/// find コマンドの実装
fn find_commands(query: &str, flags: &[String]) -> Result<(), Box<dyn Error>> {
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

    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(command.run.clone())?;
    println!("`{}` をコピーしました。", command.run);
    Ok(())
}

/// open コマンドの実装 (config.json を既定のエディタで開く)
fn open_config() -> Result<(), Box<dyn Error>> {
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(&["/C", "start", "config.json"])
            .spawn()?;
    }
    #[cfg(target_os = "macos")]
    {
        Command::new("open").arg("config.json").spawn()?;
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        Command::new("xdg-open").arg("config.json").spawn()?;
    }
    println!("config.json を開きました。");
    Ok(())
}

fn create_category() -> Result<(), Box<dyn Error>> {
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

fn set_command() -> Result<(), Box<dyn Error>> {
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

fn delete_category() -> Result<(), Box<dyn Error>> {
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

fn delete_command() -> Result<(), Box<dyn Error>> {
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
