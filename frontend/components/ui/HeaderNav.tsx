'use client';

import React from 'react';
import Link from 'next/link';
import { usePathname } from 'next/navigation';

export function HeaderNav() {
  const pathname = usePathname();

  const isActive = (path: string) => {
    if (path === '/inventory') {
      // Highlight "Inventory" when viewing the list or a certificate detail page
      return pathname === '/' || pathname === '/inventory' || pathname.startsWith('/inventory/');
    }
    return pathname === path;
  };

  const linkClass = (path: string) => {
    const base = "text-[10px] uppercase tracking-[0.25em] font-medium transition-all duration-200 py-1.5 px-3.5 rounded-md border";
    return isActive(path)
      ? `${base} text-gold bg-gold-12 border-gold-20/40 font-semibold shadow-[0_0_12px_rgba(201,146,42,0.08)]`
      : `${base} text-cream/50 border-transparent hover:text-cream hover:bg-gold-12/20`;
  };

  return (
    <nav className="flex items-center gap-8 h-full">
      <Link href="/inventory" className={linkClass('/inventory')}>
        Inventory
      </Link>
      <Link href="/certificates/new" className={linkClass('/certificates/new')}>
        Issue Certificate
      </Link>
      <Link href="/certificates/import" className={linkClass('/certificates/import')}>
        Import Certificate
      </Link>
    </nav>
  );
}
