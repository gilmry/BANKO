<script lang="ts">
  import { currentLocale, locales } from '$lib/i18n/i18n';

  let { class: className = '' }: { class?: string } = $props();

  const direction = $derived(locales[$currentLocale].dir);
  const fontFamily = $derived($currentLocale === 'ar' ? "'Noto Sans Arabic', sans-serif" : "sans-serif");
  const computedStyle = $derived(`direction: ${direction}; font-family: ${fontFamily};`);
</script>

<div class={className} style={computedStyle} dir={direction}>
  <slot />
</div>

<style>
  div {
    display: contents;
  }
</style>
