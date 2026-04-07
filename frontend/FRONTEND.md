# BANKO Frontend Architecture

Professional banking dashboard built with **Astro 4.x + Svelte 5 + Tailwind CSS + TypeScript**.

## Project Structure

```
src/
├── components/
│   ├── common/              # Reusable UI components
│   │   ├── DataTable.svelte      # Sortable/filterable table
│   │   ├── KpiCard.svelte        # KPI metric card
│   │   ├── SearchBar.svelte      # Search with debounce
│   │   ├── StatusBadge.svelte    # Status indicator
│   │   └── Sidebar.svelte        # Navigation sidebar
│   ├── account/             # Account-specific components
│   ├── customer/            # Customer-specific components
│   ├── dashboard/           # Dashboard components
│   ├── auth/                # Authentication components
│   ├── aml/                 # AML-specific components
│   ├── audit/               # Audit-specific components
│   └── credit/              # Credit-specific components
├── layouts/
│   ├── Base.astro           # Root HTML layout
│   ├── AppLayout.astro      # Basic app layout
│   └── DashboardLayout.astro # Dashboard layout (sidebar + header)
├── lib/
│   ├── api/
│   │   ├── client.ts            # Base HTTP client with error handling
│   │   ├── customers.ts         # Customer endpoints
│   │   ├── accounts.ts          # Account endpoints
│   │   ├── auth.ts              # Authentication endpoints
│   │   ├── payments.ts          # Payment endpoints
│   │   ├── credit.ts            # Credit endpoints
│   │   ├── aml.ts               # AML endpoints
│   │   ├── audit.ts             # Audit endpoints
│   │   └── prudential.ts        # Prudential endpoints
│   ├── types/
│   │   ├── common.ts            # Common types (Money, Currency, etc)
│   │   ├── customer.ts          # Customer types
│   │   ├── account.ts           # Account types
│   │   └── index.ts             # Export all types
│   ├── utils/
│   │   └── formatting.ts        # Formatting utilities
│   └── i18n/
│       ├── i18n.ts              # i18n setup
│       └── types.ts             # Translation types
├── stores/
│   ├── auth.store.ts        # Authentication state
│   ├── async.store.ts       # Async operation state
│   ├── loading.store.ts     # Loading state
│   └── toast.store.ts       # Toast notifications
├── pages/
│   ├── index.astro          # Home page
│   ├── login.astro          # Login page
│   ├── register.astro       # Registration page
│   ├── dashboard.astro      # Main dashboard
│   ├── customers/
│   │   ├── index.astro      # Customer list
│   │   └── [id].astro       # Customer detail
│   ├── accounts/
│   │   ├── index.astro      # Account list
│   │   ├── list.astro       # Alternative list view
│   │   └── [id].astro       # Account detail
│   ├── payments/
│   │   └── index.astro      # Payment dashboard
│   ├── credit/
│   │   └── index.astro      # Credit dashboard
│   ├── aml/
│   │   └── index.astro      # AML dashboard
│   ├── audit/
│   │   └── log.astro        # Audit logs
│   └── dashboards/
│       └── risk.astro       # Risk dashboard
└── env.d.ts                 # Astro type definitions
```

## Type System

### Common Types (`lib/types/common.ts`)

```typescript
type Currency = 'TND' | 'EUR' | 'USD' | 'GBP';

interface Money {
  amount: number;
  currency: Currency;
}

interface PaginatedResponse<T> {
  data: T[];
  total: number;
  page: number;
  limit: number;
  total_pages: number;
}

type AsyncState<T> =
  | { status: 'idle' }
  | { status: 'pending' }
  | { status: 'success'; data: T }
  | { status: 'error'; error: ApiError };
```

### Customer Types (`lib/types/customer.ts`)

```typescript
interface CustomerResponse {
  id: string;
  first_name: string;
  last_name: string;
  email: string;
  date_of_birth: string;
  gender: Gender;
  cin: string;
  nationality: string;
  kyc_status: KycStatus;
  customer_type: CustomerType;
  created_at: string;
  updated_at: string;
}
```

### Account Types (`lib/types/account.ts`)

```typescript
interface AccountResponse {
  id: string;
  customer_id: string;
  account_type: AccountType;
  account_number: string;
  iban: string;
  bic: string;
  balance: Money;
  available_balance: Money;
  status: AccountStatus;
  currency: string;
  opened_at: string;
  closed_at: string | null;
}
```

## API Client

### Typed HTTP Client (`lib/api/client.ts`)

