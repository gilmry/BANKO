<script lang="ts">
  import { onMount } from 'svelte';
  import { amlApi, type AmlAlert, type AmlStats, type Investigation } from '../../lib/api/aml';

  let stats: AmlStats | null = null;
  let recentAlerts: AmlAlert[] = [];
  let investigations: Investigation[] = [];
  let loading = true;
  let error = '';

  const statusLabels: Record<string, string> = {
    open: 'Ouvert',
    investigating: 'En investigation',
    resolved: 'Résolu',
    dismissed: 'Rejeté',
  };

  const riskLevelLabels: Record<string, string> = {
    low: 'Faible',
    medium: 'Moyen',
    high: 'Élevé',
  };

  const riskLevelColors: Record<string, { bg: string; text: string; border: string }> = {
    low: { bg: 'bg-green-50', text: 'text-green-700', border: 'border-green-200' },
    medium: { bg: 'bg-yellow-50', text: 'text-yellow-700', border: 'border-yellow-200' },
    high: { bg: 'bg-red-50', text: 'text-red-700', border: 'border-red-200' },
  };

  const investigationStatusLabels: Record<string, string> = {
    pending: 'En attente',
    in_progress: 'En cours',
    completed: 'Complétée',
    closed: 'Fermée',
  };

  async function loadDashboard() {
    loading = true;
    error = '';
    try {
      const [statsData, alertsData, invData] = await Promise.all([
        amlApi.getAlertStats(),
        amlApi.listAlerts({ limit: 10 }),
        amlApi.listInvestigations({ limit: 5 }),
      ]);
      stats = statsData;
      recentAlerts = alertsData.data || [];
      investigations = invData.data || [];
    } catch (e: any) {
      error = e.message || 'Erreur au chargement du tableau de bord';
    }
    loading = false;
  }

  onMount(() => {
    loadDashboard();
  });
</script>

