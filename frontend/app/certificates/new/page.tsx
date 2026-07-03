'use client';

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import { useMutation } from '@tanstack/react-query';
import { certificateApi } from '../../../services/certificate-api';
import { Button } from '../../../components/ui/button';
import { Input } from '../../../components/ui/input';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '../../../components/ui/card';
import { Shield, Key, AlertTriangle, CheckCircle, ArrowLeft, Copy, Download } from 'lucide-react';
import Link from 'next/link';
import { IssueCertificateResponse } from '../../../types';

// Validation Schema
const issueSchema = z.object({
  subject: z
    .string()
    .min(1, 'Common Name (Subject) is required')
    .max(64, 'Common Name cannot exceed 64 characters'),
  validity_days: z
    .number({
      message: 'Validity period is required',
    })
    .refine((val) => !isNaN(val), 'Validity period is required')
    .refine((val) => Number.isInteger(val), 'Validity must be a whole number')
    .refine((val) => val >= 1, 'Validity must be at least 1 day')
    .refine((val) => val <= 825, 'Validity cannot exceed 825 days (approx. 2.2 years)'),
  san_names_raw: z.string().optional(),
});

type IssueFormValues = z.infer<typeof issueSchema>;

export default function IssueCertificatePage() {
  const router = useRouter();
  const [issuedCert, setIssuedCert] = useState<IssueCertificateResponse | null>(null);

  const {
    register,
    handleSubmit,
    setValue,
    formState: { errors },
  } = useForm<IssueFormValues>({
    resolver: zodResolver(issueSchema),
    defaultValues: {
      subject: '',
      validity_days: 365,
      san_names_raw: '',
    },
  });

  const mutation = useMutation({
    mutationFn: (values: IssueFormValues) => {
      // Parse SAN names from comma-separated string to string array
      const sanNames = values.san_names_raw
        ? values.san_names_raw
            .split(',')
            .map((s) => s.trim())
            .filter((s) => s.length > 0)
        : [];

      return certificateApi.issueCertificate({
        subject: values.subject,
        validity_days: values.validity_days,
        san_dns_names: sanNames,
      });
    },
    onSuccess: (data) => {
      setIssuedCert(data);
    },
  });

  const onSubmit = (values: IssueFormValues) => {
    mutation.mutate(values);
  };

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
    alert('Copied to clipboard!');
  };

  const downloadFile = (filename: string, content: string) => {
    const blob = new Blob([content], { type: 'text/plain;charset=utf-8' });
    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.download = filename;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    URL.revokeObjectURL(url);
  };

  return (
    <div className="w-full max-w-4xl mx-auto flex flex-col gap-6">
      {/* Back button */}
      <div>
        <Link href="/inventory">
          <Button variant="secondary" size="sm" className="gap-2">
            <ArrowLeft className="w-4 h-4" />
            <span>Cancel</span>
          </Button>
        </Link>
      </div>

      {issuedCert ? (
        /* Success Screen */
        <Card className="border-gold">
          <CardHeader>
            <div className="flex items-center gap-3 text-gold mb-2">
              <CheckCircle className="w-6 h-6 animate-pulse" />
              <CardTitle className="text-xl">Certificate Provisioned Successfully</CardTitle>
            </div>
            <CardDescription>
              A new cryptographically verifiable machine identity and private key pair have been generated.
            </CardDescription>
          </CardHeader>
          <CardContent className="flex flex-col gap-6">
            
            {/* Warning Banner */}
            <div className="bg-alert/10 border border-alert/20 p-4 rounded flex items-start gap-3">
              <AlertTriangle className="w-5 h-5 text-alert shrink-0 mt-0.5 animate-bounce" />
              <div>
                <h4 className="text-sm font-semibold text-alert">Security Warning: Private Key Displayed Once!</h4>
                <p className="text-xs text-cream/75 mt-1 leading-relaxed">
                  The private key displayed below will <strong>never</strong> be shown again and is not stored in our systems. 
                  Please copy or download it immediately to prevent losing access to this credential.
                </p>
              </div>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
              {/* Certificate Metadata */}
              <div className="flex flex-col gap-1">
                <span className="text-xs uppercase tracking-wider text-cream/50">Identity ID</span>
                <span className="font-mono text-sm bg-void border border-gold-12 px-3 py-2 rounded text-gold select-all">
                  {issuedCert.certificate.id}
                </span>
              </div>
              <div className="flex flex-col gap-1">
                <span className="text-xs uppercase tracking-wider text-cream/50">Common Name (Subject)</span>
                <span className="font-mono text-sm bg-void border border-gold-12 px-3 py-2 rounded text-cream select-all">
                  {issuedCert.certificate.subject}
                </span>
              </div>
            </div>

            {/* Public Certificate and Private Key blocks */}
            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
              {/* Certificate PEM */}
              <div className="flex flex-col gap-3">
                <div className="flex justify-between items-center">
                  <span className="text-xs uppercase tracking-wider text-cream/50 font-medium">Public Certificate (PEM)</span>
                  <div className="flex items-center gap-1.5">
                    <button
                      onClick={() => copyToClipboard(issuedCert.certificate_pem)}
                      className="text-[10px] flex items-center gap-1 text-gold hover:text-gold/80 hover:bg-gold-12/30 border border-gold-20/40 px-2 py-1 rounded transition-all"
                    >
                      <Copy className="w-3 h-3" />
                      <span>Copy</span>
                    </button>
                    <button
                      onClick={() => downloadFile(`${issuedCert.certificate.subject}.crt`, issuedCert.certificate_pem)}
                      className="text-[10px] flex items-center gap-1 text-gold hover:text-gold/80 hover:bg-gold-12/30 border border-gold-20/40 px-2 py-1 rounded transition-all"
                    >
                      <Download className="w-3 h-3" />
                      <span>Download</span>
                    </button>
                  </div>
                </div>
                <pre className="font-mono text-[9px] bg-void border border-gold-12 p-3 rounded text-cream/80 overflow-x-auto h-64 select-all whitespace-pre leading-normal">
                  {issuedCert.certificate_pem}
                </pre>
              </div>

              {/* Private Key PEM */}
              <div className="flex flex-col gap-3">
                <div className="flex justify-between items-center">
                  <span className="text-xs uppercase tracking-wider text-alert font-medium">Private Key (Sensitive PEM)</span>
                  <div className="flex items-center gap-1.5">
                    <button
                      onClick={() => copyToClipboard(issuedCert.private_key_pem)}
                      className="text-[10px] flex items-center gap-1 text-alert hover:bg-alert/15 border border-alert/20 px-2 py-1 rounded transition-all"
                    >
                      <Copy className="w-3 h-3" />
                      <span>Copy</span>
                    </button>
                    <button
                      onClick={() => downloadFile(`${issuedCert.certificate.subject}.key`, issuedCert.private_key_pem)}
                      className="text-[10px] flex items-center gap-1 text-alert hover:bg-alert/15 border border-alert/20 px-2 py-1 rounded transition-all"
                    >
                      <Download className="w-3 h-3" />
                      <span>Download</span>
                    </button>
                  </div>
                </div>
                <pre className="font-mono text-[9px] bg-void border border-gold-12 p-3 rounded text-alert/80 overflow-x-auto h-64 select-all whitespace-pre leading-normal">
                  {issuedCert.private_key_pem}
                </pre>
              </div>
            </div>

            <div className="border-t border-gold-12 pt-4 flex justify-end">
              <Button onClick={() => router.push('/inventory')} variant="primary">
                Return to Ledger
              </Button>
            </div>
          </CardContent>
        </Card>
      ) : (
        /* Form Screen */
        <Card className="max-w-2xl mx-auto">
          <CardHeader>
            <div className="flex items-center gap-2.5 text-gold">
              <Shield className="w-5 h-5" />
              <CardTitle>Issue Machine Identity</CardTitle>
            </div>
            <CardDescription className="mt-1">
              Provision a new X.509 certificate for an AI agent, service workload, or bot.
            </CardDescription>
          </CardHeader>
          <CardContent>
            <form onSubmit={handleSubmit(onSubmit)} className="flex flex-col gap-5">
              <Input
                label="Common Name (Subject)"
                placeholder="e.g. auth-agent-01, localhost"
                error={errors.subject?.message}
                {...register('subject')}
                suppressHydrationWarning
              />

              <div className="flex flex-col gap-1 w-full">
                <Input
                  label="Validity Period (Days)"
                  type="number"
                  min={1}
                  max={825}
                  onInput={(e) => {
                    const val = e.currentTarget.value;
                    if (val.length > 3) {
                      e.currentTarget.value = val.slice(0, 3);
                    }
                  }}
                  placeholder="e.g. 365 (Max 825)"
                  error={errors.validity_days?.message}
                  {...register('validity_days', { valueAsNumber: true })}
                  suppressHydrationWarning
                />
                {!errors.validity_days && (
                  <span className="text-[10px] text-cream/40 font-mono mt-0.5">
                    Maximum allowed validity period is 825 days (~2.2 years) due to CA compliance.
                  </span>
                )}
              </div>

              <Input
                label="Subject Alternative Names (SANs) - Comma Separated"
                placeholder="e.g. localhost, billing-service.local, agent-01"
                error={errors.san_names_raw?.message}
                {...register('san_names_raw')}
                suppressHydrationWarning
              />

              {mutation.isError && (
                <div className="bg-alert/10 border border-alert/20 p-4 rounded text-alert text-xs flex gap-2 items-center">
                  <AlertTriangle className="w-4 h-4 shrink-0" />
                  <span>
                    Failed to issue certificate:{' '}
                    {(mutation.error as any).response?.data?.message ||
                      (mutation.error as any).message ||
                      'Internal Connection Error'}
                  </span>
                </div>
              )}

              <div className="flex justify-end gap-3 mt-4 border-t border-gold-12 pt-6">
                <Link href="/inventory">
                  <Button type="button" variant="secondary" suppressHydrationWarning>
                    Cancel
                  </Button>
                </Link>
                <Button type="submit" variant="primary" disabled={mutation.isPending} suppressHydrationWarning>
                  {mutation.isPending ? 'Provisioning Identity...' : 'Generate Certificate'}
                </Button>
              </div>
            </form>
          </CardContent>
        </Card>
      )}
    </div>
  );
}
