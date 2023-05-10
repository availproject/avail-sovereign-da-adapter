use sovereign_sdk::serial::DeserializationError;
use std::error::Error;

// NOTE: Errors are defined as DataTooShort since other errors cannot be expressed with given enum.

pub fn deserialization_error(_error: impl Error) -> DeserializationError {
    data_too_short_error()
}

pub fn data_too_short_error() -> DeserializationError {
    DeserializationError::DataTooShort {
        expected: 0,
        got: 0,
    }
}
