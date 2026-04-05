import { writable, derived } from 'svelte/store';
import { authApi } from '../lib/api/auth';

export interface User {
  id: string;
  email: string;
  role: string;
}

const user = writable<User | null>(null);

export const authStore = {
  subscribe: user.subscribe,
  user,
  isAuthenticated: derived(user, ($user) => $user !== null),

  async login(email: string, password: string): Promise<void> {
    const response = await authApi.login({ email, password });
    user.set(response.user);
  },

  async logout(): Promise<void> {
    await authApi.logout();
    user.set(null);
  },

  async checkAuth(): Promise<void> {
    try {
      const userData = await authApi.me();
      user.set(userData);
    } catch {
      user.set(null);
    }
  },

  clear(): void {
    user.set(null);
  },
};
