# BANKO Frontend Implementation Guide

## Overview

The BANKO frontend foundation has been built with production-quality TypeScript, Svelte 5, and Astro. This guide covers how to build features on top of this foundation.

## What's Been Created

### Type System (✅ Complete)

Four comprehensive type modules covering all major domain areas:

- **`lib/types/common.ts`** - Shared types (Money, Currency, AsyncState, etc.)
- **`lib/types/customer.ts`** - Customer types (Customer, CustomerResponse, etc.)
- **`lib/types/account.ts`** - Account types (Account, Movement, Transfer, etc.)
- **`lib/types/index.ts`** - Central export point

### API Client (✅ Enhanced)

- **`lib/api/client.ts`** - Robust typed HTTP client with error handling
  - Full CRUD support (GET, POST, PUT, PATCH, DELETE)
  - Proper error handling via `HttpError` class
  - Implements ApiError interface for type safety
  - Supports both successful responses and 204 No Content

### Reusable Components (✅ Complete)

Professional, accessible Svelte 5 components in `components/common/`:

- **StatusBadge.svelte** - Status indicators with color coding
- **KpiCard.svelte** - Metric cards with trends and variants
- **SearchBar.svelte** - Debounced search input
- **DataTable.svelte** - Sortable, filterable table with pagination-ready
- **Sidebar.svelte** - Responsive navigation with mobile drawer

### Layouts (✅ Complete)

- **DashboardLayout.astro** - Professional dashboard with sidebar, header, breadcrumbs
- **AppLayout.astro** - Basic application layout
- **Base.astro** - Root HTML layout

### Pages (✅ Implemented)

- **dashboard.astro** - Main dashboard with KPI cards, transactions, quick actions
- **customers/index.astro** - Customer list with search, filters, pagination UI
- **customers/[id].astro** - Customer detail page with accounts and actions
- **accounts/list.astro** - Account list with type/status filters
- **accounts/[id].astro** - Account detail page (stub)

### Utilities (✅ Complete)

- **`lib/utils/formatting.ts`** - Currency, date, number formatting
- **`lib/utils/validation.ts`** - Form validation for customers, accounts, transfers

### State Management (✅ Complete)

- **`stores/async.store.ts`** - Generic async state container with execute pattern
- **`stores/auth.store.ts`** - Authentication state management
- **`stores/loading.store.ts`** - Global loading state
- **`stores/toast.store.ts`** - Toast notifications

## How to Build Features

### 1. Create a New Page

**Example: Building a "New Account" page**

```
src/pages/accounts/create.astro
```

```astro
---
import DashboardLayout from '@/layouts/DashboardLayout.astro';

interface Props {
  customerId?: string;
}

const { customerId } = Astro.props;
---

<DashboardLayout
  title="Créer un compte"
  breadcrumb={[
    { label: 'Comptes', href: '/accounts' },
    { label: 'Créer un compte' }
  ]}
>
  <AccountForm {customerId} client:load />
</DashboardLayout>
```

Key patterns:
- Use `DashboardLayout` for admin pages
- Pass breadcrumb prop for navigation context
- Use Astro's static typing for props
- Add `client:load` to interactive components

### 2. Create a Form Component

**Example: `components/account/AccountForm.svelte`**

```svelte
<script lang="ts">
  import { createAsyncStore } from '@/stores/async.store';
  import { api } from '@/lib/api/client';
  import { validateAccountForm } from '@/lib/utils/validation';
  import type { CreateAccountRequest } from '@/lib/types';

  interface Props {
    customerId: string;
  }

  const { customerId }: Props = $props();

  let formData = $state({
    account_type: 'checking',
    currency: 'TND',
    initial_balance: 0,
  });

  let errors = $state<Record<string, string>>({});
  const submitStore = createAsyncStore();

  function handleValidation() {
    const validationErrors = validateAccountForm(formData);
    errors = {};
    validationErrors.forEach(err => {
      errors[err.field] = err.message;
    });
    return validationErrors.length === 0;
  }

  async function handleSubmit() {
    if (!handleValidation()) return;

    const payload: CreateAccountRequest = {
      customer_id: customerId,
      account_type: formData.account_type as any,
      currency: formData.currency,
      initial_balance: formData.initial_balance ? {
        amount: formData.initial_balance,
        currency: formData.currency as any,
      } : undefined,
    };

    await submitStore.execute(() => api.post('/accounts', payload));

    if ($submitStore.status === 'success') {
      // Navigate or show success message
      alert('Compte créé avec succès');
    }
  }
</script>

<form onsubmit|preventDefault={handleSubmit} class="space-y-6">
  <!-- Form fields here -->
</form>
```

### 3. Create a Data Table Page

**Example: Enhanced customer list with real data**

```svelte
<script lang="ts">
  import { onMount } from 'svelte';
  import { createPaginatedAsyncStore } from '@/stores/async.store';
  import { customersApi } from '@/lib/api/customers';
  import DataTable from '@/components/common/DataTable.svelte';
  import SearchBar from '@/components/common/SearchBar.svelte';
  import type { Column } from '@/components/common/DataTable.svelte';

  const customersStore = createPaginatedAsyncStore(10);

  let searchQuery = $state('');
  let sortBy = $state('created_at');

  async function loadCustomers() {
    await customersStore.execute(() =>
      customersApi.list({
        page: 1,
        limit: 10,
        search: searchQuery,
      })
    );
  }

  onMount(loadCustomers);

  const columns: Column[] = [
    { key: 'first_name', label: 'Name', sortable: true },
    { key: 'email', label: 'Email', sortable: true },
    { key: 'kyc_status', label: 'KYC Status', sortable: true },
  ];
</script>

<DataTable
  columns={columns}
  rows={$customersStore.status === 'success' ? $customersStore.data : []}
  loading={$customersStore.status === 'pending'}
  sortBy={sortBy}
  onSort={(key) => { sortBy = key; loadCustomers(); }}
  rowLink={(row) => `/customers/${row.id}`}
/>
```

