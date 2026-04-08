<script lang="ts">
  import { onMount } from 'svelte';

  interface Props {
    placeholder?: string;
    debounceMs?: number;
    onSearch?: (query: string) => void;
  }

  const { placeholder = 'Rechercher...', debounceMs = 300, onSearch }: Props = $props();

  let query = $state('');
  let debounceTimer: NodeJS.Timeout | null = null;

  function handleInput(value: string) {
    query = value;

    if (debounceTimer) {
      clearTimeout(debounceTimer);
    }

    debounceTimer = setTimeout(() => {
      onSearch?.(query);
    }, debounceMs);
  }

  function handleClear() {
    query = '';
    onSearch?.('');
  }

  onMount(() => {
    return () => {
      if (debounceTimer) {
        clearTimeout(debounceTimer);
      }
    };
  });
</script>

<div class="relative" data-testid="searchbar">
  <div class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
    <svg class="h-5 w-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
    </svg>
  </div>

  <input
    type="text"
    value={query}
    oninput={(e) => handleInput(e.currentTarget.value)}
    placeholder={placeholder}
    data-testid="searchbar-input"
    class="block w-full rounded-lg border border-gray-300 bg-white py-2 pl-10 pr-10 text-sm placeholder-gray-500 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
  />

  {#if query}
    <button
      type="button"
      onclick={handleClear}
      class="absolute inset-y-0 right-0 flex items-center pr-3 text-gray-400 hover:text-gray-600"
      aria-label="Effacer la recherche"
      data-testid="searchbar-clear"
    >
      <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
      </svg>
    </button>
  {/if}
</div>
