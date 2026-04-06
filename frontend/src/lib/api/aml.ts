import { api } from './client';

export interface AmlAlert {
  id: string;
  customer_id: string;
  alert_type: string;
  risk_level: 'low' | 'medium' | 'high';
  status: 'open' | 'investigating' | 'resolved' | 'dismissed';
  created_at: string;
  updated_at: string;
  description: string;
}

export interface AmlStats {
  total_alerts: number;
  open_alerts: number;
  investigating_alerts: number;
  resolved_alerts: number;
  high_risk_count: number;
  medium_risk_count: number;
  low_risk_count: number;
}

export interface Investigation {
  id: string;
  alert_id: string;
  status: 'pending' | 'in_progress' | 'completed' | 'closed';
  started_at: string;
  completed_at?: string;
  assigned_to?: string;
}

export const amlApi = {
  listAlerts: (params?: Record<string, any>) => api.get<{ data: AmlAlert[]; total: number }>('/aml/alerts', params),
  getAlertStats: () => api.get<AmlStats>('/aml/alerts/stats'),
  listInvestigations: (params?: Record<string, any>) => api.get<{ data: Investigation[]; total: number }>('/aml/investigations', params),
};
