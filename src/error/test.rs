#[cfg(test)]
mod unit_tests {
    #[cfg(any(feature = "serde_json", feature = "serde_yaml", feature = "serde_cbor"))]
    use crate::error::{EncodingError, Error};

    #[test]
    #[cfg(all(feature = "serde", feature = "serde_cbor"))]
    fn test_from_cbor_encoding_error() {
        assert_eq!(
            Error::EncodingError(EncodingError::CborError),
            EncodingError::CborError.into()
        );
    }

    #[test]
    #[cfg(all(feature = "serde", feature = "serde_json"))]
    fn test_from_json_encoding_error() {
        assert_eq!(
            Error::EncodingError(EncodingError::JsonError),
            EncodingError::JsonError.into()
        );
    }

    #[test]
    #[cfg(all(feature = "serde", feature = "serde_yaml"))]
    fn test_from_yaml_encoding_error() {
        assert_eq!(
            Error::EncodingError(EncodingError::YamlError),
            EncodingError::YamlError.into()
        );
    }
}
