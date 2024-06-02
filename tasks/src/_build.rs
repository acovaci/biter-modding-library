use crate::fs::{get_clib_main, get_lib_path, get_staticlib_path};

pub fn build() -> Result<(), ()> {
    println!("Building library.");

    let c_lib_src = get_clib_main();
    let output_path = get_lib_path();
    let staticlib_path = get_staticlib_path();

    let res = std::process::Command::new("cargo")
        .args(&["build"])
        .status()
        .expect("Failed to build library.")
        .success();

    if !res {
        return Err(());
    }

    let res = std::process::Command::new("gcc")
        .args(&[
            "-dynamiclib",
            "-o",
            output_path.to_str().unwrap(),
            c_lib_src.to_str().unwrap(),
            staticlib_path.to_str().unwrap(),
            "-framework",
            "CoreFoundation",
        ])
        .status()
        .expect("Failed to build wrapper.")
        .success();

    if !res {
        return Err(());
    }

    println!("Build complete.");

    Ok(())
}
