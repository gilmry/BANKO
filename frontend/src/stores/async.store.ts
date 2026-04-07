import { writable, derived, type Readable } from 'svelte/store';
import type { AsyncState, ApiError } from '@/lib/types';

interface AsyncStoreType<T> extends Readable<AsyncState<T>> {
  set: (value: AsyncState<T>) => void;
  setLoading: () => void;
  setData: (data: T) => void;
  setError: (error: ApiError) => void;
  reset: () => void;
  execute: (fn: () => Promise<T>) => Promise<T | null>;
}

export function createAsyncStore<T>(): AsyncStoreType<T> {
  const { subscribe, set } = writable<AsyncState<T>>({ status: 'idle' });

  return {
    subscribe,
    set,

    setLoading() {
      set({ status: 'pending' });
    },

    setData(data: T) {
      set({ status: 'success', data });
    },

    setError(error: ApiError) {
      set({ status: 'error', error });
    },

    reset() {
      set({ status: 'idle' });
    },

    async execute(fn: () => Promise<T>): Promise<T | null> {
      this.setLoading();
      try {
        const data = await fn();
        this.setData(data);
        return data;
      } catch (error) {
        const apiError = error instanceof Error
          ? new (error.constructor as any)(error.message) as ApiError
          : new Error('Unknown error') as ApiError;
        this.setError(apiError);
        return null;
      }
    },
  };
}

interface PaginatedAsyncStoreType<T> extends Readable<AsyncState<T[]>> {
  subscribe: (run: (value: AsyncState<T[]>) => void) => () => void;
  setLoading: () => void;
  setData: (data: T[], total: number, page: number) => void;
  setError: (error: ApiError) => void;
  reset: () => void;
  execute: (fn: () => Promise<{ data: T[]; total: number; page: number }>) => Promise<T[] | null>;
  metadata: Readable<{ total: number; page: number; pageSize: number }>;
}

export function createPaginatedAsyncStore<T>(pageSize = 10): PaginatedAsyncStoreType<T> {
  const store = writable<AsyncState<T[]>>({ status: 'idle' });
  const metadata = writable<{ total: number; page: number; pageSize: number }>({
    total: 0,
    page: 1,
    pageSize,
  });

  return {
    subscribe: store.subscribe,
    metadata: { subscribe: metadata.subscribe } as Readable<{ total: number; page: number; pageSize: number }>,

    setLoading() {
      store.set({ status: 'pending' });
    },

    setData(data: T[], total: number, page: number) {
      store.set({ status: 'success', data });
      metadata.set({ total, page, pageSize });
    },

    setError(error: ApiError) {
      store.set({ status: 'error', error });
    },

    reset() {
      store.set({ status: 'idle' });
      metadata.set({ total: 0, page: 1, pageSize });
    },

    async execute(fn: () => Promise<{ data: T[]; total: number; page: number }>): Promise<T[] | null> {
      this.setLoading();
      try {
        const result = await fn();
        this.setData(result.data, result.total, result.page);
        return result.data;
      } catch (error) {
        const apiError = error instanceof Error
          ? new (error.constructor as any)(error.message) as ApiError
          : new Error('Unknown error') as ApiError;
        this.setError(apiError);
        return null;
      }
    },
  };
}
