import { api } from './client';

export interface AuditEntry {
  id: string;
  timestamp: string;
  user_id: string;
  action: string;
  resource_type: string;
  resource_id: string;
  ip_address?: string;
  details?: string;
}

export interface AuditListResponse {
  data: AuditEntry[];
  total: number;
  page: number;
  total_pages: number;
  limit: number;
}

export interface AuditFilter {
  page?: number;
  limit?: number;
  action?: string;
  resource_type?: string;
  date_from?: string;
  date_to?: string;
  actor_id?: string;
}

export interface IntegrityCheckResponse {
  is_valid: boolean;
  checked_entries: number;
  invalid_entries: number;
  details: string;
}

export const auditApi = {
  list: (filters?: AuditFilter) => {
    const query = new URLSearchParams();
    if (filters?.page) query.set('page', String(filters.page));
    if (filters?.limit) query.set('limit', String(filters.limit));
    if (filters?.action) query.set('action', filters.action);
    if (filters?.resource_type)
      query.set('resource_type', filters.resource_type);
    if (filters?.date_from) query.set('date_from', filters.date_from);
    if (filters?.date_to) query.set('date_to', filters.date_to);
    if (filters?.actor_id) query.set('actor_id', filters.actor_id);
    return api.get<AuditListResponse>(`/bct/audit/entries?${query}`);
  },

  exportEntries: (format: 'csv' | 'json', filters?: AuditFilter) => {
    const query = new URLSearchParams();
    query.set('format', format);
    if (filters?.action) query.set('action', filters.action);
    if (filters?.resource_type)
      query.set('resource_type', filters.resource_type);
    if (filters?.date_from) query.set('date_from', filters.date_from);
    if (filters?.date_to) query.set('date_to', filters.date_to);
    return api.get<{ data: string }>(
      `/bct/audit/entries/export?${query}`,
    );
  },

  checkIntegrity: () =>
    api.get<IntegrityCheckResponse>('/bct/audit/integrity'),
};
