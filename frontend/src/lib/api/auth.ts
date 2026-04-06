import { api } from './client';

export interface LoginRequest {
  email: string;
  password: string;
}

export interface LoginResponse {
  user: { id: string; email: string; role: string };
  message: string;
}

export interface RegisterRequest {
  email: string;
  password: string;
}

export interface RegisterResponse {
  user: { id: string; email: string };
  message: string;
}

export interface TwoFactorSetupResponse {
  qr_code_url: string;
  secret: string;
  backup_codes: string[];
}

export interface TwoFactorVerifyRequest {
  code: string;
}

export const authApi = {
  login: (data: LoginRequest) => api.post<LoginResponse>('/auth/login', data),

  register: (data: RegisterRequest) =>
    api.post<RegisterResponse>('/auth/register', data),

  logout: () => api.post<{ message: string }>('/auth/logout'),

  enable2fa: () => api.post<TwoFactorSetupResponse>('/auth/2fa/enable'),

  verify2fa: (data: TwoFactorVerifyRequest) =>
    api.post<{ message: string }>('/auth/2fa/verify', data),

  me: () => api.get<{ id: string; email: string; role: string }>('/auth/me'),
};
