use crate::fs::{get_game_dir, get_game_path};

pub fn restore() {
    println!("Restoring game.");

    let game_dir = get_game_dir();
    let game_path = get_game_path();
    let backup_path = game_dir.join("factorio.bak");

    std::process::Command::new("mv")
        .args(&[backup_path, game_path])
        .status()
        .expect("Failed to restore game.");

    println!("Restore complete.");
}
