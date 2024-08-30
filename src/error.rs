#[derive(Debug)]
pub enum Error {
    SingletonAlreadyTaken,
    SerialWriteFailed,
}

impl core::error::Error for Error {}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::SingletonAlreadyTaken => write!(
                f,
                "The singleton you're trying to acquire is already taken."
            ),
            Self::SerialWriteFailed => write!(f, "Serial::write(_) has failed (reason unknown)"),
        }
    }
}
