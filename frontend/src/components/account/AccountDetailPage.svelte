<script lang="ts">
  import { accountsApi, type Account } from '../../lib/api/accounts';
  import AccountDetails from './AccountDetails.svelte';
  import MovementsList from './MovementsList.svelte';
  import TransferModal from './TransferModal.svelte';

  let account = $state<Account | null>(null);
  let loading = $state(true);
  let error = $state('');
  let showTransfer = $state(false);
  let accountId = $state('');

  async function loadAccount() {
    loading = true;
    error = '';
    try {
      account = await accountsApi.get(accountId);
    } catch (err: unknown) {
      error = err instanceof Error ? err.message : 'Erreur de chargement';
    } finally {
      loading = false;
    }
  }

  $effect(() => {
    const params = new URLSearchParams(window.location.search);
    accountId = params.get('id') || '';
    if (accountId) {
      loadAccount();
    } else {
      error = 'Aucun identifiant de compte fourni';
      loading = false;
    }
  });
</script>

<div class="space-y-6">
  {#if error}
    <div role="alert" aria-live="polite" class="rounded-md border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-700">
      {error}
    </div>
  {/if}

  {#if loading}
    <div class="text-center text-sm text-gray-500" role="status">
      Chargement du compte...
    </div>
  {:else if account}
    <div class="flex items-center justify-between">
      <a
        href="/accounts"
        class="text-sm text-blue-600 hover:text-blue-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600"
        data-testid="account-detail-back-link"
      >
        ← Retour aux comptes
      </a>
      <button
        type="button"
        onclick={() => (showTransfer = true)}
        class="rounded-md bg-blue-600 px-4 py-2 text-sm font-semibold text-white shadow-sm hover:bg-blue-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600"
        data-testid="account-detail-transfer-btn"
      >
        Effectuer un virement
      </button>
    </div>

    <AccountDetails {account} />
    <MovementsList {accountId} />

    {#if showTransfer}
      <TransferModal fromAccountId={accountId} onclose={() => { showTransfer = false; loadAccount(); }} />
    {/if}
  {/if}
</div>
