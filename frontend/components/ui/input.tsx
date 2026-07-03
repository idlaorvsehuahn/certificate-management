import React from 'react';

interface InputProps extends React.InputHTMLAttributes<HTMLInputElement> {
  label?: string;
  error?: string;
}

export const Input = React.forwardRef<HTMLInputElement, InputProps>(
  ({ label, error, className = '', ...props }, ref) => {
    return (
      <div className="w-full flex flex-col gap-1.5">
        {label && (
          <label className="text-xs uppercase tracking-wider text-cream/70 font-medium">
            {label}
          </label>
        )}
        <input
          ref={ref}
          className={`w-full bg-void/50 border border-gold-12 rounded px-3 py-2 text-sm text-cream placeholder-cream/30 focus:outline-none focus:border-gold/50 focus:ring-1 focus:ring-gold/50 transition-colors ${
            error ? 'border-alert focus:border-alert focus:ring-alert' : ''
          } ${className}`}
          {...props}
        />
        {error && (
          <span className="text-xs text-alert mt-0.5">{error}</span>
        )}
      </div>
    );
  }
);

Input.displayName = 'Input';
