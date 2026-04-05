<script lang="ts">
  interface Props {
    name: string;
    currentValue: number;
    targetValue: number;
    status: 'compliant' | 'warning' | 'breach';
  }

  let { name, currentValue, targetValue, status }: Props = $props();

  const statusConfig = {
    compliant: { label: 'Conforme', bg: 'bg-green-50', border: 'border-green-200', text: 'text-green-700', dot: 'bg-green-500' },
    warning: { label: 'Attention', bg: 'bg-orange-50', border: 'border-orange-200', text: 'text-orange-700', dot: 'bg-orange-500' },
    breach: { label: 'Depassement', bg: 'bg-red-50', border: 'border-red-200', text: 'text-red-700', dot: 'bg-red-500' },
  };

  let config = $derived(statusConfig[status]);
</script>

<article
  class="rounded-lg border p-5 {config.bg} {config.border}"
  aria-label="{name}: {currentValue}% - {config.label}"
>
  <div class="flex items-center justify-between">
    <h3 class="text-sm font-medium text-gray-700">{name}</h3>
    <span class="inline-flex items-center gap-1.5 rounded-full px-2 py-0.5 text-xs font-medium {config.text}">
      <span class="h-2 w-2 rounded-full {config.dot}" aria-hidden="true"></span>
      {config.label}
    </span>
  </div>
  <div class="mt-3">
    <p class="text-3xl font-bold {config.text}">
      {currentValue.toFixed(2)}%
    </p>
    <p class="mt-1 text-sm text-gray-500">
      Cible: {targetValue.toFixed(2)}%
    </p>
  </div>
</article>
