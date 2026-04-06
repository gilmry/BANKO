<script lang="ts">
  import type { Account } from '../../lib/api/accounts';

  interface Props {
    account: Account;
  }

  let { account }: Props = $props();

  let formattedBalance = $derived(
    new Intl.NumberFormat('fr-MA', { style: 'currency', currency: account.currency }).format(account.balance),
  );

  const typeLabels: Record<string, string> = {
    current: 'Compte courant',
    savings: 'Compte epargne',
  };
</script>

<section aria-labelledby="account-details-heading" class="rounded-lg border border-gray-200 bg-white p-6">
  <h2 id="account-details-heading" class="mb-4 text-xl font-bold text-gray-900">
    Details du compte
  </h2>

  <dl class="grid grid-cols-1 gap-4 sm:grid-cols-2">
    <div>
      <dt class="text-sm text-gray-500">Type de compte</dt>
      <dd class="text-sm font-medium text-gray-900">{typeLabels[account.account_type] ?? account.account_type}</dd>
    </div>
    <div>
      <dt class="text-sm text-gray-500">Numero de compte</dt>
      <dd class="text-sm font-medium text-gray-900 font-mono">{account.account_number}</dd>
    </div>
    <div>
      <dt class="text-sm text-gray-500">IBAN</dt>
      <dd class="text-sm font-medium text-gray-900 font-mono">{account.iban}</dd>
    </div>
    <div>
      <dt class="text-sm text-gray-500">Devise</dt>
      <dd class="text-sm font-medium text-gray-900">{account.currency}</dd>
    </div>
    <div>
      <dt class="text-sm text-gray-500">Solde</dt>
      <dd class="text-2xl font-bold text-gray-900">{formattedBalance}</dd>
    </div>
    <div>
      <dt class="text-sm text-gray-500">Statut</dt>
      <dd>
        <span
          class="inline-flex items-center rounded-full px-2 py-0.5 text-xs font-medium
            {account.status === 'active' ? 'bg-green-100 text-green-700' : 'bg-gray-100 text-gray-600'}"
        >
          {account.status === 'active' ? 'Actif' : account.status}
        </span>
      </dd>
    </div>
    <div>
      <dt class="text-sm text-gray-500">Date d'ouverture</dt>
      <dd class="text-sm font-medium text-gray-900">{new Date(account.created_at).toLocaleDateString('fr-FR')}</dd>
    </div>
  </dl>
</section>
