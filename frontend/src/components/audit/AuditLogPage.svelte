<script lang="ts">
  import { auditApi, type AuditEntry, type AuditFilter as AuditFilterParams } from '../../lib/api/audit';
  import AuditFilter from './AuditFilter.svelte';
  import AuditTable from './AuditTable.svelte';
  import ExportButton from './ExportButton.svelte';

  let entries = $state<AuditEntry[]>([]);
  let loading = $state(true);
  let error = $state('');
  let page = $state(1);
  let totalPages = $state(1);
  let total = $state(0);
  let sortField = $state('timestamp');
  let sortDir = $state<'asc' | 'desc'>('desc');

  // Filter state
  let filterAction = $state('');
  let filterResourceType = $state('');
  let filterDateFrom = $state('');
  let filterDateTo = $state('');
  let filterActorId = $state('');

  let currentFilters = $derived<AuditFilterParams>({
    page,
    limit: 20,
    action: filterAction || undefined,
    resource_type: filterResourceType || undefined,
    date_from: filterDateFrom || undefined,
    date_to: filterDateTo || undefined,
    actor_id: filterActorId || undefined,
  });

  async function loadEntries() {
    loading = true;
    error = '';
    try {
      const res = await auditApi.list(currentFilters);
      entries = res.data;
      totalPages = res.total_pages;
      total = res.total;
    } catch (err: unknown) {
      error = err instanceof Error ? err.message : 'Erreur de chargement';
    } finally {
      loading = false;
    }
  }

  function handleFilter() {
    page = 1;
    loadEntries();
  }

  function handleClear() {
    page = 1;
    loadEntries();
  }

  function handleSort(field: string) {
    if (sortField === field) {
      sortDir = sortDir === 'asc' ? 'desc' : 'asc';
    } else {
      sortField = field;
      sortDir = 'desc';
    }
    // Client-side sort since server pagination handles data
    entries = [...entries].sort((a, b) => {
      const aVal = (a as Record<string, unknown>)[field] as string;
      const bVal = (b as Record<string, unknown>)[field] as string;
      const cmp = aVal < bVal ? -1 : aVal > bVal ? 1 : 0;
      return sortDir === 'asc' ? cmp : -cmp;
    });
  }

  function handlePage(p: number) {
    page = p;
    loadEntries();
  }

  $effect(() => {
    loadEntries();
  });
</script>

<div class="space-y-6">
  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-bold text-gray-900" data-testid="audit-heading">Journal d'audit</h1>
    <ExportButton filters={currentFilters} />
  </div>

  {#if error}
    <div role="alert" aria-live="polite" class="rounded-md border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-700" data-testid="audit-error">
      {error}
    </div>
  {/if}

  <AuditFilter
    bind:action={filterAction}
    bind:resourceType={filterResourceType}
    bind:dateFrom={filterDateFrom}
    bind:dateTo={filterDateTo}
    bind:actorId={filterActorId}
    onfilter={handleFilter}
    onclear={handleClear}
  />

  <AuditTable
    {entries}
    {page}
    {totalPages}
    {total}
    {loading}
    {sortField}
    {sortDir}
    onsort={handleSort}
    onpage={handlePage}
  />
</div>
