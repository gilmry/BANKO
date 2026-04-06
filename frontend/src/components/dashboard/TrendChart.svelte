<script lang="ts">
  interface DataPoint {
    date: string;
    value: number;
  }

  interface Props {
    data: DataPoint[];
    label: string;
    thresholdValue?: number;
  }

  let { data, label, thresholdValue }: Props = $props();

  let showTable = $state(false);

  let maxValue = $derived(Math.max(...data.map((d) => d.value), thresholdValue ?? 0, 1));
  let chartHeight = 160;
  let chartWidth = $derived(Math.max(data.length * 24, 200));

  function barHeight(value: number): number {
    return (value / maxValue) * chartHeight * 0.85;
  }

  let thresholdY = $derived(
    thresholdValue !== undefined
      ? chartHeight - (thresholdValue / maxValue) * chartHeight * 0.85
      : undefined,
  );
</script>

<div class="rounded-lg border border-gray-200 bg-white p-4">
  <div class="mb-3 flex items-center justify-between">
    <h3 class="text-sm font-medium text-gray-700">{label}</h3>
    <button
      type="button"
      onclick={() => (showTable = !showTable)}
      class="text-xs text-blue-600 hover:text-blue-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600"
      aria-expanded={showTable}
      aria-controls="trend-table"
    >
      {showTable ? 'Voir le graphique' : 'Voir les donnees'}
    </button>
  </div>

  {#if showTable}
    <!-- Accessible data table alternative -->
    <div id="trend-table" class="overflow-x-auto">
      <table class="w-full text-sm">
        <caption class="sr-only">{label} - donnees tabulaires</caption>
        <thead class="bg-gray-50">
          <tr>
            <th scope="col" class="px-3 py-2 text-start text-sm font-medium text-gray-500">Date</th>
            <th scope="col" class="px-3 py-2 text-end text-sm font-medium text-gray-500">Valeur</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-gray-200">
          {#each data as point}
            <tr>
              <td class="px-3 py-1 text-gray-700">{new Date(point.date).toLocaleDateString('fr-FR')}</td>
              <td class="px-3 py-1 text-end font-medium text-gray-900">{point.value.toFixed(2)}%</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {:else}
    <!-- SVG Bar chart -->
    <div class="overflow-x-auto" role="img" aria-label="{label}: graphique de tendance sur {data.length} jours">
      <svg viewBox="0 0 {chartWidth} {chartHeight}" class="w-full" style="min-width: {chartWidth}px; max-height: {chartHeight}px;" aria-hidden="true">
        <!-- Bars -->
        {#each data as point, i}
          {@const h = barHeight(point.value)}
          {@const x = i * 24 + 4}
          <rect
            {x}
            y={chartHeight - h}
            width="16"
            height={h}
            rx="2"
            fill={thresholdValue && point.value < thresholdValue ? '#ef4444' : '#3b82f6'}
            opacity="0.8"
          >
            <title>{new Date(point.date).toLocaleDateString('fr-FR')}: {point.value.toFixed(2)}%</title>
          </rect>
        {/each}

        <!-- Threshold line -->
        {#if thresholdY !== undefined}
          <line
            x1="0"
            y1={thresholdY}
            x2={chartWidth}
            y2={thresholdY}
            stroke="#dc2626"
            stroke-width="1"
            stroke-dasharray="4,4"
          />
        {/if}
      </svg>
    </div>
    <span class="sr-only">
      Graphique de tendance pour {label}. {data.length} points de donnees.
      {#if thresholdValue}Seuil a {thresholdValue.toFixed(2)}%.{/if}
    </span>
  {/if}
</div>
