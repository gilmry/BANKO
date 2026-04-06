<script lang="ts">
  interface Props {
    name: string;
    value: number;
    threshold: number;
    max?: number;
  }

  let { name, value, threshold, max = 100 }: Props = $props();

  // SVG arc calculations for a semi-circle gauge
  let percentage = $derived(Math.min((value / max) * 100, 100));
  let thresholdPercentage = $derived(Math.min((threshold / max) * 100, 100));

  // Arc goes from -90 degrees to +90 degrees (semi-circle)
  let angle = $derived(-90 + (percentage / 100) * 180);
  let thresholdAngle = $derived(-90 + (thresholdPercentage / 100) * 180);

  let isAboveThreshold = $derived(value >= threshold);

  let color = $derived(isAboveThreshold ? '#16a34a' : '#dc2626');
  let thresholdX = $derived(50 + 40 * Math.cos((thresholdAngle * Math.PI) / 180));
  let thresholdY = $derived(50 + 40 * Math.sin((thresholdAngle * Math.PI) / 180));

  // Create arc path
  function describeArc(startAngle: number, endAngle: number): string {
    const cx = 50, cy = 50, r = 40;
    const start = polarToCartesian(cx, cy, r, endAngle);
    const end = polarToCartesian(cx, cy, r, startAngle);
    const largeArc = endAngle - startAngle > 180 ? 1 : 0;
    return `M ${end.x} ${end.y} A ${r} ${r} 0 ${largeArc} 1 ${start.x} ${start.y}`;
  }

  function polarToCartesian(cx: number, cy: number, r: number, angleDeg: number) {
    const rad = (angleDeg * Math.PI) / 180;
    return { x: cx + r * Math.cos(rad), y: cy + r * Math.sin(rad) };
  }

  let bgArc = $derived(describeArc(-90, 90));
  let valueArc = $derived(describeArc(-90, angle));
</script>

<div class="flex flex-col items-center" role="img" aria-label="{name}: {value.toFixed(2)}% (seuil: {threshold.toFixed(2)}%)">
  <svg viewBox="0 0 100 60" class="w-full max-w-[200px]" aria-hidden="true">
    <!-- Background arc -->
    <path d={bgArc} fill="none" stroke="#e5e7eb" stroke-width="8" stroke-linecap="round" />
    <!-- Value arc -->
    <path d={valueArc} fill="none" stroke={color} stroke-width="8" stroke-linecap="round" />
    <!-- Threshold marker -->
    <circle cx={thresholdX} cy={thresholdY} r="2" fill="#6b7280" />
    <!-- Value text -->
    <text x="50" y="48" text-anchor="middle" class="text-xs font-bold" fill={color}>
      {value.toFixed(1)}%
    </text>
  </svg>
  <p class="mt-1 text-sm font-medium text-gray-700">{name}</p>
  <span class="sr-only">
    {name}: valeur actuelle {value.toFixed(2)}%, seuil {threshold.toFixed(2)}%.
    {isAboveThreshold ? 'Au-dessus du seuil (conforme).' : 'En dessous du seuil (non conforme).'}
  </span>
</div>
