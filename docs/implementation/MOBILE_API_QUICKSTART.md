# Mobile Banking API - Quick Start Guide

## 5-Minute Setup

### 1. Build the Project
```bash
cd /sessions/nice-vigilant-rubin/mnt/BANKO
make setup          # Install dependencies
make migrate        # Run database migrations (includes mobile schema)
cargo build --release  # Build project
```

### 2. Run Tests
```bash
# All mobile tests
cargo test -p banko-application -- mobile

# Specific service tests
cargo test -p banko-application mobile_auth_service
cargo test -p banko-application mobile_service
cargo test -p banko-application mobile_payment_service
```

### 3. Start Development Environment
```bash
make dev            # Start all services (backend, database, etc.)
```

The API will be available at: `http://localhost/api/v1/mobile`

## API Endpoints Overview

### Device Registration & Auth
```
POST   /devices                    - Register device
POST   /auth/login                 - Login with PIN/biometric
POST   /auth/refresh               - Refresh session
GET    /devices                    - List registered devices
POST   /devices/{id}/pin           - Set PIN
POST   /devices/{id}/biometric     - Enable biometric
DELETE /devices/{id}               - Deactivate device
```

### Account & Dashboard
```
GET    /dashboard?locale=en        - Get dashboard (locale: en/fr/ar)
GET    /offline-cache              - Get offline data
POST   /sync                       - Sync changes since last sync
```

### Payments
```
POST   /payments/transfer          - Quick transfer (auto-detect IBAN/phone)
GET    /payments/beneficiaries     - Get top 5 beneficiaries
POST   /payments/scan-qr           - Scan and parse QR code
```

## Quick Test with cURL

### 1. Register Device
```bash
curl -X POST http://localhost:8080/api/v1/mobile/devices \
  -H "Content-Type: application/json" \
  -d '{
    "device_id": "test_device_001",
    "device_name": "Test iPhone",
    "platform": "Ios"
  }'
```

Expected Response:
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "device_id": "test_device_001",
  "device_name": "Test iPhone",
  "platform": "Ios",
  "biometric_enabled": false,
  "registered_at": "2026-04-06T10:30:00Z",
  "last_active_at": "2026-04-06T10:30:00Z",
  "is_active": true
}
```

### 2. Set PIN
```bash
curl -X POST http://localhost:8080/api/v1/mobile/devices/test_device_001/pin \
  -H "Content-Type: application/json" \
  -d '{
    "pin": "1234"
  }'
```

Expected Response:
```json
{
  "message": "PIN set successfully"
}
```

### 3. Login
```bash
curl -X POST http://localhost:8080/api/v1/mobile/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "device_id": "test_device_001",
    "pin_or_biometric": "1234"
  }'
```

Expected Response:
```json
{
  "session_id": "550e8400-e29b-41d4-a716-446655440001",
  "token": "mobile_550e8400e29b41d4a716446655440002",
  "refresh_token": "refresh_550e8400e29b41d4a716446655440003",
  "expires_at": "2026-04-06T11:00:00Z"
}
```

Save the `token` for use in authenticated requests.

### 4. Get Dashboard (Requires Token)
```bash
curl -X GET 'http://localhost:8080/api/v1/mobile/dashboard?locale=en' \
  -H "Authorization: Bearer mobile_550e8400e29b41d4a716446655440002"
```

Expected Response:
```json
{
  "customer_name": "Ahmed Ben Ali",
  "greeting": "Good afternoon, Ahmed!",
  "total_balance_tnd": 15000.50,
  "accounts": [...],
  "cards": [...],
  "pending_actions": [...],
  "unread_notifications": 2
}
```

### 5. Quick Transfer
```bash
curl -X POST 'http://localhost:8080/api/v1/mobile/payments/transfer' \
  -H "Authorization: Bearer mobile_550e8400e29b41d4a716446655440002" \
  -H "Content-Type: application/json" \
  -d '{
    "from_account_id": "550e8400-e29b-41d4-a716-446655440000",
    "to_iban_or_phone": "TN5910005355869143604711",
    "amount": "500.50",
    "currency": "TND",
    "note": "Payment"
  }'
