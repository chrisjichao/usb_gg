use thiserror::Error;


#[derive(Debug, Error)]
pub enum UGError {
    #[error("Failed to mount configfs")]
    ConfigfsMountFailed,
    #[error("Configfs is not supported")]
    ConfigfsNotSupported,
    #[error("UDC not found")]
    UDCNotFound,
}
