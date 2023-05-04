use sovereign_sdk::serial::DeserializationError;

pub fn data_to_short(expected: usize, got: usize) -> DeserializationError {
    DeserializationError::DataTooShort { expected, got }
}
