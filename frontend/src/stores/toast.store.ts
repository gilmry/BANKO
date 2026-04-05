import { writable } from 'svelte/store';

export interface Toast {
  id: string;
  message: string;
  type: 'success' | 'error' | 'info' | 'warning';
  duration?: number;
}

const toasts = writable<Toast[]>([]);

let counter = 0;

export const toastStore = {
  subscribe: toasts.subscribe,

  addToast(
    message: string,
    type: Toast['type'] = 'info',
    duration = 5000,
  ): void {
    const id = `toast-${++counter}`;
    toasts.update((all) => [...all, { id, message, type, duration }]);
    if (duration > 0) {
      setTimeout(() => {
        toastStore.removeToast(id);
      }, duration);
    }
  },

  removeToast(id: string): void {
    toasts.update((all) => all.filter((t) => t.id !== id));
  },
};
