# EPIC-18 Quick Reference — Multi-Devise & Frais

## Story Mapping

| Story | Component | Status | Files |
|-------|-----------|--------|-------|
| MCUR-01 | Multi-Currency Support | ✅ Complete | `multi_currency.rs`, `multi_currency_service.rs` |
| MCUR-02 | Multi-Currency Transfers | ✅ Complete | `multi_currency_service.rs`, handlers |
| FEE-01 | Fee Definitions & Categories | ✅ Complete | `fees.rs`, `fee_service.rs` |
| FEE-02 | Segment-Based Pricing (Grids) | ✅ Complete | `fees.rs::FeeGrid`, `fee_service.rs` |
| FEE-03 | Monthly Fee Calculation & Charging | ✅ Complete | `fee_service.rs::calculate_monthly_fees()` |

## Architecture: Hexagonal Pattern

### Domain Layer (Pure Business Logic)
```
crates/domain/src/
├── account/
│   ├── multi_currency.rs          ← Currency, MultiCurrencyBalance, CurrencyConverter
│   └── mod.rs                     ← Updated: pub mod multi_currency
└── accounting/
    ├── fees.rs                    ← FeeCategory, FeeDefinition, FeeCharge, FeeGrid
    └── mod.rs                     ← Updated: pub mod fees
```

### Application Layer (Use Cases & Orchestration)
```
crates/application/src/
├── accounting/
│   ├── fee_service.rs             ← FeeService (orchestration logic)
│   ├── ports.rs                   ← IFeeDefinitionRepository, IFeeChargeRepository, IFeeGridRepository
│   └── mod.rs                     ← Updated: mod fee_service
└── account/
    ├── multi_currency_service.rs  ← MultiCurrencyService
    └── mod.rs                     ← Updated: mod multi_currency_service
```

### Infrastructure Layer (Adapters & Web)
```
crates/infrastructure/src/
├── accounting/
│   ├── fee_handlers.rs            ← HTTP handlers for fees
│   └── mod.rs                     ← Updated: pub mod fee_handlers
├── account/
│   ├── multi_currency_handlers.rs ← HTTP handlers for multi-currency
│   └── mod.rs                     ← Updated: pub mod multi_currency_handlers
└── database/
    └── migrations/
        └── 20260406000020_*.sql   ← Database schema
```

## Key Classes & Methods

### Currency Entity
```rust
Currency::TND | EUR | USD | GBP | SAR | AED | LYD | DZD | MAD
Currency::code() → &str                    // ISO 4217
Currency::name_fr() → &str                // French name
Currency::decimal_places() → u8           // TND=3, others=2
Currency::from_code("TND") → Option<Currency>
```

### MultiCurrencyBalance
```rust
MultiCurrencyBalance::new() → Self
balance.add(Currency::EUR, Decimal::from(100)) → Result
balance.get(&Currency::EUR) → Decimal
balance.total_in_base(&rates_map, Currency::TND) → Result<Decimal>
```

### CurrencyConverter
```rust
CurrencyConverter::new() → Self                           // 2% margin (default)
CurrencyConverter::with_margin(Decimal::from(3)) → Self // Custom margin
converter.convert(
    amount: Decimal,
    from: Currency,
    to: Currency,
    market_rate: Decimal,
    is_buying_base: bool                // true=buy (expensive), false=sell (cheap)
) → Result<ConversionResult>

CurrencyConverter::check_monthly_limit(
    customer_id,
    currency,
    amount,
    monthly_limit,
    already_converted
) → Result
```

### FeeDefinition
```rust
FeeDefinition::new(
    name, category, fixed_amount, rate_percent,
    min_amount, max_amount, condition, segments, currency
) → Result<FeeDefinition>

definition.calculate(transaction_amount: Decimal) → Decimal
// Logic: rate% → fixed → max(both) → apply min/max bounds

definition.applies_to_segment("VIP") → bool
definition.is_condition_met(balance, transaction_amount, day_of_month) → bool
```

### FeeCharge
```rust
FeeCharge::new(fee_def_id, account_id, amount, description) → Result
charge.mark_charged()
charge.mark_unpaid()
charge.mark_waived()
charge.mark_reversed()
```

