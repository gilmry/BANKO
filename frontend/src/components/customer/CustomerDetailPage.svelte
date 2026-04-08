<script lang="ts">
  import StatusBadge from '../common/StatusBadge.svelte';
  import { api } from '../../lib/api/client';

  interface Account {
    id: string;
    account_type: string;
    account_number: string;
    balance: { amount: number; currency: string };
    status: string;
  }

  interface Customer {
    id: string;
    full_name: string;
    email: string;
    phone: string;
    date_of_birth: string;
    nationality: string;
    cin: string;
    kyc_status: string;
    customer_type: string;
    segment: string;
    created_at: string;
    accounts: Account[];
  }

  let customer = $state<Customer | null>(null);
  let loading = $state(true);
  let error = $state('');
  let customerId = $state('');

  async function loadCustomer() {
    loading = true;
    error = '';
    try {
      customer = await api.get<Customer>(`/customers/${customerId}`);
    } catch (err: unknown) {
      error = err instanceof Error ? err.message : 'Erreur de chargement';
    } finally {
      loading = false;
    }
  }

  $effect(() => {
    const params = new URLSearchParams(window.location.search);
    customerId = params.get('id') || '';
    if (customerId) {
      loadCustomer();
    } else {
      error = 'Aucun identifiant client fourni';
      loading = false;
    }
  });

  function formatDate(iso: string): string {
    return new Date(iso).toLocaleDateString('fr-FR');
  }

  function formatDateTime(iso: string): string {
    return new Date(iso).toLocaleDateString('fr-FR', {
      year: 'numeric', month: 'long', day: 'numeric',
      hour: '2-digit', minute: '2-digit',
    });
  }

  function formatCurrency(amount: number, currency: string): string {
    return new Intl.NumberFormat('fr-TN', { style: 'currency', currency }).format(amount);
  }
</script>

<div class="space-y-6">
  {#if error}
    <div role="alert" data-testid="customer-detail-error" class="rounded-md border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-700">
      {error}
    </div>
  {/if}

  {#if loading}
    <div class="text-center text-sm text-gray-500 py-12" role="status" data-testid="customer-detail-loading">
      Chargement du client...
    </div>
  {:else if customer}
    <!-- Header -->
    <div class="flex items-center justify-between">
      <a href="/customers" data-testid="customer-detail-back" class="text-sm text-blue-600 hover:text-blue-500">
        ← Retour aux clients
      </a>
    </div>

    <!-- Customer Info Card -->
    <div class="rounded-lg border border-gray-200 bg-white p-6 shadow-sm" data-testid="customer-detail-info">
      <div class="flex items-start justify-between mb-6">
        <div>
          <h1 class="text-2xl font-bold text-gray-900" data-testid="customer-detail-name">{customer.full_name}</h1>
          <p class="mt-1 text-gray-600">{customer.email}</p>
        </div>
        <StatusBadge status={customer.kyc_status} />
      </div>

      <div class="grid gap-6 md:grid-cols-2">
        <div>
          <h2 class="text-lg font-semibold text-gray-900 mb-4">Informations Personnelles</h2>
          <dl class="space-y-3">
            <div class="flex justify-between">
              <dt class="text-gray-600">Téléphone</dt>
              <dd class="font-medium text-gray-900">{customer.phone}</dd>
            </div>
            {#if customer.date_of_birth}
              <div class="flex justify-between">
                <dt class="text-gray-600">Date de naissance</dt>
                <dd class="font-medium text-gray-900">{formatDate(customer.date_of_birth)}</dd>
              </div>
            {/if}
            <div class="flex justify-between">
              <dt class="text-gray-600">Nationalité</dt>
              <dd class="font-medium text-gray-900">{customer.nationality}</dd>
            </div>
            {#if customer.cin}
              <div class="flex justify-between">
                <dt class="text-gray-600">CIN</dt>
                <dd class="font-medium text-gray-900 font-mono">{customer.cin}</dd>
              </div>
            {/if}
            {#if customer.segment}
              <div class="flex justify-between">
                <dt class="text-gray-600">Segment</dt>
                <dd class="font-medium text-gray-900 capitalize">{customer.segment}</dd>
              </div>
            {/if}
          </dl>
        </div>

        <div>
          <h2 class="text-lg font-semibold text-gray-900 mb-4">Comptes Associés</h2>
          {#if customer.accounts && customer.accounts.length > 0}
            <div class="space-y-2">
              {#each customer.accounts as account}
                <a href="/accounts/detail?id={account.id}" class="block rounded border border-gray-200 p-3 hover:bg-gray-50 transition-colors">
                  <p class="text-sm font-medium text-gray-900 capitalize">{account.account_type}</p>
                  <p class="text-xs text-gray-500 font-mono">{account.account_number}</p>
                  <div class="mt-2 flex justify-between items-center">
                    <span class="text-sm font-semibold text-gray-900">
                      {formatCurrency(account.balance.amount, account.balance.currency)}
                    </span>
                    <StatusBadge status={account.status} />
                  </div>
                </a>
              {/each}
            </div>
          {:else}
            <p class="text-sm text-gray-500">Aucun compte associé</p>
          {/if}
        </div>
      </div>

      <div class="mt-6 border-t border-gray-200 pt-6">
        <p class="text-xs text-gray-500">
          Client créé le {formatDateTime(customer.created_at)}
        </p>
      </div>
    </div>

    <!-- Actions -->
    <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-4" data-testid="customer-detail-actions">
      <a href="/accounts/list?customer_id={customerId}" data-testid="customer-action-accounts" class="rounded-lg border border-blue-200 bg-blue-50 p-4 hover:bg-blue-100 transition-colors text-left">
        <div class="text-2xl mb-2">💳</div>
        <h3 class="font-semibold text-gray-900">Comptes</h3>
        <p class="text-sm text-gray-600">Voir les comptes</p>
      </a>
      <button data-testid="customer-action-edit" class="rounded-lg border border-green-200 bg-green-50 p-4 hover:bg-green-100 transition-colors text-left">
        <div class="text-2xl mb-2">✏️</div>
        <h3 class="font-semibold text-gray-900">Modifier</h3>
        <p class="text-sm text-gray-600">Éditer le profil</p>
      </button>
      <button data-testid="customer-action-documents" class="rounded-lg border border-purple-200 bg-purple-50 p-4 hover:bg-purple-100 transition-colors text-left">
        <div class="text-2xl mb-2">📄</div>
        <h3 class="font-semibold text-gray-900">Documents</h3>
        <p class="text-sm text-gray-600">KYC et fichiers</p>
      </button>
      <button data-testid="customer-action-suspend" class="rounded-lg border border-red-200 bg-red-50 p-4 hover:bg-red-100 transition-colors text-left">
        <div class="text-2xl mb-2">🚫</div>
        <h3 class="font-semibold text-gray-900">Suspendre</h3>
        <p class="text-sm text-gray-600">Désactiver le client</p>
      </button>
    </div>
  {/if}
</div>
