import { writable, derived } from 'svelte/store';
import fr from './fr.json';
import en from './en.json';
import ar from './ar.json';

export type Locale = 'fr' | 'en' | 'ar';
export const defaultLocale: Locale = 'fr';

export const locales = {
  ar: { name: 'العربية', dir: 'rtl' as const, dateFormat: 'DD/MM/YYYY' },
  fr: { name: 'Français', dir: 'ltr' as const, dateFormat: 'DD/MM/YYYY' },
  en: { name: 'English', dir: 'ltr' as const, dateFormat: 'MM/DD/YYYY' },
};

const translations: Record<Locale, Record<string, any>> = { fr, en, ar };

// Svelte store for current locale
export const currentLocale = writable<Locale>(detectLocale());

export const currentDir = derived(currentLocale, ($locale) => locales[$locale].dir);

function detectLocale(): Locale {
  if (typeof window !== 'undefined') {
    const saved = document.cookie.match(/locale=(\w+)/)?.[1];
    if (saved && saved in locales) return saved as Locale;
    const browser = navigator.language.split('-')[0];
    if (browser in locales) return browser as Locale;
  }
  return defaultLocale;
}

export function setLocale(locale: Locale) {
  currentLocale.set(locale);
  if (typeof document !== 'undefined') {
    document.cookie = `locale=${locale};path=/;max-age=31536000`;
    document.documentElement.lang = locale;
    document.documentElement.dir = locales[locale].dir;
  }
}

// Nested key lookup: t('auth.login.title') -> translations[locale].auth.login.title
export function t(
  key: string,
  locale: Locale = defaultLocale,
  params?: Record<string, string>,
): string {
  const keys = key.split('.');
  let value: any = translations[locale];
  for (const k of keys) {
    value = value?.[k];
    if (value === undefined) {
      // Fallback to French
      value = translations.fr;
      for (const fk of keys) {
        value = value?.[fk];
        if (value === undefined) break;
      }
      break;
    }
  }
  if (typeof value !== 'string') return key;
  if (params) {
    return Object.entries(params).reduce(
      (s, [k, v]) => s.replace(`{${k}}`, v),
      value,
    );
  }
  return value;
}

// Reactive t function using store
export const tStore = derived(currentLocale, ($locale) => {
  return (key: string, params?: Record<string, string>) =>
    t(key, $locale, params);
});
