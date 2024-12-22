use core2::io;

#[derive(Debug)]
pub enum Error {
    SingletonAlreadyTaken,
    SerialWriteFailed(io::Error),
    LockAlreadyAcquiredError,
}

impl core::error::Error for Error {}
impl core2::error::Error for Error {}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::SingletonAlreadyTaken => writeln!(
                f,
                "The singleton you're trying to acquire is already taken."
            ),
            Self::SerialWriteFailed(e) => writeln!(f, "Serial::write(_) has failed. Error: {e}"),
            Self::LockAlreadyAcquiredError => writeln!(f, "Lock already taken by another entity."),
        }
    }
}
