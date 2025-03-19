
pub enum CtsUpLoadError {
    ReadError(String),
    WriteError(String),
}

impl std::fmt::Display for CtsUpLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CtsUpLoadError::ReadError(data) => {
                write!(f, "{}", data)
            }
            CtsUpLoadError::WriteError(data) => {
                write!(f, "{}", data)
            }
        }
    }
}