<div class="space-y-8">
  <h1 class="text-2xl font-bold text-gray-900">Tableau de bord AML</h1>

  {#if error}
    <div class="rounded-md bg-red-50 p-4 text-red-700">{error}</div>
  {/if}

  {#if loading}
    <div class="flex items-center justify-center py-12">
      <p class="text-gray-500">Chargement des données...</p>
    </div>
  {:else if stats}
    <!-- Alert Summary Cards -->
    <div class="grid grid-cols-1 gap-4 sm:grid-cols-4">
      <div class="rounded-lg bg-white p-6 shadow">
        <p class="text-sm text-gray-500">Alertes totales</p>
        <p class="text-3xl font-bold text-gray-900">{stats.total_alerts}</p>
      </div>
      <div class="rounded-lg bg-white p-6 shadow">
        <p class="text-sm text-gray-500">Alertes ouvertes</p>
        <p class="text-3xl font-bold text-blue-600">{stats.open_alerts}</p>
      </div>
      <div class="rounded-lg bg-white p-6 shadow">
        <p class="text-sm text-gray-500">En investigation</p>
        <p class="text-3xl font-bold text-yellow-600">{stats.investigating_alerts}</p>
      </div>
      <div class="rounded-lg bg-white p-6 shadow">
        <p class="text-sm text-gray-500">Résolues</p>
        <p class="text-3xl font-bold text-green-600">{stats.resolved_alerts}</p>
      </div>
    </div>

    <!-- Risk Level Summary -->
    <div class="rounded-lg bg-white p-6 shadow">
      <h2 class="mb-4 text-lg font-semibold">Distribution par niveau de risque</h2>
      <div class="grid grid-cols-1 gap-4 sm:grid-cols-3">
        <div class="rounded-lg border-l-4 border-red-400 bg-red-50 p-4">
          <p class="text-sm text-red-700">Risque élevé</p>
          <p class="text-2xl font-bold text-red-900">{stats.high_risk_count}</p>
          <p class="text-xs text-red-600 mt-1">{((stats.high_risk_count / stats.total_alerts) * 100).toFixed(1)}% du total</p>
        </div>
        <div class="rounded-lg border-l-4 border-yellow-400 bg-yellow-50 p-4">
          <p class="text-sm text-yellow-700">Risque moyen</p>
          <p class="text-2xl font-bold text-yellow-900">{stats.medium_risk_count}</p>
          <p class="text-xs text-yellow-600 mt-1">{((stats.medium_risk_count / stats.total_alerts) * 100).toFixed(1)}% du total</p>
        </div>
        <div class="rounded-lg border-l-4 border-green-400 bg-green-50 p-4">
          <p class="text-sm text-green-700">Risque faible</p>
          <p class="text-2xl font-bold text-green-900">{stats.low_risk_count}</p>
          <p class="text-xs text-green-600 mt-1">{((stats.low_risk_count / stats.total_alerts) * 100).toFixed(1)}% du total</p>
        </div>
      </div>
    </div>

    <!-- Recent Alerts Table -->
    {#if recentAlerts.length > 0}
      <div class="rounded-lg bg-white p-6 shadow">
        <h2 class="mb-4 text-lg font-semibold">Alertes récentes</h2>
        <div class="overflow-x-auto">
          <table class="w-full text-left text-sm">
            <thead>
              <tr class="border-b text-gray-500">
                <th class="pb-2">Type</th>
                <th class="pb-2">Client</th>
                <th class="pb-2">Niveau de risque</th>
                <th class="pb-2">Statut</th>
                <th class="pb-2">Date</th>
              </tr>
            </thead>
            <tbody>
              {#each recentAlerts as alert}
                {@const colors = riskLevelColors[alert.risk_level]}
                <tr class="border-b hover:bg-gray-50">
                  <td class="py-2 font-medium text-gray-900">{alert.alert_type}</td>
                  <td class="py-2 font-mono text-xs text-gray-600">{alert.customer_id.substring(0, 8)}</td>
                  <td class="py-2">
                    <span class="inline-flex rounded-full px-2 py-1 text-xs font-semibold {colors.bg} {colors.text}">
                      {riskLevelLabels[alert.risk_level]}
                    </span>
                  </td>
                  <td class="py-2">
                    <span class="inline-flex rounded px-2 py-1 text-xs font-medium" class:bg-blue-100={alert.status === 'open'} class:text-blue-800={alert.status === 'open'} class:bg-yellow-100={alert.status === 'investigating'} class:text-yellow-800={alert.status === 'investigating'} class:bg-green-100={alert.status === 'resolved'} class:text-green-800={alert.status === 'resolved'} class:bg-gray-100={alert.status === 'dismissed'} class:text-gray-800={alert.status === 'dismissed'}>
                      {statusLabels[alert.status]}
                    </span>
                  </td>
                  <td class="py-2 text-gray-600">{new Date(alert.created_at).toLocaleDateString('fr-FR')}</td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      </div>
    {/if}

    <!-- Investigation Pipeline -->
    {#if investigations.length > 0}
      <div class="rounded-lg bg-white p-6 shadow">
        <h2 class="mb-4 text-lg font-semibold">Pipeline d'investigations</h2>
        <div class="space-y-3">
          {#each investigations as inv}
            <div class="rounded-lg border border-gray-200 p-4">
              <div class="mb-2 flex items-center justify-between">
                <span class="font-mono text-xs text-gray-600">Alerte: {inv.alert_id.substring(0, 8)}</span>
                <span class="inline-flex rounded px-2 py-1 text-xs font-medium" class:bg-blue-100={inv.status === 'pending'} class:text-blue-800={inv.status === 'pending'} class:bg-orange-100={inv.status === 'in_progress'} class:text-orange-800={inv.status === 'in_progress'} class:bg-green-100={inv.status === 'completed'} class:text-green-800={inv.status === 'completed'} class:bg-gray-100={inv.status === 'closed'} class:text-gray-800={inv.status === 'closed'}>
                  {investigationStatusLabels[inv.status]}
                </span>
              </div>
              <div class="grid grid-cols-2 gap-2 text-sm text-gray-600">
                <div>
                  <span class="text-xs font-semibold">Démarrée</span>
                  <p>{new Date(inv.started_at).toLocaleDateString('fr-FR')}</p>
                </div>
                {#if inv.completed_at}
                  <div>
                    <span class="text-xs font-semibold">Complétée</span>
                    <p>{new Date(inv.completed_at).toLocaleDateString('fr-FR')}</p>
                  </div>
                {/if}
                {#if inv.assigned_to}
                  <div class="col-span-2">
                    <span class="text-xs font-semibold">Assignée à</span>
                    <p class="font-mono">{inv.assigned_to}</p>
                  </div>
                {/if}
              </div>
            </div>
          {/each}
        </div>
      </div>
    {/if}
  {/if}
</div>
