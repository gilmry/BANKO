<script lang="ts">
  import { accountsApi, type Account } from '../../lib/api/accounts';
  import AccountCard from './AccountCard.svelte';

  let accounts = $state<Account[]>([]);
  let loading = $state(true);
  let error = $state('');

  async function loadAccounts() {
    loading = true;
    error = '';
    try {
      const res = await accountsApi.list({ limit: 50 });
      accounts = res.data;
    } catch (err: unknown) {
      error = err instanceof Error ? err.message : 'Erreur de chargement';
    } finally {
      loading = false;
    }
  }

  $effect(() => {
    loadAccounts();
  });
</script>

<div class="space-y-6">
  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-bold text-gray-900">Mes comptes</h1>
  </div>

  {#if error}
    <div role="alert" aria-live="polite" class="rounded-md border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-700">
      {error}
    </div>
  {/if}

  {#if loading}
    <div class="text-center text-sm text-gray-500" role="status">
      Chargement des comptes...
    </div>
  {:else if accounts.length === 0}
    <div class="rounded-md border border-gray-200 bg-gray-50 p-8 text-center text-sm text-gray-500">
      Aucun compte trouve
    </div>
  {:else}
    <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3" role="list" aria-label="Liste des comptes">
      {#each accounts as account}
        <div role="listitem">
          <AccountCard
            id={account.id}
            account_type={account.account_type}
            account_number={account.account_number}
            balance={account.balance}
            currency={account.currency}
            status={account.status}
          />
        </div>
      {/each}
    </div>
  {/if}
</div>