### 4. Connect to Backend API

**Creating a new API module: `lib/api/payments.ts`**

```typescript
import { api } from './client';
import type { PaymentResponse, CreatePaymentRequest } from '@/lib/types';

export const paymentsApi = {
  list: (params?: { page?: number; limit?: number }) => {
    const query = new URLSearchParams();
    if (params?.page) query.set('page', String(params.page));
    if (params?.limit) query.set('limit', String(params.limit));
    return api.get(`/payments?${query}`);
  },

  create: (data: CreatePaymentRequest) =>
    api.post<PaymentResponse>('/payments', data),

  get: (id: string) =>
    api.get<PaymentResponse>(`/payments/${id}`),

  approve: (id: string) =>
    api.post(`/payments/${id}/approve`, {}),

  reject: (id: string, reason: string) =>
    api.post(`/payments/${id}/reject`, { reason }),
};
```

### 5. Add Form Validation

**Using validation utilities:**

```svelte
<script lang="ts">
  import { validateEmail, validatePhoneNumber, getFieldErrors } from '@/lib/utils/validation';

  let email = $state('');
  let phone = $state('');

  function handleBlur(field: string) {
    const isValid = field === 'email'
      ? validateEmail(email)
      : validatePhoneNumber(phone);

    if (!isValid) {
      errors[field] = `${field} invalide`;
    }
  }
</script>

<input
  type="email"
  value={email}
  onblur={() => handleBlur('email')}
  class={errors.email ? 'border-red-500' : ''}
/>
{#if errors.email}
  <span class="text-red-600 text-sm">{errors.email}</span>
{/if}
```

## Feature Checklist

When building a new feature:

- [ ] Create types in `lib/types/` if needed
- [ ] Add API functions in `lib/api/[domain].ts`
- [ ] Create Svelte components in `components/[domain]/`
- [ ] Create pages in `pages/[domain]/`
- [ ] Use DashboardLayout for authenticated pages
- [ ] Add validation using `lib/utils/validation.ts`
- [ ] Implement error handling with try-catch
- [ ] Use async stores for data fetching
- [ ] Add proper loading/error states
- [ ] Test responsive design (mobile/tablet/desktop)
- [ ] Test keyboard navigation and accessibility
- [ ] Add focus visible styles
- [ ] Handle 404 and error states

## Common Patterns

### Loading States

```svelte
{#if $store.status === 'pending'}
  <p>Chargement...</p>
{:else if $store.status === 'success'}
  <!-- Show data -->
{:else if $store.status === 'error'}
  <p>Erreur: {$store.error.message}</p>
{/if}
```

### Error Handling

```typescript
try {
  await api.post('/customers', payload);
} catch (error) {
  if (error instanceof HttpError) {
    if (error.statusCode === 409) {
      // Handle conflict (duplicate)
    } else if (error.statusCode === 400) {
      // Handle validation error
      console.log(error.details);
    }
  }
}
```

### Debounced Search

```svelte
<SearchBar
  placeholder="Rechercher..."
  debounceMs={300}
  onSearch={(query) => {
    searchQuery = query;
    loadData();
  }}
/>
```

### Format Money

```typescript
import { formatMoney } from '@/lib/utils/formatting';

const balance = { amount: 15000, currency: 'TND' as const };
const formatted = formatMoney(balance); // "15,000.000 TND"
```

## Testing

### Unit Test Example (Vitest)

```typescript
import { describe, it, expect } from 'vitest';
import { validateEmail, validateCIN } from '@/lib/utils/validation';

describe('Validation', () => {
  it('should validate email', () => {
    expect(validateEmail('test@example.com')).toBe(true);
    expect(validateEmail('invalid')).toBe(false);
  });

  it('should validate CIN', () => {
    expect(validateCIN('07645826')).toBe(true);
    expect(validateCIN('123')).toBe(false);
  });
});
```

### E2E Test Example (Playwright)

```typescript
import { test, expect } from '@playwright/test';

test('Create customer', async ({ page }) => {
  await page.goto('http://localhost:3000/customer/onboarding');
  await page.fill('input[name="first_name"]', 'Ahmed');
  await page.fill('input[name="last_name"]', 'Ben Ali');
  await page.fill('input[name="email"]', 'ahmed@example.com');
  await page.click('button[type="submit"]');
  await expect(page).toHaveURL(/\/customers\/\d+/);
});
```

## Performance Tips

1. **Use `client:visible`** for components below the fold
2. **Lazy load images** with Astro's `<Image>` component
3. **Cache API responses** in stores
4. **Debounce searches** (300ms default in SearchBar)
5. **Paginate large data sets** (use PaginatedAsyncStore)
6. **Split components** to enable code splitting

## Deployment

Frontend is deployed as a Node.js server (via `@astrojs/node`).

```bash
npm run build
npm run preview
```

The build creates an optimized production bundle in `dist/`.

## Next Steps

1. Implement remaining pages (payments, credit, AML, etc.)
2. Connect all API endpoints
3. Add comprehensive error boundaries
4. Implement toast notifications
5. Add dark mode toggle
6. Set up E2E tests with Playwright
7. Configure CI/CD pipeline
8. Add analytics tracking
9. Implement audit logging UI
10. Add advanced search/filtering

---

For questions about architecture, see `FRONTEND.md`. For questions about the backend, see `../CLAUDE.md`.
