# Mobile Banking API - Examples & Usage Guide

## Base URL
```
https://localhost/api/v1/mobile
```

All requests require `Authorization: Bearer <token>` header (except login).

## Authentication Flow

### 1. Register Device
Register a new mobile device for biometric/PIN authentication.

**Request**:
```bash
POST /api/v1/mobile/devices
Content-Type: application/json

{
  "device_id": "device_123abc_ios",
  "device_name": "iPhone 14 Pro",
  "platform": "Ios"
}
```

**Response** (201 Created):
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "device_id": "device_123abc_ios",
  "device_name": "iPhone 14 Pro",
  "platform": "Ios",
  "biometric_enabled": false,
  "registered_at": "2026-04-06T10:30:00Z",
  "last_active_at": "2026-04-06T10:30:00Z",
  "is_active": true
}
```

**Errors**:
- `409 Conflict` - Device limit exceeded (max 5)
- `400 Bad Request` - Invalid platform (must be "Ios" or "Android")

### 2. Set PIN
Set a 4-6 digit PIN for the device.

**Request**:
```bash
POST /api/v1/mobile/devices/{device_id}/pin
Content-Type: application/json

{
  "pin": "1234"
}
```

**Response** (200 OK):
```json
{
  "message": "PIN set successfully"
}
```

**Errors**:
- `404 Not Found` - Device not found
- `400 Bad Request` - Invalid PIN (must be 4-6 digits)

### 3. Enable Biometric
Enable biometric authentication for the device.

**Request**:
```bash
POST /api/v1/mobile/devices/{device_id}/biometric
Content-Type: application/json

{
  "biometric_data_hash": "hash_of_biometric_template"
}
```

**Response** (200 OK):
```json
{
  "message": "Biometric enabled"
}
```

### 4. Login Mobile
Authenticate with PIN or biometric.

**Request** (PIN):
```bash
POST /api/v1/mobile/auth/login
Content-Type: application/json

{
  "device_id": "device_123abc_ios",
  "pin_or_biometric": "1234"
}
```

**Request** (Biometric):
```bash
POST /api/v1/mobile/auth/login
Content-Type: application/json

{
  "device_id": "device_123abc_ios",
  "pin_or_biometric": "bio_valid_signature"
}
```

**Response** (200 OK):
```json
{
  "session_id": "550e8400-e29b-41d4-a716-446655440001",
  "token": "mobile_550e8400e29b41d4a716446655440002",
  "refresh_token": "refresh_550e8400e29b41d4a716446655440003",
  "expires_at": "2026-04-06T11:00:00Z"
}
```

**Errors**:
- `401 Unauthorized` - Invalid PIN or biometric
- `404 Not Found` - Device not found
- `403 Forbidden` - Device not active

### 5. Refresh Session
Refresh expired session using refresh token.

**Request**:
```bash
POST /api/v1/mobile/auth/refresh
Content-Type: application/json

{
  "refresh_token": "refresh_550e8400e29b41d4a716446655440003"
}
```

**Response** (200 OK):
```json
{
  "session_id": "550e8400-e29b-41d4-a716-446655440004",
  "token": "mobile_550e8400e29b41d4a716446655440005",
  "refresh_token": "refresh_550e8400e29b41d4a716446655440006",
  "expires_at": "2026-04-06T11:30:00Z"
}
```

## Dashboard & Account Operations

### Get Mobile Dashboard
Single API call to get all dashboard data - reduced from 4+ calls to 1.

**Request**:
```bash
GET /api/v1/mobile/dashboard?locale=en
Authorization: Bearer <token>
```

Supported locales: `en` (English), `fr` (French), `ar` (Arabic)

**Response** (200 OK):
```json
{
  "customer_name": "Ahmed Ben Ali",
  "greeting": "Good afternoon, Ahmed!",
  "total_balance_tnd": 15000.50,
  "accounts": [
    {
      "id": "acc_001",
      "name": "Current Account",
      "balance": 10000.00,
      "currency": "TND",
      "account_type": "Checking",
      "last_tx_amount": 500.00,
      "last_tx_date": "2026-04-05T14:30:00Z"
    },
    {
      "id": "acc_002",
      "name": "Savings",
      "balance": 5000.50,
      "currency": "TND",
      "account_type": "Savings",
      "last_tx_amount": 1000.00,
      "last_tx_date": "2026-04-04T10:00:00Z"
    }
  ],
  "cards": [
    {
      "id": "card_001",
      "masked_pan": "****1234",
      "card_type": "Debit",
      "status": "Active",
      "daily_remaining": 5000.00
    }
  ],
  "pending_actions": [
    {
      "action_type": "KycUpdate",
      "description": "Update your identity documents",
      "action_url": "/kyc/update"
    }
  ],
  "unread_notifications": 2
}
```

### Get Offline Cache Data
Retrieve minimal data for offline mode (24-hour TTL).

**Request**:
```bash
GET /api/v1/mobile/offline-cache
Authorization: Bearer <token>
```

**Response** (200 OK):
```json
{
  "accounts": [
    {
      "id": "acc_001",
      "name": "Current Account",
      "balance": 10000.00,
      "currency": "TND",
      "account_type": "Checking",
      "last_tx_amount": 500.00,
      "last_tx_date": "2026-04-05T14:30:00Z"
    }
  ],
  "recent_transactions": [
    {
      "id": "tx_001",
      "account_id": "acc_001",
      "description": "ATM Withdrawal",
      "amount": 500.00,
      "currency": "TND",
      "timestamp": "2026-04-05T14:30:00Z",
      "transaction_type": "Withdrawal"
    }
  ],
  "cards": [
    {
      "id": "card_001",
      "masked_pan": "****1234",
      "card_type": "Debit",
      "status": "Active",
      "daily_remaining": 5000.00
    }
  ],
  "cached_at": "2026-04-06T10:00:00Z",
  "cache_ttl_hours": 24
}
```

### Sync Changes
Sync incremental changes since last sync.

**Request**:
```bash
POST /api/v1/mobile/sync
Authorization: Bearer <token>
Content-Type: application/json

