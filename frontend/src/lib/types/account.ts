import type { Money, AccountStatus, TransactionStatus } from './common';

export type AccountType = 'checking' | 'savings' | 'investment' | 'credit';

export interface AccountResponse {
  id: string;
  customer_id: string;
  account_type: AccountType;
  account_number: string;
  iban: string;
  bic: string;
  balance: Money;
  available_balance: Money;
  status: AccountStatus;
  currency: string;
  opened_at: string;
  closed_at: string | null;
}

export interface CreateAccountRequest {
  customer_id: string;
  account_type: AccountType;
  currency: string;
  initial_balance?: Money;
}

export interface UpdateAccountRequest {
  status?: AccountStatus;
  daily_limit?: number;
}

export interface AccountListQuery {
  page?: number;
  limit?: number;
  customer_id?: string;
  status?: AccountStatus;
  account_type?: AccountType;
}

export interface Movement {
  id: string;
  account_id: string;
  transaction_id: string;
  type: 'debit' | 'credit';
  amount: Money;
  description: string;
  status: TransactionStatus;
  created_at: string;
  reference: string;
}

export interface MovementResponse {
  data: Movement[];
  total: number;
  page: number;
  limit: number;
  total_pages: number;
}

export interface TransferRequest {
  from_account_id: string;
  to_account_id: string;
  amount: Money;
  description?: string;
  reference?: string;
}

export interface TransferResponse {
  id: string;
  status: TransactionStatus;
  from_account_id: string;
  to_account_id: string;
  amount: Money;
  created_at: string;
}
