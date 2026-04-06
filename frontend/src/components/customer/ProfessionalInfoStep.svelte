<script lang="ts">
  interface Props {
    data: {
      profession: string;
      employer: string;
      monthly_income: string;
      source_of_funds: string;
    };
    onnext: () => void;
    onprev: () => void;
  }

  let { data = $bindable(), onnext, onprev }: Props = $props();

  let formValid = $derived(
    data.profession.length > 0 && data.monthly_income.length > 0,
  );

  function handleNext(e: Event) {
    e.preventDefault();
    if (formValid) onnext();
  }
</script>

<form onsubmit={handleNext} class="space-y-4" novalidate>
  <div>
    <label for="kyc-profession" class="block text-sm font-medium text-gray-700">
      Profession <span class="text-red-500" aria-hidden="true">*</span>
    </label>
    <input
      id="kyc-profession"
      type="text"
      required
      aria-required="true"
      bind:value={data.profession}
      class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 sm:text-sm"
    />
  </div>

  <div>
    <label for="kyc-employer" class="block text-sm font-medium text-gray-700">
      Employeur
    </label>
    <input
      id="kyc-employer"
      type="text"
      bind:value={data.employer}
      class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 sm:text-sm"
    />
  </div>

  <div>
    <label for="kyc-income" class="block text-sm font-medium text-gray-700">
      Revenu mensuel (MAD) <span class="text-red-500" aria-hidden="true">*</span>
    </label>
    <input
      id="kyc-income"
      type="number"
      min="0"
      required
      aria-required="true"
      bind:value={data.monthly_income}
      class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 sm:text-sm"
    />
  </div>

  <div>
    <label for="kyc-funds" class="block text-sm font-medium text-gray-700">
      Origine des fonds
    </label>
    <select
      id="kyc-funds"
      bind:value={data.source_of_funds}
      class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 sm:text-sm"
    >
      <option value="">-- Choisir --</option>
      <option value="salary">Salaire</option>
      <option value="business">Activite commerciale</option>
      <option value="inheritance">Heritage</option>
      <option value="savings">Epargne</option>
      <option value="other">Autre</option>
    </select>
  </div>

  <div class="flex justify-between pt-4">
    <button
      type="button"
      onclick={onprev}
      class="rounded-md border border-gray-300 bg-white px-6 py-2 text-sm font-semibold text-gray-700 shadow-sm hover:bg-gray-50 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600"
    >
      Precedent
    </button>
    <button
      type="submit"
      disabled={!formValid}
      class="rounded-md bg-blue-600 px-6 py-2 text-sm font-semibold text-white shadow-sm hover:bg-blue-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600 disabled:cursor-not-allowed disabled:opacity-50"
    >
      Suivant
    </button>
  </div>
</form>