```typescript
import { api, HttpError } from '@/lib/api/client';

// GET request
const customer = await api.get<CustomerResponse>('/customers/123');

// POST request
const newCustomer = await api.post<CustomerResponse>('/customers', {
  first_name: 'Ahmed',
  last_name: 'Ben Ali',
  // ...
});

// PUT request
await api.put('/customers/123', { first_name: 'Mohamed' });

// PATCH request
await api.patch('/customers/123', { email: 'new@example.com' });

// DELETE request
await api.delete('/customers/123');

// Error handling
try {
  await api.get('/customers/invalid');
} catch (error) {
  if (error instanceof HttpError) {
    console.log(error.statusCode);
    console.log(error.code);
    console.log(error.details);
  }
}
```

### Feature-specific API modules

Each bounded context has its own module:

- `customers.ts` - Customer management
- `accounts.ts` - Account operations
- `payments.ts` - Payment operations
- `credit.ts` - Credit management
- `aml.ts` - AML monitoring
- `audit.ts` - Audit logs
- `auth.ts` - Authentication

## Components

### StatusBadge Component

```svelte
<StatusBadge
  client:load
  status="approved"
  label="Custom Label"
/>
```

Supports statuses: `active`, `suspended`, `closed`, `pending`, `submitted`, `approved`, `rejected`, `clear`, `warning`, `alert`, `blocked`, `completed`, `failed`, `reversed`, `compliant`, `breach`.

### KpiCard Component

```svelte
<KpiCard
  client:load
  title="Total Clients"
  value="2,847"
  icon="👥"
  trend={{ value: 12.5, isPositive: true }}
  variant="info"
/>
```

Variants: `default`, `success`, `warning`, `danger`, `info`.

### SearchBar Component

```svelte
<SearchBar
  client:load
  placeholder="Search..."
  debounceMs={300}
  onSearch={(query) => console.log(query)}
/>
```

### DataTable Component

```svelte
<script>
  import DataTable from '@/components/common/DataTable.svelte';
  import type { Column } from '@/components/common/DataTable.svelte';

  const columns: Column<Customer>[] = [
    { key: 'first_name', label: 'First Name', sortable: true },
    { key: 'email', label: 'Email', sortable: true },
    { key: 'kyc_status', label: 'KYC Status', sortable: true },
  ];

  const rows: Customer[] = [...];
</script>

<DataTable
  client:load
  {columns}
  {rows}
  loading={false}
  sortBy="first_name"
  sortOrder="asc"
  onSort={(key) => console.log('Sort by', key)}
  rowLink={(row) => `/customers/${row.id}`}
/>
```

## State Management

### Async Store

```typescript
import { createAsyncStore } from '@/stores/async.store';

const customerStore = createAsyncStore<CustomerResponse>();

// Execute async operation
await customerStore.execute(async () => {
  return api.get<CustomerResponse>('/customers/123');
});

// Subscribe to state
customerStore.subscribe(state => {
  if (state.status === 'success') {
    console.log(state.data);
  } else if (state.status === 'error') {
    console.log(state.error.message);
  }
});
```

### Paginated Async Store

```typescript
const customersStore = createPaginatedAsyncStore<CustomerResponse>(10);

await customersStore.execute(async () => {
  const response = await api.get<CustomerListResponse>('/customers?page=1&limit=10');
  return {
    data: response.data,
    total: response.total,
    page: response.page,
  };
});

// Access metadata
customersStore.metadata.subscribe(meta => {
  console.log(`Page ${meta.page} of ${Math.ceil(meta.total / meta.pageSize)}`);
});
```

### Authentication Store

```typescript
import { authStore } from '@/stores/auth.store';

// Login
await authStore.login('email@example.com', 'password');

// Check if authenticated
authStore.isAuthenticated.subscribe(isAuth => {
  console.log('Is authenticated:', isAuth);
});

// Logout
await authStore.logout();
```

## Formatting Utilities

```typescript
import {
  formatMoney,
  formatCurrency,
  formatDate,
  formatDateTime,
  formatTime,
  formatPercentage,
  formatAccountNumber,
  formatPhoneNumber,
  formatFullName,
  truncateString,
  capitalizeFirstLetter,
  relativeDateFromNow,
} from '@/lib/utils/formatting';

// Money formatting
formatMoney({ amount: 15000, currency: 'TND' }); // "15,000.000 TND"
formatCurrency(15000, 'EUR'); // "€15,000.00"

// Date formatting
formatDate('2026-04-07T10:30:00Z'); // "7 avril 2026"
formatDateTime('2026-04-07T10:30:00Z'); // "7 avr. 2026 10:30"
formatTime('2026-04-07T10:30:00Z'); // "10:30:00"
relativeDateFromNow('2026-04-07T10:30:00Z'); // "il y a X minutes"

// String formatting
formatAccountNumber('FR76000000000000000000000001'); // "...0001"
formatPhoneNumber('21620123456'); // "+216 201 234 56"
formatFullName('Ahmed', 'Ben Ali'); // "Ahmed Ben Ali"
truncateString('Long text...', 10); // "Long te..."
```

