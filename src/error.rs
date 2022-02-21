use inkwell::execution_engine::FunctionLookupError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to parse: {message}")]
    Parse { message: String },

    #[error("Failed to create a execution engine: {message}")]
    ExecutionEngine { message: String },

    #[error(transparent)]
    Residual(#[from] FunctionLookupError),
}
