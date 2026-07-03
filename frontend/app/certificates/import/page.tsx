'use client';
import React, { useState } from 'react';
import { useRouter } from 'next/navigation';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { certificateApi } from '../../../services/certificate-api';
import { Button } from '../../../components/ui/button';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '../../../components/ui/card';
import { formatUtcDate } from '../../../lib/utils';
import { 
  Shield, Upload, FileText, CheckCircle, AlertTriangle, 
  ArrowLeft, Clock, Key, Globe, Eye, ChevronRight
} from 'lucide-react';
import Link from 'next/link';
import { ParsedCertificateResponse } from '../../../types';

export default function ImportCertificatePage() {
  const router = useRouter();
  const queryClient = useQueryClient();
  const [activeTab, setActiveTab] = useState<'upload' | 'paste'>('upload');
  const [rawText, setRawText] = useState('');
  const [dragActive, setDragActive] = useState(false);
  const [previewData, setPreviewData] = useState<ParsedCertificateResponse | null>(null);
  const [fileName, setFileName] = useState('');

  // 1. Mutation for parsing the certificate
  const parseMutation = useMutation({
    mutationFn: (pem: string) => certificateApi.parseCertificate(pem),
    onSuccess: (data) => {
      setPreviewData(data);
    },
  });

  // 2. Mutation for importing/saving the certificate
  const importMutation = useMutation({
    mutationFn: (pem: string) => certificateApi.importCertificate(pem),
    onSuccess: () => {
      // Invalidate dashboard stats and inventory lists so they fetch fresh data
      queryClient.invalidateQueries({ queryKey: ['dashboard-stats'] });
      queryClient.invalidateQueries({ queryKey: ['certificates'] });
      router.push('/inventory');
    },
  });

  const handleTextSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (rawText.trim()) {
      parseMutation.mutate(rawText);
    }
  };

  const processFile = (file: File) => {
    setFileName(file.name);
    const reader = new FileReader();
    
    // For DER (binary), read as ArrayBuffer, otherwise read as text
    const isDer = file.name.endsWith('.der');
    
    reader.onload = (e) => {
      const result = e.target?.result;
      if (result) {
        if (isDer && result instanceof ArrayBuffer) {
          // Convert array buffer to binary string, then pass it
          const bytes = new Uint8Array(result);
          // Standard x509-parser can digest both DER and PEM.
          // Since our backend accepts bytes or strings, we can convert to base64 or pass it
          // Wait! Our parse API takes `{ pem: String }`.
          // If we read as text, it's fine for PEM. If it's binary, let's convert DER buffer to base64
          // or binary string, or standard PEM string before posting!
          // Actually, let's write a simple helper to encode buffer to DER bytes format:
          // Wait, if it's DER, let's encode to standard base64 and add the PEM header so the backend can parse it directly as PEM!
          const binary = Array.from(bytes).map(b => String.fromCharCode(b)).join('');
          const b64 = btoa(binary);
          let pem = '-----BEGIN CERTIFICATE-----\n';
          for (let i = 0; i < b64.length; i += 64) {
            pem += b64.substring(i, i + 64) + '\n';
          }
          pem += '-----END CERTIFICATE-----';
          parseMutation.mutate(pem);
        } else if (typeof result === 'string') {
          parseMutation.mutate(result);
        }
      }
    };

    if (isDer) {
      reader.readAsArrayBuffer(file);
    } else {
      reader.readAsText(file);
    }
  };

  const handleDrag = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    if (e.type === 'dragenter' || e.type === 'dragover') {
      setDragActive(true);
    } else if (e.type === 'dragleave') {
      setDragActive(false);
    }
  };

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setDragActive(false);
    if (e.dataTransfer.files && e.dataTransfer.files[0]) {
      processFile(e.dataTransfer.files[0]);
    }
  };

  const handleFileInput = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files[0]) {
      processFile(e.target.files[0]);
    }
  };

  const handleImport = () => {
    if (previewData) {
      importMutation.mutate(previewData.pem);
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'Valid':
        return 'border-emerald-500/30 bg-emerald-500/10 text-emerald-400';
      case 'Expiring Soon':
        return 'border-amber-500/30 bg-amber-500/10 text-amber-400';
      case 'Expired':
        return 'border-red-500/30 bg-red-500/10 text-red-400';
      default:
        return 'border-gold-12/30 bg-void text-cream';
    }
  };

  const formatDate = formatUtcDate;

  return (
    <div className="w-full max-w-5xl mx-auto flex flex-col gap-6">
      {/* Back navigation */}
      <div>
        <Link href="/inventory">
          <Button variant="secondary" size="sm" className="gap-2">
            <ArrowLeft className="w-4 h-4" />
            <span>Cancel</span>
          </Button>
        </Link>
      </div>

      {!previewData ? (
        /* 1. UPLOAD OR PASTE WIZARD */
        <Card className="max-w-2xl mx-auto w-full">
          <CardHeader>
            <div className="flex items-center gap-2.5 text-gold">
              <Shield className="w-5 h-5" />
              <CardTitle>Import Machine Credential</CardTitle>
            </div>
            <CardDescription className="mt-1">
              Upload or paste an existing X.509 certificate to register its cryptographic identity in the ledger.
            </CardDescription>
          </CardHeader>
          <CardContent className="flex flex-col gap-5">
            {/* Tabs */}
            <div className="flex border-b border-gold-12">
              <button
                className={`px-4 py-2 text-xs font-semibold uppercase tracking-wider border-b-2 transition-all ${
                  activeTab === 'upload' ? 'border-gold text-gold bg-gold-12/10' : 'border-transparent text-cream/50 hover:text-cream'
                }`}
                onClick={() => { setActiveTab('upload'); parseMutation.reset(); }}
              >
                File Upload
              </button>
              <button
                className={`px-4 py-2 text-xs font-semibold uppercase tracking-wider border-b-2 transition-all ${
                  activeTab === 'paste' ? 'border-gold text-gold bg-gold-12/10' : 'border-transparent text-cream/50 hover:text-cream'
                }`}
                onClick={() => { setActiveTab('paste'); parseMutation.reset(); }}
              >
                Paste PEM Text
              </button>
            </div>

            {activeTab === 'upload' ? (
              /* Drag and Drop Zone */
              <div className="flex flex-col gap-4">
                <div
                  onDragEnter={handleDrag}
                  onDragOver={handleDrag}
                  onDragLeave={handleDrag}
                  onDrop={handleDrop}
                  className={`border-2 border-dashed rounded-lg p-10 flex flex-col items-center justify-center gap-4 transition-all bg-void/50 ${
                    dragActive ? 'border-gold bg-gold-12/20 scale-[1.01]' : 'border-gold-12 hover:border-gold/50'
                  }`}
                >
                  <input
                    type="file"
                    id="file-input"
                    className="hidden"
                    accept=".pem,.crt,.cer,.der"
                    onChange={handleFileInput}
                  />
                  <div className="p-4 rounded-full bg-gold-12/25 border border-gold-20/40 text-gold animate-pulse">
                    <Upload className="w-8 h-8" />
                  </div>
                  <div className="text-center flex flex-col gap-1">
                    <p className="text-sm text-cream font-medium">
                      Drag & drop your certificate file here
                    </p>
                    <p className="text-xs text-cream/40">
                      Supports PEM, CRT, CER, or binary DER files
                    </p>
                  </div>
                  <label
                    htmlFor="file-input"
                    className="px-4 py-2 bg-gold-12/30 hover:bg-gold-12/50 border border-gold-20/50 text-gold text-xs uppercase tracking-widest font-semibold rounded cursor-pointer transition-all"
                  >
                    Select File
                  </label>
                </div>
                {fileName && (
                  <div className="flex items-center gap-2 text-xs text-cream/60 px-1 font-mono">
                    <FileText className="w-3.5 h-3.5 text-gold" />
                    <span>Loaded: {fileName}</span>
                  </div>
                )}
              </div>
            ) : (
              /* Text Area Paste */
              <form onSubmit={handleTextSubmit} className="flex flex-col gap-4">
                <div className="flex flex-col gap-1.5">
                  <label className="text-xs uppercase tracking-wider text-cream/70 font-semibold">
                    Certificate PEM Block
                  </label>
                  <textarea
                    rows={12}
                    className="w-full bg-void border border-gold-12 p-3 font-mono text-[10px] text-cream/80 rounded focus:outline-none focus:border-gold/50 focus:ring-1 focus:ring-gold/20"
                    placeholder="-----BEGIN CERTIFICATE-----\nMIIF3jCCA8agAwIBAgIQCg...\n-----END CERTIFICATE-----"
                    value={rawText}
                    onChange={(e) => setRawText(e.target.value)}
                  />
                </div>
                <Button
                  type="submit"
                  variant="primary"
                  disabled={parseMutation.isPending || !rawText.trim()}
                >
                  {parseMutation.isPending ? 'Reading block...' : 'Parse Certificate'}
                </Button>
              </form>
            )}

            {/* Parsing Status Error */}
            {parseMutation.isPending && activeTab === 'upload' && (
              <div className="h-20 flex items-center justify-center bg-void border border-gold-12/30 rounded animate-pulse">
                <span className="text-xs uppercase tracking-widest text-gold animate-pulse">
                  Extracting Cryptographic Context...
                </span>
              </div>
            )}

            {parseMutation.isError && (
              <div className="bg-red-500/10 border border-red-500/20 p-4 rounded text-red-400 text-xs flex gap-2 items-center leading-relaxed">
                <AlertTriangle className="w-4 h-4 shrink-0 mt-0.5 text-red-500" />
                <span>
                  Failed to parse certificate:{' '}
                  {(parseMutation.error as any).response?.data?.error?.message ||
                    (parseMutation.error as any).response?.data?.message ||
                    (parseMutation.error as any).message ||
                    'Invalid formatting or corrupt binary DER'}
                </span>
              </div>
            )}
          </CardContent>
        </Card>
      ) : (
        /* 2. PREMIUM PREVIEW SCREEN */
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 items-start">
          {/* Metadata Cards */}
          <div className="lg:col-span-2 flex flex-col gap-6">
            <Card className="border-gold">
              <CardHeader className="border-b border-gold-12/40">
                <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3">
                  <div className="flex items-center gap-2.5 text-gold">
                    <Eye className="w-5 h-5 animate-pulse" />
                    <CardTitle>Verify Machine Identity</CardTitle>
                  </div>
                  <span className={`border text-[10px] uppercase font-bold tracking-widest px-2.5 py-1 rounded-full ${getStatusColor(previewData.expiration_status)}`}>
                    {previewData.expiration_status}
                  </span>
                </div>
                <CardDescription className="mt-2">
                  Review extracted credential parameters before saving to ledger database.
                </CardDescription>
              </CardHeader>
              <CardContent className="flex flex-col gap-6 pt-6">
                
                {/* 1. General Info */}
                <div className="flex flex-col gap-4">
                  <h3 className="text-xs uppercase tracking-widest text-gold/80 font-bold border-b border-gold-12/30 pb-1 flex items-center gap-1.5">
                    <Shield className="w-3.5 h-3.5" />
                    <span>General Identity Data</span>
                  </h3>
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div className="flex flex-col gap-1 bg-void/30 p-2.5 rounded border border-gold-12/10">
                      <span className="text-[10px] uppercase tracking-wider text-cream/40">Common Name (Subject)</span>
                      <span className="text-sm font-semibold text-cream break-all">{previewData.subject}</span>
                    </div>
                    <div className="flex flex-col gap-1 bg-void/30 p-2.5 rounded border border-gold-12/10">
                      <span className="text-[10px] uppercase tracking-wider text-cream/40">Issuer Authority</span>
                      <span className="text-sm font-mono text-xs text-cream/80 break-all">{previewData.issuer}</span>
                    </div>
                    <div className="flex flex-col gap-1 bg-void/30 p-2.5 rounded border border-gold-12/10">
                      <span className="text-[10px] uppercase tracking-wider text-cream/40">Serial Number</span>
                      <span className="text-xs font-mono text-gold break-all">{previewData.serial_number}</span>
                    </div>
                    <div className="flex flex-col gap-1 bg-void/30 p-2.5 rounded border border-gold-12/10">
                      <span className="text-[10px] uppercase tracking-wider text-cream/40">Signature Algorithm</span>
                      <span className="text-xs font-mono text-cream/80">{previewData.signature_algorithm}</span>
                    </div>
                    <div className="flex flex-col gap-1 bg-void/30 p-2.5 rounded border border-gold-12/10">
                      <span className="text-[10px] uppercase tracking-wider text-cream/40">X.509 Format Version</span>
                      <span className="text-xs font-semibold text-cream/80">Version {previewData.version}</span>
                    </div>
                  </div>
                </div>

                {/* 2. Validity */}
                <div className="flex flex-col gap-4">
                  <h3 className="text-xs uppercase tracking-widest text-gold/80 font-bold border-b border-gold-12/30 pb-1 flex items-center gap-1.5">
                    <Clock className="w-3.5 h-3.5" />
                    <span>Validity & Lifespan</span>
                  </h3>
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div className="flex flex-col gap-1 bg-void/30 p-2.5 rounded border border-gold-12/10">
                      <span className="text-[10px] uppercase tracking-wider text-cream/40">Valid From (Not Before)</span>
                      <span className="text-xs font-mono text-cream/80">{formatDate(previewData.not_before)}</span>
                    </div>
                    <div className="flex flex-col gap-1 bg-void/30 p-2.5 rounded border border-gold-12/10">
                      <span className="text-[10px] uppercase tracking-wider text-cream/40">Expires On (Not After)</span>
                      <span className="text-xs font-mono text-cream/80">{formatDate(previewData.not_after)}</span>
                    </div>
                    <div className="flex flex-col gap-1 bg-void/30 p-2.5 rounded border border-gold-12/10">
                      <span className="text-[10px] uppercase tracking-wider text-cream/40">Time Remaining</span>
                      <span className="text-xs font-semibold text-cream/80">
                        {previewData.days_remaining > 0 
                          ? `${previewData.days_remaining} Days Remaining` 
                          : 'Expired'
                        }
                      </span>
                    </div>
                  </div>
                </div>

                {/* 3. Cryptographic parameters */}
                <div className="flex flex-col gap-4">
                  <h3 className="text-xs uppercase tracking-widest text-gold/80 font-bold border-b border-gold-12/30 pb-1 flex items-center gap-1.5">
                    <Key className="w-3.5 h-3.5" />
                    <span>Cryptographic Parameters</span>
                  </h3>
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div className="flex flex-col gap-1 bg-void/30 p-2.5 rounded border border-gold-12/10">
                      <span className="text-[10px] uppercase tracking-wider text-cream/40">Public Key Type</span>
                      <span className="text-xs font-mono text-cream/80">{previewData.public_key_algorithm}</span>
                    </div>
                    <div className="flex flex-col gap-1 bg-void/30 p-2.5 rounded border border-gold-12/10">
                      <span className="text-[10px] uppercase tracking-wider text-cream/40">Key Strength / Size</span>
                      <span className="text-xs font-mono text-cream/80">{previewData.key_size} bits</span>
                    </div>
                    <div className="flex flex-col gap-1 bg-void/30 p-2.5 rounded border border-gold-12/10 md:col-span-2">
                      <span className="text-[10px] uppercase tracking-wider text-cream/40">SHA-1 Fingerprint</span>
                      <span className="text-[10px] font-mono text-cream/60 break-all uppercase">{previewData.sha1_fingerprint}</span>
                    </div>
                    <div className="flex flex-col gap-1 bg-void/30 p-2.5 rounded border border-gold-12/10 md:col-span-2">
                      <span className="text-[10px] uppercase tracking-wider text-cream/40">SHA-256 Fingerprint</span>
                      <span className="text-[10px] font-mono text-gold/70 break-all uppercase">{previewData.sha256_fingerprint}</span>
                    </div>
                  </div>
                </div>

                {/* 4. Extensions */}
                <div className="flex flex-col gap-4">
                  <h3 className="text-xs uppercase tracking-widest text-gold/80 font-bold border-b border-gold-12/30 pb-1 flex items-center gap-1.5">
                    <Globe className="w-3.5 h-3.5" />
                    <span>Constraints & Key Usage Extensions</span>
                  </h3>
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div className="flex flex-col gap-1 bg-void/30 p-2.5 rounded border border-gold-12/10">
                      <span className="text-[10px] uppercase tracking-wider text-cream/40">Basic Constraints</span>
                      <span className="text-xs font-semibold text-cream/80">
                        {previewData.is_ca 
                          ? `Certificate Authority (CA) ${previewData.path_len_constraint !== undefined ? `· Max Depth: ${previewData.path_len_constraint}` : ''}` 
                          : 'End Entity (Not a CA)'
                        }
                      </span>
                    </div>
                    {previewData.key_usages.length > 0 && (
                      <div className="flex flex-col gap-1 bg-void/30 p-2.5 rounded border border-gold-12/10">
                        <span className="text-[10px] uppercase tracking-wider text-cream/40">Key Usage Flags</span>
                        <div className="flex flex-wrap gap-1 mt-1">
                          {previewData.key_usages.map((u, i) => (
                            <span key={i} className="text-[10px] bg-gold-12/20 border border-gold-20/20 px-1.5 py-0.5 rounded text-gold">
                              {u}
                            </span>
                          ))}
                        </div>
                      </div>
                    )}
                    {previewData.extended_key_usages.length > 0 && (
                      <div className="flex flex-col gap-1 bg-void/30 p-2.5 rounded border border-gold-12/10 md:col-span-2">
                        <span className="text-[10px] uppercase tracking-wider text-cream/40">Extended Key Usage (EKU)</span>
                        <div className="flex flex-wrap gap-1 mt-1">
                          {previewData.extended_key_usages.map((u, i) => (
                            <span key={i} className="text-[10px] bg-gold-12/20 border border-gold-20/20 px-1.5 py-0.5 rounded text-cream/85">
                              {u}
                            </span>
                          ))}
                        </div>
                      </div>
                    )}
                  </div>
                </div>

                {/* 5. Subject Alternative Names (SANs) */}
                {(previewData.san_dns_names.length > 0 || previewData.san_ips.length > 0 || previewData.san_emails.length > 0) && (
                  <div className="flex flex-col gap-4">
                    <h3 className="text-xs uppercase tracking-widest text-gold/80 font-bold border-b border-gold-12/30 pb-1 flex items-center gap-1.5">
                      <Globe className="w-3.5 h-3.5" />
                      <span>Subject Alternative Names (SANs)</span>
                    </h3>
                    <div className="flex flex-col gap-3 bg-void/30 p-3 rounded border border-gold-12/10 font-mono text-xs">
                      {previewData.san_dns_names.length > 0 && (
                        <div className="flex flex-col gap-1">
                          <span className="text-[9px] uppercase tracking-wider text-cream/40 font-sans font-semibold">DNS Entries</span>
                          <span className="text-cream/90 break-all">{previewData.san_dns_names.join(', ')}</span>
                        </div>
                      )}
                      {previewData.san_ips.length > 0 && (
                        <div className="flex flex-col gap-1 mt-1 border-t border-gold-12/10 pt-2">
                          <span className="text-[9px] uppercase tracking-wider text-cream/40 font-sans font-semibold">IP Entries</span>
                          <span className="text-cream/90">{previewData.san_ips.join(', ')}</span>
                        </div>
                      )}
                      {previewData.san_emails.length > 0 && (
                        <div className="flex flex-col gap-1 mt-1 border-t border-gold-12/10 pt-2">
                          <span className="text-[9px] uppercase tracking-wider text-cream/40 font-sans font-semibold">Email Entries</span>
                          <span className="text-cream/90">{previewData.san_emails.join(', ')}</span>
                        </div>
                      )}
                    </div>
                  </div>
                )}
              </CardContent>
            </Card>
          </div>

          {/* Sidebar Control Panel */}
          <div className="flex flex-col gap-6">
            <Card className="border-gold-12 bg-void/50">
              <CardHeader>
                <CardTitle className="text-sm uppercase tracking-wider text-cream">Confirm Registration</CardTitle>
                <CardDescription className="text-xs leading-relaxed">
                  Verify the parameters. Once committed, the credentials will be permanently indexed into the Ledger.
                </CardDescription>
              </CardHeader>
              <CardContent className="flex flex-col gap-4">
                {importMutation.isError && (
                  <div className="bg-red-500/10 border border-red-500/20 p-3 rounded text-red-400 text-xs flex gap-1.5 items-center leading-relaxed">
                    <AlertTriangle className="w-4 h-4 shrink-0 text-red-500" />
                    <span>
                      Import failed:{' '}
                      {(importMutation.error as any).response?.data?.error?.message ||
                        (importMutation.error as any).response?.data?.message ||
                        (importMutation.error as any).message ||
                        'Internal connection failure'}
                    </span>
                  </div>
                )}
                <div className="flex flex-col gap-2">
                  <Button
                    onClick={handleImport}
                    variant="primary"
                    className="w-full gap-2"
                    disabled={importMutation.isPending}
                  >
                    {importMutation.isPending ? 'Registering...' : 'Confirm & Save'}
                    <ChevronRight className="w-4 h-4" />
                  </Button>
                  <Button
                    onClick={() => { setPreviewData(null); parseMutation.reset(); }}
                    variant="secondary"
                    className="w-full"
                    disabled={importMutation.isPending}
                  >
                    Clear & Start Over
                  </Button>
                </div>
              </CardContent>
            </Card>

            {/* Raw Certificate block */}
            <Card className="border-gold-12 bg-void/50">
              <CardHeader className="py-3">
                <span className="text-xs uppercase tracking-wider text-cream/50">Raw Certificate PEM Block</span>
              </CardHeader>
              <CardContent className="pt-0">
                <pre className="font-mono text-[8px] bg-void border border-gold-12 p-3 rounded text-cream/50 overflow-x-auto h-40 select-all leading-normal">
                  {previewData.pem}
                </pre>
              </CardContent>
            </Card>
          </div>
        </div>
      )}
    </div>
  );
}
