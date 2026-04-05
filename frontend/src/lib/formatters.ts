export function formatCurrency(
  amount: number,
  currency: string = 'TND',
  locale: string = 'fr',
): string {
  return new Intl.NumberFormat(locale === 'ar' ? 'ar-TN' : locale, {
    style: 'currency',
    currency,
    minimumFractionDigits: currency === 'TND' ? 3 : 2,
    maximumFractionDigits: currency === 'TND' ? 3 : 2,
  }).format(amount);
}

export function formatDate(
  date: Date | string,
  locale: string = 'fr',
): string {
  const d = typeof date === 'string' ? new Date(date) : date;
  return new Intl.DateTimeFormat(locale === 'ar' ? 'ar-TN' : locale, {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  }).format(d);
}

export function formatNumber(num: number, locale: string = 'fr'): string {
  return new Intl.NumberFormat(locale === 'ar' ? 'ar-TN' : locale, {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  }).format(num);
}

export function formatDateShort(
  date: Date | string,
  locale: string = 'fr',
): string {
  const d = typeof date === 'string' ? new Date(date) : date;
  return new Intl.DateTimeFormat(locale === 'ar' ? 'ar-TN' : locale, {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
  }).format(d);
}

export function formatDateTime(
  date: Date | string,
  locale: string = 'fr',
): string {
  const d = typeof date === 'string' ? new Date(date) : date;
  return new Intl.DateTimeFormat(locale === 'ar' ? 'ar-TN' : locale, {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  }).format(d);
}

export function formatPercent(num: number, locale: string = 'fr'): string {
  return new Intl.NumberFormat(locale === 'ar' ? 'ar-TN' : locale, {
    style: 'percent',
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  }).format(num / 100);
}
