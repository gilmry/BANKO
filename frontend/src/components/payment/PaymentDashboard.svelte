<script lang="ts">
  import { onMount } from 'svelte';
  import { paymentsApi, type Payment, type PaymentStatus, type ClearingStatus } from '../../lib/api/payments';

  let paymentStatus: PaymentStatus | null = null;
  let clearingStatus: ClearingStatus | null = null;
  let recentPayments: Payment[] = [];
  let loading = true;
  let error = '';

  const statusLabels: Record<string, string> = {
    pending: 'En attente',
    screened: 'Contrôlée',
    submitted: 'Soumise',
    executed: 'Exécutée',
    failed: 'Échec',
    rejected: 'Rejetée',
  };

  const paymentTypeLabels: Record<string, string> = {
    domestic: 'Domestique',
    sepa: 'SEPA',
    swift: 'SWIFT',
  };

  const statusColors: Record<string, { bg: string; text: string }> = {
    pending: { bg: 'bg-gray-100', text: 'text-gray-800' },
    screened: { bg: 'bg-blue-100', text: 'text-blue-800' },
    submitted: { bg: 'bg-yellow-100', text: 'text-yellow-800' },
    executed: { bg: 'bg-green-100', text: 'text-green-800' },
    failed: { bg: 'bg-red-100', text: 'text-red-800' },
    rejected: { bg: 'bg-red-100', text: 'text-red-800' },
  };

  const clearingStatusColors: Record<string, string> = {
    operational: 'bg-green-100 text-green-800',
    degraded: 'bg-yellow-100 text-yellow-800',
    unavailable: 'bg-red-100 text-red-800',
  };

  const clearingStatusLabels: Record<string, string> = {
    operational: 'Opérationnel',
    degraded: 'Dégradé',
    unavailable: 'Indisponible',
  };

  async function loadDashboard() {
    loading = true;
    error = '';
    try {
      const [statusData, clearingData, paymentsData] = await Promise.all([
        paymentsApi.getPaymentStatus(),
        paymentsApi.getClearingStatus(),
        paymentsApi.listPayments({ limit: 10 }),
      ]);
      paymentStatus = statusData;
      clearingStatus = clearingData;
      recentPayments = paymentsData.data || [];
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
  <h1 class="text-2xl font-bold text-gray-900">Tableau de bord Paiements</h1>

  {#if error}
    <div class="rounded-md bg-red-50 p-4 text-red-700">{error}</div>
  {/if}

  {#if loading}
    <div class="flex items-center justify-center py-12">
      <p class="text-gray-500">Chargement des données...</p>
    </div>
  {:else if paymentStatus && clearingStatus}
    <!-- Clearing System Status Alert -->
    <div class="rounded-lg border border-gray-200 p-4 {clearingStatusColors[clearingStatus.status]}">
      <div class="flex items-center justify-between">
        <div>
          <p class="font-semibold">État du système de compensation</p>
          <p class="text-sm text-gray-600 mt-1">Dernier sync: {new Date(clearingStatus.last_sync).toLocaleString('fr-FR')}</p>
        </div>
        <span class="inline-flex rounded-full px-3 py-1 text-sm font-semibold {clearingStatusColors[clearingStatus.status]}">
          {clearingStatusLabels[clearingStatus.status]}
        </span>
      </div>
      {#if clearingStatus.pending_submissions > 0 || clearingStatus.failed_submissions > 0}
        <div class="mt-3 grid grid-cols-2 gap-4 text-sm">
          {#if clearingStatus.pending_submissions > 0}
            <div>
              <p class="font-medium">Soumissions en attente</p>
              <p class="text-lg font-bold">{clearingStatus.pending_submissions}</p>
            </div>
          {/if}
          {#if clearingStatus.failed_submissions > 0}
            <div>
              <p class="font-medium">Soumissions échouées</p>
              <p class="text-lg font-bold text-red-600">{clearingStatus.failed_submissions}</p>
            </div>
          {/if}
        </div>
      {/if}
    </div>

    <!-- Payment Summary Cards -->
    <div class="grid grid-cols-1 gap-4 sm:grid-cols-4">
      <div class="rounded-lg bg-white p-6 shadow">
        <p class="text-sm text-gray-500">Total des paiements</p>
        <p class="text-3xl font-bold text-gray-900">{paymentStatus.total_payments}</p>
      </div>
      <div class="rounded-lg bg-white p-6 shadow">
        <p class="text-sm text-gray-500">Volume total</p>
        <p class="text-2xl font-bold text-blue-600">{formatCurrency(paymentStatus.total_volume)}</p>
      </div>
      <div class="rounded-lg bg-white p-6 shadow">
        <p class="text-sm text-gray-500">Montant moyen</p>
        <p class="text-2xl font-bold text-green-600">{formatCurrency(paymentStatus.average_amount)}</p>
      </div>
      <div class="rounded-lg bg-white p-6 shadow">
        <p class="text-sm text-gray-500">Écarts</p>
        <p class="text-2xl font-bold text-red-600">{paymentStatus.failed_count + paymentStatus.rejected_count}</p>
      </div>
    </div>

    <!-- Status Breakdown -->
    <div class="rounded-lg bg-white p-6 shadow">
      <h2 class="mb-4 text-lg font-semibold">Répartition par statut</h2>
      <div class="grid grid-cols-1 gap-4 sm:grid-cols-3">
        <div class="rounded-lg border border-gray-200 p-4">
          <p class="text-sm font-semibold text-gray-700">En traitement</p>
          <div class="mt-3 space-y-2">
            <div class="flex justify-between text-sm">
              <span>En attente</span>
              <span class="font-bold">{paymentStatus.pending_count}</span>
            </div>
            <div class="flex justify-between text-sm">
              <span>Contrôlée</span>
              <span class="font-bold">{paymentStatus.screened_count}</span>
            </div>
            <div class="flex justify-between text-sm">
              <span>Soumise</span>
              <span class="font-bold">{paymentStatus.submitted_count}</span>
            </div>
          </div>
        </div>
        <div class="rounded-lg border border-green-200 bg-green-50 p-4">
          <p class="text-sm font-semibold text-green-700">Complétées</p>
          <p class="mt-3 text-3xl font-bold text-green-600">{paymentStatus.executed_count}</p>
        </div>
        <div class="rounded-lg border border-red-200 bg-red-50 p-4">
          <p class="text-sm font-semibold text-red-700">Problèmes</p>
          <div class="mt-3 space-y-2">
            <div class="flex justify-between text-sm">
              <span>Échec</span>
              <span class="font-bold">{paymentStatus.failed_count}</span>
            </div>
            <div class="flex justify-between text-sm">
              <span>Rejetée</span>
              <span class="font-bold">{paymentStatus.rejected_count}</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Recent Payments Table -->
    {#if recentPayments.length > 0}
      <div class="rounded-lg bg-white p-6 shadow">
        <h2 class="mb-4 text-lg font-semibold">Paiements récents</h2>
        <div class="overflow-x-auto">
          <table class="w-full text-left text-sm">
            <thead>
              <tr class="border-b text-gray-500">
                <th class="pb-2">ID</th>
                <th class="pb-2">Montant</th>
                <th class="pb-2">Type</th>
                <th class="pb-2">Statut</th>
                <th class="pb-2">Date</th>
              </tr>
            </thead>
            <tbody>
              {#each recentPayments as payment}
                {@const colors = statusColors[payment.status]}
                <tr class="border-b hover:bg-gray-50">
                  <td class="py-2 font-mono text-xs text-gray-600">{payment.id.substring(0, 8)}</td>
                  <td class="py-2 font-semibold text-gray-900">{formatCurrency(payment.amount)}</td>
                  <td class="py-2">
                    <span class="inline-flex rounded px-2 py-1 text-xs font-medium bg-gray-100 text-gray-800">
                      {paymentTypeLabels[payment.payment_type] || payment.payment_type}
                    </span>
                  </td>
                  <td class="py-2">
                    <span class="inline-flex rounded px-2 py-1 text-xs font-medium {colors.bg} {colors.text}">
                      {statusLabels[payment.status] || payment.status}
                    </span>
                  </td>
                  <td class="py-2 text-gray-600">{new Date(payment.created_at).toLocaleDateString('fr-FR')}</td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      </div>
    {/if}
  {/if}
</div>
