import type { ApiError } from '@/lib/types';

const API_BASE = import.meta.env.PUBLIC_API_URL || '/api/v1';

export class HttpError extends Error implements ApiError {
  statusCode: number;
  code?: string;
  details?: Record<string, unknown>;

  constructor(statusCode: number, message: string, code?: string, details?: Record<string, unknown>) {
    super(message);
    this.name = 'HttpError';
    this.statusCode = statusCode;
    this.code = code;
    this.details = details;
  }
}

async function request<T>(path: string, options: RequestInit = {}): Promise<T> {
  const url = `${API_BASE}${path}`;

  try {
    const res = await fetch(url, {
      credentials: 'include', // sends httpOnly cookies
      headers: {
        'Content-Type': 'application/json',
        ...(options.headers as Record<string, string>),
      },
      ...options,
    });

    if (!res.ok) {
      let errorData: { error?: string; code?: string; details?: Record<string, unknown> };
      try {
        errorData = await res.json();
      } catch {
        errorData = { error: res.statusText };
      }

      throw new HttpError(
        res.status,
        errorData.error || res.statusText,
        errorData.code,
        errorData.details
      );
    }

    // Handle 204 No Content
    if (res.status === 204) {
      return undefined as T;
    }

    return res.json();
  } catch (error) {
    if (error instanceof HttpError) {
      throw error;
    }
    throw new HttpError(0, error instanceof Error ? error.message : 'Unknown error');
  }
}

export const api = {
  get: <T>(path: string, params?: Record<string, unknown>) => {
    if (params) {
      const query = new URLSearchParams();
      for (const [key, value] of Object.entries(params)) {
        if (value !== undefined && value !== null) {
          query.set(key, String(value));
        }
      }
      const qs = query.toString();
      if (qs) {
        return request<T>(`${path}?${qs}`);
      }
    }
    return request<T>(path);
  },

  post: <T>(path: string, body?: unknown) =>
    request<T>(path, {
      method: 'POST',
      body: body ? JSON.stringify(body) : undefined,
    }),

  put: <T>(path: string, body?: unknown) =>
    request<T>(path, {
      method: 'PUT',
      body: body ? JSON.stringify(body) : undefined,
    }),

  patch: <T>(path: string, body?: unknown) =>
    request<T>(path, {
      method: 'PATCH',
      body: body ? JSON.stringify(body) : undefined,
    }),

  delete: <T>(path: string) => request<T>(path, { method: 'DELETE' }),
};
