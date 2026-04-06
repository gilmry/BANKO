<script lang="ts">
  import { onMount } from 'svelte';

  const API_BASE = '/api/v1/bct';

  // --- State ---
  let stats: any = null;
  let dailyTrend: any[] = [];
  let suspicious: any[] = [];
  let entries: any[] = [];
  let pagination = { total: 0, page: 1, total_pages: 0, limit: 20 };
  let loading = true;
  let error = '';

  // --- Filters ---
  let filterAction = '';
  let filterResourceType = '';
  let filterDateFrom = '';
  let filterDateTo = '';
  let currentPage = 1;

  async function apiFetch(url: string) {
    const token = localStorage.getItem('banko_token') || '';
    const res = await fetch(url, {
      headers: { Authorization: `Bearer ${token}` },
    });
    if (!res.ok) throw new Error(`API error: ${res.status}`);
    return res.json();
  }

  async function loadDashboard() {
    loading = true;
    error = '';
    try {
      const [statsData, trendData, suspiciousData] = await Promise.all([
        apiFetch(`${API_BASE}/dashboard/stats`),
        apiFetch(`${API_BASE}/dashboard/daily-trend?days=30`),
        apiFetch(`${API_BASE}/dashboard/suspicious`),
      ]);
      stats = statsData;
      dailyTrend = trendData;
      suspicious = suspiciousData;
    } catch (e: any) {
      error = e.message || 'Failed to load dashboard';
    }
    loading = false;
  }

  async function loadEntries() {
    loading = true;
    error = '';
    try {
      const params = new URLSearchParams();
      params.set('page', String(currentPage));
      params.set('limit', '20');
      if (filterAction) params.set('action', filterAction);
      if (filterResourceType) params.set('resource_type', filterResourceType);
      if (filterDateFrom) params.set('date_from', filterDateFrom);
      if (filterDateTo) params.set('date_to', filterDateTo);

      const data = await apiFetch(`${API_BASE}/audit/entries?${params}`);
      entries = data.data || [];
      pagination = {
        total: data.total,
        page: data.page,
        total_pages: data.total_pages,
        limit: data.limit,
      };
    } catch (e: any) {
      error = e.message || 'Failed to load entries';
    }
    loading = false;
  }

  async function exportData(format: string) {
    try {
      const params = new URLSearchParams();
      params.set('format', format);
      if (filterAction) params.set('action', filterAction);
      if (filterResourceType) params.set('resource_type', filterResourceType);
      if (filterDateFrom) params.set('date_from', filterDateFrom);
      if (filterDateTo) params.set('date_to', filterDateTo);

      const data = await apiFetch(`${API_BASE}/audit/entries/export?${params}`);

      const blob = new Blob(
        [data.data],
        { type: format === 'csv' ? 'text/csv' : 'application/json' }
      );
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `audit_export.${format}`;
      a.click();
      URL.revokeObjectURL(url);
    } catch (e: any) {
      error = e.message || 'Export failed';
    }
  }

  function applyFilters() {
    currentPage = 1;
    loadEntries();
  }

  function goToPage(page: number) {
    currentPage = page;
    loadEntries();
  }

  onMount(() => {
    loadDashboard();
    loadEntries();
  });
</script>

