<script lang="ts">
  import { accountsApi, type Movement } from '../../lib/api/accounts';

  interface Props {
    accountId: string;
  }

  let { accountId }: Props = $props();

  let movements = $state<Movement[]>([]);
  let loading = $state(true);
  let error = $state('');
  let page = $state(1);
  let totalPages = $state(1);
  let total = $state(0);
  let sortField = $state<'date' | 'amount'>('date');
  let sortDir = $state<'asc' | 'desc'>('desc');

  async function loadMovements() {
    loading = true;
    error = '';
    try {
      const res = await accountsApi.movements(accountId, { page, limit: 20 });
      movements = res.data;
      totalPages = res.total_pages;
      total = res.total;
    } catch (err: unknown) {
      error = err instanceof Error ? err.message : 'Erreur de chargement';
    } finally {
      loading = false;
    }
  }

  function toggleSort(field: 'date' | 'amount') {
    if (sortField === field) {
      sortDir = sortDir === 'asc' ? 'desc' : 'asc';
    } else {
      sortField = field;
      sortDir = 'desc';
    }
  }

  let sortedMovements = $derived.by(() => {
    const sorted = [...movements];
    sorted.sort((a, b) => {
      let cmp = 0;
      if (sortField === 'date') {
        cmp = new Date(a.date).getTime() - new Date(b.date).getTime();
      } else {
        cmp = a.amount - b.amount;
      }
      return sortDir === 'asc' ? cmp : -cmp;
    });
    return sorted;
  });

  function goToPage(p: number) {
    page = p;
    loadMovements();
  }

  function sortLabel(field: string): string {
    if (sortField !== field) return 'none';
    return sortDir === 'asc' ? 'ascending' : 'descending';
  }

  $effect(() => {
    loadMovements();
  });
</script>

<section aria-labelledby="movements-heading" class="rounded-lg border border-gray-200 bg-white">
  <h2 id="movements-heading" class="border-b border-gray-200 px-6 py-4 text-lg font-semibold text-gray-900">
    Mouvements
  </h2>

  {#if error}
    <div role="alert" aria-live="polite" class="mx-6 mt-4 rounded-md border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-700">
      {error}
    </div>
  {/if}

  {#if loading}
    <div class="p-6 text-center text-sm text-gray-500" role="status">
      Chargement des mouvements...
    </div>
  {:else if sortedMovements.length === 0}
    <div class="p-6 text-center text-sm text-gray-500">
      Aucun mouvement trouve
    </div>
  {:else}
    <div class="overflow-x-auto">
      <table class="w-full text-sm">
        <caption class="sr-only">Mouvements du compte - {total} entrees</caption>
        <thead class="bg-gray-50">
          <tr>
            <th
              scope="col"
              class="cursor-pointer px-6 py-3 text-start text-sm font-medium text-gray-500 hover:text-gray-700"
              aria-sort={sortLabel('date')}
              onclick={() => toggleSort('date')}
              onkeydown={(e) => e.key === 'Enter' && toggleSort('date')}
              tabindex={0}
              role="columnheader"
            >
              Date {sortField === 'date' ? (sortDir === 'asc' ? '↑' : '↓') : ''}
            </th>
            <th scope="col" class="px-6 py-3 text-start text-sm font-medium text-gray-500">
              Description
            </th>
            <th
              scope="col"
              class="cursor-pointer px-6 py-3 text-end text-sm font-medium text-gray-500 hover:text-gray-700"
              aria-sort={sortLabel('amount')}
              onclick={() => toggleSort('amount')}
              onkeydown={(e) => e.key === 'Enter' && toggleSort('amount')}
              tabindex={0}
              role="columnheader"
            >
              Montant {sortField === 'amount' ? (sortDir === 'asc' ? '↑' : '↓') : ''}
            </th>
            <th scope="col" class="px-6 py-3 text-end text-sm font-medium text-gray-500">
              Solde apres
            </th>
          </tr>
        </thead>
        <tbody class="divide-y divide-gray-200">
          {#each sortedMovements as m}
            <tr class="hover:bg-gray-50">
              <td class="whitespace-nowrap px-6 py-3 text-gray-700">
                {new Date(m.date).toLocaleDateString('fr-FR')}
              </td>
              <td class="px-6 py-3 text-gray-700">
                {m.description}
              </td>
              <td class="whitespace-nowrap px-6 py-3 text-end font-medium {m.amount >= 0 ? 'text-green-600' : 'text-red-600'}">
                {m.amount >= 0 ? '+' : ''}{m.amount.toLocaleString('fr-FR', { minimumFractionDigits: 2 })}
              </td>
              <td class="whitespace-nowrap px-6 py-3 text-end text-gray-500">
                {m.balance_after.toLocaleString('fr-FR', { minimumFractionDigits: 2 })}
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>

    <!-- Pagination -->
    {#if totalPages > 1}
      <div class="flex items-center justify-between border-t border-gray-200 px-6 py-3">
        <span class="text-sm text-gray-500">
          Page {page} sur {totalPages} ({total} mouvements)
        </span>
        <div class="flex gap-2">
          <button
            type="button"
            onclick={() => goToPage(page - 1)}
            disabled={page <= 1}
            class="rounded-md border border-gray-300 bg-white px-3 py-1 text-sm text-gray-700 hover:bg-gray-50 disabled:cursor-not-allowed disabled:opacity-50 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600"
            aria-label="Page precedente"
          >
            Precedent
          </button>
          <button
            type="button"
            onclick={() => goToPage(page + 1)}
            disabled={page >= totalPages}
            class="rounded-md border border-gray-300 bg-white px-3 py-1 text-sm text-gray-700 hover:bg-gray-50 disabled:cursor-not-allowed disabled:opacity-50 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600"
            aria-label="Page suivante"
          >
            Suivant
          </button>
        </div>
      </div>
    {/if}
  {/if}
</section>
