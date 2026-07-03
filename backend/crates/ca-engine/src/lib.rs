pub mod authority;
pub mod builder;
pub mod errors;
pub mod types;

pub use builder::CertificateGenerator;
pub use types::{CertificateInput, GeneratedCertificate};
pub use errors::CertificateGenerationError;

// Placeholders for future features
pub mod signer {}
pub mod parser {}
pub mod pem {}
pub mod keys {}
