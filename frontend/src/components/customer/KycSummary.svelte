<script lang="ts">
  interface Beneficiary {
    name: string;
    relationship: string;
    ownership_percentage: number;
  }

  interface UploadedDoc {
    name: string;
    type: string;
    preview?: string;
  }

  interface Props {
    basicInfo: {
      first_name: string;
      last_name: string;
      date_of_birth: string;
      gender: string;
      cin: string;
      nationality: string;
    };
    professionalInfo: {
      profession: string;
      employer: string;
      monthly_income: string;
      source_of_funds: string;
    };
    beneficiaries: Beneficiary[];
    documents: UploadedDoc[];
    onsubmit: () => void;
    onprev: () => void;
    submitting: boolean;
  }

  let { basicInfo, professionalInfo, beneficiaries, documents, onsubmit, onprev, submitting }: Props = $props();

  const genderLabel: Record<string, string> = { male: 'Homme', female: 'Femme' };
  const fundsLabel: Record<string, string> = {
    salary: 'Salaire',
    business: 'Activite commerciale',
    inheritance: 'Heritage',
    savings: 'Epargne',
    other: 'Autre',
  };
</script>

<div class="space-y-6">
  <h3 class="text-lg font-medium text-gray-900">Recapitulatif</h3>

  <!-- Personal info -->
  <section aria-labelledby="summary-personal" class="rounded-lg border border-gray-200 bg-white p-4">
    <h4 id="summary-personal" class="mb-3 text-sm font-semibold text-gray-500 uppercase tracking-wide">
      Informations personnelles
    </h4>
    <dl class="grid grid-cols-1 gap-x-4 gap-y-2 sm:grid-cols-2">
      <div>
        <dt class="text-sm text-gray-500">Prenom</dt>
        <dd class="text-sm font-medium text-gray-900">{basicInfo.first_name}</dd>
      </div>
      <div>
        <dt class="text-sm text-gray-500">Nom</dt>
        <dd class="text-sm font-medium text-gray-900">{basicInfo.last_name}</dd>
      </div>
      <div>
        <dt class="text-sm text-gray-500">Date de naissance</dt>
        <dd class="text-sm font-medium text-gray-900">{basicInfo.date_of_birth}</dd>
      </div>
      <div>
        <dt class="text-sm text-gray-500">Genre</dt>
        <dd class="text-sm font-medium text-gray-900">{genderLabel[basicInfo.gender] ?? basicInfo.gender}</dd>
      </div>
      <div>
        <dt class="text-sm text-gray-500">CIN</dt>
        <dd class="text-sm font-medium text-gray-900">{basicInfo.cin}</dd>
      </div>
      <div>
        <dt class="text-sm text-gray-500">Nationalite</dt>
        <dd class="text-sm font-medium text-gray-900">{basicInfo.nationality}</dd>
      </div>
    </dl>
  </section>

  <!-- Professional info -->
  <section aria-labelledby="summary-professional" class="rounded-lg border border-gray-200 bg-white p-4">
    <h4 id="summary-professional" class="mb-3 text-sm font-semibold text-gray-500 uppercase tracking-wide">
      Informations professionnelles
    </h4>
    <dl class="grid grid-cols-1 gap-x-4 gap-y-2 sm:grid-cols-2">
      <div>
        <dt class="text-sm text-gray-500">Profession</dt>
        <dd class="text-sm font-medium text-gray-900">{professionalInfo.profession}</dd>
      </div>
      <div>
        <dt class="text-sm text-gray-500">Employeur</dt>
        <dd class="text-sm font-medium text-gray-900">{professionalInfo.employer || '-'}</dd>
      </div>
      <div>
        <dt class="text-sm text-gray-500">Revenu mensuel</dt>
        <dd class="text-sm font-medium text-gray-900">{professionalInfo.monthly_income} MAD</dd>
      </div>
      <div>
        <dt class="text-sm text-gray-500">Origine des fonds</dt>
        <dd class="text-sm font-medium text-gray-900">{fundsLabel[professionalInfo.source_of_funds] ?? (professionalInfo.source_of_funds || '-')}</dd>
      </div>
    </dl>
  </section>

  <!-- Beneficiaries -->
  {#if beneficiaries.length > 0}
    <section aria-labelledby="summary-beneficiaries" class="rounded-lg border border-gray-200 bg-white p-4">
      <h4 id="summary-beneficiaries" class="mb-3 text-sm font-semibold text-gray-500 uppercase tracking-wide">
        Beneficiaires effectifs ({beneficiaries.length})
      </h4>
      <ul class="space-y-1">
        {#each beneficiaries as b}
          <li class="text-sm text-gray-700">
            {b.name} — {b.relationship} — {b.ownership_percentage}%
          </li>
        {/each}
      </ul>
    </section>
  {/if}

  <!-- Documents -->
  <section aria-labelledby="summary-documents" class="rounded-lg border border-gray-200 bg-white p-4">
    <h4 id="summary-documents" class="mb-3 text-sm font-semibold text-gray-500 uppercase tracking-wide">
      Documents ({documents.length})
    </h4>
    {#if documents.length === 0}
      <p class="text-sm text-gray-500">Aucun document telecharge</p>
    {:else}
      <ul class="space-y-1">
        {#each documents as doc}
          <li class="text-sm text-gray-700">{doc.name}</li>
        {/each}
      </ul>
    {/if}
  </section>

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
      onclick={onsubmit}
      disabled={submitting}
      class="rounded-md bg-green-600 px-6 py-2 text-sm font-semibold text-white shadow-sm hover:bg-green-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-green-600 disabled:opacity-50"
    >
      {submitting ? 'Envoi en cours...' : 'Confirmer et soumettre'}
    </button>
  </div>
</div>
