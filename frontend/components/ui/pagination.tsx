import React from 'react';
import { Button } from './button';
import { ChevronLeft, ChevronRight } from 'lucide-react';

interface PaginationProps {
  page: number;
  total: number;
  limit: number;
  onPageChange: (newPage: number) => void;
}

export function Pagination({ page, total, limit, onPageChange }: PaginationProps) {
  const totalPages = Math.max(1, Math.ceil(total / limit));

  return (
    <div className="flex items-center justify-between border-t border-gold-12 pt-6">
      <span className="text-xs text-cream/50 uppercase tracking-wider">
        Showing Page {page} of {totalPages} ({total} Records)
      </span>
      <div className="flex gap-2">
        <Button
          variant="secondary"
          size="sm"
          disabled={page <= 1}
          onClick={() => onPageChange(page - 1)}
          className="gap-1"
        >
          <ChevronLeft className="w-4 h-4" />
          <span>Prev</span>
        </Button>
        <Button
          variant="secondary"
          size="sm"
          disabled={page >= totalPages}
          onClick={() => onPageChange(page + 1)}
          className="gap-1"
        >
          <span>Next</span>
          <ChevronRight className="w-4 h-4" />
        </Button>
      </div>
    </div>
  );
}
