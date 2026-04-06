<script lang="ts">
  import { currentLocale, locales } from '$lib/i18n/i18n';

  export let class: string = '';

  // Determine direction based on current locale
  $: direction = locales[$currentLocale].dir;

  // Determine font-family based on current locale
  $: fontFamily = $currentLocale === 'ar' ? "'Noto Sans Arabic', sans-serif" : "sans-serif";

  // Merge custom styles with RTL-specific styles
  $: computedStyle = `direction: ${direction}; font-family: ${fontFamily};`;
</script>

<div {class} style={computedStyle} dir={direction}>
  <slot />
</div>

<style>
  div {
    display: contents;
  }
</style>
