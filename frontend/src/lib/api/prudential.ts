import { api } from './client';

export interface PrudentialRatio {
  id: string;
  name: string;
  current_value: number;
  target_value: number;
  threshold_warning: number;
  threshold_breach: number;
  status: 'compliant' | 'warning' | 'breach';
  updated_at: string;
}

export interface SolvencyCheck {
  is_solvent: boolean;
  tier1_ratio: number;
  total_capital_ratio: number;
  leverage_ratio: number;
  details: string;
}

export interface TrendDataPoint {
  date: string;
  value: number;
}

export const prudentialApi = {
  getRatios: () => api.get<PrudentialRatio[]>('/prudential/ratios'),

  checkSolvency: () =>
    api.get<SolvencyCheck>('/prudential/solvency'),

  getTrend: (ratioId: string, days?: number) => {
    const query = new URLSearchParams();
    if (days) query.set('days', String(days));
    return api.get<TrendDataPoint[]>(
      `/prudential/ratios/${ratioId}/trend?${query}`,
    );
  },
};