### FeeGrid
```rust
FeeGrid::new(name, segment, fee_overrides, effective_from, effective_to) → FeeGrid
grid.get_fee_for_category(&FeeCategory::MonthlyAccountFee) → Option<Decimal>
grid.is_effective_at(Utc::now()) → bool
grid.deactivate()
```

## Service APIs

### FeeService
```rust
service.calculate_monthly_fees(
    account_id, product_id, balance, segment, day_of_month
) → Result<Vec<FeeCharge>>

service.charge_fees(account_id, fees) → Result<ChargeResult>
// Returns: {total_charged, total_unpaid, amount_charged, amount_unpaid}

service.list_account_fees(account_id) → Result<Vec<FeeCharge>>
service.create_fee_definition(definition) → Result<FeeDefinition>
service.get_fee_definition(id) → Result<Option<FeeDefinition>>
service.list_fee_definitions() → Result<Vec<FeeDefinition>>
service.create_fee_grid(grid) → Result<FeeGrid>
service.get_fee_grid_for_segment(segment) → Result<Option<FeeGrid>>
service.list_fee_grids() → Result<Vec<FeeGrid>>
```

### MultiCurrencyService
```rust
service.get_consolidated_balance(customer_id)
→ Result<ConsolidatedBalance>
// Returns: {balances: Vec<(Currency, Decimal)>, total_tnd, rates_used}

service.convert_between_accounts(
    from_account_id, to_account_id, amount, customer_id
) → Result<ConversionResult>

service.get_monthly_conversion_usage(customer_id, currency, month)
→ Result<Decimal>
```

## HTTP Endpoints

### Fees
```
POST   /api/v1/fees/definitions           Create fee definition
GET    /api/v1/fees/definitions           List all definitions
GET    /api/v1/fees/definitions/{id}      Get definition by ID
GET    /api/v1/fees/accounts/{account_id} List account fees
POST   /api/v1/fees/grids                 Create fee grid
GET    /api/v1/fees/grids                 List all grids
GET    /api/v1/fees/grids/{segment}       Get grid for segment
```

### Multi-Currency
```
GET    /api/v1/multi-currency/customers/{customer_id}/balance
       Get consolidated balance

POST   /api/v1/multi-currency/customers/{customer_id}/convert
       Convert between accounts
       Body: {from_account_id, to_account_id, amount}

GET    /api/v1/multi-currency/customers/{customer_id}/usage/{currency}/{month}
       Get monthly usage

GET    /api/v1/multi-currency/customers/{customer_id}/history
       Get conversion history
```

## Database Schema

### fee_definitions
```sql
id UUID PRIMARY KEY
name VARCHAR(200)
category VARCHAR(30)  -- MonthlyAccountFee, TransactionFee, etc.
fixed_amount DECIMAL(18,3)
rate_percent DECIMAL(10,6)
min_amount DECIMAL(18,3)
max_amount DECIMAL(18,3)
condition_type VARCHAR(30)  -- Always, BalanceBelow, etc.
condition_value DECIMAL(18,3)
currency VARCHAR(3)  -- TND, EUR, etc.
created_at TIMESTAMPTZ

-- Indexes:
CREATE INDEX idx_fee_definitions_category
CREATE INDEX idx_fee_definitions_currency
```

### fee_segment_applicability
```sql
fee_definition_id UUID REFERENCES fee_definitions(id)
segment VARCHAR(20)  -- VIP, Standard, Junior
PRIMARY KEY (fee_definition_id, segment)
```

### fee_charges
```sql
id UUID PRIMARY KEY
fee_definition_id UUID REFERENCES fee_definitions(id)
account_id UUID
amount DECIMAL(18,3)
status VARCHAR(20)  -- Pending, Charged, Unpaid, Waived, Reversed
charged_at TIMESTAMPTZ
description TEXT

-- Indexes:
CREATE INDEX idx_fee_charges_account
CREATE INDEX idx_fee_charges_status
CREATE INDEX idx_fee_charges_fee_def
```

### fee_grids
```sql
id UUID PRIMARY KEY
name VARCHAR(200)
segment VARCHAR(20)  -- VIP, Standard, Junior
effective_from TIMESTAMPTZ
effective_to TIMESTAMPTZ
active BOOLEAN
created_at TIMESTAMPTZ

-- Indexes:
CREATE INDEX idx_fee_grids_segment
CREATE INDEX idx_fee_grids_active
```

