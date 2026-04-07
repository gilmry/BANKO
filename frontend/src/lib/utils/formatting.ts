import type { Money, Currency } from '@/lib/types';

const currencyFormatters: Record<Currency, Intl.NumberFormat> = {
  TND: new Intl.NumberFormat('fr-TN', { style: 'currency', currency: 'TND', minimumFractionDigits: 3 }),
  EUR: new Intl.NumberFormat('fr-FR', { style: 'currency', currency: 'EUR' }),
  USD: new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD' }),
  GBP: new Intl.NumberFormat('en-GB', { style: 'currency', currency: 'GBP' }),
};

export function formatMoney(money: Money | null | undefined): string {
  if (!money) return '—';
  const formatter = currencyFormatters[money.currency];
  return formatter.format(money.amount);
}

export function formatCurrency(amount: number, currency: Currency): string {
  const formatter = currencyFormatters[currency];
  return formatter.format(amount);
}

export function formatDate(dateStr: string, locale = 'fr-FR'): string {
  try {
    const date = new Date(dateStr);
    return new Intl.DateTimeFormat(locale, {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
    }).format(date);
  } catch {
    return dateStr;
  }
}

export function formatDateTime(dateStr: string, locale = 'fr-FR'): string {
  try {
    const date = new Date(dateStr);
    return new Intl.DateTimeFormat(locale, {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    }).format(date);
  } catch {
    return dateStr;
  }
}

export function formatTime(dateStr: string, locale = 'fr-FR'): string {
  try {
    const date = new Date(dateStr);
    return new Intl.DateTimeFormat(locale, {
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
    }).format(date);
  } catch {
    return dateStr;
  }
}

export function formatPercentage(value: number, decimals = 2): string {
  return `${value.toFixed(decimals)}%`;
}

export function formatAccountNumber(iban: string): string {
  if (!iban) return '—';
  // Display last 4 digits of IBAN
  return `...${iban.slice(-4)}`;
}

export function formatPhoneNumber(phone: string, locale = 'fr-TN'): string {
  if (!phone) return '—';
  // Basic formatting - can be enhanced per locale
  return phone.replace(/(\d{2})(\d{3})(\d{3})(\d{3})/, '+$1 $2 $3 $4');
}

export function formatFullName(firstName: string, lastName: string): string {
  return `${firstName} ${lastName}`.trim();
}

export function truncateString(str: string, maxLength: number): string {
  if (str.length <= maxLength) return str;
  return `${str.slice(0, maxLength - 3)}...`;
}

export function capitalizeFirstLetter(str: string): string {
  return str.charAt(0).toUpperCase() + str.slice(1);
}

export function relativeDateFromNow(dateStr: string, locale = 'fr'): string {
  const date = new Date(dateStr);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffSecs = Math.floor(diffMs / 1000);
  const diffMins = Math.floor(diffSecs / 60);
  const diffHours = Math.floor(diffMins / 60);
  const diffDays = Math.floor(diffHours / 24);

  const translations = {
    fr: {
      justNow: 'à l\'instant',
      minutesAgo: (n: number) => `il y a ${n} minute${n > 1 ? 's' : ''}`,
      hoursAgo: (n: number) => `il y a ${n} heure${n > 1 ? 's' : ''}`,
      daysAgo: (n: number) => `il y a ${n} jour${n > 1 ? 's' : ''}`,
    },
    en: {
      justNow: 'just now',
      minutesAgo: (n: number) => `${n} minute${n > 1 ? 's' : ''} ago`,
      hoursAgo: (n: number) => `${n} hour${n > 1 ? 's' : ''} ago`,
      daysAgo: (n: number) => `${n} day${n > 1 ? 's' : ''} ago`,
    },
  };

  const t = translations[locale as keyof typeof translations] || translations.en;

  if (diffSecs < 60) return t.justNow;
  if (diffMins < 60) return t.minutesAgo(diffMins);
  if (diffHours < 24) return t.hoursAgo(diffHours);
  if (diffDays < 30) return t.daysAgo(diffDays);

  return formatDate(dateStr);
}
