import type { Metadata } from 'next';
import './globals.css';
import Providers from './providers';
import Link from 'next/link';

import { HeaderNav } from '../components/ui/HeaderNav';

export const metadata: Metadata = {
  title: 'Arkion — Certificate Management Platform',
  description: 'Non-Human Identity Governance and Certificate Management Platform',
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" className="h-full antialiased dark" suppressHydrationWarning>
      <body className="min-h-full flex flex-col bg-void text-cream selection:bg-gold/30 selection:text-cream" suppressHydrationWarning>
        <Providers>
          {/* Header */}
          <header className="sticky top-0 inset-x-0 z-50 backdrop-blur-md bg-void/80 border-b border-gold-12">
            <div className="max-w-7xl mx-auto px-6 h-16 flex items-center justify-between">
              {/* Logo / Brand */}
              <Link href="/inventory" className="flex items-center gap-3.5 group">
                <svg width="24" height="24" viewBox="0 0 100 100" aria-label="Arkion logo" role="img">
                  <rect x="14" y="22" width="11" height="64" fill="#C9922A"></rect>
                  <rect x="44" y="6" width="11" height="80" fill="#C9922A"></rect>
                  <rect x="74" y="22" width="11" height="64" fill="#C9922A"></rect>
                  <rect x="5" y="41" width="90" height="6" fill="#C9922A"></rect>
                </svg>
                <span className="w-px h-5 bg-gold-20"></span>
                <span className="text-cream font-light tracking-[0.4em] text-sm uppercase">
                  ARKION
                </span>
              </Link>

              {/* Navigation Links */}
              <HeaderNav />

              {/* Tag / Version */}
              <div className="hidden sm:flex items-center gap-2 text-[10px] text-cream/40 tracking-wider uppercase font-mono">
                <span className="w-1.5 h-1.5 bg-gold rounded-full animate-pulse"></span>
                <span>Ledger v2026.01</span>
              </div>
            </div>
          </header>

          {/* Main Layout Area */}
          <main className="flex-1 max-w-7xl mx-auto px-6 py-10 w-full flex flex-col">
            {children}
          </main>

          {/* Footer */}
          <footer className="border-t border-gold-12/30 py-6 text-center text-[10px] uppercase tracking-widest text-cream/35">
            © {new Date().getFullYear()} Arkion.ai · Non-Human Identity Governance
          </footer>
        </Providers>
      </body>
    </html>
  );
}
