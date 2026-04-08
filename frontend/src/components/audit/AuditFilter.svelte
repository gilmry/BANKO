<script lang="ts">
  interface Props {
    action: string;
    resourceType: string;
    dateFrom: string;
    dateTo: string;
    actorId: string;
    onfilter: () => void;
    onclear: () => void;
  }

  let {
    action = $bindable(),
    resourceType = $bindable(),
    dateFrom = $bindable(),
    dateTo = $bindable(),
    actorId = $bindable(),
    onfilter,
    onclear,
  }: Props = $props();

  function handleSubmit(e: Event) {
    e.preventDefault();
    onfilter();
  }

  function handleClear() {
    action = '';
    resourceType = '';
    dateFrom = '';
    dateTo = '';
    actorId = '';
    onclear();
  }
</script>

<form onsubmit={handleSubmit} class="rounded-lg border border-gray-200 bg-white p-4" aria-label="Filtres d'audit" data-testid="audit-filter-form">
  <div class="grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-5">
    <div>
      <label for="audit-action" class="block text-xs font-medium text-gray-500">Action</label>
      <select
        id="audit-action"
        bind:value={action}
        class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500"
        data-testid="audit-filter-action"
      >
        <option value="">Toutes les actions</option>
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
    </div>

    <div>
      <label for="audit-resource" class="block text-xs font-medium text-gray-500">Type de ressource</label>
      <select
        id="audit-resource"
        bind:value={resourceType}
        class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500"
        data-testid="audit-filter-resource"
      >
        <option value="">Toutes les ressources</option>
        <option value="Customer">Customer</option>
        <option value="Account">Account</option>
        <option value="Loan">Loan</option>
        <option value="Transaction">Transaction</option>
        <option value="Payment">Payment</option>
        <option value="User">User</option>
        <option value="System">System</option>
      </select>
    </div>

    <div>
      <label for="audit-actor" class="block text-xs font-medium text-gray-500">ID Acteur</label>
      <input
        id="audit-actor"
        type="text"
        bind:value={actorId}
        placeholder="UUID..."
        class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500"
        data-testid="audit-filter-actor"
      />
    </div>

    <div>
      <label for="audit-date-from" class="block text-xs font-medium text-gray-500">Date debut</label>
      <input
        id="audit-date-from"
        type="date"
        bind:value={dateFrom}
        class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500"
        data-testid="audit-filter-date-from"
      />
    </div>

    <div>
      <label for="audit-date-to" class="block text-xs font-medium text-gray-500">Date fin</label>
      <input
        id="audit-date-to"
        type="date"
        bind:value={dateTo}
        class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500"
        data-testid="audit-filter-date-to"
      />
    </div>
  </div>

  <div class="mt-3 flex items-center gap-2">
    <button
      type="submit"
      class="rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600"
      data-testid="audit-filter-submit"
    >
      Filtrer
    </button>
    <button
      type="button"
      onclick={handleClear}
      class="rounded-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600"
      data-testid="audit-filter-clear"
    >
      Effacer
    </button>
  </div>
</form>
