<script lang="ts">
  import { accountsApi } from '../../lib/api/accounts';

  interface Props {
    fromAccountId: string;
    onclose: () => void;
  }

  let { fromAccountId, onclose }: Props = $props();

  let toIban = $state('');
  let amount = $state('');
  let currency = $state('MAD');
  let reference = $state('');
  let error = $state('');
  let success = $state(false);
  let submitting = $state(false);

  let formValid = $derived(toIban.length > 0 && Number(amount) > 0);

  async function handleSubmit(e: Event) {
    e.preventDefault();
    if (!formValid || submitting) return;

    submitting = true;
    error = '';

    try {
      await accountsApi.transfer({
        from_account_id: fromAccountId,
        to_iban: toIban,
        amount: Number(amount),
        currency,
        reference: reference || undefined,
      });
      success = true;
    } catch (err: unknown) {
      error = err instanceof Error ? err.message : 'Erreur lors du virement';
    } finally {
      submitting = false;
    }
  }
</script>

<div
  class="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
  role="dialog"
  aria-modal="true"
  aria-labelledby="transfer-modal-title"
>
  <div class="w-full max-w-md rounded-lg bg-white p-6 shadow-xl">
    <div class="mb-4 flex items-center justify-between">
      <h2 id="transfer-modal-title" class="text-lg font-semibold text-gray-900">
        Effectuer un virement
      </h2>
      <button
        type="button"
        onclick={onclose}
        class="modal-close rounded-md p-1 text-gray-400 hover:text-gray-600 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600"
        aria-label="Fermer"
      >
        <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
        </svg>
      </button>
    </div>

    {#if success}
      <div class="rounded-md border border-green-200 bg-green-50 p-4 text-center" role="status">
        <p class="text-sm font-medium text-green-800">Virement effectue avec succes</p>
        <button
          type="button"
          onclick={onclose}
          class="mt-3 rounded-md bg-green-600 px-4 py-2 text-sm font-medium text-white hover:bg-green-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-green-600"
        >
          Fermer
        </button>
      </div>
    {:else}
      {#if error}
        <div role="alert" aria-live="polite" class="mb-4 rounded-md border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-700">
          {error}
        </div>
      {/if}

      <form onsubmit={handleSubmit} class="space-y-4" novalidate>
        <div>
          <label for="transfer-iban" class="block text-sm font-medium text-gray-700">
            IBAN destinataire <span class="text-red-500" aria-hidden="true">*</span>
          </label>
          <input
            id="transfer-iban"
            type="text"
            required
            aria-required="true"
            bind:value={toIban}
            placeholder="MA00 0000 0000 0000 0000 0000 000"
            class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 font-mono text-sm shadow-sm focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>

        <div>
          <label for="transfer-amount" class="block text-sm font-medium text-gray-700">
            Montant ({currency}) <span class="text-red-500" aria-hidden="true">*</span>
          </label>
          <input
            id="transfer-amount"
            type="number"
            min="0.01"
            step="0.01"
            required
            aria-required="true"
            bind:value={amount}
            class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 sm:text-sm"
          />
        </div>

        <div>
          <label for="transfer-reference" class="block text-sm font-medium text-gray-700">
            Reference (optionnel)
          </label>
          <input
            id="transfer-reference"
            type="text"
            bind:value={reference}
            class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 sm:text-sm"
          />
        </div>

        <div class="flex justify-end gap-3 pt-2">
          <button
            type="button"
            onclick={onclose}
            class="rounded-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600"
          >
            Annuler
          </button>
          <button
            type="submit"
            disabled={!formValid || submitting}
            class="rounded-md bg-blue-600 px-4 py-2 text-sm font-semibold text-white shadow-sm hover:bg-blue-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600 disabled:opacity-50"
          >
            {submitting ? 'Envoi...' : 'Envoyer le virement'}
          </button>
        </div>
      </form>
    {/if}
  </div>
</div>
