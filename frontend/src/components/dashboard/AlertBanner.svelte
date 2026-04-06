<script lang="ts">
  interface Props {
    message: string;
    type?: 'warning' | 'breach';
  }

  let { message, type = 'breach' }: Props = $props();

  let styles = $derived(
    type === 'breach'
      ? 'bg-red-50 border-red-300 text-red-800'
      : 'bg-orange-50 border-orange-300 text-orange-800',
  );

  let icon = $derived(
    type === 'breach'
      ? 'M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z'
      : 'M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z',
  );
</script>

<div
  role="alert"
  aria-live="assertive"
  class="flex items-center gap-3 rounded-lg border px-4 py-3 {styles}"
>
  <svg class="h-5 w-5 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d={icon}></path>
  </svg>
  <p class="text-sm font-medium">{message}</p>
</div>
