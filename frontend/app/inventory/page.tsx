import { inventoryApi } from '../../services/inventory-api';
import { InventoryClient } from '../../components/inventory/InventoryClient';
import { InventoryListResponse, DashboardStatsResponse } from '../../types';

export const dynamic = 'force-dynamic';

export default async function InventoryPage() {
  let initialStats: DashboardStatsResponse = {
    total_certificates: 0,
    active_certificates: 0,
    revoked_certificates: 0,
    expiring_soon_certificates: 0,
  };
  
  let initialData: InventoryListResponse = {
    items: [],
    page: 1,
    page_size: 3,
    total_items: 0,
    total_pages: 0,
  };

  try {
    const [stats, data] = await Promise.all([
      inventoryApi.getDashboardStats(),
      inventoryApi.listCertificates({ page: 1, page_size: 3 }),
    ]);
    initialStats = stats;
    initialData = data;
  } catch (error: any) {
    console.error('SSR Pre-fetch connection warning:', error.message || error);
  }

  return <InventoryClient initialStats={initialStats} initialData={initialData} />;
}
