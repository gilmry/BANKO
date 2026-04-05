<script lang="ts">
  interface Props {
    data: {
      first_name: string;
      last_name: string;
      date_of_birth: string;
      gender: string;
      cin: string;
      nationality: string;
    };
    onnext: () => void;
  }

  let { data = $bindable(), onnext }: Props = $props();

  let formValid = $derived(
    data.first_name.length > 0 &&
      data.last_name.length > 0 &&
      data.date_of_birth.length > 0 &&
      data.gender.length > 0 &&
      data.cin.length > 0 &&
      data.nationality.length > 0,
  );

  function handleNext(e: Event) {
    e.preventDefault();
    if (formValid) onnext();
  }
</script>

<form onsubmit={handleNext} class="space-y-4" novalidate>
  <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
    <div>
      <label for="kyc-firstname" class="block text-sm font-medium text-gray-700">
        <!-- TODO: use t('customer.basicInfo.firstName') -->
        Prenom <span class="text-red-500" aria-hidden="true">*</span>
      </label>
      <input
        id="kyc-firstname"
        type="text"
        required
        aria-required="true"
        bind:value={data.first_name}
        class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 sm:text-sm"
      />
    </div>
    <div>
      <label for="kyc-lastname" class="block text-sm font-medium text-gray-700">
        Nom <span class="text-red-500" aria-hidden="true">*</span>
      </label>
      <input
        id="kyc-lastname"
        type="text"
        required
        aria-required="true"
        bind:value={data.last_name}
        class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 sm:text-sm"
      />
    </div>
  </div>

  <div>
    <label for="kyc-dob" class="block text-sm font-medium text-gray-700">
      Date de naissance <span class="text-red-500" aria-hidden="true">*</span>
    </label>
    <input
      id="kyc-dob"
      type="date"
      required
      aria-required="true"
      bind:value={data.date_of_birth}
      class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 sm:text-sm"
    />
  </div>

  <div>
    <label for="kyc-gender" class="block text-sm font-medium text-gray-700">
      Genre <span class="text-red-500" aria-hidden="true">*</span>
    </label>
    <select
      id="kyc-gender"
      required
      aria-required="true"
      bind:value={data.gender}
      class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 sm:text-sm"
    >
      <option value="">-- Choisir --</option>
      <option value="male">Homme</option>
      <option value="female">Femme</option>
    </select>
  </div>

  <div>
    <label for="kyc-cin" class="block text-sm font-medium text-gray-700">
      Numero CIN <span class="text-red-500" aria-hidden="true">*</span>
    </label>
    <input
      id="kyc-cin"
      type="text"
      required
      aria-required="true"
      bind:value={data.cin}
      class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 sm:text-sm"
    />
  </div>

  <div>
    <label for="kyc-nationality" class="block text-sm font-medium text-gray-700">
      Nationalite <span class="text-red-500" aria-hidden="true">*</span>
    </label>
    <input
      id="kyc-nationality"
      type="text"
      required
      aria-required="true"
      bind:value={data.nationality}
      class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 sm:text-sm"
    />
  </div>

  <div class="flex justify-end pt-4">
    <button
      type="submit"
      disabled={!formValid}
      class="rounded-md bg-blue-600 px-6 py-2 text-sm font-semibold text-white shadow-sm hover:bg-blue-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600 disabled:cursor-not-allowed disabled:opacity-50"
    >
      Suivant
    </button>
  </div>
</form>
