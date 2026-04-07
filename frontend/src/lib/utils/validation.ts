// Validation utilities for form inputs and API data

export interface ValidationError {
  field: string;
  message: string;
}

export type ValidationResult = ValidationError[];

// Email validation
export function validateEmail(email: string): boolean {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return emailRegex.test(email);
}

// Phone validation (Tunisia format)
export function validatePhoneNumber(phone: string): boolean {
  const phoneRegex = /^(\+216)?[0-9]{8}$/;
  return phoneRegex.test(phone.replace(/\s|-/g, ''));
}

// CIN validation (Tunisia format)
export function validateCIN(cin: string): boolean {
  // Tunisian CIN: 8 digits
  return /^\d{8}$/.test(cin);
}

// IBAN validation (basic)
export function validateIBAN(iban: string): boolean {
  const ibanRegex = /^[A-Z]{2}[0-9]{2}[A-Z0-9]{1,30}$/;
  return ibanRegex.test(iban.replace(/\s/g, ''));
}

// Password validation
export function validatePassword(password: string): ValidationResult {
  const errors: ValidationError[] = [];

  if (password.length < 12) {
    errors.push({
      field: 'password',
      message: 'Le mot de passe doit contenir au moins 12 caractères',
    });
  }

  if (!/[A-Z]/.test(password)) {
    errors.push({
      field: 'password',
      message: 'Le mot de passe doit contenir au moins une majuscule',
    });
  }

  if (!/[a-z]/.test(password)) {
    errors.push({
      field: 'password',
      message: 'Le mot de passe doit contenir au moins une minuscule',
    });
  }

  if (!/[0-9]/.test(password)) {
    errors.push({
      field: 'password',
      message: 'Le mot de passe doit contenir au moins un chiffre',
    });
  }

  if (!/[!@#$%^&*()_+\-=\[\]{};':"\\|,.<>\/?]/.test(password)) {
    errors.push({
      field: 'password',
      message: 'Le mot de passe doit contenir au moins un caractère spécial',
    });
  }

  return errors;
}

// Customer form validation
export function validateCustomerForm(data: {
  first_name?: string;
  last_name?: string;
  email?: string;
  phone?: string;
  date_of_birth?: string;
  cin?: string;
}): ValidationResult {
  const errors: ValidationError[] = [];

  if (!data.first_name?.trim()) {
    errors.push({ field: 'first_name', message: 'Le prénom est requis' });
  }

  if (!data.last_name?.trim()) {
    errors.push({ field: 'last_name', message: 'Le nom est requis' });
  }

  if (!data.email?.trim()) {
    errors.push({ field: 'email', message: 'L\'email est requis' });
  } else if (!validateEmail(data.email)) {
    errors.push({ field: 'email', message: 'Email invalide' });
  }

  if (data.phone && !validatePhoneNumber(data.phone)) {
    errors.push({ field: 'phone', message: 'Numéro de téléphone invalide' });
  }

  if (!data.date_of_birth) {
    errors.push({ field: 'date_of_birth', message: 'La date de naissance est requise' });
  } else {
    const dob = new Date(data.date_of_birth);
    const today = new Date();
    const age = today.getFullYear() - dob.getFullYear();

    if (age < 18) {
      errors.push({ field: 'date_of_birth', message: 'L\'âge minimum est 18 ans' });
    }
  }

  if (!data.cin?.trim()) {
    errors.push({ field: 'cin', message: 'Le CIN est requis' });
  } else if (!validateCIN(data.cin)) {
    errors.push({ field: 'cin', message: 'CIN invalide (8 chiffres)' });
  }

  return errors;
}

// Account creation validation
export function validateAccountForm(data: {
  account_type?: string;
  currency?: string;
  initial_balance?: number;
}): ValidationResult {
  const errors: ValidationError[] = [];

  if (!data.account_type) {
    errors.push({ field: 'account_type', message: 'Le type de compte est requis' });
  }

  if (!data.currency) {
    errors.push({ field: 'currency', message: 'La devise est requise' });
  }

  if (data.initial_balance !== undefined) {
    if (data.initial_balance < 0) {
      errors.push({ field: 'initial_balance', message: 'Le montant doit être positif' });
    }
    if (data.initial_balance > 1000000000) {
      errors.push({ field: 'initial_balance', message: 'Le montant dépasse la limite' });
    }
  }

  return errors;
}

// Money amount validation
export function validateAmount(amount: number, minAmount = 0, maxAmount = 1000000000): ValidationResult {
  const errors: ValidationError[] = [];

  if (isNaN(amount) || amount === null || amount === undefined) {
    errors.push({ field: 'amount', message: 'Le montant est requis' });
  } else if (amount < minAmount) {
    errors.push({ field: 'amount', message: `Le montant minimum est ${minAmount}` });
  } else if (amount > maxAmount) {
    errors.push({ field: 'amount', message: `Le montant maximum est ${maxAmount}` });
  }

  return errors;
}

// Transfer validation
export function validateTransfer(data: {
  from_account_id?: string;
  to_account_id?: string;
  amount?: number;
}): ValidationResult {
  const errors: ValidationError[] = [];

  if (!data.from_account_id) {
    errors.push({ field: 'from_account_id', message: 'Le compte source est requis' });
  }

  if (!data.to_account_id) {
    errors.push({ field: 'to_account_id', message: 'Le compte destinataire est requis' });
  }

  if (data.from_account_id === data.to_account_id) {
    errors.push({ field: 'to_account_id', message: 'Les comptes source et destinataire ne peuvent pas être identiques' });
  }

  const amountErrors = validateAmount(data.amount, 0.01);
  errors.push(...amountErrors);

  return errors;
}

// Helper to get all errors for a field
export function getFieldErrors(errors: ValidationError[], field: string): string[] {
  return errors.filter(e => e.field === field).map(e => e.message);
}

// Helper to check if form is valid
export function isFormValid(errors: ValidationError[]): boolean {
  return errors.length === 0;
}
