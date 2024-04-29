use std::path::PathBuf;

// 获取可执行文件位置
pub fn get_exe_dir() -> PathBuf {
    std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

// 获取配置目录位置
pub fn config_dir() -> PathBuf {
    std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("userdata")
        .unwrap()
        .to_path_buf()
}

