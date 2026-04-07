// Common types across the BANKO platform

export type Currency = 'TND' | 'EUR' | 'USD' | 'GBP';

export interface Money {
  amount: number;
  currency: Currency;
}

export interface PaginatedResponse<T> {
  data: T[];
  total: number;
  page: number;
  limit: number;
  total_pages: number;
}

export interface ErrorResponse {
  error: string;
  code?: string;
  details?: Record<string, unknown>;
}

export interface ApiError extends Error {
  statusCode: number;
  code?: string;
  details?: Record<string, unknown>;
}

export type AsyncState<T> =
  | { status: 'idle' }
  | { status: 'pending' }
  | { status: 'success'; data: T }
  | { status: 'error'; error: ApiError };

export interface FileUpload {
  id: string;
  name: string;
  mime_type: string;
  size: number;
  uploaded_at: string;
  url: string;
}

export type KycStatus = 'pending' | 'submitted' | 'approved' | 'rejected';
export type AccountStatus = 'active' | 'suspended' | 'closed';
export type TransactionStatus = 'pending' | 'completed' | 'failed' | 'reversed';
export type AmlStatus = 'clear' | 'warning' | 'alert' | 'blocked';