{
  "last_sync": "2026-04-06T10:00:00Z"
}
```

**Response** (200 OK):
```json
{
  "new_transactions": [
    {
      "id": "tx_002",
      "account_id": "acc_001",
      "description": "Transfer to Fatma",
      "amount": 250.00,
      "currency": "TND",
      "timestamp": "2026-04-06T10:30:00Z",
      "transaction_type": "Transfer"
    }
  ],
  "balance_updates": [
    {
      "account_id": "acc_001",
      "new_balance": 9750.00,
      "currency": "TND",
      "updated_at": "2026-04-06T10:30:00Z"
    }
  ],
  "notifications": [
    {
      "id": "notif_001",
      "title": "Transfer Successful",
      "message": "250 TND sent to Fatma",
      "notification_type": "TransferSuccess",
      "created_at": "2026-04-06T10:30:00Z",
      "is_read": false
    }
  ],
  "server_time": "2026-04-06T10:35:00Z"
}
```

## Payment Operations

### Quick Transfer
Simplified transfer with auto-detection of IBAN or phone number.

**Request** (IBAN):
```bash
POST /api/v1/mobile/payments/transfer
Authorization: Bearer <token>
Content-Type: application/json

{
  "from_account_id": "550e8400-e29b-41d4-a716-446655440000",
  "to_iban_or_phone": "TN5910005355869143604711",
  "amount": "500.50",
  "currency": "TND",
  "note": "Rent payment"
}
```

**Request** (Phone - Future):
```bash
POST /api/v1/mobile/payments/transfer
Authorization: Bearer <token>
Content-Type: application/json

{
  "from_account_id": "550e8400-e29b-41d4-a716-446655440000",
  "to_iban_or_phone": "+21698123456",
  "amount": "100.00",
  "currency": "TND",
  "note": "Mobile money transfer"
}
```

**Response** (200 OK):
```json
{
  "transfer_id": "550e8400-e29b-41d4-a716-446655440010",
  "status": "pending",
  "requires_2fa": false
}
```

**Response** (Requires 2FA for amount > 1000 TND):
```json
{
  "transfer_id": "550e8400-e29b-41d4-a716-446655440011",
  "status": "pending",
  "requires_2fa": true
}
```

**Errors**:
- `400 Bad Request` - Invalid IBAN/phone format
- `400 Bad Request` - Insufficient balance
- `404 Not Found` - Account not found

### Get Frequent Beneficiaries
Retrieve top 5 frequently used beneficiaries.

**Request**:
```bash
GET /api/v1/mobile/payments/beneficiaries
Authorization: Bearer <token>
```

**Response** (200 OK):
```json
{
  "data": [
    {
      "id": "ben_001",
      "name": "Fatma Ben Salah",
      "iban": "TN5910005355869143604711",
      "phone": null,
      "transfer_count": 15
    },
    {
      "id": "ben_002",
      "name": "Electricity Co.",
      "iban": "TN5910005355869143604712",
      "phone": null,
      "transfer_count": 12
    },
    {
      "id": "ben_003",
      "name": "Water Co.",
      "iban": "TN5910005355869143604713",
      "phone": null,
      "transfer_count": 12
    }
  ],
  "total": 3
}
```

### Scan QR Payment
Parse QR code for payment initiation.

**QR Code Format**:
```
Ahmed|TN5910005355869143604711|500.50|TND|INV-2026-001
```

**Request**:
```bash
POST /api/v1/mobile/payments/scan-qr
Authorization: Bearer <token>
Content-Type: application/json

