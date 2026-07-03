import { certificateClient } from '../lib/api/api-client';
import { IssueCertificateRequest, CertificateResponse, IssueCertificateResponse } from '../types';

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
};
    