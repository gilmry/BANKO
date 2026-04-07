# ReferenceData Bounded Context (BC19)

## Overview

The **ReferenceData** bounded context manages centralized reference data for the entire BANKO banking system. It provides a single source of truth for currency codes, country codes, bank information, regulatory classifications, and configurable system parameters.

**Classification**: P0 / MVP (Promoted from P1)

**Status**: Complete - Production-Ready

## Domain Model

### Core Entities

#### 1. **CountryCode**
- **Domain**: ISO 3166-1 country codes
- **Attributes**:
  - `id`: ReferenceDataId (UUID)
  - `code`: CountryCodeVo (iso_alpha2, iso_alpha3, iso_numeric)
  - `name_en`, `name_fr`, `name_ar`: Multilingual names
  - `is_sanctioned`: Boolean flag for international sanctions
  - `effective_from`, `effective_to`: Temporal validity
  - `created_at`, `updated_at`: Audit timestamps

- **Validation Rules**:
  - ISO Alpha-2: Exactly 2 uppercase letters
  - ISO Alpha-3: Exactly 3 uppercase letters
  - ISO Numeric: Exactly 3 digits
  - Names cannot be empty (for all languages)

- **Key Methods**:
  - `is_active()`: Check if active at current time
  - `is_sanctioned()`: Check sanctions status

#### 2. **CurrencyReference**
- **Domain**: ISO 4217 currency codes
- **Attributes**:
  - `id`: ReferenceDataId (UUID)
  - `code`: CurrencyCodeVo (3 uppercase letters)
  - `name_en`, `name_fr`: Multilingual names
  - `decimal_places`: Integer (0-8)
  - `is_active`: Boolean
  - `effective_from`, `effective_to`: Temporal validity

- **Validation Rules**:
  - Code length: Exactly 3 characters
  - Decimal places: 0-8 range
  - Names cannot be empty

#### 3. **BankCode**
- **Domain**: BIC/SWIFT codes with bank information
- **Attributes**:
  - `id`: ReferenceDataId (UUID)
  - `bic`: BicCodeVo (8 or 11 characters)
  - `bank_name`: String
  - `country_code`: CountryCodeVo
  - `is_active`: Boolean

- **Validation Rules**:
  - BIC: 8 or 11 chars (ignoring hyphens)
  - First 6 chars must be alphabetic
  - Bank name cannot be empty

#### 4. **BranchCode**
- **Domain**: Bank branch identifiers
- **Attributes**:
  - `id`: ReferenceDataId (UUID)
  - `branch_code`: String (unique per context)
  - `branch_name`: String
  - `bank_bic`: BicCodeVo (FK to BankCode)
  - `city`, `address`: Location info
  - `is_active`: Boolean

#### 5. **HolidayCalendar**
- **Domain**: Banking holidays (Tunisian banking holidays)
- **Attributes**:
  - `id`: ReferenceDataId (UUID)
  - `holiday_date`: DateTime
  - `holiday_name_en`, `holiday_name_fr`, `holiday_name_ar`: Multilingual names
  - `holiday_type`: HolidayType enum (National, Banking, Religious)
  - `is_banking_holiday`: Boolean

- **Use Cases**:
  - Check if date is banking holiday
  - Calculate next business day
  - Adjust settlement dates

#### 6. **SystemParameter**
- **Domain**: Configurable system parameters and thresholds
- **Attributes**:
  - `id`: ReferenceDataId (UUID)
  - `key`: String (unique)
  - `value`: String (stored as text, type-validated on write)
  - `parameter_type`: SystemParameterType (Integer, Decimal, String, Boolean)
  - `category`: String (Limits, Rates, Features, etc.)
  - `description`: String
  - `is_active`: Boolean
  - `effective_from`, `effective_to`: Temporal validity

- **Validation Rules**:
  - Integer: Must parse to i64
  - Decimal: Must parse to f64
  - Boolean: Must be "true" or "false"
  - String: Any value allowed

- **Examples**:
  - `MAX_DAILY_TRANSFER`: 50000000 (cents)
  - `OVERDRAFT_INTEREST_RATE`: 12.5
  - `ENABLE_REAL_TIME_NOTIFICATIONS`: true

#### 7. **RegulatoryCode**
- **Domain**: Regulatory classification codes
- **Attributes**:
  - `id`: ReferenceDataId (UUID)
  - `code`: String (unique)
  - `description_en`, `description_fr`: Multilingual descriptions
  - `classification`: RegulatoryClassification enum
    - BCT Asset Classes: StandardRisk, LowerRisk, HigherRisk
    - IFRS Categories: AmortizedCost, FairValueThroughOci, FairValueThroughPl
  - `is_active`: Boolean
  - `effective_from`, `effective_to`: Temporal validity

- **Use Cases**:
  - Asset classification (BCT prudential requirements)
  - Financial reporting (IFRS standards)

