//! Module contains a library error.

use std::{fmt, time::Duration};

use windows::{core as win, Win32::Foundation::WAIT_EVENT};

/// Error is a crate's erorr type.
#[derive(Debug)]
pub enum Error {
    /// Internal windows error.
    Win(win::Error),
    /// A error which is returned in case timeout was reached.
    Timeout(Duration),
    /// wait for process end failed due to misc. reasons.
    WaitFailed(WAIT_EVENT),
    /// Input already closed
    InputClosed,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Win(err) => writeln!(f, "Windows error: {}", err),
            Self::Timeout(limit) => writeln!(f, "A timeout {:?} was reached", limit),
            Self::WaitFailed(event_id) => writeln!(f, "Waiting failed. WAIT_EVENT: {:?}", event_id),
            Self::InputClosed => writeln!(f, "The input is already closed"),
        }
    }
}

impl From<win::Error> for Error {
    fn from(err: win::Error) -> Self {
        Error::Win(err)
    }
}

impl From<Error> for std::io::Error {
    fn from(err: Error) -> Self {
        match err {
            Error::Win(err) => std::io::Error::from(err),
            Error::Timeout(time) => std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                format!("timeout reached ({:?})", time),
            ),
            Error::InputClosed => std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Input to console was already closed"),
            ),
            Error::WaitFailed(wait_event) => std::io::Error::new(
                std::io::ErrorKind::Interrupted,
                format!("Waiting for process failed. WAIT_EVENT: {:?}", wait_event),
            ),
        }
    }
}
