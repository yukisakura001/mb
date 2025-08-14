use arboard::Clipboard;
use std::error::Error;
use std::path::PathBuf;
use std::process::Command;

pub fn copy_to_clipboard(text: &str) -> Result<(), Box<dyn Error>> {
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(text.to_string())?;
    println!("`{}` をコピーしました。", text);
    Ok(())
}

pub fn get_config_path() -> Result<PathBuf, Box<dyn Error>> {
    let exe_path = std::env::current_exe()?;
    let exe_dir = exe_path
        .parent()
        .ok_or("実行ファイルのディレクトリを取得できませんでした")?;
    Ok(exe_dir.join("config.json"))
}

pub fn open_config() -> Result<(), Box<dyn Error>> {
    let config_path = get_config_path()?;
    let config_str = config_path.to_string_lossy();

    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(&["/C", "start", &config_str])
            .spawn()?;
    }
    #[cfg(target_os = "macos")]
    {
        Command::new("open").arg(&config_str).spawn()?;
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        Command::new("xdg-open").arg(&config_str).spawn()?;
    }
    println!("config.json を開きました。");
    Ok(())
}
