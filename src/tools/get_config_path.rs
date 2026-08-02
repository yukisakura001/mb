pub fn get_config_path() -> std::path::PathBuf {
    let get_config_path = match std::env::current_exe() {
        Ok(path_buf) => {
            let current_folder = path_buf.parent().unwrap();
            let config_path = current_folder.join("config.json");
            config_path
        }
        Err(e) => {
            panic!("Failed to get current exe path: {}", e);
        }
    };
    get_config_path
}