<div class="space-y-8">
  <h1 class="text-2xl font-bold text-gray-900">BCT Audit Dashboard</h1>

  {#if error}
    <div class="rounded-md bg-red-50 p-4 text-red-700">{error}</div>
  {/if}

  <!-- Stats Cards -->
  {#if stats}
    <div class="grid grid-cols-1 gap-4 sm:grid-cols-3">
      <div class="rounded-lg bg-white p-6 shadow">
        <p class="text-sm text-gray-500">Total Entries</p>
        <p class="text-3xl font-bold text-gray-900">{stats.total_entries.toLocaleString()}</p>
      </div>
      <div class="rounded-lg bg-white p-6 shadow">
        <p class="text-sm text-gray-500">Today</p>
        <p class="text-3xl font-bold text-blue-600">{stats.entries_today.toLocaleString()}</p>
      </div>
      <div class="rounded-lg bg-white p-6 shadow">
        <p class="text-sm text-gray-500">This Week</p>
        <p class="text-3xl font-bold text-green-600">{stats.entries_this_week.toLocaleString()}</p>
      </div>
    </div>

    <!-- Actions Breakdown -->
    {#if stats.actions_breakdown && stats.actions_breakdown.length > 0}
      <div class="rounded-lg bg-white p-6 shadow">
        <h2 class="mb-4 text-lg font-semibold">Actions Breakdown (7 days)</h2>
        <div class="flex flex-wrap gap-3">
          {#each stats.actions_breakdown as ab}
            <span class="inline-flex items-center rounded-full bg-gray-100 px-3 py-1 text-sm">
              {ab.action}: <strong class="ml-1">{ab.count}</strong>
            </span>
          {/each}
        </div>
      </div>
    {/if}

    <!-- Top Actors -->
    {#if stats.top_actors && stats.top_actors.length > 0}
      <div class="rounded-lg bg-white p-6 shadow">
        <h2 class="mb-4 text-lg font-semibold">Top Actors (7 days)</h2>
        <div class="space-y-2">
          {#each stats.top_actors as actor}
            <div class="flex justify-between text-sm">
              <span class="font-mono text-gray-600">{actor.user_id.substring(0, 8)}...</span>
              <span class="font-semibold">{actor.count}</span>
            </div>
          {/each}
        </div>
      </div>
    {/if}
  {/if}

  <!-- Daily Trend -->
  {#if dailyTrend.length > 0}
    <div class="rounded-lg bg-white p-6 shadow">
      <h2 class="mb-4 text-lg font-semibold">Daily Trend (30 days)</h2>
      <div class="flex items-end gap-1" style="height:120px;">
        {#each dailyTrend as day}
          {@const maxCount = Math.max(...dailyTrend.map(d => d.count), 1)}
          <div
            class="flex-1 rounded-t bg-blue-500"
            style="height: {Math.max((day.count / maxCount) * 100, 2)}%"
            title="{day.date}: {day.count}"
          ></div>
        {/each}
      </div>
    </div>
  {/if}

  <!-- Suspicious Activities -->
  {#if suspicious.length > 0}
    <div class="rounded-lg border border-red-200 bg-red-50 p-6">
      <h2 class="mb-4 text-lg font-semibold text-red-800">Suspicious Activities</h2>
      <div class="overflow-x-auto">
        <table class="w-full text-left text-sm">
          <thead>
            <tr class="border-b text-gray-500">
              <th class="pb-2">Time</th>
              <th class="pb-2">User</th>
              <th class="pb-2">Action</th>
              <th class="pb-2">Resource</th>
            </tr>
          </thead>
          <tbody>
            {#each suspicious.slice(0, 10) as s}
              <tr class="border-b border-red-100">
                <td class="py-1">{new Date(s.timestamp).toLocaleString()}</td>
                <td class="py-1 font-mono text-xs">{s.user_id.substring(0, 8)}...</td>
                <td class="py-1 font-semibold text-red-700">{s.action}</td>
                <td class="py-1">{s.resource_type}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>
  {/if}

  <!-- Audit Entries Table with Filters -->
  <div class="rounded-lg bg-white p-6 shadow">
    <h2 class="mb-4 text-lg font-semibold">Audit Entries</h2>

    <!-- Filters -->
    <div class="mb-4 grid grid-cols-1 gap-3 sm:grid-cols-5">
      <select bind:value={filterAction} class="rounded border px-3 py-2 text-sm">
        <option value="">All Actions</option>
        <option value="Create">Create</option>
        <option value="Read">Read</option>
        <option value="Update">Update</option>
        <option value="Delete">Delete</option>
        <option value="Login">Login</option>
        <option value="Logout">Logout</option>
        <option value="Approve">Approve</option>
        <option value="Reject">Reject</option>
        <option value="Submit">Submit</option>
        <option value="Export">Export</option>
      </select>
      <select bind:value={filterResourceType} class="rounded border px-3 py-2 text-sm">
        <option value="">All Resources</option>
        <option value="Customer">Customer</option>
        <option value="Account">Account</option>
        <option value="Loan">Loan</option>
        <option value="Transaction">Transaction</option>
        <option value="Payment">Payment</option>
        <option value="User">User</option>
        <option value="System">System</option>
      </select>
      <input type="date" bind:value={filterDateFrom} class="rounded border px-3 py-2 text-sm" placeholder="From" />
      <input type="date" bind:value={filterDateTo} class="rounded border px-3 py-2 text-sm" placeholder="To" />
      <button on:click={applyFilters} class="rounded bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700">
        Filter
      </button>
    </div>

    <!-- Export Buttons -->
    <div class="mb-4 flex gap-2">
      <button on:click={() => exportData('csv')} class="rounded border px-3 py-1 text-sm hover:bg-gray-50">
        Export CSV
      </button>
      <button on:click={() => exportData('json')} class="rounded border px-3 py-1 text-sm hover:bg-gray-50">
        Export JSON
      </button>
    </div>

    <!-- Table -->
    {#if loading}
      <p class="text-gray-500">Loading...</p>
    {:else}
      <div class="overflow-x-auto">
        <table class="w-full text-left text-sm">
          <thead>
            <tr class="border-b text-gray-500">
              <th class="pb-2">Timestamp</th>
              <th class="pb-2">User</th>
              <th class="pb-2">Action</th>
              <th class="pb-2">Resource</th>
              <th class="pb-2">Resource ID</th>
              <th class="pb-2">IP</th>
            </tr>
          </thead>
          <tbody>
            {#each entries as entry}
              <tr class="border-b hover:bg-gray-50">
                <td class="py-2">{new Date(entry.timestamp).toLocaleString()}</td>
                <td class="py-2 font-mono text-xs">{entry.user_id.substring(0, 8)}...</td>
                <td class="py-2">{entry.action}</td>
                <td class="py-2">{entry.resource_type}</td>
                <td class="py-2 font-mono text-xs">{entry.resource_id.substring(0, 8)}...</td>
                <td class="py-2 text-gray-500">{entry.ip_address || '-'}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>

      <!-- Pagination -->
      {#if pagination.total_pages > 1}
        <div class="mt-4 flex items-center justify-between">
          <span class="text-sm text-gray-500">
            Page {pagination.page} of {pagination.total_pages} ({pagination.total} entries)
          </span>
          <div class="flex gap-2">
            <button
              on:click={() => goToPage(pagination.page - 1)}
              disabled={pagination.page <= 1}
              class="rounded border px-3 py-1 text-sm disabled:opacity-50"
            >
              Previous
            </button>
            <button
              on:click={() => goToPage(pagination.page + 1)}
              disabled={pagination.page >= pagination.total_pages}
              class="rounded border px-3 py-1 text-sm disabled:opacity-50"
            >
              Next
            </button>
          </div>
        </div>
      {/if}
    {/if}
  </div>
</div>
