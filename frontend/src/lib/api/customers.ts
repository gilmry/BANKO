import { api } from './client';

export interface Customer {
  id: string;
  first_name: string;
  last_name: string;
  email: string;
  date_of_birth: string;
  gender: string;
  cin: string;
  nationality: string;
  kyc_status: string;
  created_at: string;
}

export interface CreateCustomerRequest {
  first_name: string;
  last_name: string;
  email: string;
  date_of_birth: string;
  gender: string;
  cin: string;
  nationality: string;
  profession?: string;
  employer?: string;
  monthly_income?: number;
  source_of_funds?: string;
  beneficiaries?: Beneficiary[];
}

export interface Beneficiary {
  name: string;
  relationship: string;
  ownership_percentage: number;
}

export interface KycSubmission {
  customer_id: string;
  documents: string[];
}

export interface CustomerListResponse {
  data: Customer[];
  total: number;
  page: number;
  total_pages: number;
}

export const customersApi = {
  create: (data: CreateCustomerRequest) =>
    api.post<Customer>('/customers', data),

  get: (id: string) => api.get<Customer>(`/customers/${id}`),

  list: (params?: { page?: number; limit?: number; search?: string }) => {
    const query = new URLSearchParams();
    if (params?.page) query.set('page', String(params.page));
    if (params?.limit) query.set('limit', String(params.limit));
    if (params?.search) query.set('search', params.search);
    return api.get<CustomerListResponse>(`/customers?${query}`);
  },

  submitKyc: (data: KycSubmission) =>
    api.post<{ message: string }>('/customers/kyc', data),
};
