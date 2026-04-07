# EPIC-15 Product Catalogue API Endpoints

## Overview
Complete REST API specification for BANKO's Product Catalogue system (EPIC-15).

---

## Product Management Endpoints

### Create Product
**Endpoint**: `POST /api/v1/products`
**Auth**: Required (AuthenticatedUser)
**Status**: 201 Created

**Request Body**:
```json
{
  "name": "Premium Savings Account",
  "product_type": "SavingsAccount",
  "interest_rate": {
    "annual_rate": 4.5,
    "calc_method": "Compound",
    "floor_rate": 0.5,
    "ceiling_rate": 10.0
  },
  "fees": [
    {
      "fee_type": "Monthly",
      "fixed_amount": 5.00,
      "rate": null,
      "min_amount": null,
      "max_amount": null,
      "charged_on": 1
    }
  ],
  "eligibility": {
    "min_age": 18,
    "max_age": null,
    "min_income": 30000.00,
    "required_segment": "Premium",
    "min_credit_score": 700
  },
  "segment_pricing": {
    "VIP": 5.5,
    "Premium": 4.8
  },
  "min_balance": 5000.00,
  "currency": "TND"
}
```

**Response** (201 Created):
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "Premium Savings Account",
  "product_type": "SavingsAccount",
  "status": "Draft",
  "interest_rate": {
    "annual_rate": 4.5,
    "calc_method": "Compound",
    "floor_rate": 0.5,
    "ceiling_rate": 10.0
  },
  "fees": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440001",
      "fee_type": "Monthly",
      "fixed_amount": 5.00,
      "rate": null,
      "min_amount": null,
      "max_amount": null,
      "charged_on": 1
    }
  ],
  "eligibility": {
    "min_age": 18,
    "max_age": null,
    "min_income": 30000.00,
    "required_segment": "Premium",
    "min_credit_score": 700
  },
  "segment_pricing": {
    "VIP": "5.5",
    "Premium": "4.8"
  },
  "min_balance": 5000.00,
  "currency": "TND",
  "version": 1,
  "created_at": "2026-04-06T15:30:00Z",
  "updated_at": "2026-04-06T15:30:00Z"
}
```

---

### Get Product by ID
**Endpoint**: `GET /api/v1/products/{id}`
**Auth**: Required
**Status**: 200 OK | 404 Not Found

**Response** (200 OK):
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "Premium Savings Account",
  "product_type": "SavingsAccount",
  "status": "Draft",
  ...
}
```

---

### List All Products
**Endpoint**: `GET /api/v1/products`
**Auth**: Required
**Status**: 200 OK

**Response** (200 OK):
```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "Premium Savings Account",
    "product_type": "SavingsAccount",
    "status": "Draft",
    ...
  },
  {
    "id": "550e8400-e29b-41d4-a716-446655440001",
    "name": "Business Current Account",
    "product_type": "CurrentAccount",
    "status": "Active",
    ...
  }
]
```

---

### Activate Product
**Endpoint**: `POST /api/v1/products/{id}/activate`
**Auth**: Required
**Status**: 200 OK | 400 Bad Request | 404 Not Found

**Request Body**: (empty)

**Response** (200 OK):
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "Premium Savings Account",
  "product_type": "SavingsAccount",
  "status": "Active",
  "version": 2,
  ...
}
```

**Error Response** (400 Bad Request):
```json
{
  "error": "Domain error: Product is already active"
}
```

---

### Suspend Product
**Endpoint**: `POST /api/v1/products/{id}/suspend`
**Auth**: Required
**Status**: 200 OK | 400 Bad Request | 404 Not Found

**Response** (200 OK):
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "Suspended",
  "version": 3,
  ...
}
```

---

## Pricing Endpoints

### Calculate Price Quote
**Endpoint**: `POST /api/v1/products/pricing/calculate`
**Auth**: Required
**Status**: 200 OK | 400 Bad Request | 404 Not Found

**Request Body**:
```json
{
  "product_id": "550e8400-e29b-41d4-a716-446655440000",
  "customer_segment": "Premium",
  "amount": 50000.00
}
```

