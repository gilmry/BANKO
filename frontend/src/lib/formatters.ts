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

/**
 * Format RIB (Relevé d'Identité Bancaire) with dashes
 * RIB format: 23 digits => XXX XXX XX XXXXXXXXXX XXX
 * @example formatAccountNumber('23001000100072897843120') => '230 010 00 1000728978 431 20'
 */
export function formatAccountNumber(rib: string): string {
  const cleaned = rib.replace(/\D/g, '');
  if (cleaned.length !== 23) {
    return rib;
  }
  return `${cleaned.slice(0, 3)} ${cleaned.slice(3, 8)} ${cleaned.slice(8, 10)} ${cleaned.slice(10, 20)} ${cleaned.slice(20, 23)}`;
}

/**
 * Format IBAN with spaces every 4 characters
 * @example formatIBAN('TN5910006032181591361337') => 'TN59 1000 6032 1815 9136 1337'
 */
export function formatIBAN(iban: string): string {
  const cleaned = iban.toUpperCase().replace(/\s/g, '');
  return cleaned.replace(/(.{4})/g, '$1 ').trim();
}

/**
 * Format currency with support for TND (3 decimals), EUR/USD (2 decimals)
 */
export function formatCurrencyWithLocale(
  amount: number,
  currency: string = 'TND',
  locale: string = 'fr',
): string {
  const isArabic = locale === 'ar';
  const localeCode = isArabic ? 'ar-TN' : locale === 'en' ? 'en-US' : 'fr-FR';

  return new Intl.NumberFormat(localeCode, {
    style: 'currency',
    currency,
    minimumFractionDigits: currency === 'TND' ? 3 : 2,
    maximumFractionDigits: currency === 'TND' ? 3 : 2,
  }).format(amount);
}

/**
 * Format date with custom format options
 */
export function formatDateCustom(
  date: Date | string,
  locale: string = 'fr',
  options?: Intl.DateTimeFormatOptions,
): string {
  const d = typeof date === 'string' ? new Date(date) : date;
  const localeCode = locale === 'ar' ? 'ar-TN' : locale === 'en' ? 'en-US' : 'fr-FR';

  const defaultOptions: Intl.DateTimeFormatOptions = {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  };

  return new Intl.DateTimeFormat(localeCode, { ...defaultOptions, ...options }).format(d);
}
