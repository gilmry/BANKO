<script lang="ts">
  interface Props {
    title: string;
    value: string | number;
    icon?: string;
    trend?: {
      value: number;
      isPositive: boolean;
    };
    variant?: 'default' | 'success' | 'warning' | 'danger' | 'info';
  }

  const { title, value, icon, trend, variant = 'default' }: Props = $props();

  const variantConfig = {
    default: 'bg-gradient-to-br from-blue-50 to-blue-100 border-blue-200',
    success: 'bg-gradient-to-br from-green-50 to-green-100 border-green-200',
    warning: 'bg-gradient-to-br from-amber-50 to-amber-100 border-amber-200',
    danger: 'bg-gradient-to-br from-red-50 to-red-100 border-red-200',
    info: 'bg-gradient-to-br from-purple-50 to-purple-100 border-purple-200',
  };

  const trendColor = trend
    ? trend.isPositive
      ? 'text-green-600'
      : 'text-red-600'
    : '';
</script>

<article class="rounded-lg border p-6 shadow-sm hover:shadow-md transition-shadow {variantConfig[variant]}">
  <div class="flex items-start justify-between">
    <div class="flex-1">
      <p class="text-sm font-medium text-gray-600">{title}</p>
      <p class="mt-2 text-3xl font-bold text-gray-900">
        {value}
      </p>
      {#if trend}
        <p class="mt-2 text-sm {trendColor}">
          <span class="font-semibold">
            {trend.isPositive ? '+' : ''}{trend.value}%
          </span>
          vs. période précédente
        </p>
      {/if}
    </div>
    {#if icon}
      <div class="text-3xl text-gray-400">
        {icon}
      </div>
    {/if}
  </div>
</article>
