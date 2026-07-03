import axios from 'axios';
import https from 'https';

const httpsAgent = typeof window === 'undefined'
  ? new https.Agent({ rejectUnauthorized: false })
  : undefined;

export const certificateClient = axios.create({
  baseURL: typeof window === 'undefined'
    ? (process.env.NEXT_PUBLIC_CERTIFICATE_API || 'https://localhost:8080')
    : '/api/certificate',
  httpsAgent,
});

export const inventoryClient = axios.create({
  baseURL: typeof window === 'undefined'
    ? (process.env.NEXT_PUBLIC_INVENTORY_API || 'https://localhost:8081')
    : '/api/inventory',
  httpsAgent,
});
