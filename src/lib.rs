use core::injection::Injection;

use frida_gum::NativePointer;
use libc::c_void;

pub mod core;
pub mod error;

fn lua_newstate() {
    log::info!("lua_newstate called");
}

#[no_mangle]
pub extern "C" fn rust_constructor() {
    core::logging::init();

    log::info!("Constructor called");

    let injections = vec![Injection::new(
        "lua_newstate",
        NativePointer(lua_newstate as *mut c_void),
        true,
    )];

    core::inject::inject(injections).unwrap();

    log::info!("Injection completed");

    // std::process::exit(0);
}

#[no_mangle]
pub extern "C" fn rust_destructor() {}
