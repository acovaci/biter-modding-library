use frida_gum::interceptor::Interceptor;
use frida_gum::{Gum, Module, NativePointer};

use crate::error::{BiterError, Res};

use super::injection::Injection;

lazy_static::lazy_static! {
    static ref GUM: Gum = unsafe { Gum::obtain() };
}

pub fn inject(injections: Vec<Injection>) -> Res<Vec<Injection>> {
    let mut interceptor = Interceptor::obtain(&GUM);

    injections
        .into_iter()
        .map(|injection| {
            let target = find_symbol_by_name(&injection.symbol_name)?;
            let original = interceptor
                .replace(
                    target,
                    injection.replacement,
                    NativePointer(std::ptr::null_mut()),
                )
                .map_err(|e| {
                    BiterError::InjectionFailed(injection.symbol_name.clone(), e.to_string())
                })?;

            Ok(injection.with_original(original))
        })
        .collect()
}

fn find_symbol_by_name(symbol_name: &str) -> Res<NativePointer> {
    let modules = Module::enumerate_modules()
        .iter()
        .map(|module| module.name.clone())
        .collect::<Vec<_>>();

    let target = modules
        .into_iter()
        .find_map(|module| Module::find_symbol_by_name(&module, symbol_name));

    target.ok_or(BiterError::SymbolNotFound(symbol_name.to_string()))
}
