<script lang="ts">
  import { authApi } from '../../lib/api/auth';

  let qrCodeUrl = $state('');
  let backupCodes = $state<string[]>([]);
  let code = $state('');
  let error = $state('');
  let step = $state<'setup' | 'verify' | 'done'>('setup');
  let loading = $state(false);

  async function enableTwoFactor() {
    loading = true;
    error = '';
    try {
      const response = await authApi.enable2fa();
      qrCodeUrl = response.qr_code_url;
      backupCodes = response.backup_codes;
      step = 'verify';
    } catch (err: unknown) {
      error = err instanceof Error ? err.message : 'Erreur lors de la configuration 2FA';
    } finally {
      loading = false;
    }
  }

  async function verifyCode(e: Event) {
    e.preventDefault();
    if (code.length !== 6) return;

    loading = true;
    error = '';
    try {
      await authApi.verify2fa({ code });
      step = 'done';
    } catch (err: unknown) {
      error = err instanceof Error ? err.message : 'Code invalide';
    } finally {
      loading = false;
    }
  }
</script>

<section class="mx-auto w-full max-w-md" aria-labelledby="2fa-heading">
  <h2 id="2fa-heading" class="mb-6 text-2xl font-bold text-gray-900">
    <!-- TODO: use t('auth.twoFactor.title') -->
    Authentification a deux facteurs
  </h2>

  {#if error}
    <div
      role="alert"
      aria-live="polite"
      class="mb-4 rounded-md border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-700"
    >
      {error}
    </div>
  {/if}

  {#if step === 'setup'}
    <p class="mb-4 text-sm text-gray-600">
      Activez l'authentification a deux facteurs pour renforcer la securite de votre compte.
    </p>
    <button
      type="button"
      onclick={enableTwoFactor}
      disabled={loading}
      class="w-full rounded-md bg-blue-600 px-4 py-2 text-sm font-semibold text-white shadow-sm hover:bg-blue-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600 disabled:opacity-50"
    >
      {loading ? 'Chargement...' : 'Activer la 2FA'}
    </button>
  {:else if step === 'verify'}
    <div class="space-y-6">
      <div>
        <p class="mb-3 text-sm text-gray-600">
          <!-- TODO: use t('auth.twoFactor.scanQr') -->
          Scannez le QR code avec votre application d'authentification.
        </p>
        <div
          class="flex items-center justify-center rounded-lg border border-gray-200 bg-white p-4"
          role="img"
          aria-label="QR code pour l'authentification a deux facteurs"
        >
          {#if qrCodeUrl}
            <img src={qrCodeUrl} alt="QR Code 2FA" class="h-48 w-48" />
          {:else}
            <div class="flex h-48 w-48 items-center justify-center bg-gray-100 text-gray-400">
              QR Code
            </div>
          {/if}
        </div>
      </div>

      <form onsubmit={verifyCode} class="space-y-4">
        <div>
          <label for="2fa-code" class="block text-sm font-medium text-gray-700">
            <!-- TODO: use t('auth.twoFactor.enterCode') -->
            Entrez le code a 6 chiffres
          </label>
          <input
            id="2fa-code"
            type="text"
            inputmode="numeric"
            pattern="[0-9]{6}"
            maxlength={6}
            required
            aria-required="true"
            bind:value={code}
            class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 text-center text-lg tracking-widest shadow-sm focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500"
            placeholder="000000"
          />
        </div>
        <button
          type="submit"
          disabled={code.length !== 6 || loading}
          class="w-full rounded-md bg-blue-600 px-4 py-2 text-sm font-semibold text-white shadow-sm hover:bg-blue-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600 disabled:opacity-50"
        >
          <!-- TODO: use t('auth.twoFactor.verify') -->
          {loading ? 'Verification...' : 'Verifier'}
        </button>
      </form>

      {#if backupCodes.length > 0}
        <div class="rounded-md border border-yellow-200 bg-yellow-50 p-4">
          <h3 class="mb-2 text-sm font-semibold text-yellow-800">
            <!-- TODO: use t('auth.twoFactor.backupCodes') -->
            Codes de secours
          </h3>
          <p class="mb-3 text-xs text-yellow-700">
            <!-- TODO: use t('auth.twoFactor.backupCodesWarning') -->
            Conservez ces codes en lieu sur. Chacun ne peut etre utilise qu'une seule fois.
          </p>
          <div class="grid grid-cols-2 gap-2" role="list" aria-label="Codes de secours">
            {#each backupCodes as codeItem}
              <code class="rounded bg-white px-2 py-1 text-center text-sm font-mono" role="listitem">
                {codeItem}
              </code>
            {/each}
          </div>
        </div>
      {/if}
    </div>
  {:else}
    <div class="rounded-md border border-green-200 bg-green-50 p-6 text-center" role="status">
      <svg class="mx-auto mb-3 h-12 w-12 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
      </svg>
      <p class="text-lg font-semibold text-green-800">
        Authentification a deux facteurs activee
      </p>
    </div>
  {/if}
</section>
