import { inventoryClient } from '../lib/api/api-client';
import {
  InventoryListQuery,
  InventoryListResponse,
  CertificateResponse,
  DashboardStatsResponse,
} from '../types';

export const inventoryApi = {
  listCertificates: async (
    query: InventoryListQuery
  ): Promise<InventoryListResponse> => {
    const response = await inventoryClient.get<InventoryListResponse>(
      '/inventory',
      { params: query }
    );
    return response.data;
  },

  getCertificate: async (id: string): Promise<CertificateResponse> => {
    const response = await inventoryClient.get<CertificateResponse>(
      `/inventory/${id}`
    );
    return response.data;
  },

  getDashboardStats: async (): Promise<DashboardStatsResponse> => {
    const response = await inventoryClient.get<DashboardStatsResponse>(
      '/dashboard'
    );
    return response.data;
  },
};
