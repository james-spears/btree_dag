#[cfg(feature = "fmt")]
use core::fmt::{Display, Formatter, Result};

mod test;

#[cfg(any(feature = "serde_json", feature = "serde_yaml", feature = "serde_cbor"))]
use try_encoding_from::Error as EncodingError;


#[cfg(feature = "fmt")]
static VERTEX_DOES_NOT_EXIST_ERROR: &str = "dag Error: Vertex does not exist";
#[cfg(feature = "fmt")]
static EDGE_EXISTS_ERROR: &str = "dag Error: Edge exists";

/// Errors which may occur during normal usage of the library.
#[derive(PartialEq, Debug)]
pub enum Error {
    VertexDoesNotExist,
    EdgeExistsError,
    #[cfg(any(feature = "serde_cbor", feature = "serde_json", feature = "serde_yaml"))]
    EncodingError(EncodingError),
}

#[cfg(feature = "fmt")]
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Error::EdgeExistsError => write!(f, "{}", EDGE_EXISTS_ERROR),
            Error::VertexDoesNotExist => write!(f, "{}", VERTEX_DOES_NOT_EXIST_ERROR),
            #[cfg(any(feature = "serde_cbor", feature = "serde_json", feature = "serde_yaml"))]
            Error::EncodingError(err) => write!(f, "{}", err),
        }
    }
}

#[cfg(any(feature = "serde_cbor", feature = "serde_json", feature = "serde_yaml"))]
impl From<EncodingError> for Error {
    fn from(e: EncodingError) -> Error {
        Error::EncodingError(e)
    }
}

#[cfg(feature = "serde_cbor")]
impl From<try_encoding_from::serde_cbor::Error> for Error {
    fn from(e: try_encoding_from::serde_cbor::Error) -> Error {
        Error::EncodingError(EncodingError::from(e))
    }
}

#[cfg(feature = "serde_json")]
impl From<try_encoding_from::serde_json::Error> for Error {
    fn from(e: try_encoding_from::serde_json::Error) -> Error {
        Error::EncodingError(EncodingError::from(e))
    }
}

#[cfg(feature = "serde_yaml")]
impl From<try_encoding_from::serde_yaml::Error> for Error {
    fn from(e: try_encoding_from::serde_yaml::Error) -> Error {
        Error::EncodingError(EncodingError::from(e))
    }
}
