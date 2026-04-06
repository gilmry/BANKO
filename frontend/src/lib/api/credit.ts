import { api } from './client';

export interface Loan {
  id: string;
  customer_id: string;
  amount: number;
  status: 'pending' | 'approved' | 'disbursed' | 'active' | 'completed' | 'defaulted';
  asset_class: number;
  created_at: string;
  updated_at: string;
}

export interface LoanClassification {
  total: number;
  by_status: Record<string, number>;
  by_asset_class: Record<number, number>;
  average_amount: number;
}

export const creditApi = {
  listLoans: (params?: Record<string, any>) => api.get<{ data: Loan[]; total: number }>('/credit/loans', params),
  getLoan: (id: string) => api.get<Loan>(`/credit/loans/${id}`),
  getClassification: () => api.get<LoanClassification>('/credit/loans/classification/summary'),
};
