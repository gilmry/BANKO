<script lang="ts">
  import type { AuditEntry } from '../../lib/api/audit';

  interface Props {
    entries: AuditEntry[];
    page: number;
    totalPages: number;
    total: number;
    loading: boolean;
    sortField: string;
    sortDir: 'asc' | 'desc';
    onsort: (field: string) => void;
    onpage: (page: number) => void;
  }

  let { entries, page, totalPages, total, loading, sortField, sortDir, onsort, onpage }: Props = $props();

  function sortLabel(field: string): 'ascending' | 'descending' | 'none' {
    if (sortField !== field) return 'none';
    return sortDir === 'asc' ? 'ascending' : 'descending';
  }

  function sortIndicator(field: string): string {
    if (sortField !== field) return '';
    return sortDir === 'asc' ? ' ↑' : ' ↓';
  }
</script>

<div class="rounded-lg border border-gray-200 bg-white">
  {#if loading}
    <div class="p-6 text-center text-sm text-gray-500" role="status">
      Chargement...
    </div>
  {:else if entries.length === 0}
    <div class="p-6 text-center text-sm text-gray-500">
      Aucune entree d'audit trouvee
    </div>
  {:else}
    <div class="overflow-x-auto">
      <table class="w-full text-sm">
        <caption class="sr-only">Journal d'audit - {total} entrees</caption>
        <thead class="bg-gray-50">
          <tr>
            <th
              scope="col"
              class="cursor-pointer px-4 py-3 text-start text-sm font-medium text-gray-500 hover:text-gray-700"
              aria-sort={sortLabel('timestamp')}
              onclick={() => onsort('timestamp')}
              onkeydown={(e) => e.key === 'Enter' && onsort('timestamp')}
              tabindex={0}
              role="columnheader"
            >
              Horodatage{sortIndicator('timestamp')}
            </th>
            <th
              scope="col"
              class="cursor-pointer px-4 py-3 text-start text-sm font-medium text-gray-500 hover:text-gray-700"
              aria-sort={sortLabel('user_id')}
              onclick={() => onsort('user_id')}
              onkeydown={(e) => e.key === 'Enter' && onsort('user_id')}
              tabindex={0}
              role="columnheader"
            >
              Acteur{sortIndicator('user_id')}
            </th>
            <th
              scope="col"
              class="cursor-pointer px-4 py-3 text-start text-sm font-medium text-gray-500 hover:text-gray-700"
              aria-sort={sortLabel('action')}
              onclick={() => onsort('action')}
              onkeydown={(e) => e.key === 'Enter' && onsort('action')}
              tabindex={0}
              role="columnheader"
            >
              Action{sortIndicator('action')}
            </th>
            <th scope="col" class="px-4 py-3 text-start text-sm font-medium text-gray-500">
              Type de ressource
            </th>
            <th scope="col" class="px-4 py-3 text-start text-sm font-medium text-gray-500">
              ID ressource
            </th>
          </tr>
        </thead>
        <tbody class="divide-y divide-gray-200">
          {#each entries as entry}
            <tr class="hover:bg-gray-50">
              <td class="whitespace-nowrap px-4 py-3 text-gray-700">
                {new Date(entry.timestamp).toLocaleString('fr-FR')}
              </td>
              <td class="px-4 py-3 font-mono text-xs text-gray-600">
                {entry.user_id.length > 8 ? entry.user_id.substring(0, 8) + '...' : entry.user_id}
              </td>
              <td class="px-4 py-3">
                <span class="inline-flex rounded-full bg-gray-100 px-2 py-0.5 text-xs font-medium text-gray-700">
                  {entry.action}
                </span>
              </td>
              <td class="px-4 py-3 text-gray-700">
                {entry.resource_type}
              </td>
              <td class="px-4 py-3 font-mono text-xs text-gray-600">
                {entry.resource_id.length > 8 ? entry.resource_id.substring(0, 8) + '...' : entry.resource_id}
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>

    <!-- Pagination -->
    {#if totalPages > 1}
      <div class="flex items-center justify-between border-t border-gray-200 px-4 py-3">
        <span class="text-sm text-gray-500">
          Page {page} sur {totalPages} ({total} entrees)
        </span>
        <div class="flex gap-2">
          <button
            type="button"
            onclick={() => onpage(page - 1)}
            disabled={page <= 1}
            class="rounded-md border border-gray-300 bg-white px-3 py-1 text-sm text-gray-700 hover:bg-gray-50 disabled:cursor-not-allowed disabled:opacity-50 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600"
            aria-label="Page precedente"
          >
            Precedent
          </button>
          <button
            type="button"
            onclick={() => onpage(page + 1)}
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
</div>
