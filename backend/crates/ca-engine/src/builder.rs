use chrono::{DateTime, Utc};
use rcgen::{Certificate, CertificateParams, DistinguishedName, DnType, KeyUsagePurpose, ExtendedKeyUsagePurpose, SerialNumber};
use time::OffsetDateTime;
use crate::errors::CertificateGenerationError;
use crate::types::{CertificateInput, GeneratedCertificate};
use crate::authority::{dummy_ca, DUMMY_CA_COMMON_NAME};

#[derive(Clone, Default)]
pub struct CertificateGenerator;

impl CertificateGenerator {
    pub fn generate(
        &self,
        input: CertificateInput,
    ) -> Result<GeneratedCertificate, CertificateGenerationError> {
        let ca = dummy_ca()?;
        let mut params = CertificateParams::new(input.san_dns_names.clone());
        params.distinguished_name = DistinguishedName::new();
        params
            .distinguished_name
              .push(DnType::CommonName, input.subject);
        params.serial_number = Some(SerialNumber::from(input.serial_number));
        params.not_before = to_offset_datetime(input.not_before)?;
        params.not_after = to_offset_datetime(input.not_after)?;
        params.key_usages.push(KeyUsagePurpose::DigitalSignature);
        params.key_usages.push(KeyUsagePurpose::KeyEncipherment);
        params
            .extended_key_usages
            .push(ExtendedKeyUsagePurpose::ServerAuth);

        let certificate = Certificate::from_params(params)?;
        let pem = certificate.serialize_pem_with_signer(&ca)?;

        let private_key_pem = certificate.serialize_private_key_pem();

        Ok(GeneratedCertificate {
            issuer: DUMMY_CA_COMMON_NAME.to_string(),
            san_dns_names: input.san_dns_names,
            pem,
            private_key_pem,
        })
    }
}

fn to_offset_datetime(value: DateTime<Utc>) -> Result<OffsetDateTime, CertificateGenerationError> {
    Ok(OffsetDateTime::from_unix_timestamp(value.timestamp())?)
}
