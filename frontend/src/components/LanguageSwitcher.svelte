<script lang="ts">
  import { currentLocale, setLocale, locales, type Locale } from '../lib/i18n/i18n';

  let isOpen = $state(false);
  let currentLang = $state<Locale>('fr');

  currentLocale.subscribe((value) => {
    currentLang = value;
  });

  function selectLocale(locale: Locale) {
    setLocale(locale);
    isOpen = false;
  }

  function toggleDropdown() {
    isOpen = !isOpen;
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      isOpen = false;
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="relative inline-block text-left">
  <button
    type="button"
    class="inline-flex items-center gap-1.5 rounded-md border border-gray-300 bg-white px-3 py-1.5 text-sm font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-1"
    onclick={toggleDropdown}
    aria-expanded={isOpen}
    aria-haspopup="true"
  >
    <span>{locales[currentLang].name}</span>
    <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
    </svg>
  </button>

  {#if isOpen}
    <div
      class="absolute right-0 z-50 mt-1 w-40 origin-top-right rounded-md border border-gray-200 bg-white shadow-lg ring-1 ring-black/5 focus:outline-none"
      role="menu"
    >
      <div class="py-1">
        {#each Object.entries(locales) as [code, meta]}
          <button
            type="button"
            class="flex w-full items-center gap-2 px-4 py-2 text-sm hover:bg-gray-100 {code === currentLang ? 'bg-blue-50 font-semibold text-blue-700' : 'text-gray-700'}"
            role="menuitem"
            onclick={() => selectLocale(code as Locale)}
          >
            <span>{meta.name}</span>
            {#if meta.dir === 'rtl'}
              <span class="text-xs text-gray-400">RTL</span>
            {/if}
          </button>
        {/each}
      </div>
    </div>
  {/if}
</div>
