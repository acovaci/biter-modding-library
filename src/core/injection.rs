use frida_gum::NativePointer;

#[derive(Clone)]
pub struct Injection {
    pub symbol_name: String,
    pub replacement: NativePointer,
    pub original: Option<NativePointer>,
    pub replace: bool,
}

impl Injection {
    pub fn new(symbol_name: &str, replacement: NativePointer, replace: bool) -> Self {
        Self {
            symbol_name: symbol_name.to_string(),
            replacement,
            original: None,
            replace,
        }
    }

    pub fn with_original(mut self, original: NativePointer) -> Self {
        self.original = Some(original);
        self
    }
}
