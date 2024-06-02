use crate::{_build::build, _deploy::deploy, fs::get_game_path};

pub fn run() {
    let game_path = get_game_path();

    build().expect("Failed to build library.");
    deploy().expect("Failed to deploy library.");

    println!("Running library.");

    std::process::Command::new(game_path)
        .status()
        .expect("Failed to run game.");

    println!("Run complete.");
}
