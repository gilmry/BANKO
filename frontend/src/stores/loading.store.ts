import { writable } from 'svelte/store';

export const isLoading = writable<boolean>(false);

export function withLoading<T>(fn: () => Promise<T>): Promise<T> {
  isLoading.set(true);
  return fn().finally(() => isLoading.set(false));
}
