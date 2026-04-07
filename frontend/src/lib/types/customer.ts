import type { KycStatus } from './common';

export type Gender = 'male' | 'female' | 'other';
export type CustomerType = 'individual' | 'business';
export type DocumentType =
  | 'id_card'
  | 'passport'
  | 'driving_license'
  | 'residence_permit'
  | 'business_registration'
  | 'tax_certificate';

export interface CustomerResponse {
  id: string;
  first_name: string;
  last_name: string;
  email: string;
  phone?: string;
  date_of_birth: string;
  gender: Gender;
  cin: string;
  nationality: string;
  kyc_status: KycStatus;
  customer_type: CustomerType;
  created_at: string;
  updated_at: string;
}

export interface CreateCustomerRequest {
  first_name: string;
  last_name: string;
  email: string;
  date_of_birth: string;
  gender: Gender;
  cin: string;
  nationality: string;
  phone?: string;
  customer_type: CustomerType;
}

export interface UpdateCustomerRequest {
  first_name?: string;
  last_name?: string;
  email?: string;
  phone?: string;
  nationality?: string;
}

export interface CustomerSearchQuery {
  page?: number;
  limit?: number;
  search?: string;
  kyc_status?: KycStatus;
  customer_type?: CustomerType;
  sort_by?: 'created_at' | 'name';
  sort_order?: 'asc' | 'desc';
}

export interface CustomerListResponse {
  data: CustomerResponse[];
  total: number;
  page: number;
  limit: number;
  total_pages: number;
}

export interface ProfessionalInfo {
  profession: string;
  employer: string;
  monthly_income: number;
  source_of_funds: string;
  years_of_employment: number;
}

export interface Beneficiary {
  id: string;
  name: string;
  relationship: string;
  date_of_birth: string;
  ownership_percentage: number;
}

export interface KycDocument {
  id: string;
  customer_id: string;
  document_type: DocumentType;
  file_url: string;
  uploaded_at: string;
  verified_at: string | null;
  status: 'pending' | 'verified' | 'rejected';
}

export interface KycSubmission {
  customer_id: string;
  professional_info: ProfessionalInfo;
  beneficiaries: Beneficiary[];
  documents: KycDocument[];
  declaration_date: string;
}
