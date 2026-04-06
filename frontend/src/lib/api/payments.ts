import { api } from './client';

export interface Payment {
  id: string;
  source_account_id: string;
  destination_account_id: string;
  amount: number;
  currency: string;
  status: 'pending' | 'screened' | 'submitted' | 'executed' | 'failed' | 'rejected';
  payment_type: 'domestic' | 'sepa' | 'swift';
  created_at: string;
  updated_at: string;
}

export interface PaymentStatus {
  total_payments: number;
  pending_count: number;
  screened_count: number;
  submitted_count: number;
  executed_count: number;
  failed_count: number;
  rejected_count: number;
  total_volume: number;
  average_amount: number;
}

export interface ClearingStatus {
  status: 'operational' | 'degraded' | 'unavailable';
  last_sync: string;
  pending_submissions: number;
  failed_submissions: number;
}

export const paymentsApi = {
  listPayments: (params?: Record<string, any>) => api.get<{ data: Payment[]; total: number }>('/payments', params),
  getPayment: (id: string) => api.get<Payment>(`/payments/${id}`),
  getPaymentStatus: () => api.get<PaymentStatus>('/payments/status'),
  getClearingStatus: () => api.get<ClearingStatus>('/payments/clearing/status'),
};