**Response** (200 OK):
```json
{
  "product_id": "550e8400-e29b-41d4-a716-446655440000",
  "rate": 4.8,
  "fees": 5.00,
  "total_cost": 5.00,
  "segment_applied": "Premium",
  "currency": "TND"
}
```

**Notes**:
- `rate`: Annual interest rate (from segment override or default)
- `fees`: Total monthly/transaction fees for the amount
- `total_cost`: Fee amount (rate is informational)
- `segment_applied`: Which segment's pricing was used

---

## Eligibility Endpoints

### Check Eligibility for Product
**Endpoint**: `POST /api/v1/products/eligibility/check`
**Auth**: Required
**Status**: 200 OK | 400 Bad Request | 404 Not Found

**Request Body**:
```json
{
  "product_id": "550e8400-e29b-41d4-a716-446655440000",
  "age": 35,
  "income": 75000.00,
  "segment": "Premium",
  "credit_score": 750
}
```

**Response** (200 OK - Eligible):
```json
{
  "eligible": true,
  "product_id": "550e8400-e29b-41d4-a716-446655440000",
  "reasons": []
}
```

**Response** (200 OK - Not Eligible):
```json
{
  "eligible": false,
  "product_id": "550e8400-e29b-41d4-a716-446655440000",
  "reasons": [
    "Minimum income requirement not met: 30000 TND required",
    "Minimum credit score not met: 700 required"
  ]
}
```

---

### Get Eligible Products
**Endpoint**: `POST /api/v1/products/eligibility/eligible`
**Auth**: Required
**Status**: 200 OK | 400 Bad Request

**Request Body**:
```json
{
  "age": 28,
  "income": 45000.00,
  "segment": "Standard",
  "credit_score": 680
}
```

**Response** (200 OK):
```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "Standard Savings Account",
    "product_type": "SavingsAccount",
    "status": "Active",
    ...
  },
  {
    "id": "550e8400-e29b-41d4-a716-446655440001",
    "name": "Personal Loan",
    "product_type": "ConsumerLoan",
    "status": "Active",
    ...
  }
]
```

---

## Interest Calculation Endpoints

### Calculate Daily Interest
**Endpoint**: `POST /api/v1/products/interest/daily`
**Auth**: Required
**Status**: 200 OK | 400 Bad Request

**Request Body**:
```json
{
  "account_balance": 50000.00,
  "annual_rate": 4.5,
  "calc_method": "Daily"
}
```

**Response** (200 OK):
```json
{
  "account_balance": 50000.00,
  "annual_rate": 4.5,
  "calc_method": "Daily",
  "daily_interest": 6.164
}
```

**Formula**: `principal * (annual_rate / 365 / 100)`

---

### Calculate Term Deposit Maturity
**Endpoint**: `POST /api/v1/products/interest/maturity`
**Auth**: Required
**Status**: 200 OK | 400 Bad Request

**Request Body**:
```json
{
  "principal": 100000.00,
  "annual_rate": 5.0,
  "months": 12,
  "currency": "TND"
}
```

**Response** (200 OK):
```json
{
  "principal": 100000.00,
  "total_interest": 5120.75,
  "final_amount": 105120.75,
  "currency": "TND"
}
```

**Formula**: Compound interest with monthly compounding
`final_amount = principal * (1 + monthly_rate)^months`

---

## Admin Endpoints

### Create Pricing Grid
**Endpoint**: `POST /api/v1/admin/pricing-grids`
**Auth**: Required (Admin role)
**Status**: 201 Created | 400 Bad Request | 404 Not Found

**Request Body**:
```json
{
  "product_id": "550e8400-e29b-41d4-a716-446655440000",
  "bands": [
    {
      "min_amount": 0.00,
      "max_amount": 10000.00,
      "rate": 3.5,
      "fees_override": null,
      "sort_order": 0
    },
    {
      "min_amount": 10000.00,
      "max_amount": 50000.00,
      "rate": 4.0,
      "fees_override": null,
      "sort_order": 1
    },
    {
      "min_amount": 50000.00,
      "max_amount": null,
      "rate": 4.5,
      "fees_override": null,
      "sort_order": 2
    }
  ],
  "effective_from": "2026-04-06T00:00:00Z",
  "effective_to": null
}
```

