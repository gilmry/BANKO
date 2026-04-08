<script lang="ts">
  interface Beneficiary {
    name: string;
    relationship: string;
    ownership_percentage: number;
  }

  interface Props {
    beneficiaries: Beneficiary[];
    onnext: () => void;
    onprev: () => void;
  }

  let { beneficiaries = $bindable(), onnext, onprev }: Props = $props();

  let showModal = $state(false);
  let newName = $state('');
  let newRelationship = $state('');
  let newOwnership = $state(0);

  function addBeneficiary() {
    if (newName.length === 0 || newOwnership <= 0) return;
    beneficiaries = [
      ...beneficiaries,
      { name: newName, relationship: newRelationship, ownership_percentage: newOwnership },
    ];
    newName = '';
    newRelationship = '';
    newOwnership = 0;
    showModal = false;
  }

  function removeBeneficiary(index: number) {
    beneficiaries = beneficiaries.filter((_, i) => i !== index);
  }

  function handleNext() {
    onnext();
  }
</script>

<div class="space-y-4">
  <div class="flex items-center justify-between">
    <h3 class="text-lg font-medium text-gray-900">
      Beneficiaires effectifs
    </h3>
    <button
      type="button"
      onclick={() => (showModal = true)}
      class="rounded-md bg-blue-600 px-4 py-2 text-sm font-semibold text-white shadow-sm hover:bg-blue-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600"
      aria-label="Ajouter un beneficiaire effectif"
      data-testid="kyc-beneficiary-add-btn"
    >
      + Ajouter
    </button>
  </div>

  {#if beneficiaries.length === 0}
    <p class="rounded-md border border-gray-200 bg-gray-50 p-6 text-center text-sm text-gray-500">
      Aucun beneficiaire effectif ajoute. Vous pouvez passer cette etape.
    </p>
  {:else}
    <div class="overflow-hidden rounded-lg border border-gray-200">
      <table class="w-full text-sm">
        <caption class="sr-only">Liste des beneficiaires effectifs</caption>
        <thead class="bg-gray-50">
          <tr>
            <th scope="col" class="px-4 py-3 text-start text-sm font-medium text-gray-500">Nom</th>
            <th scope="col" class="px-4 py-3 text-start text-sm font-medium text-gray-500">Relation</th>
            <th scope="col" class="px-4 py-3 text-start text-sm font-medium text-gray-500">Part (%)</th>
            <th scope="col" class="px-4 py-3 text-end text-sm font-medium text-gray-500">Actions</th>
          </tr>
        </thead>
        <tbody>
          {#each beneficiaries as b, i}
            <tr class="border-t border-gray-200">
              <td class="px-4 py-3">{b.name}</td>
              <td class="px-4 py-3">{b.relationship}</td>
              <td class="px-4 py-3">{b.ownership_percentage}%</td>
              <td class="px-4 py-3 text-end">
                <button
                  type="button"
                  onclick={() => removeBeneficiary(i)}
                  class="text-sm text-red-600 hover:text-red-800 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-red-600"
                  aria-label="Retirer {b.name}"
                >
                  Retirer
                </button>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}

  <div class="flex justify-between pt-4">
    <button
      type="button"
      onclick={onprev}
      class="rounded-md border border-gray-300 bg-white px-6 py-2 text-sm font-semibold text-gray-700 shadow-sm hover:bg-gray-50 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600"
    >
      Precedent
    </button>
    <button
      type="button"
      onclick={handleNext}
      class="rounded-md bg-blue-600 px-6 py-2 text-sm font-semibold text-white shadow-sm hover:bg-blue-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600"
    >
      Suivant
    </button>
  </div>
</div>

<!-- Modal -->
{#if showModal}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
    role="dialog"
    aria-modal="true"
    aria-labelledby="beneficiary-modal-title"
  >
    <div class="w-full max-w-md rounded-lg bg-white p-6 shadow-xl">
      <h3 id="beneficiary-modal-title" class="mb-4 text-lg font-semibold text-gray-900">
        Ajouter un beneficiaire
      </h3>
      <div class="space-y-4">
        <div>
          <label for="ben-name" class="block text-sm font-medium text-gray-700">
            Nom complet <span class="text-red-500" aria-hidden="true">*</span>
          </label>
          <input
            id="ben-name"
            type="text"
            required
            aria-required="true"
            bind:value={newName}
            data-testid="kyc-beneficiary-name"
            class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 sm:text-sm"
          />
        </div>
        <div>
          <label for="ben-relationship" class="block text-sm font-medium text-gray-700">
            Relation
          </label>
          <input
            id="ben-relationship"
            type="text"
            bind:value={newRelationship}
            data-testid="kyc-beneficiary-relationship"
            class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 sm:text-sm"
          />
        </div>
        <div>
          <label for="ben-ownership" class="block text-sm font-medium text-gray-700">
            Part de propriete (%) <span class="text-red-500" aria-hidden="true">*</span>
          </label>
          <input
            id="ben-ownership"
            type="number"
            min="0"
            max="100"
            required
            aria-required="true"
            bind:value={newOwnership}
            data-testid="kyc-beneficiary-ownership"
            class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 sm:text-sm"
          />
        </div>
      </div>
      <div class="mt-6 flex justify-end gap-3">
        <button
          type="button"
          onclick={() => (showModal = false)}
          class="rounded-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600"
        >
          Annuler
        </button>
        <button
          type="button"
          onclick={addBeneficiary}
          disabled={newName.length === 0 || newOwnership <= 0}
          class="rounded-md bg-blue-600 px-4 py-2 text-sm font-semibold text-white shadow-sm hover:bg-blue-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600 disabled:opacity-50"
        >
          Ajouter
        </button>
      </div>
    </div>
  </div>
{/if}
