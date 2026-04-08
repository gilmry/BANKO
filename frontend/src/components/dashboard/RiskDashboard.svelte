<script lang="ts">
  import { prudentialApi, type PrudentialRatio, type TrendDataPoint } from '../../lib/api/prudential';
  import MetricCard from './MetricCard.svelte';
  import AlertBanner from './AlertBanner.svelte';
  import RatioGauge from './RatioGauge.svelte';
  import TrendChart from './TrendChart.svelte';

  let ratios = $state<PrudentialRatio[]>([]);
  let trendData = $state<Record<string, TrendDataPoint[]>>({});
  let loading = $state(true);
  let error = $state('');

  let breachRatios = $derived(ratios.filter((r) => r.status === 'breach'));
  let warningRatios = $derived(ratios.filter((r) => r.status === 'warning'));

  async function loadData() {
    loading = true;
    error = '';
    try {
      ratios = await prudentialApi.getRatios();
      // Load trends for each ratio
      const trendResults: Record<string, TrendDataPoint[]> = {};
      await Promise.all(
        ratios.map(async (r) => {
          try {
            trendResults[r.id] = await prudentialApi.getTrend(r.id, 30);
          } catch {
            trendResults[r.id] = [];
          }
        }),
      );
      trendData = trendResults;
    } catch (err: unknown) {
      error = err instanceof Error ? err.message : 'Erreur de chargement';
    } finally {
      loading = false;
    }
  }

  $effect(() => {
    loadData();
  });
</script>

<div class="space-y-6">
  <h1 class="text-2xl font-bold text-gray-900" data-testid="risk-heading">
    Tableau de bord prudentiel
  </h1>

  {#if error}
    <div role="alert" aria-live="polite" class="rounded-md border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-700" data-testid="risk-error">
      {error}
    </div>
  {/if}

  {#if loading}
    <div class="text-center text-sm text-gray-500" role="status" data-testid="risk-loading">
      Chargement des ratios prudentiels...
    </div>
  {:else}
    <!-- Breach alerts -->
    {#each breachRatios as r}
      <AlertBanner
        message="ALERTE: Le ratio {r.name} ({r.current_value.toFixed(2)}%) est en depassement du seuil ({r.threshold_breach.toFixed(2)}%)"
        type="breach"
      />
    {/each}

    <!-- Warning alerts -->
    {#each warningRatios as r}
      <AlertBanner
        message="Attention: Le ratio {r.name} ({r.current_value.toFixed(2)}%) approche du seuil ({r.threshold_warning.toFixed(2)}%)"
        type="warning"
      />
    {/each}

    <!-- Metric cards -->
    <section aria-label="Ratios prudentiels" data-testid="risk-ratios-section">
      <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
        {#each ratios as r}
          <MetricCard
            name={r.name}
            currentValue={r.current_value}
            targetValue={r.target_value}
            status={r.status}
          />
        {/each}
      </div>
    </section>

    <!-- Gauges -->
    {#if ratios.length > 0}
      <section aria-label="Jauges des ratios" data-testid="risk-gauges-section">
        <h2 class="mb-4 text-lg font-semibold text-gray-900">Vue d'ensemble</h2>
        <div class="grid grid-cols-2 gap-6 sm:grid-cols-3 lg:grid-cols-4">
          {#each ratios as r}
            <RatioGauge
              name={r.name}
              value={r.current_value}
              threshold={r.target_value}
            />
          {/each}
        </div>
      </section>
    {/if}

    <!-- Trend charts -->
    {#if Object.keys(trendData).length > 0}
      <section aria-label="Tendances" data-testid="risk-trends-section">
        <h2 class="mb-4 text-lg font-semibold text-gray-900">Tendance sur 30 jours</h2>
        <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
          {#each ratios as r}
            {#if trendData[r.id] && trendData[r.id].length > 0}
              <TrendChart
                data={trendData[r.id]}
                label={r.name}
                thresholdValue={r.target_value}
              />
            {/if}
          {/each}
        </div>
      </section>
    {/if}
  {/if}
</div>
