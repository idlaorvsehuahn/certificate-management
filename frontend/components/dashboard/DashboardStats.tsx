'use client';

import React from 'react';
import { Card, CardContent } from '../ui/card';
import { Shield, ShieldCheck, ShieldAlert, Clock } from 'lucide-react';
import { DashboardStatsResponse } from '../../types';

interface DashboardStatsProps {
  stats: DashboardStatsResponse;
  isLoading: boolean;
}

export function DashboardStats({ stats, isLoading }: DashboardStatsProps) {
  const items = [
    {
      title: 'Total Identities',
      value: stats.total_certificates,
      desc: 'Registered machine credentials',
      icon: Shield,
      color: 'text-cream',
    },
    {
      title: 'Active Trust State',
      value: stats.active_certificates,
      desc: 'Active verified certs in use',
      icon: ShieldCheck,
      color: 'text-gold',
    },
    {
      title: 'Revoked Certificates',
      value: stats.revoked_certificates,
      desc: 'Invalidated or rogue identities',
      icon: ShieldAlert,
      color: 'text-alert',
    },
    {
      title: 'Expiring Soon',
      value: stats.expiring_soon_certificates,
      desc: 'Due for rotation in 30 days',
      icon: Clock,
      color: 'text-cream/50',
    },
  ];

  return (
    <div className="grid grid-cols-2 md:grid-cols-4 gap-4 md:gap-6 mb-10">
      {items.map((item, index) => {
        const IconComponent = item.icon;
        return (
          <Card key={index} className="flex flex-col justify-between">
            <CardContent className="flex flex-col gap-4">
              <div className="flex items-center justify-between">
                <span className="text-[10px] uppercase tracking-widest text-cream/50 font-medium">
                  {item.title}
                </span>
                <IconComponent className={`w-4 h-4 ${item.color}`} />
              </div>
              <div className="flex flex-col gap-1">
                {isLoading ? (
                  <div className="h-10 w-20 bg-gold-12 rounded animate-pulse" />
                ) : (
                  <span className={`text-4xl md:text-5xl font-light tracking-tight ${item.color}`}>
                    {item.value}
                  </span>
                )}
                <span className="text-[10px] text-cream/45">{item.desc}</span>
              </div>
            </CardContent>
          </Card>
        );
      })}
    </div>
  );
}
export default DashboardStats;
