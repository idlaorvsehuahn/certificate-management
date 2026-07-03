import React from 'react';
import Link from 'next/link';
import { X509Certificate } from 'crypto';
import { certificateApi } from '../../../services/certificate-api';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '../../../components/ui/card';
import { Badge } from '../../../components/ui/badge';
import { Button } from '../../../components/ui/button';
import { ArrowLeft, Shield, Calendar, Key, AlertTriangle, Cpu } from 'lucide-react';

export const dynamic = 'force-dynamic';

interface PageProps {
  params: Promise<{
    id: string;
  }>;
}

export default async function CertificateDetailPage({ params }: PageProps) {
  const { id } = await params;
  let cert = null;
  let fetchError = '';

  try {
    cert = await certificateApi.getCertificate(id);
  } catch (error: any) {
    console.error('Failed to pre-fetch certificate details:', error);
    fetchError = error.message || 'Identity not found or network connection failed.';
  }

  const formatDate = (isoString: string) => {
    return new Date(isoString).toLocaleString(undefined, {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    });
  };

  // Parse details using Node.js native X509Certificate class
  let sha256Fingerprint = 'N/A';
  let publicKeyAlgo = 'ECDSA (Prime256v1)';
  let signatureAlgo = 'ECDSA-SHA256';

  if (cert && cert.pem) {
    try {
      const x509 = new X509Certificate(cert.pem);
      sha256Fingerprint = x509.fingerprint256 || 'N/A';
      
      const keyType = x509.publicKey?.asymmetricKeyType?.toUpperCase() || 'ECDSA';
      publicKeyAlgo = keyType === 'EC' ? 'ECDSA (Prime256v1)' : `${keyType}`;
    } catch (e) {
      console.error('Failed to parse X.509 certificate:', e);
    }
  }

  return (
    <div className="flex flex-col gap-6 w-full max-w-4xl mx-auto">
      {/* Back navigation */}
      <div>
        <Link href="/inventory">
          <Button variant="secondary" size="sm" className="gap-2">
            <ArrowLeft className="w-4 h-4" />
            <span>Back to Inventory</span>
          </Button>
        </Link>
      </div>

      {fetchError || !cert ? (
        <Card className="border-alert/50">
          <CardHeader>
            <div className="flex items-center gap-3 text-alert">
              <AlertTriangle className="w-5 h-5" />
              <CardTitle>Syncing Error</CardTitle>
            </div>
            <CardDescription className="text-alert/70 mt-2">
              {fetchError || 'The requested identity details could not be retrieved.'}
            </CardDescription>
          </CardHeader>
        </Card>
      ) : (
        <div className="flex flex-col gap-6">
          {/* Header Card */}
          <Card>
            <CardContent className="flex flex-col md:flex-row md:items-center justify-between gap-6">
              <div className="flex flex-col gap-2">
                <span className="text-[10px] uppercase tracking-widest text-cream/45 font-mono">
                  Identity ID: {cert.id}
                </span>
                <h1 className="text-2xl md:text-3xl font-light text-cream tracking-wide">
                  {cert.subject}
                </h1>
                <p className="text-xs text-cream/60">
                  Issued by: <span className="font-mono text-gold">{cert.issuer}</span>
                </p>
              </div>
              <div className="flex flex-col items-start md:items-end gap-2 shrink-0">
                <Badge status={cert.status} />
                <span className="text-[10px] text-cream/40 uppercase font-mono">
                  Serial: {cert.serial_number}
                </span>
              </div>
            </CardContent>
          </Card>

          {/* Details Grid */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            {/* Validity Timeline */}
            <Card>
              <CardHeader>
                <div className="flex items-center gap-2 text-gold">
                  <Calendar className="w-4 h-4" />
                  <CardTitle className="text-base">Trust Validity</CardTitle>
                </div>
              </CardHeader>
              <CardContent className="flex flex-col gap-4 text-sm">
                <div className="flex justify-between border-b border-gold-12 pb-2">
                  <span className="text-cream/50">Issued At</span>
                  <span className="font-mono text-xs">{formatDate(cert.issued_at)}</span>
                </div>
                <div className="flex justify-between pb-1">
                  <span className="text-cream/50">Expiration</span>
                  <span className="font-mono text-xs text-gold">{formatDate(cert.expiration)}</span>
                </div>
              </CardContent>
            </Card>

            {/* Cryptographic Details Card */}
            <Card>
              <CardHeader>
                <div className="flex items-center gap-2 text-gold">
                  <Cpu className="w-4 h-4" />
                  <CardTitle className="text-base">Signature & Key Properties</CardTitle>
                </div>
              </CardHeader>
              <CardContent className="flex flex-col gap-4 text-sm">
                <div className="flex justify-between border-b border-gold-12 pb-2">
                  <span className="text-cream/50">Signature Algorithm</span>
                  <span className="font-mono text-xs text-cream">{signatureAlgo}</span>
                </div>
                <div className="flex justify-between border-b border-gold-12 pb-2">
                  <span className="text-cream/50">Public Key Algorithm</span>
                  <span className="font-mono text-xs text-cream">{publicKeyAlgo}</span>
                </div>
              </CardContent>
            </Card>

            {/* Fingerprint Card */}
            <Card className="md:col-span-2">
              <CardHeader>
                <div className="flex items-center gap-2 text-gold">
                  <Shield className="w-4 h-4" />
                  <CardTitle className="text-base">SHA-256 Fingerprint</CardTitle>
                </div>
              </CardHeader>
              <CardContent>
                <span className="font-mono text-xs break-all bg-void border border-gold-12 px-3 py-2.5 rounded text-gold block">
                  {sha256Fingerprint}
                </span>
              </CardContent>
            </Card>

            {/* SAN Entries */}
            <Card className="md:col-span-2">
              <CardHeader>
                <div className="flex items-center gap-2 text-gold">
                  <Shield className="w-4 h-4" />
                  <CardTitle className="text-base">Subject Alternative Names (SANs)</CardTitle>
                </div>
              </CardHeader>
              <CardContent>
                {cert.san_dns_names && cert.san_dns_names.length > 0 ? (
                  <div className="flex flex-wrap gap-2">
                    {cert.san_dns_names.map((san: string, idx: number) => (
                      <span
                        key={idx}
                        className="bg-gold-12 text-gold border border-gold-20/30 px-2 py-1 rounded text-xs font-mono"
                      >
                        DNS:{san}
                      </span>
                    ))}
                  </div>
                ) : (
                  <span className="text-xs text-cream/40">No SAN entries declared.</span>
                )}
              </CardContent>
            </Card>
          </div>

          {/* Collapsible PEM Block (Accordion) */}
          {cert.pem && (
            <details className="group border border-gold-12 rounded-lg bg-void/50 mt-4 overflow-hidden transition-all">
              <summary className="flex items-center justify-between px-5 py-4 cursor-pointer text-sm font-semibold text-gold select-none hover:bg-gold-12/10 transition-colors">
                <div className="flex items-center gap-2">
                  <Key className="w-4 h-4" />
                  <span>Inspect Raw PEM Certificate</span>
                </div>
                <span className="text-xs text-cream/40 group-open:rotate-180 transform transition-transform duration-200">
                  ▼
                </span>
              </summary>
              <div className="px-5 pb-5 pt-2 border-t border-gold-12 bg-void/30">
                <pre className="p-4 bg-void border border-gold-12 rounded text-[10px] font-mono overflow-x-auto text-cream/70 select-all whitespace-pre leading-relaxed">
                  {cert.pem}
                </pre>
              </div>
            </details>
          )}
        </div>
      )}
    </div>
  );
}
