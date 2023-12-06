
#[derive(Debug, Clone, Copy)]
pub enum LoteriaError{
    DirsError,
}

pub type LoteriaResult<T> = Result<T, LoteriaError>;
