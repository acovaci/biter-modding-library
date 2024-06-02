use crate::fs::{get_game_path, get_lib_path};

pub fn deploy() -> Result<(), ()> {
    println!("Deploying library.");

    let game_path = get_game_path();
    let lib_path = get_lib_path();

    let res = std::process::Command::new("codesign")
        .args(&[
            "-f",
            "-s",
            "-",
            "--timestamp=none",
            "--all-architectures",
            lib_path.to_str().unwrap(),
        ])
        .status()
        .expect("Failed to codesign library.");

    if !res.success() {
        return Err(());
    }

    let res = std::process::Command::new("codesign")
        .args(&[
            "-f",
            "-s",
            "-",
            "--deep",
            "--timestamp=none",
            "--all-architectures",
            game_path.to_str().unwrap(),
        ])
        .status()
        .expect("Failed to codesign game.")
        .success();

    if !res {
        return Err(());
    }

    println!("Deploy complete.");

    Ok(())
}