**Response** (201 Created):
```json
{
  "id": "660f9500-f40c-52e5-b827-557766551111",
  "product_id": "550e8400-e29b-41d4-a716-446655440000",
  "bands": [
    {
      "id": "660f9500-f40c-52e5-b827-557766551112",
      "min_amount": 0.00,
      "max_amount": 10000.00,
      "rate": 3.5,
      "fees_override": null,
      "sort_order": 0
    },
    ...
  ],
  "effective_from": "2026-04-06T00:00:00Z",
  "effective_to": null,
  "active": true,
  "created_by": "admin-user-id",
  "created_at": "2026-04-06T15:30:00Z"
}
```

---

## Error Responses

### 400 Bad Request
```json
{
  "error": "Invalid input: Unknown product type: InvalidType"
}
```

### 404 Not Found
```json
{
  "error": "Product not found"
}
```

### 500 Internal Server Error
```json
{
  "error": "Repository error: Database connection failed"
}
```

---

## Status Codes Summary

| Code | Meaning | Scenarios |
|------|---------|-----------|
| 200 | OK | Successful GET, POST status changes |
| 201 | Created | Product/grid created successfully |
| 400 | Bad Request | Invalid input, validation errors, domain errors |
| 404 | Not Found | Product or grid not found |
| 500 | Internal Server Error | Database/system errors |

---

## Authentication & Authorization

- All endpoints require valid JWT token in Authorization header: `Authorization: Bearer {token}`
- Token provided during login (`POST /api/v1/auth/login`)
- Token contains user info and roles
- Admin endpoints check for admin role

---

## Rate Limiting & Performance

- Daily interest calculation: sub-millisecond (arithmetic operation)
- Maturity calculation: sub-millisecond (arithmetic operation)
- Eligibility check: O(eligibility rules) - typically < 1ms
- Price calculation: O(pricing bands) - typically < 5ms
- Product lookups: O(1) with in-memory storage, O(log n) with indexes

Target P99 latency: < 5ms for all endpoints (as per BANKO requirements)

---

## Example Workflows

### Workflow 1: Create and Activate a Product
```
1. POST /api/v1/products (create in Draft status)
2. POST /api/v1/products/{id}/activate (move to Active)
3. POST /api/v1/admin/pricing-grids (create pricing grid)
4. POST /api/v1/products/pricing/calculate (get quote for customer)
```

### Workflow 2: Customer Self-Service Eligibility Check
```
1. POST /api/v1/products/eligibility/eligible (find products I can open)
2. GET /api/v1/products/{id} (view product details)
3. POST /api/v1/products/pricing/calculate (see fees/rates)
```

### Workflow 3: Interest Accrual Calculation
```
1. For each account:
   GET /api/v1/products/{product_id} (get annual rate)
   POST /api/v1/products/interest/daily (calculate daily accrual)
   INSERT into interest_accruals table (persist)
```

---

## Data Types

- **Decimal**: High-precision monetary amounts (DECIMAL(18,3) in DB, Decimal in Rust)
- **UUID**: All IDs (Uuid v4)
- **DateTime**: ISO 8601 format with UTC timezone (TIMESTAMPTZ in DB)
- **Enum strings**: lowercase or PascalCase as specified per type

---

## Versioning & Concurrency

- Products include `version` field for optimistic locking
- When updating, include expected version (future enhancement)
- If version mismatch, return 409 Conflict with new version (future)

---

## References

- Full domain model: `/crates/domain/src/product/entities.rs`
- Service implementation: `/crates/application/src/product/service.rs`
- Database schema: `/migrations/20260406000018_product_catalog_schema.sql`
- Handler code: `/crates/infrastructure/src/web/handlers/product_handlers.rs`