### currency_conversions
```sql
id UUID PRIMARY KEY
customer_id UUID
from_account_id UUID
to_account_id UUID
original_amount DECIMAL(18,3)
original_currency VARCHAR(3)
converted_amount DECIMAL(18,3)
target_currency VARCHAR(3)
market_rate DECIMAL(18,8)
bank_rate DECIMAL(18,8)
margin_applied DECIMAL(10,4)
conversion_date TIMESTAMPTZ

-- Indexes:
CREATE INDEX idx_conversions_customer
CREATE INDEX idx_conversions_date
```

## Test Coverage

### Domain Tests (Embedded)
- Run: `cargo test --lib domain::account::multi_currency`
- Run: `cargo test --lib domain::accounting::fees`

- Coverage: 27+ tests
- Areas: validation, calculations, conditions, edge cases

### Application Tests
- Run: `cargo test --lib banko_application::accounting::fee_service`
- Run: `cargo test --lib banko_application::account::multi_currency_service`

- Coverage: 12+ tests with mock repositories

## Next Implementation Steps

1. **Repository Layer** (Infrastructure):
   - Create `PgFeeDefinitionRepository` in `crates/infrastructure/src/accounting/repository.rs`
   - Implement `IFeeDefinitionRepository`, `IFeeChargeRepository`, `IFeeGridRepository`
   - Implement `currency_conversions` table access

2. **Integration**:
   - Wire FeeService into Actix-web DI container
   - Wire MultiCurrencyService into Actix-web DI container
   - Update routes.rs to register endpoints

3. **Batch Processing**:
   - Create scheduler for monthly fee calculation
   - Create job to sync exchange rates

4. **Testing**:
   - Integration tests with test database
   - End-to-end API tests
   - Load tests for monthly fee batch

## Common Operations

### Calculate Monthly Fees for an Account
```rust
let fees = fee_service.calculate_monthly_fees(
    account_id,
    product_id,
    balance,
    "VIP",      // segment
    15          // day_of_month
).await?;

// Result: Vec<FeeCharge> with Pending status

let charged = fee_service.charge_fees(account_id, fees).await?;
// Result: ChargeResult { total_charged, total_unpaid, ... }
```

### Convert Between Currencies
```rust
let result = currency_service.convert_between_accounts(
    from_account_id,
    to_account_id,
    Decimal::from(100),
    customer_id
).await?;

// Result: ConversionResult with applied bank margin
```

### Get Customer's Total Balance
```rust
let balance = currency_service.get_consolidated_balance(customer_id).await?;
// Returns all accounts converted to TND
// Example: {
//   balances: [(TND, 5000), (EUR, 100)],
//   total_tnd: 5300,
//   rates_used: [(TND, 1), (EUR, 3)]
// }
```

## Error Handling

### Domain Errors
```rust
DomainError::InvalidMovement(String)
DomainError::InsufficientFunds
DomainError::AccountClosed
```

### Service Errors
```rust
AccountingServiceError::InvalidEntry(String)
AccountingServiceError::DomainError(String)
AccountingServiceError::Internal(String)

AccountServiceError::InvalidEntry(String)
AccountServiceError::AccountNotFound
AccountServiceError::Internal(String)
```

### HTTP Responses
```
200 OK            - Success
201 Created       - Resource created
400 Bad Request   - Invalid input
404 Not Found     - Resource not found
422 Unprocessable Entity - Domain validation error
500 Internal Server Error - Unexpected error
```

## Performance Tips

- Fee definitions and grids: Cache in-memory (low change frequency)
- Monthly fee calculation: Run as batch job, not per-request
- Currency conversions: Cache exchange rates (15-60 minute TTL)
- Indexes on: account_id, status, customer_id, segment, conversion_date

## Useful Commands

```bash
# Run all domain tests
make test-unit

# Run specific test
cargo test domain::account::multi_currency::tests::test_currency_code

# Check code coverage
make coverage

# Format code
make format

# Lint code
make lint
```

---

**Last Updated**: 2026-04-06
**Implementation Status**: Complete (Domain + Application + Infrastructure Layers)
**Ready for**: Code Review → Repository Implementation → Integration Testing
