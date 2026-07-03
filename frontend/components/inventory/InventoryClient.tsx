'use client';

import React, { useState, useEffect } from 'react';
import { useQuery } from '@tanstack/react-query';
import { useRouter } from 'next/navigation';
import { inventoryApi } from '../../services/inventory-api';
import { DashboardStats } from '../dashboard/DashboardStats';
import { Table, TableHeader, TableBody, TableRow, TableHead, TableCell } from '../ui/table';
import { Badge } from '../ui/badge';
import { Input } from '../ui/input';
import { Button } from '../ui/button';
import { Pagination } from '../ui/pagination';
import {
  InventoryListResponse,
  DashboardStatsResponse,
  CertificateStatus,
} from '../../types';
import { RefreshCw } from 'lucide-react';

interface InventoryClientProps {
  initialStats: DashboardStatsResponse;
  initialData: InventoryListResponse;
}

export function InventoryClient({ initialStats, initialData }: InventoryClientProps) {
  const router = useRouter();

  // Filters State
  const [subject, setSubject] = useState('');
  const [status, setStatus] = useState<CertificateStatus | ''>('');
  const [expiringDays, setExpiringDays] = useState<number | ''>('');
  const [page, setPage] = useState(1);
  const limit = 3;

  // Active query parameters (subject is debounced, dropdowns apply immediately)
  const [searchParams, setSearchParams] = useState({
    subject: '',
    status: '' as CertificateStatus | '',
    expiringDays: '' as number | '',
  });

  // Debounce the Subject input query by 400ms
  useEffect(() => {
    const timer = setTimeout(() => {
      setSearchParams((prev) => ({ ...prev, subject }));
      setPage(1);
    }, 400);
    return () => clearTimeout(timer);
  }, [subject]);

  // Handle immediate dropdown filter triggers
  const handleStatusChange = (val: CertificateStatus | '') => {
    setStatus(val);
    setSearchParams((prev) => ({ ...prev, status: val }));
    setPage(1);
  };

  const handleExpiryChange = (val: number | '') => {
    setExpiringDays(val);
    setSearchParams((prev) => ({ ...prev, expiringDays: val }));
    setPage(1);
  };

  // Query Dashboard Stats (Polling disabled as stats change locally on issuance)
  const statsQuery = useQuery({
    queryKey: ['dashboard-stats'],
    queryFn: () => inventoryApi.getDashboardStats(),
    initialData: initialStats,
  });

  // Query Certificates List
  const listQuery = useQuery({
    queryKey: ['certificates', page, searchParams],
    queryFn: () =>
      inventoryApi.listCertificates({
        page,
        page_size: limit,
        subject: searchParams.subject || undefined,
        status: searchParams.status || undefined,
        expiring_days: searchParams.expiringDays || undefined,
      }),
    initialData: page === 1 && !searchParams.subject && !searchParams.status && !searchParams.expiringDays
      ? initialData
      : undefined,
  });

  const handleReset = () => {
    setSubject('');
    setStatus('');
    setExpiringDays('');
    setPage(1);
    setSearchParams({
      subject: '',
      status: '',
      expiringDays: '',
    });
  };

  const formatDate = (isoString: string) => {
    const date = new Date(isoString);
    const yyyy = date.getUTCFullYear();
    const mm = String(date.getUTCMonth() + 1).padStart(2, '0');
    const dd = String(date.getUTCDate()).padStart(2, '0');
    return `${yyyy}-${mm}-${dd}`;
  };

  return (
    <div className="flex flex-col gap-6 w-full">
      {/* Page Title */}
      <div className="flex items-center justify-between border-b border-gold-12 pb-4">
        <div>
          <h1 className="text-3xl font-light tracking-wide text-cream uppercase">
            NHI Ledger
          </h1>
          <p className="text-xs text-cream/45 mt-1">
            Machine and AI Agent Identity Registry
          </p>
        </div>
        <Button
          variant="secondary"
          size="sm"
          onClick={() => {
            statsQuery.refetch();
            listQuery.refetch();
          }}
          className="gap-2"
        >
          <RefreshCw className={`w-3.5 h-3.5 ${(statsQuery.isFetching || listQuery.isFetching) ? 'animate-spin' : ''}`} />
          <span>Refresh</span>
        </Button>
      </div>

      {/* Top statistics overview */}
      <DashboardStats
        stats={statsQuery.data}
        isLoading={statsQuery.isLoading && !statsQuery.data}
      />

      {/* Filter and Search Form */}
      <div
        className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-4 gap-4 items-end bg-void border border-gold-12 p-5 rounded"
      >
        <Input
          label="Subject"
          placeholder="Filter by subject CN..."
          value={subject}
          onChange={(e) => setSubject(e.target.value)}
        />
        
        <div className="flex flex-col gap-1.5 w-full">
          <label className="text-xs uppercase tracking-wider text-cream/70 font-medium">
            Status
          </label>
          <select
            value={status}
            onChange={(e) => handleStatusChange(e.target.value as CertificateStatus | '')}
            className="w-full bg-void border border-gold-12 rounded px-3 py-2 text-sm text-cream focus:outline-none focus:border-gold/50 transition-colors"
          >
            <option value="">All Statuses</option>
            <option value="ACTIVE">Active</option>
            <option value="REVOKED">Revoked</option>
            <option value="EXPIRED">Expired</option>
          </select>
        </div>
        
        <div className="flex flex-col gap-1.5 w-full">
          <label className="text-xs uppercase tracking-wider text-cream/70 font-medium">
            Rotation State
          </label>
          <select
            value={expiringDays}
            onChange={(e) =>
              handleExpiryChange(e.target.value ? Number(e.target.value) : '')
            }
            className="w-full bg-void border border-gold-12 rounded px-3 py-2 text-sm text-cream focus:outline-none focus:border-gold/50 transition-colors"
          >
            <option value="">Any Expiry</option>
            <option value="30">Expiring in 30 Days</option>
          </select>
        </div>
        
        <div className="w-full">
          <Button
            type="button"
            variant="secondary"
            onClick={handleReset}
            className="w-full"
          >
            Reset Filters
          </Button>
        </div>
      </div>

      {/* Main Results Table */}
      {listQuery.isLoading && !listQuery.data ? (
        <div className="h-64 flex items-center justify-center border border-gold-12/30 rounded bg-void/50 animate-pulse">
          <span className="text-xs uppercase tracking-widest text-gold animate-pulse">
            Syncing Ledger Registry...
          </span>
        </div>
      ) : listQuery.data?.items.length === 0 ? (
        <div className="h-64 flex flex-col gap-2 items-center justify-center border border-gold-12 rounded bg-void/50">
          <span className="text-sm text-cream/70 uppercase tracking-wider font-semibold">
            No Records Found
          </span>
          <span className="text-xs text-cream/40">
            No identities match the selected search boundaries.
          </span>
        </div>
      ) : (
        <div className="flex flex-col gap-6">
          <Table>
            <TableHeader>
              <TableRow className="pointer-events-none">
                <TableHead>Common Name (Subject)</TableHead>
                <TableHead>Issuer Authority</TableHead>
                <TableHead>Certificate Status</TableHead>
                <TableHead>Expires On</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {listQuery.data?.items.map((cert) => (
                <TableRow
                  key={cert.id}
                  onClick={() => router.push(`/inventory/${cert.id}`)}
                >
                  <TableCell className="font-semibold text-gold">
                    {cert.subject}
                  </TableCell>
                  <TableCell className="font-mono text-xs text-cream/80">
                    {cert.issuer}
                  </TableCell>
                  <TableCell>
                    <Badge status={cert.status} />
                  </TableCell>
                  <TableCell className="font-mono text-xs text-cream/70" suppressHydrationWarning>
                    {formatDate(cert.expires_at)}
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>

          {listQuery.data && (
            <Pagination
              page={page}
              total={listQuery.data.total_items}
              limit={limit}
              onPageChange={setPage}
            />
          )}
        </div>
      )}
    </div>
  );
}
export default InventoryClient;