#### 8. **FeeScheduleReference**
- **Domain**: Fee schedule configuration
- **Attributes**:
  - `id`: ReferenceDataId (UUID)
  - `fee_type`: FeeType enum
    - AccountMaintenance, Transaction, Transfer, ForeignExchange
    - LatePayment, Overdraft, ATMWithdrawal, CheckIssue
  - `amount_cents`: i64 (fee amount in currency's smallest unit)
  - `currency_code`: CurrencyCodeVo
  - `description_en`, `description_fr`: Multilingual descriptions
  - `is_active`: Boolean
  - `effective_from`, `effective_to`: Temporal validity

- **Validation Rules**:
  - amount_cents >= 0
  - Currency must exist
  - Descriptions cannot be empty

## Value Objects

### Identifiers
- **ReferenceDataId**: UUID-based identifier for all reference data

### Codes & Codes
- **CountryCodeVo**: Validates ISO 3166 codes
- **CurrencyCodeVo**: Validates ISO 4217 codes
- **BicCodeVo**: Validates SWIFT/BIC codes

### Enumerations
- **FeeType**: 8 variants for fee classification
- **RegulatoryClassification**: 6 variants (BCT + IFRS)
- **HolidayType**: National, Banking, Religious
- **SystemParameterType**: Integer, Decimal, String, Boolean

## Application Layer

### Service: ReferenceDataService

**Constructor**:
```rust
pub fn new(repository: Arc<dyn IReferenceDataRepository>) -> Self
```

**Country Code Operations**:
- `create_country(CreateCountryCodeRequest) -> Result<CountryCodeResponse>`
- `get_country_by_id(id: &str) -> Result<CountryCodeResponse>`
- `get_country_by_iso_alpha2(code: &str) -> Result<CountryCodeResponse>`
- `get_country_by_iso_alpha3(code: &str) -> Result<CountryCodeResponse>`
- `list_countries(limit, offset) -> Result<Vec<CountryCodeResponse>>`
- `list_active_countries() -> Result<Vec<CountryCodeResponse>>`

**Currency Operations**:
- `create_currency(CreateCurrencyReferenceRequest) -> Result<CurrencyReferenceResponse>`
- `get_currency_by_code(code: &str) -> Result<CurrencyReferenceResponse>`
- `list_active_currencies() -> Result<Vec<CurrencyReferenceResponse>>`

**Bank Code Operations**:
- `create_bank_code(CreateBankCodeRequest) -> Result<BankCodeResponse>`
- `get_bank_code_by_bic(bic: &str) -> Result<BankCodeResponse>`
- `list_active_bank_codes() -> Result<Vec<BankCodeResponse>>`

**Holiday Operations**:
- `create_holiday(CreateHolidayCalendarRequest) -> Result<HolidayCalendarResponse>`
- `is_banking_holiday(date: DateTime<Utc>) -> Result<bool>`
- `find_banking_holidays(from, to) -> Result<Vec<HolidayCalendarResponse>>`

**System Parameter Operations**:
- `create_system_parameter(CreateSystemParameterRequest) -> Result<SystemParameterResponse>`
- `get_system_parameter_by_key(key: &str) -> Result<SystemParameterResponse>`
- `list_system_parameters_by_category(category: &str) -> Result<Vec<SystemParameterResponse>>`

**Regulatory Code Operations**:
- `create_regulatory_code(CreateRegulatoryCodeRequest) -> Result<RegulatoryCodeResponse>`
- `get_regulatory_code_by_code(code: &str) -> Result<RegulatoryCodeResponse>`
- `list_active_regulatory_codes() -> Result<Vec<RegulatoryCodeResponse>>`

**Fee Schedule Operations**:
- `create_fee_schedule(CreateFeeScheduleRequest) -> Result<FeeScheduleReferenceResponse>`
- `find_fee_schedules_by_type(fee_type: &str) -> Result<Vec<FeeScheduleReferenceResponse>>`
- `list_active_fee_schedules() -> Result<Vec<FeeScheduleReferenceResponse>>`

### Port: IReferenceDataRepository

Async trait defining persistence operations:

**Country Methods**: save, find_by_id, find_by_iso_alpha2, find_by_iso_alpha3, list, list_active
**Currency Methods**: save, find_by_id, find_by_code, list, list_active
**Bank Code Methods**: save, find_by_id, find_by_bic, list, list_active
**Branch Code Methods**: save, find_by_id, find_by_code, find_by_bank_bic, list, list_active
**Holiday Methods**: save, find_by_id, find_by_date_range, find_banking_holidays, is_banking_holiday
**System Parameter Methods**: save, find_by_id, find_by_key, find_by_category, list_active
**Regulatory Code Methods**: save, find_by_id, find_by_code, list, list_active
**Fee Schedule Methods**: save, find_by_id, find_by_type, list, list_active

### Error Types: ReferenceDataServiceError

```rust
CountryCodeNotFound(String)
CurrencyNotFound(String)
BankCodeNotFound(String)
BranchCodeNotFound(String)
HolidayNotFound
SystemParameterNotFound(String)
RegulatoryCodeNotFound(String)
FeeScheduleNotFound
DuplicateEntry(String)
InvalidInput(String)
DomainError(String)
Internal(String)
```

## Infrastructure Layer

### Implementation: PgReferenceDataRepository

PostgreSQL implementation of IReferenceDataRepository with:

- **8 database tables** (one per entity)
- **Comprehensive indexing** for fast lookups
- **Foreign key constraints** (branch_codes → bank_codes, fee_schedules → currency_references)
- **Row mappers** for domain conversion
- **Full CRUD operations** via sqlx with typed queries

### Database Schema

**Tables** (in schema `reference_data`):
1. `country_codes` - ISO 3166 countries
2. `currency_references` - ISO 4217 currencies
3. `bank_codes` - BIC/SWIFT codes
4. `branch_codes` - Bank branches
5. `holiday_calendar` - Banking holidays
6. `system_parameters` - Configuration parameters
7. `regulatory_codes` - BCT/IFRS classifications
8. `fee_schedule_references` - Fee schedules

**Indexes**:
- Primary lookups (id)
- Code-based lookups (iso_alpha2, iso_alpha3, bic, code, key)
- Status/activity filters (is_active, is_banking_holiday)
- Temporal queries (effective_from, effective_to)
- Relationships (bank_bic, currency_code)

## Integration Points

### Cross-Bounded Context Usage

This BC is consumed by:

1. **Account BC**:
   - Currency validation
   - Holiday calendar for settlement date calculation

2. **Payment BC**:
   - Bank codes for SEPA/SWIFT transfers
   - Fee schedules for fee calculation
   - Currency rates

3. **ForeignExchange BC**:
   - Currency references
   - Exchange rate calculations

4. **Accounting BC**:
   - Regulatory codes for classification
   - System parameters for thresholds
   - Fee schedules for posting

5. **Prudential BC**:
   - Regulatory classifications (BCT asset classes)
   - System parameters for capital calculations

6. **Compliance BC**:
   - Country sanctions flags
   - Holiday calendar for reporting

## Data Consistency

### Immutable Reference Data
- Once created, country/currency codes should not change
- Effective date ranges allow for version management

### Temporal Validity
- All data supports effective_from/effective_to
- Queries automatically filter by current date
- Allows managing future changes ahead of time

### No Cascading Deletes
- Reference data is never deleted, only deactivated
- Maintains audit trail and historical accuracy

## Testing Strategy

### Unit Tests
- Domain entity validation tests
- Value object constraint tests
- Service logic tests

### BDD Feature Tests
- Country code CRUD operations
- Currency management
- Bank and branch code lookups
- Holiday calendar queries
- System parameter retrieval
- Regulatory code classification
- Fee schedule management
- Effective date range handling
- Data consistency checks

### Integration Tests
- Database persistence
- Repository query accuracy
- Foreign key constraints
- Index performance

## Performance Considerations

- **Caching**: Reference data is stable, ideal for application-level caching
- **Indexing**: All lookup columns indexed
- **Queries**: Pagination support (limit/offset)
- **Bulk Operations**: Consider batch insert for large data loads

## Security Considerations

- **No PII**: Reference data contains no personal information
- **Read-Heavy**: Most operations are queries, not writes
- **Audit Trail**: created_at/updated_at timestamps for all records
- **Immutability**: Historical versions preserved via effective dates

## File Locations

### Domain Layer
- `/crates/domain/src/reference_data/mod.rs`
- `/crates/domain/src/reference_data/entities.rs` - 8 entities + tests
- `/crates/domain/src/reference_data/value_objects.rs` - 6 value objects + tests

### Application Layer
- `/crates/application/src/reference_data/mod.rs`
- `/crates/application/src/reference_data/service.rs` - ReferenceDataService
- `/crates/application/src/reference_data/ports.rs` - IReferenceDataRepository trait
- `/crates/application/src/reference_data/dto.rs` - Request/Response DTOs
- `/crates/application/src/reference_data/errors.rs` - Error types

### Infrastructure Layer
- `/crates/infrastructure/src/reference_data/mod.rs`
- `/crates/infrastructure/src/reference_data/repository.rs` - PgReferenceDataRepository

### Database
- `/migrations/20260407000001_reference_data_schema.sql` - 8 tables + indexes

### Tests
- `/tests/bdd/features/reference-data.feature` - 80+ BDD scenarios

## Next Steps

1. **Run migrations**: `make migrate`
2. **Load test data**: Create seed script for standard reference data
3. **API handlers**: Create REST endpoints in web layer
4. **Caching layer**: Add Redis caching for frequently accessed data
5. **Admin interface**: Dashboard for managing reference data
