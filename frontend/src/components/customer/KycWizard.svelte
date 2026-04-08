<script lang="ts">
  import KycStepper from './KycStepper.svelte';
  import BasicInfoStep from './BasicInfoStep.svelte';
  import ProfessionalInfoStep from './ProfessionalInfoStep.svelte';
  import BeneficiaryStep from './BeneficiaryStep.svelte';
  import DocumentUpload from './DocumentUpload.svelte';
  import KycSummary from './KycSummary.svelte';
  import { customersApi } from '../../lib/api/customers';

  const TOTAL_STEPS = 5;
  let currentStep = $state(1);
  let submitting = $state(false);
  let error = $state('');
  let success = $state(false);

  let basicInfo = $state({
    first_name: '',
    last_name: '',
    date_of_birth: '',
    gender: '',
    cin: '',
    nationality: '',
  });

  let professionalInfo = $state({
    profession: '',
    employer: '',
    monthly_income: '',
    source_of_funds: '',
  });

  let beneficiaries = $state<Array<{ name: string; relationship: string; ownership_percentage: number }>>([]);

  let documents = $state<Array<{ name: string; type: string; preview?: string }>>([]);

  function nextStep() {
    if (currentStep < TOTAL_STEPS) currentStep++;
  }

  function prevStep() {
    if (currentStep > 1) currentStep--;
  }

  async function handleSubmit() {
    submitting = true;
    error = '';
    try {
      await customersApi.create({
        ...basicInfo,
        profession: professionalInfo.profession,
        employer: professionalInfo.employer,
        monthly_income: professionalInfo.monthly_income ? Number(professionalInfo.monthly_income) : undefined,
        source_of_funds: professionalInfo.source_of_funds || undefined,
        beneficiaries: beneficiaries.length > 0 ? beneficiaries : undefined,
      });
      success = true;
    } catch (err: unknown) {
      error = err instanceof Error ? err.message : "Erreur lors de l'enregistrement";
    } finally {
      submitting = false;
    }
  }
</script>

<div class="space-y-6" data-testid="kyc-wizard">
  {#if success}
    <div class="rounded-md border border-green-200 bg-green-50 p-6 text-center" role="status" data-testid="kyc-success">
      <svg class="mx-auto mb-3 h-12 w-12 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
      </svg>
      <p class="text-lg font-semibold text-green-800">Enregistrement soumis avec succes</p>
      <a href="/accounts" class="mt-4 inline-block text-sm font-medium text-green-600 hover:text-green-500">
        Retour aux comptes
      </a>
    </div>
  {:else}
    <KycStepper {currentStep} totalSteps={TOTAL_STEPS} />

    {#if error}
      <div role="alert" aria-live="polite" data-testid="kyc-error" class="rounded-md border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-700">
        {error}
      </div>
    {/if}

    <div aria-live="polite">
      {#if currentStep === 1}
        <BasicInfoStep bind:data={basicInfo} onnext={nextStep} />
      {:else if currentStep === 2}
        <ProfessionalInfoStep bind:data={professionalInfo} onnext={nextStep} onprev={prevStep} />
      {:else if currentStep === 3}
        <BeneficiaryStep bind:beneficiaries={beneficiaries} onnext={nextStep} onprev={prevStep} />
      {:else if currentStep === 4}
        <DocumentUpload bind:documents={documents} onnext={nextStep} onprev={prevStep} />
      {:else if currentStep === 5}
        <KycSummary
          {basicInfo}
          {professionalInfo}
          {beneficiaries}
          {documents}
          onsubmit={handleSubmit}
          onprev={prevStep}
          {submitting}
        />
      {/if}
    </div>
  {/if}
</div>
