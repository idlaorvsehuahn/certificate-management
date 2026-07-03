import React from 'react';
import { CertificateStatus } from '../../types';

interface BadgeProps extends React.HTMLAttributes<HTMLSpanElement> {
  status: CertificateStatus;
}

export function Badge({ status, className = '', ...props }: BadgeProps) {
  const baseStyle =
    'inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-semibold uppercase tracking-wider';

  const styles = {
    ACTIVE: 'bg-gold/10 text-gold border border-gold-20',
    REVOKED: 'bg-alert/10 text-alert border border-alert/20',
    EXPIRED: 'bg-cream/10 text-cream/60 border border-cream/20',
  };

  return (
    <span
      className={`${baseStyle} ${styles[status] || styles.EXPIRED} ${className}`}
      {...props}
    >
      {status}
    </span>
  );
}
