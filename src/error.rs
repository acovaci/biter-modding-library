pub type Res<T> = Result<T, BiterError>;

#[derive(thiserror::Error, Debug)]
pub enum BiterError {
    #[error("Could not find symbol by name: {0}")]
    SymbolNotFound(String),

    #[error("Could not inject for symbol: {0} with error: {1}")]
    InjectionFailed(String, String),
}