## Layouts

### Base Layout (`layouts/Base.astro`)

Root HTML layout with meta tags, stylesheets, and scripts. All pages extend this.

### Dashboard Layout (`layouts/DashboardLayout.astro`)

Professional dashboard layout with:
- Sidebar navigation with collapsible sections
- Top header with breadcrumbs, notifications, user menu
- Responsive design (mobile sidebar drawer)
- Dark/light mode support
- Footer

Usage:

```astro
---
import DashboardLayout from '@/layouts/DashboardLayout.astro';
---

<DashboardLayout
  title="Page Title"
  breadcrumb={[
    { label: 'Customers', href: '/customers' },
    { label: 'John Doe' }
  ]}
>
  <!-- Content here -->
</DashboardLayout>
```

## i18n Support

The frontend supports 3 languages: French (`fr`), Arabic (`ar`), English (`en`).

Configuration in `astro.config.mjs`:

```javascript
i18n: {
  defaultLocale: "fr",
  locales: ["fr", "ar", "en"],
}
```

## Development

### Environment Variables

Create `.env.local`:

```
PUBLIC_API_URL=http://localhost:8080/api/v1
```

### Commands

```bash
# Development
npm run dev

# Build
npm run build

# Preview production build
npm run preview

# Format code
npm run format

# Lint
npm run lint
```

## Best Practices

### Component Organization

1. **Reusable components** go in `components/common/`
2. **Domain-specific components** go in subdirectories (`account/`, `customer/`, etc)
3. **One component per file**
4. **Svelte 5 rune syntax**: Use `$state()`, `$derived()`, `$effect()` instead of stores for local component state

### Type Safety

1. **Always type props** with `interface Props`
2. **Use discriminated unions** for complex state
3. **Import types from `@/lib/types`**
4. **Avoid `any`** - use generics instead

### Error Handling

```typescript
try {
  const customer = await api.get<CustomerResponse>('/customers/123');
} catch (error) {
  if (error instanceof HttpError) {
    if (error.statusCode === 404) {
      // Handle not found
    } else if (error.statusCode === 401) {
      // Handle unauthorized
    }
  }
}
```

### Data Fetching

Use the async stores for data fetching:

```svelte
<script>
  import { onMount } from 'svelte';
  import { createAsyncStore } from '@/stores/async.store';
  import { customersApi } from '@/lib/api/customers';

  const customersStore = createAsyncStore();

  onMount(async () => {
    await customersStore.execute(() => customersApi.list());
  });
</script>

{#if $customersStore.status === 'pending'}
  <p>Loading...</p>
{:else if $customersStore.status === 'success'}
  {#each $customersStore.data as customer}
    <p>{customer.first_name} {customer.last_name}</p>
  {/each}
{:else if $customersStore.status === 'error'}
  <p>Error: {$customersStore.error.message}</p>
{/if}
```

## Performance

- **Code splitting**: Astro automatically splits code by route
- **Lazy loading**: Use `client:load` or `client:visible` for Svelte components
- **Image optimization**: Use Astro's `<Image>` component for images
- **Caching**: API responses cached in stores
- **Debouncing**: SearchBar debounces input by default (300ms)

## Accessibility (a11y)

- All interactive elements have proper ARIA labels
- Keyboard navigation supported on all components
- Focus visible states on buttons and links
- Semantic HTML structure
- Color contrast meets WCAG AA standards
- Screen reader support

## Styling

- **Tailwind CSS** for utility-first styling
- **Dark mode**: Supported via Tailwind's `dark:` prefix
- **Responsive design**: Mobile-first approach
- **Typography**: Uses Tailwind's typography plugin for readable text

## Contributing

### Adding a New Page

1. Create file in `src/pages/`
2. Use appropriate layout (Base, AppLayout, or DashboardLayout)
3. Add TypeScript types
4. Use API client for data fetching
5. Test responsive design

### Adding a New Component

1. Create file in appropriate `components/` subdirectory
2. Define `Props` interface
3. Use Svelte 5 runes (`$state`, `$derived`, `$effect`)
4. Add JSDoc comments
5. Use consistent styling with Tailwind

### API Integration

1. Add types in `lib/types/`
2. Create API functions in `lib/api/[domain].ts`
3. Use typed API client
4. Export from API module
5. Handle errors properly

---

**For more information, see the main CLAUDE.md file.**
