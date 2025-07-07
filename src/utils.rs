use arboard::Clipboard;
use std::error::Error;
use std::process::Command;

pub fn copy_to_clipboard(text: &str) -> Result<(), Box<dyn Error>> {
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(text.to_string())?;
    println!("`{}` をコピーしました。", text);
    Ok(())
}

pub fn open_config() -> Result<(), Box<dyn Error>> {
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
