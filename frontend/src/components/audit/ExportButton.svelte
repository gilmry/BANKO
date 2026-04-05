<script lang="ts">
  import { auditApi, type AuditFilter } from '../../lib/api/audit';

  interface Props {
    filters: AuditFilter;
  }

  let { filters }: Props = $props();

  let exporting = $state(false);
  let error = $state('');

  async function exportData(format: 'csv' | 'json') {
    exporting = true;
    error = '';
    try {
      const result = await auditApi.exportEntries(format, filters);
      const mimeType = format === 'csv' ? 'text/csv' : 'application/json';
      const blob = new Blob([result.data], { type: mimeType });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `audit_export.${format}`;
      a.click();
      URL.revokeObjectURL(url);
    } catch (err: unknown) {
      error = err instanceof Error ? err.message : "Erreur lors de l'export";
    } finally {
      exporting = false;
    }
  }
</script>

<div class="flex items-center gap-2">
  <button
    type="button"
    onclick={() => exportData('csv')}
    disabled={exporting}
    class="rounded-md border border-gray-300 bg-white px-3 py-1.5 text-sm font-medium text-gray-700 hover:bg-gray-50 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600 disabled:opacity-50"
    aria-label="Exporter au format CSV"
  >
    Exporter CSV
  </button>
  <button
    type="button"
    onclick={() => exportData('json')}
    disabled={exporting}
    class="rounded-md border border-gray-300 bg-white px-3 py-1.5 text-sm font-medium text-gray-700 hover:bg-gray-50 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600 disabled:opacity-50"
    aria-label="Exporter au format JSON"
  >
    Exporter JSON
  </button>
  {#if error}
    <span class="text-sm text-red-600" role="alert">{error}</span>
  {/if}
</div>
