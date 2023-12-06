use image::error::ImageError;

#[derive(Debug)]
pub enum LoteriaError{
    DirsError,
    ImgError(ImageError),
    DeckNotFoundAtPath(String),
    GenericError(String),
}

impl From<ImageError> for LoteriaError{
    fn from(value: ImageError) -> Self {
        Self::ImgError(value)
    }
}

pub type LoteriaResult<T> = Result<T, LoteriaError>;

