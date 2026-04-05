import { api } from './client';

export interface Account {
  id: string;
  customer_id: string;
  account_number: string;
  iban: string;
  account_type: string;
  currency: string;
  balance: number;
  status: string;
  created_at: string;
}

export interface Movement {
  id: string;
  account_id: string;
  date: string;
  description: string;
  amount: number;
  balance_after: number;
  type: string;
}

export interface TransferRequest {
  from_account_id: string;
  to_iban: string;
  amount: number;
  currency: string;
  reference?: string;
}

export interface AccountListResponse {
  data: Account[];
  total: number;
  page: number;
  total_pages: number;
}

export interface MovementListResponse {
  data: Movement[];
  total: number;
  page: number;
  total_pages: number;
}

export const accountsApi = {
  list: (params?: { page?: number; limit?: number }) => {
    const query = new URLSearchParams();
    if (params?.page) query.set('page', String(params.page));
    if (params?.limit) query.set('limit', String(params.limit));
    return api.get<AccountListResponse>(`/accounts?${query}`);
  },

  get: (id: string) => api.get<Account>(`/accounts/${id}`),

  movements: (
    id: string,
    params?: { page?: number; limit?: number },
  ) => {
    const query = new URLSearchParams();
    if (params?.page) query.set('page', String(params.page));
    if (params?.limit) query.set('limit', String(params.limit));
    return api.get<MovementListResponse>(
      `/accounts/${id}/movements?${query}`,
    );
  },

  transfer: (data: TransferRequest) =>
    api.post<{ message: string; transfer_id: string }>(
      '/transfers',
      data,
    ),
};
