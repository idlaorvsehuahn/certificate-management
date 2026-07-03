import React from 'react';

interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: 'primary' | 'secondary' | 'outline' | 'ghost' | 'danger';
  size?: 'sm' | 'md' | 'lg';
}

export function Button({
  children,
  variant = 'primary',
  size = 'md',
  className = '',
  ...props
}: ButtonProps) {
  const baseStyle =
    'inline-flex items-center justify-center font-medium rounded transition-all duration-200 focus:outline-none focus:ring-1 focus:ring-gold disabled:opacity-50 disabled:pointer-events-none cursor-pointer';

  const variants = {
    primary: 'bg-gold text-void hover:bg-gold/90 border border-gold',
    secondary: 'bg-void text-cream hover:bg-void/80 border border-gold-12',
    outline: 'border border-gold text-gold bg-transparent hover:bg-gold hover:text-void',
    ghost: 'text-cream hover:bg-gold-12',
    danger: 'bg-alert text-cream hover:bg-alert/90 border border-alert',
  };

  const sizes = {
    sm: 'px-3 py-1.5 text-xs',
    md: 'px-4 py-2 text-sm',
    lg: 'px-6 py-3 text-base',
  };

  return (
    <button
      className={`${baseStyle} ${variants[variant]} ${sizes[size]} ${className}`}
      {...props}
    >
      {children}
    </button>
  );
}
