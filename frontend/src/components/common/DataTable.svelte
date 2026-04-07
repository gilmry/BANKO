<script lang="ts">
  interface Column<T> {
    key: keyof T;
    label: string;
    sortable?: boolean;
    render?: (value: unknown, row: T) => string;
  }

  interface Props<T> {
    columns: Column<T>[];
    rows: T[];
    loading?: boolean;
    sortBy?: keyof T;
    sortOrder?: 'asc' | 'desc';
    onSort?: (key: keyof T) => void;
    rowLink?: (row: T) => string;
  }

  type T = $$Generic;

  const { columns, rows, loading = false, sortBy, sortOrder = 'asc', onSort, rowLink }: Props<T> = $props();

  function getSortIcon(column: Column<T>) {
    if (!column.sortable || sortBy !== column.key) {
      return '⇅';
    }
    return sortOrder === 'asc' ? '↑' : '↓';
  }

  function handleSort(column: Column<T>) {
    if (column.sortable) {
      onSort?.(column.key);
    }
  }
</script>

<div class="overflow-x-auto rounded-lg border border-gray-200">
  <table class="w-full divide-y divide-gray-200 text-sm">
    <thead class="bg-gray-50">
      <tr>
        {#each columns as column (column.key)}
          <th
            scope="col"
            class="px-6 py-3 text-left font-medium text-gray-900 {column.sortable
              ? 'cursor-pointer hover:bg-gray-100 select-none'
              : ''}"
            onclick={() => handleSort(column)}
          >
            <div class="flex items-center gap-2">
              {column.label}
              {#if column.sortable}
                <span class="text-xs text-gray-400">
                  {getSortIcon(column)}
                </span>
              {/if}
            </div>
          </th>
        {/each}
      </tr>
    </thead>

    <tbody class="divide-y divide-gray-200 bg-white">
      {#if loading}
        <tr>
          <td colspan={columns.length} class="px-6 py-4 text-center text-gray-500">
            Chargement...
          </td>
        </tr>
      {:else if rows.length === 0}
        <tr>
          <td colspan={columns.length} class="px-6 py-4 text-center text-gray-500">
            Aucun résultat trouvé
          </td>
        </tr>
      {:else}
        {#each rows as row (row)}
          {@const href = rowLink?.(row)}
          <tr class={href ? 'hover:bg-gray-50 cursor-pointer' : ''}>
            {#each columns as column (column.key)}
              <td class="px-6 py-4">
                {#if href}
                  <a {href} class="text-blue-600 hover:text-blue-800 hover:underline">
                    {column.render?.(row[column.key], row) || row[column.key]}
                  </a>
                {:else}
                  {column.render?.(row[column.key], row) || row[column.key]}
                {/if}
              </td>
            {/each}
          </tr>
        {/each}
      {/if}
    </tbody>
  </table>
</div>
