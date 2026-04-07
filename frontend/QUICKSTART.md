# BANKO Frontend - Quick Start

## 5-Minute Setup

### 1. Install Dependencies

```bash
cd frontend
npm install
```

### 2. Start Development Server

```bash
npm run dev
```

Open http://localhost:3000 - you'll see the landing page.

### 3. Navigate to Dashboard

Click any link or go to:
- Dashboard: http://localhost:3000/dashboard
- Customers: http://localhost:3000/customers
- Accounts: http://localhost:3000/accounts/list

## Project Structure (Essential)

```
frontend/
├── src/
│   ├── pages/          ← URL routes (each .astro = URL)
│   ├── components/     ← Reusable UI elements
│   ├── layouts/        ← Page wrappers (sidebar, header)
│   ├── lib/
│   │   ├── api/        ← Backend API calls
│   │   ├── types/      ← TypeScript interfaces
│   │   └── utils/      ← Formatting, validation
│   └── stores/         ← Shared state (auth, data)
├── astro.config.mjs    ← Framework config
├── tsconfig.json       ← TypeScript config
└── tailwind.config.mjs ← Styling config
```

## Key Concepts (Understand These First)

### 1. Pages = URLs

```
src/pages/customers/index.astro  →  /customers
src/pages/customers/[id].astro   →  /customers/123
src/pages/accounts/create.astro  →  /accounts/create
```

### 2. Components Are Svelte Files

```svelte
<script lang="ts">
  let count = $state(0);  // Svelte 5 rune
</script>

<button onclick={() => count++}>{count}</button>
```

### 3. Types First

```typescript
// Define types in lib/types/
interface Customer {
  id: string;
  first_name: string;
  email: string;
}

// Use in components
const customer: Customer = { /* ... */ };
```

### 4. API Calls

```typescript
import { api } from '@/lib/api/client';

// GET
const customer = await api.get('/customers/123');

// POST
const newCustomer = await api.post('/customers', { /* ... */ });
```

## Building a Feature (10 Minutes)

### Example: New Account Form

**Step 1: Define Types** (`lib/types/account.ts`)

```typescript
export interface CreateAccountRequest {
  customer_id: string;
  account_type: 'checking' | 'savings';
  currency: string;
}
```

**Step 2: Create Component** (`components/account/AccountForm.svelte`)

```svelte
<script lang="ts">
  import { api } from '@/lib/api/client';
  import type { CreateAccountRequest } from '@/lib/types';

  let formData = $state({
    customer_id: '',
    account_type: 'checking',
    currency: 'TND',
  });

  async function handleSubmit() {
    const response = await api.post('/accounts', formData);
    console.log('Account created:', response);
  }
</script>

<form onsubmit|preventDefault={handleSubmit}>
  <input bind:value={formData.customer_id} placeholder="Customer ID" />
  <select bind:value={formData.account_type}>
    <option value="checking">Checking</option>
    <option value="savings">Savings</option>
  </select>
  <button type="submit">Create Account</button>
</form>
```

**Step 3: Create Page** (`pages/accounts/create.astro`)

```astro
---
import DashboardLayout from '@/layouts/DashboardLayout.astro';
import AccountForm from '@/components/account/AccountForm.svelte';
---

<DashboardLayout title="Create Account">
  <AccountForm client:load />
</DashboardLayout>
```

**Step 4: Visit** http://localhost:3000/accounts/create

## Common Tasks

### Display Data in a Table

```svelte
<script lang="ts">
  import DataTable from '@/components/common/DataTable.svelte';
  import type { Column } from '@/components/common/DataTable.svelte';

  interface Row {
    id: string;
    name: string;
    email: string;
  }

  const columns: Column<Row>[] = [
    { key: 'name', label: 'Name', sortable: true },
    { key: 'email', label: 'Email', sortable: true },
  ];

  const rows: Row[] = [
    { id: '1', name: 'Ahmed', email: 'ahmed@example.com' },
  ];
</script>

<DataTable
  {columns}
  {rows}
  rowLink={(row) => `/customers/${row.id}`}
/>
```

### Show Loading State

```svelte
<script lang="ts">
  import { createAsyncStore } from '@/stores/async.store';
  import { api } from '@/lib/api/client';

  const store = createAsyncStore();

  async function load() {
    await store.execute(() => api.get('/customers'));
  }
</script>

{#if $store.status === 'pending'}
  <p>Loading...</p>
{:else if $store.status === 'success'}
  <p>Data: {JSON.stringify($store.data)}</p>
{:else if $store.status === 'error'}
  <p>Error: {$store.error.message}</p>
{/if}

<button onclick={load}>Load</button>
```

### Format Money

