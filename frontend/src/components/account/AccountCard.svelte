<script lang="ts">
  interface Props {
    id: string;
    account_type: string;
    account_number: string;
    balance: number;
    currency: string;
    status: string;
  }

  let { id, account_type, account_number, balance, currency, status }: Props = $props();

  const typeIcons: Record<string, string> = {
    current: 'M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z',
    savings: 'M17 9V7a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2m2 4h10a2 2 0 002-2v-6a2 2 0 00-2-2H9a2 2 0 00-2 2v6a2 2 0 002 2z',
  };

  const typeLabels: Record<string, string> = {
    current: 'Compte courant',
    savings: 'Compte epargne',
  };

  let formattedBalance = $derived(
    new Intl.NumberFormat('fr-MA', { style: 'currency', currency }).format(balance),
  );
</script>

<article
  class="group rounded-lg border border-gray-200 bg-white p-5 shadow-sm transition hover:shadow-md"
  aria-label="Compte {typeLabels[account_type] ?? account_type} - {account_number}"
>
  <div class="flex items-start justify-between">
    <div class="flex items-center gap-3">
      <div class="rounded-lg bg-blue-50 p-2" aria-hidden="true">
        <svg class="h-6 w-6 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d={typeIcons[account_type] ?? typeIcons.current}></path>
        </svg>
      </div>
      <div>
        <h3 class="text-sm font-medium text-gray-900">
          {typeLabels[account_type] ?? account_type}
        </h3>
        <p class="text-xs text-gray-500 font-mono">{account_number}</p>
      </div>
    </div>
    <span
      class="inline-flex items-center rounded-full px-2 py-0.5 text-xs font-medium
        {status === 'active' ? 'bg-green-100 text-green-700' : 'bg-gray-100 text-gray-600'}"
    >
      {status === 'active' ? 'Actif' : status}
    </span>
  </div>

  <div class="mt-4">
    <p class="text-sm text-gray-500">Solde</p>
    <p class="text-2xl font-bold text-gray-900">{formattedBalance}</p>
  </div>

  <div class="mt-4">
    <a
      href="/accounts/{id}"
      class="text-sm font-medium text-blue-600 hover:text-blue-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600"
    >
      Voir les details
    </a>
  </div>
</article>