```

Expected Response:
```json
{
  "transfer_id": "550e8400-e29b-41d4-a716-446655440010",
  "status": "pending",
  "requires_2fa": false
}
```

## Key Files Location

### Source Code
```
crates/application/src/identity/mobile_auth_service.rs      - Auth logic
crates/application/src/account/mobile_service.rs            - Dashboard
crates/application/src/payment/mobile_payment_service.rs    - Payments
crates/infrastructure/src/web/handlers/mobile_handlers.rs   - HTTP handlers
```

### Database
```
migrations/20260406000026_mobile_schema.sql                 - Schema
```

### Documentation
```
docs/sprints/SPRINT_K_MOBILE_API.md                         - Detailed docs
docs/api/MOBILE_API_EXAMPLES.md                             - API examples
SPRINT_K_IMPLEMENTATION_SUMMARY.md                          - Overview
MOBILE_API_QUICKSTART.md                                    - This file
```

## Testing Checklist

- [ ] Device registration (max 5 per customer)
- [ ] PIN setup (4-6 digits)
- [ ] PIN-based login
- [ ] Biometric enablement
- [ ] Session refresh
- [ ] Device deactivation
- [ ] Dashboard retrieval (single call, all data)
- [ ] Offline cache (24-hour TTL)
- [ ] Incremental sync
- [ ] Quick transfer (IBAN auto-detect)
- [ ] 2FA threshold (> 1000 TND)
- [ ] Frequent beneficiaries
- [ ] QR code parsing

## Common Tasks

### Check Device Status
```bash
curl -X GET 'http://localhost:8080/api/v1/mobile/devices' \
  -H "Authorization: Bearer <token>"
```

### Get Offline Data
```bash
curl -X GET 'http://localhost:8080/api/v1/mobile/offline-cache' \
  -H "Authorization: Bearer <token>"
```

### Sync Changes
```bash
curl -X POST 'http://localhost:8080/api/v1/mobile/sync' \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{
    "last_sync": "2026-04-06T10:00:00Z"
  }'
```

### Get Frequent Beneficiaries
```bash
curl -X GET 'http://localhost:8080/api/v1/mobile/payments/beneficiaries' \
  -H "Authorization: Bearer <token>"
```

### Scan QR Code
```bash
curl -X POST 'http://localhost:8080/api/v1/mobile/payments/scan-qr' \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{
    "qr_data": "Ahmed|TN5910005355869143604711|500.50|TND|INV-2026-001"
  }'
```

## Architecture Overview

```
Mobile API Layer
├── Authentication (Device Registration + PIN/Biometric)
│   ├── Register devices (max 5 per customer)
│   ├── PIN validation (bcrypt hashed)
│   ├── Biometric enablement
│   └── Session management (30-min TTL)
│
├── Dashboard (Single Call Aggregation)
│   ├── Customer name + locale-aware greeting
│   ├── Total balance in TND
│   ├── Accounts with last transaction info
│   ├── Cards with daily remaining
│   ├── Pending actions
│   └── Unread notification count
│
├── Account Sync (Offline Support)
│   ├── Offline cache (24-hour TTL)
│   ├── Recent transactions
│   └── Incremental sync on reconnect
│
└── Payments (Quick Actions)
    ├── Quick transfer (auto IBAN/phone detection)
    ├── 2FA for amounts > 1000 TND
    ├── Frequent beneficiaries (top 5)
    └── QR code scanning
```

## Performance Targets

- Dashboard API: < 100ms (single aggregated call)
- Login: < 200ms
- Quick Transfer: < 500ms
- Sync: < 300ms
- QR Scan: < 100ms

## Error Codes

| Code | Meaning |
|------|---------|
| 200 | Success |
| 201 | Created |
| 400 | Bad Request (validation error) |
| 401 | Unauthorized (invalid credentials/token) |
| 403 | Forbidden (device not active) |
| 404 | Not Found (device/account/resource) |
| 409 | Conflict (device limit exceeded) |
| 500 | Internal Server Error |

## Next Steps

1. **Read Full Documentation**
   - `docs/sprints/SPRINT_K_MOBILE_API.md` - Comprehensive overview
   - `docs/api/MOBILE_API_EXAMPLES.md` - Detailed API examples

2. **Review Code Structure**
   - Authentication: `crates/application/src/identity/mobile_auth_service.rs`
   - Account: `crates/application/src/account/mobile_service.rs`
   - Payments: `crates/application/src/payment/mobile_payment_service.rs`

3. **Run Full Test Suite**
   ```bash
   make test           # All tests
   make lint           # Code quality
   make audit          # Security audit
   ```

4. **Integrate with Mobile App**
   - Follow mobile app integration checklist in `MOBILE_API_EXAMPLES.md`
   - Implement device registration on app installation
   - Secure storage for device_id and tokens

## Troubleshooting

### Database migration fails
```bash
make reset-db       # Reset database
make migrate        # Re-run migrations
```

### Tests fail
```bash
cargo test -p banko-application -- mobile --nocapture
```

### API returns 500 errors
```bash
make logs SERVICE=backend   # Check backend logs
```

### Device registration returns 409 (limit exceeded)
- User already has 5 registered devices
- Delete inactive devices: `DELETE /devices/{id}`

### Login fails with 401
- Verify PIN is correct (4-6 digits)
- Check if device is active
- Ensure device exists: `GET /devices`

## Support

For issues, questions, or contributions:
1. Check `SPRINT_K_IMPLEMENTATION_SUMMARY.md` for full context
2. Review test cases in source files for expected behavior
3. Check API examples in `docs/api/MOBILE_API_EXAMPLES.md`

---

**Last Updated**: 2026-04-06
**API Version**: v1
**Status**: Production Ready ✅