```svelte
<script>
  import { formatMoney, formatDate } from '@/lib/utils/formatting';
</script>

<p>{formatMoney({ amount: 15000, currency: 'TND' })}</p>
<!-- Output: "15,000.000 TND" -->

<p>{formatDate('2026-04-07T10:30:00Z')}</p>
<!-- Output: "7 avril 2026" -->
```

### Validate Form

```svelte
<script>
  import { validateEmail, validateCustomerForm } from '@/lib/utils/validation';

  const formData = { first_name: 'Ahmed', /* ... */ };
  const errors = validateCustomerForm(formData);

  if (errors.length > 0) {
    console.log('Validation failed:', errors);
  }
</script>
```

## File Structure Patterns

### Add a New Domain (e.g., Payments)

```
src/
├── lib/
│   ├── types/
│   │   └── payment.ts          ← Add payment types
│   └── api/
│       └── payments.ts         ← Add payment API functions
├── components/
│   └── payment/
│       ├── PaymentForm.svelte  ← Create payment form
│       └── PaymentList.svelte  ← Display payments
└── pages/
    └── payments/
        ├── index.astro         ← List view
        ├── create.astro        ← Creation form
        └── [id].astro          ← Detail view
```

## Environment Variables

Create `.env.local`:

```
PUBLIC_API_URL=http://localhost:8080/api/v1
```

This is automatically accessible in all files via `import.meta.env.PUBLIC_API_URL`.

## Commands

```bash
# Development
npm run dev              # Start dev server (http://localhost:3000)

# Building
npm run build            # Create production build
npm run preview          # Preview production build

# Code quality
npm run format           # Format code (Prettier)
npm run lint             # Check code style (Prettier)
```

## Troubleshooting

### "Module not found" errors

Make sure imports use the `@/` alias:

```typescript
// ✅ Good
import { api } from '@/lib/api/client';

// ❌ Bad
import { api } from '../../lib/api/client';
```

### Svelte component not updating

Use Svelte 5 runes instead of reactive statements:

```svelte
<!-- ✅ Good -->
<script>
  let count = $state(0);
</script>

<!-- ❌ Bad -->
<script>
  let count = 0;
  $: console.log(count);  // Old Svelte syntax
</script>
```

### TypeScript errors

Check that:
1. All functions have return type annotations
2. No `any` types (use generics instead)
3. Props are in a typed `interface Props`
4. Use `type` from `@/lib/types/`

### API calls not working

Check:
1. Is backend running on `localhost:8080`?
2. Is CORS enabled on backend?
3. Check browser console for errors
4. Test API manually: `curl http://localhost:8080/api/v1/customers`

## Next Steps

1. **Explore existing pages**: Look at `/src/pages/` to see patterns
2. **Review components**: Check `/src/components/common/` for reusable UI
3. **Connect backend**: Replace sample data with real API calls
4. **Read full docs**: See `FRONTEND.md` for complete architecture
5. **Build features**: Follow `IMPLEMENTATION_GUIDE.md` for step-by-step examples

## Documentation Map

| Document | For What |
|----------|----------|
| **QUICKSTART.md** (this file) | Getting started, quick reference |
| **FRONTEND.md** | Architecture, types, components, patterns |
| **IMPLEMENTATION_GUIDE.md** | Building features, examples, best practices |
| **BUILD_MANIFEST.md** | Complete file inventory, what was built |

## Key Files to Know

```
src/lib/api/client.ts          ← Core HTTP client (GET, POST, PUT, DELETE)
src/lib/types/                 ← All TypeScript interfaces
src/components/common/          ← Reusable components (Table, Badge, etc)
src/layouts/DashboardLayout.astro ← Dashboard layout (sidebar, header)
src/pages/dashboard.astro      ← Example: full dashboard page
```

## Pro Tips

1. **Use `client:load`** to make Svelte components interactive
   ```astro
   <MyComponent client:load />
   ```

2. **Use breadcrumbs** for better navigation
   ```astro
   <DashboardLayout
     title="Page Name"
     breadcrumb={[
       { label: 'Customers', href: '/customers' },
       { label: 'John Doe' }
     ]}
   >
   ```

3. **Validate early** - use validation utils before submitting
   ```typescript
   const errors = validateEmail(email);
   if (errors.length > 0) { /* show errors */ }
   ```

4. **Format consistently** - use formatting utils for display
   ```typescript
   formatMoney(amount)  // Currency
   formatDate(date)     // Dates
   formatPercentage(n)  // Percentages
   ```

5. **Handle errors gracefully**
   ```typescript
   try {
     await api.post('/customers', data);
   } catch (error) {
     if (error instanceof HttpError) {
       console.log(error.statusCode);
     }
   }
   ```

---

**You're ready to build!** Start with the dashboard page (`/dashboard`) and explore from there.

For questions, refer to the appropriate documentation above.
