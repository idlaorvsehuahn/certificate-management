use thiserror::Error;

#[derive(Debug, Error)]
pub enum CertificateGenerationError {
    #[error("failed to generate certificate: {0}")]
    Rcgen(#[from] rcgen::Error),

    #[error("certificate validity timestamp is out of range: {0}")]
    TimeRange(#[from] time::error::ComponentRange),
}
