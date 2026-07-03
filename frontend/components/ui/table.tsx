import React from 'react';

export function Table({ children, className = '', ...props }: React.HTMLAttributes<HTMLTableElement>) {
  return (
    <div className="w-full overflow-x-auto border border-gold-12 rounded bg-void/35">
      <table className={`w-full text-left border-collapse text-sm ${className}`} {...props}>
        {children}
      </table>
    </div>
  );
}

export function TableHeader({ children, className = '', ...props }: React.HTMLAttributes<HTMLTableSectionElement>) {
  return (
    <thead className={`border-b border-gold-12 bg-gold/5 ${className}`} {...props}>
      {children}
    </thead>
  );
}

export function TableBody({ children, className = '', ...props }: React.HTMLAttributes<HTMLTableSectionElement>) {
  return (
    <tbody className={`${className}`} {...props}>
      {children}
    </tbody>
  );
}

export function TableRow({ children, className = '', ...props }: React.HTMLAttributes<HTMLTableRowElement>) {
  return (
    <tr className={`border-b border-gold-12/50 hover:bg-gold/5 transition-colors cursor-pointer ${className}`} {...props}>
      {children}
    </tr>
  );
}

export function TableHead({ children, className = '', ...props }: React.HTMLAttributes<HTMLTableCellElement>) {
  return (
    <th className={`px-6 py-4 text-xs font-semibold uppercase tracking-wider text-cream/70 ${className}`} {...props}>
      {children}
    </th>
  );
}

export function TableCell({ children, className = '', ...props }: React.HTMLAttributes<HTMLTableCellElement>) {
  return (
    <td className={`px-6 py-4 text-cream/90 ${className}`} {...props}>
      {children}
    </td>
  );
}
