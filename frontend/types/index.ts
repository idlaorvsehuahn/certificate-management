export type CertificateStatus = 'ACTIVE' | 'REVOKED' | 'EXPIRED';

export interface InventorySummaryResponse {
  id: string;
  subject: string;
  issuer: string;
  status: CertificateStatus;
  expires_at: string;
  created_at: string;
}

export interface InventoryListQuery {
  subject?: string;
  issuer?: string;
  status?: CertificateStatus;
  expiring_days?: number;
  page?: number;
  page_size?: number;
}

export interface InventoryListResponse {
  items: InventorySummaryResponse[];
  page: number;
  page_size: number;
  total_items: number;
  total_pages: number;
}

export interface DashboardStatsResponse {
  total_certificates: number;
  active_certificates: number;
  revoked_certificates: number;
  expiring_soon_certificates: number;
}

export interface IssueCertificateRequest {
  subject: string;
  validity_days: number;
  san_dns_names: string[];
}

export interface CertificateResponse {
  id: string;
  subject: string;
  issuer: string;
  serial_number: string;
  status: CertificateStatus;
  issued_at: string;
  expiration: string;
  san_dns_names: string[];
  pem: string;
  parsed?: ParsedCertificateResponse;
}

export interface IssueCertificateResponse {
  certificate: CertificateResponse;
  certificate_pem: string;
  private_key_pem: string;
  warning: string;
}

export interface ParsedCertificateResponse {
  subject: string;
  issuer: string;
  version: number;
  serial_number: string;
  signature_algorithm: string;
  not_before: string;
  not_after: string;
  days_remaining: number;
  expiration_status: string;
  public_key_algorithm: string;
  key_size: number;
  sha1_fingerprint: string;
  sha256_fingerprint: string;
  is_ca: boolean;
  path_len_constraint?: number;
  key_usages: string[];
  extended_key_usages: string[];
  san_dns_names: string[];
  san_ips: string[];
  san_emails: string[];
  pem: string;
}