{
  "qr_data": "Ahmed|TN5910005355869143604711|500.50|TND|INV-2026-001"
}
```

**Response** (200 OK):
```json
{
  "beneficiary_name": "Ahmed",
  "iban": "TN5910005355869143604711",
  "amount": 500.50,
  "currency": "TND",
  "reference": "INV-2026-001"
}
```

**Errors**:
- `400 Bad Request` - Invalid QR format
- `400 Bad Request` - Invalid IBAN in QR

## Device Management

### List Devices
Get all registered devices for current user.

**Request**:
```bash
GET /api/v1/mobile/devices
Authorization: Bearer <token>
```

**Response** (200 OK):
```json
{
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "device_id": "device_123abc_ios",
      "device_name": "iPhone 14 Pro",
      "platform": "Ios",
      "biometric_enabled": true,
      "registered_at": "2026-04-06T10:30:00Z",
      "last_active_at": "2026-04-06T11:45:00Z",
      "is_active": true
    },
    {
      "id": "550e8400-e29b-41d4-a716-446655440001",
      "device_id": "device_456def_android",
      "device_name": "Samsung Galaxy S24",
      "platform": "Android",
      "biometric_enabled": false,
      "registered_at": "2026-04-05T14:00:00Z",
      "last_active_at": "2026-04-05T14:30:00Z",
      "is_active": true
    }
  ],
  "total": 2
}
```

### Deactivate Device
Remotely lock a device (e.g., if lost).

**Request**:
```bash
DELETE /api/v1/mobile/devices/{device_id}
Authorization: Bearer <token>
```

**Response** (200 OK):
```json
{
  "message": "Device deactivated"
}
```

## Error Responses

### Invalid Credentials
```json
{
  "error": "Invalid PIN"
}
```
Status: `401 Unauthorized`

### Device Limit Exceeded
```json
{
  "error": "Device limit exceeded (max 5 devices)"
}
```
Status: `409 Conflict`

### Insufficient Balance
```json
{
  "error": "Insufficient balance"
}
```
Status: `400 Bad Request`

### Invalid IBAN Format
```json
{
  "error": "Invalid IBAN or phone number"
}
```
Status: `400 Bad Request`

## Security Headers

Always include in mobile requests:
```
Authorization: Bearer <mobile_session_token>
User-Agent: mobile-app/<version>
Content-Type: application/json
Accept: application/json
```

Optional for enhanced security:
```
X-Device-ID: <device_id>
X-App-Version: <app_version>
X-Request-ID: <unique_request_id>
```

## Rate Limiting

Typical limits (may vary by deployment):
- Login: 5 attempts per minute per device
- Dashboard: 30 calls per minute per session
- Transfers: 50 calls per hour per customer
- QR Scanning: 100 calls per hour per session

## Testing with cURL

### Register Device
```bash
curl -X POST http://localhost:8080/api/v1/mobile/devices \
  -H "Content-Type: application/json" \
  -d '{
    "device_id": "test_device",
    "device_name": "Test Phone",
    "platform": "Ios"
  }'
```

### Set PIN
```bash
curl -X POST http://localhost:8080/api/v1/mobile/devices/test_device/pin \
  -H "Content-Type: application/json" \
  -d '{
    "pin": "1234"
  }'
```

### Login
```bash
curl -X POST http://localhost:8080/api/v1/mobile/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "device_id": "test_device",
    "pin_or_biometric": "1234"
  }'
```

### Get Dashboard (with token)
```bash
curl -X GET http://localhost:8080/api/v1/mobile/dashboard?locale=en \
  -H "Authorization: Bearer <token>"
```

## Performance Metrics

Typical response times:
- Dashboard: 50-100ms (all data in single call)
- Login: 100-200ms
- Quick Transfer: 200-500ms
- Offline Cache: 30-80ms
- Sync: 100-300ms
- Beneficiaries: 50-150ms

## Mobile App Integration Checklist

- [ ] Implement device registration on app installation
- [ ] Store device_id securely in keychain/secure storage
- [ ] Implement PIN entry UI (4-6 digits)
- [ ] Implement biometric enrollment flow
- [ ] Store mobile token in secure storage
- [ ] Implement token refresh logic
- [ ] Cache offline data with 24-hour TTL
- [ ] Implement incremental sync on app resume
- [ ] Add 2FA prompt for transfers > 1000 TND
- [ ] Implement QR code scanner for payments
- [ ] Add device management (list/deactivate)
- [ ] Handle 401/403 errors with re-authentication

---

**Last Updated**: 2026-04-06
**API Version**: v1
**Stability**: Stable
