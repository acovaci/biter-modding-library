use std::path::PathBuf;

pub fn get_home_dir() -> PathBuf {
    home::home_dir().expect("Failed to get home directory.")
}

pub fn get_current_dir() -> PathBuf {
    std::env::current_dir().expect("Failed to get current directory.")
}

pub fn get_game_dir() -> PathBuf {
    let home_dir = get_home_dir();

    home_dir.join(
        [
            "Library",
            "Application Support",
            "Steam",
            "steamapps",
            "common",
            "Factorio",
            "factorio.app",
            "Contents",
            "MacOS",
        ]
        .iter()
        .collect::<PathBuf>(),
    )
}

pub fn get_game_backup_path() -> PathBuf {
    get_game_dir().join("factorio.bak")
}

pub fn get_game_path() -> PathBuf {
    get_game_dir().join("factorio")
}

pub fn get_build_path() -> PathBuf {
    get_current_dir().join("build")
}

pub fn get_staticlib_path() -> PathBuf {
    get_current_dir().join(
        ["target", "debug", "libbiter.a"]
            .iter()
            .collect::<PathBuf>(),
    )
}

pub fn get_lib_path() -> PathBuf {
    get_build_path().join("libbiter.dylib")
}

pub fn get_extra_path() -> PathBuf {
    get_current_dir().join("extra")
}

pub fn get_clib_path() -> PathBuf {
    get_extra_path().join("wrapper")
}

pub fn get_clib_main() -> PathBuf {
    get_clib_path().join("main.c")
}

pub fn get_insert_dylib_dir() -> PathBuf {
    get_extra_path().join("insert_dylib")
}

pub fn get_insert_dylib_c_path() -> PathBuf {
    get_insert_dylib_dir().join(["insert_dylib", "main.c"].iter().collect::<PathBuf>())
}

pub fn get_insert_dylib_path() -> PathBuf {
    get_build_path().join("insert_dylib")
}

pub fn get_launch_json_path() -> PathBuf {
    get_current_dir().join([".vscode", "launch.json"].iter().collect::<PathBuf>())
}
