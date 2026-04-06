<script lang="ts">
  interface Props {
    currentStep: number;
    totalSteps: number;
  }

  let { currentStep, totalSteps }: Props = $props();

  const stepLabels = [
    'Informations personnelles',
    'Informations professionnelles',
    'Beneficiaires effectifs',
    'Documents',
    'Recapitulatif',
  ];
</script>

<nav aria-label="Etapes de l'enregistrement KYC" class="mb-8">
  <ol class="flex items-center" role="list">
    {#each Array(totalSteps) as _, i}
      {@const stepNum = i + 1}
      {@const isCompleted = stepNum < currentStep}
      {@const isCurrent = stepNum === currentStep}
      <li
        class="flex items-center {i < totalSteps - 1 ? 'flex-1' : ''}"
        aria-current={isCurrent ? 'step' : undefined}
      >
        <div class="flex flex-col items-center">
          <div
            class="flex h-10 w-10 items-center justify-center rounded-full text-sm font-semibold
              {isCompleted ? 'bg-green-600 text-white' : isCurrent ? 'bg-blue-600 text-white' : 'bg-gray-200 text-gray-600'}"
            aria-hidden="true"
          >
            {#if isCompleted}
              <svg class="h-5 w-5" fill="currentColor" viewBox="0 0 20 20" aria-hidden="true">
                <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"></path>
              </svg>
            {:else}
              {stepNum}
            {/if}
          </div>
          <span class="mt-2 text-xs font-medium {isCurrent ? 'text-blue-600' : 'text-gray-500'}" class:sr-only={false}>
            <span class="sr-only">Etape {stepNum}: </span>
            {stepLabels[i] ?? `Etape ${stepNum}`}
          </span>
        </div>
        {#if i < totalSteps - 1}
          <div
            class="mx-2 h-0.5 flex-1 {isCompleted ? 'bg-green-600' : 'bg-gray-200'}"
            aria-hidden="true"
          ></div>
        {/if}
      </li>
    {/each}
  </ol>
  <div class="sr-only" aria-live="polite">
    Etape {currentStep} sur {totalSteps}: {stepLabels[currentStep - 1] ?? ''}
  </div>
</nav>
