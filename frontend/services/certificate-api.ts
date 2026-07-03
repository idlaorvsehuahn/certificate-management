import { certificateClient } from '../lib/api/api-client';
import { IssueCertificateRequest, CertificateResponse, IssueCertificateResponse, ParsedCertificateResponse } from '../types';

export const certificateApi = {
  issueCertificate: async (
    data: IssueCertificateRequest
  ): Promise<IssueCertificateResponse> => {
    const response = await certificateClient.post<IssueCertificateResponse>(
      '/certificates',
      data
    );
    return response.data;
  },

  getCertificate: async (id: string): Promise<CertificateResponse> => {
    const response = await certificateClient.get<CertificateResponse>(
      `/certificates/${id}`
    );
    return response.data;
  },

  parseCertificate: async (pem: string): Promise<ParsedCertificateResponse> => {
    const response = await certificateClient.post<ParsedCertificateResponse>(
      '/certificates/parse',
      { pem }
    );
    return response.data;
  },

  importCertificate: async (pem: string): Promise<CertificateResponse> => {
    const response = await certificateClient.post<CertificateResponse>(
      '/certificates/import',
      { pem }
    );
    return response.data;
  },
};
    