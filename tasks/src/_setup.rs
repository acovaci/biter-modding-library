use crate::fs::{
    get_game_backup_path, get_game_path, get_home_dir, get_insert_dylib_c_path,
    get_insert_dylib_dir, get_insert_dylib_path, get_launch_json_path, get_lib_path,
};

#[derive(serde::Serialize)]
struct LaunchJSON {
    version: String,
    configurations: Vec<LaunchConfig>,
}

#[derive(serde::Serialize)]
struct LaunchConfig {
    #[serde(rename = "type")]
    ty: String,

    request: String,
    name: String,
    program: String,
    args: Vec<String>,
    cwd: String,

    #[serde(rename = "sourceLanguages")]
    source_languages: Vec<String>,

    #[serde(rename = "waitFor")]
    wait_for: bool,

    #[serde(rename = "initCommands")]
    init_commands: Vec<String>,

    #[serde(rename = "preLaunchTask")]
    pre_launch_task: String,
}

impl LaunchJSON {
    fn new() -> Self {
        Self {
            version: "0.2.0".to_string(),
            configurations: vec![LaunchConfig::new()],
        }
    }
}

impl LaunchConfig {
    fn new() -> Self {
        Self {
            ty: "lldb".to_string(),
            request: "attach".to_string(),
            name: "Debug 'biter' in Factorio".to_string(),
            program: get_game_path().to_str().unwrap().to_string(),
            args: vec![],
            cwd: "${workspaceFolder}".to_string(),
            source_languages: vec!["c".to_string(), "cpp".to_string(), "rust".to_string()],
            wait_for: true,
            init_commands: vec!["platform shell open steam://rungameid/427520".to_string()],
            pre_launch_task: "build-lib".to_string(),
        }
    }
}

pub fn setup() {
    println!("Setting up library.");

    let game_path = get_game_path();
    let backup_path = get_game_backup_path();
    let lib_path = get_lib_path();
    let insert_dylib_dir = get_insert_dylib_dir();
    let insert_dylib_c_path = get_insert_dylib_c_path();
    let insert_dylib_path = get_insert_dylib_path();
    let home_dir = get_home_dir();
    let launch_json_path = get_launch_json_path();

    let insert_dylib_repo = "git@github.com:tyilo/insert_dylib.git";

    if backup_path.exists() {
        println!(
            "Backup already exists. If you want to setup again, please remove the backup file."
        );
        return;
    }

    std::process::Command::new("cp")
        .args(&[game_path.clone(), backup_path])
        .status()
        .expect("Failed to backup the game.");

    println!("Cloning insert_dylib.");

    std::process::Command::new("git")
        .args(&[
            "clone",
            insert_dylib_repo,
            insert_dylib_dir.to_str().unwrap(),
        ])
        .current_dir(home_dir)
        .status()
        .expect("Failed to clone insert_dylib.");

    println!("Compiling insert_dylib.");

    std::process::Command::new("gcc")
        .args(&[
            "-o",
            insert_dylib_path.to_str().unwrap(),
            insert_dylib_c_path.to_str().unwrap(),
        ])
        .status()
        .expect("Failed to compile insert_dylib.");

    std::process::Command::new(insert_dylib_path)
        .args(&[
            "--inplace",
            lib_path.to_str().unwrap(),
            game_path.to_str().unwrap(),
        ])
        .status()
        .expect("Failed to insert dylib.");

    println!("Creating launch.json.");
    serde_json::to_writer(
        std::fs::File::create(launch_json_path).expect("Failed to create launch.json."),
        &LaunchJSON::new(),
    )
    .unwrap();

    println!("Setup complete.");
}
