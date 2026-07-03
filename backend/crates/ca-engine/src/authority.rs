use rcgen::{Certificate, CertificateParams, DistinguishedName, DnType, IsCa, BasicConstraints, KeyUsagePurpose};
use crate::errors::CertificateGenerationError;

pub const DUMMY_CA_COMMON_NAME: &str = "Arkion Dummy CA";

pub fn dummy_ca() -> Result<Certificate, CertificateGenerationError> {
    let mut params = CertificateParams::new(Vec::new());
    params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    params.distinguished_name = DistinguishedName::new();
    params
        .distinguished_name
        .push(DnType::CommonName, DUMMY_CA_COMMON_NAME);
    params.key_usages.push(KeyUsagePurpose::DigitalSignature);
    params.key_usages.push(KeyUsagePurpose::KeyCertSign);
    params.key_usages.push(KeyUsagePurpose::CrlSign);

    Ok(Certificate::from_params(params)?)
}
