<script lang="ts">
  import { onMount } from 'svelte';
  import { creditApi, type Loan, type LoanClassification } from '../../lib/api/credit';

  let classification: LoanClassification | null = null;
  let recentLoans: Loan[] = [];
  let loading = true;
  let error = '';

  const statusLabels: Record<string, string> = {
    pending: 'En attente',
    approved: 'Approuvé',
    disbursed: 'Déboursé',
    active: 'Actif',
    completed: 'Complété',
    defaulted: 'Défaut',
  };

  const assetClassLabels: Record<number, string> = {
    0: 'Non classifié',
    1: 'Faible risque',
    2: 'Risque modéré',
    3: 'Risque élevé',
    4: 'Risque très élevé',
  };

  const assetClassColors: Record<number, string> = {
    0: 'bg-gray-100',
    1: 'bg-green-100',
    2: 'bg-yellow-100',
    3: 'bg-orange-100',
    4: 'bg-red-100',
  };

  async function loadDashboard() {
    loading = true;
    error = '';
    try {
      const [classData, loansData] = await Promise.all([
        creditApi.getClassification(),
        creditApi.listLoans({ limit: 10 }),
      ]);
      classification = classData;
      recentLoans = loansData.data || [];
    } catch (e: any) {
      error = e.message || 'Erreur au chargement du tableau de bord';
    }
    loading = false;
  }

  function formatCurrency(amount: number): string {
    return new Intl.NumberFormat('fr-FR', {
      style: 'currency',
      currency: 'EUR',
    }).format(amount);
  }

  onMount(() => {
    loadDashboard();
  });
</script>

<div class="space-y-8">
  <h1 class="text-2xl font-bold text-gray-900">Tableau de bord Crédit</h1>

  {#if error}
    <div class="rounded-md bg-red-50 p-4 text-red-700">{error}</div>
  {/if}

  {#if loading}
    <div class="flex items-center justify-center py-12">
      <p class="text-gray-500">Chargement des données...</p>
    </div>
  {:else if classification}
    <!-- Summary Cards -->
    <div class="grid grid-cols-1 gap-4 sm:grid-cols-3">
      <div class="rounded-lg bg-white p-6 shadow">
        <p class="text-sm text-gray-500">Total de prêts</p>
        <p class="text-3xl font-bold text-gray-900">{classification.total}</p>
      </div>
      <div class="rounded-lg bg-white p-6 shadow">
        <p class="text-sm text-gray-500">Montant moyen</p>
        <p class="text-3xl font-bold text-blue-600">{formatCurrency(classification.average_amount)}</p>
      </div>
      <div class="rounded-lg bg-white p-6 shadow">
        <p class="text-sm text-gray-500">Portfolio total</p>
        <p class="text-3xl font-bold text-green-600">
          {formatCurrency(
            Object.values(classification.by_status).reduce((sum, count, idx) => {
              const statuses = Object.keys(classification.by_status);
              return sum + (count * classification.average_amount);
            }, 0)
          )}
        </p>
      </div>
    </div>

    <!-- Status Breakdown -->
    {#if Object.keys(classification.by_status).length > 0}
      <div class="rounded-lg bg-white p-6 shadow">
        <h2 class="mb-4 text-lg font-semibold">Répartition par statut</h2>
        <div class="space-y-3">
          {#each Object.entries(classification.by_status) as [status, count]}
            {@const percentage = (count / classification.total) * 100}
            <div class="space-y-1">
              <div class="flex justify-between text-sm">
                <span class="font-medium text-gray-700">{statusLabels[status] || status}</span>
                <span class="text-gray-600">{count} ({percentage.toFixed(1)}%)</span>
              </div>
              <div class="h-2 overflow-hidden rounded-full bg-gray-200">
                <div class="h-full bg-blue-500" style="width: {percentage}%"></div>
              </div>
            </div>
          {/each}
        </div>
      </div>
    {/if}

    <!-- Asset Class Distribution (Colored Bars) -->
    {#if Object.keys(classification.by_asset_class).length > 0}
      <div class="rounded-lg bg-white p-6 shadow">
        <h2 class="mb-4 text-lg font-semibold">Distribution par classe d'actif</h2>
        <div class="space-y-3">
          {#each Array.from({ length: 5 }, (_, i) => i) as classId}
            {@const count = classification.by_asset_class[classId] || 0}
            {@const percentage = classification.total > 0 ? (count / classification.total) * 100 : 0}
            <div class="space-y-1">
              <div class="flex justify-between text-sm">
                <span class="font-medium text-gray-700">{assetClassLabels[classId]}</span>
                <span class="text-gray-600">{count} ({percentage.toFixed(1)}%)</span>
              </div>
              <div class="flex gap-1">
                <div class="flex-1 h-3 rounded {assetClassColors[classId]}" style="width: {percentage}%"></div>
                <div class="h-3 rounded bg-gray-100" style="width: {100 - percentage}%"></div>
              </div>
            </div>
          {/each}
        </div>
      </div>
    {/if}

    <!-- Recent Loans Table -->
    {#if recentLoans.length > 0}
      <div class="rounded-lg bg-white p-6 shadow">
        <h2 class="mb-4 text-lg font-semibold">Prêts récents</h2>
        <div class="overflow-x-auto">
          <table class="w-full text-left text-sm">
            <thead>
              <tr class="border-b text-gray-500">
                <th class="pb-2">ID</th>
                <th class="pb-2">Client</th>
                <th class="pb-2">Montant</th>
                <th class="pb-2">Statut</th>
                <th class="pb-2">Classe d'actif</th>
                <th class="pb-2">Date</th>
              </tr>
            </thead>
            <tbody>
              {#each recentLoans as loan}
                <tr class="border-b hover:bg-gray-50">
                  <td class="py-2 font-mono text-xs text-gray-600">{loan.id.substring(0, 8)}</td>
                  <td class="py-2 font-mono text-xs text-gray-600">{loan.customer_id.substring(0, 8)}</td>
                  <td class="py-2 font-semibold">{formatCurrency(loan.amount)}</td>
                  <td class="py-2">
                    <span class="inline-flex rounded-full px-2 py-1 text-xs font-semibold" class:bg-yellow-100={loan.status === 'pending'} class:text-yellow-800={loan.status === 'pending'} class:bg-green-100={loan.status === 'approved'} class:text-green-800={loan.status === 'approved'} class:bg-blue-100={loan.status === 'active'} class:text-blue-800={loan.status === 'active'} class:bg-gray-100={!['pending', 'approved', 'active'].includes(loan.status)} class:text-gray-800={!['pending', 'approved', 'active'].includes(loan.status)}>
                      {statusLabels[loan.status] || loan.status}
                    </span>
                  </td>
                  <td class="py-2">
                    <span class="inline-flex rounded px-2 py-1 text-xs font-medium {assetClassColors[loan.asset_class]}">
                      {assetClassLabels[loan.asset_class]}
                    </span>
                  </td>
                  <td class="py-2 text-gray-600">{new Date(loan.created_at).toLocaleDateString('fr-FR')}</td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      </div>
    {/if}
  {/if}
</div>
