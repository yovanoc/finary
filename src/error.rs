use std::fmt;

#[derive(Debug)]
pub enum FinaryError {
    ClientBuildError,
    ClientHttpError,
    SignInResponseError,
    CodeError,
}

impl std::error::Error for FinaryError {}

impl fmt::Display for FinaryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FinaryError::ClientBuildError => write!(f, "Canno't build the client"),
            FinaryError::ClientHttpError => write!(f, "Canno't post the request"),
            FinaryError::SignInResponseError => write!(f, "Unable to connect to finary servers"),
            FinaryError::CodeError => write!(f, "StatusCode not managed"),
        }
    }
}
