<script lang="ts">
  import { authApi } from '../../lib/api/auth';

  let email = $state('');
  let password = $state('');
  let confirmPassword = $state('');
  let acceptTerms = $state(false);
  let error = $state('');
  let submitting = $state(false);

  let emailValid = $derived(email.length === 0 || /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email));
  let passwordsMatch = $derived(confirmPassword.length === 0 || password === confirmPassword);

  // Password strength: 0=empty, 1=weak, 2=medium, 3=strong
  let passwordStrength = $derived.by(() => {
    if (password.length === 0) return 0;
    let score = 0;
    if (password.length >= 8) score++;
    if (/[A-Z]/.test(password) && /[a-z]/.test(password)) score++;
    if (/[0-9]/.test(password) && /[^A-Za-z0-9]/.test(password)) score++;
    return score;
  });

  let strengthLabel = $derived(
    passwordStrength === 0
      ? ''
      : passwordStrength === 1
        ? 'Faible'
        : passwordStrength === 2
          ? 'Moyen'
          : 'Fort',
  );

  let strengthColor = $derived(
    passwordStrength === 1
      ? 'bg-red-500'
      : passwordStrength === 2
        ? 'bg-yellow-500'
        : passwordStrength === 3
          ? 'bg-green-500'
          : 'bg-gray-200',
  );

  let formValid = $derived(
    email.length > 0 &&
      emailValid &&
      password.length >= 8 &&
      passwordsMatch &&
      confirmPassword.length > 0 &&
      acceptTerms,
  );

  async function handleSubmit(e: Event) {
    e.preventDefault();
    if (!formValid || submitting) return;

    submitting = true;
    error = '';

    try {
      await authApi.register({ email, password });
      window.location.href = '/login';
    } catch (err: unknown) {
      error = err instanceof Error ? err.message : "Erreur lors de l'inscription";
    } finally {
      submitting = false;
    }
  }
</script>

<section class="mx-auto w-full max-w-md" aria-labelledby="register-heading">
  <h1 id="register-heading" class="mb-8 text-center text-3xl font-bold text-gray-900">
    <!-- TODO: use t('auth.register.title') -->
    Creer un compte
  </h1>

  {#if error}
    <div
      role="alert"
      aria-live="polite"
      class="mb-4 rounded-md border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-700"
    >
      {error}
    </div>
  {/if}

  <form onsubmit={handleSubmit} class="space-y-6" novalidate>
    <div>
      <label for="register-email" class="block text-sm font-medium text-gray-700">
        Adresse e-mail
      </label>
      <input
        id="register-email"
        type="email"
        autocomplete="email"
        required
        aria-required="true"
        aria-invalid={!emailValid}
        bind:value={email}
        class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 sm:text-sm"
        class:border-red-500={!emailValid}
      />
    </div>

    <div>
      <label for="register-password" class="block text-sm font-medium text-gray-700">
        Mot de passe
      </label>
      <input
        id="register-password"
        type="password"
        autocomplete="new-password"
        required
        aria-required="true"
        bind:value={password}
        class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 sm:text-sm"
      />
      {#if password.length > 0}
        <div class="mt-2" aria-label="Force du mot de passe: {strengthLabel}">
          <div class="flex items-center gap-2">
            <div class="h-2 flex-1 rounded-full bg-gray-200">
              <div
                class="h-2 rounded-full transition-all {strengthColor}"
                style="width: {(passwordStrength / 3) * 100}%"
              ></div>
            </div>
            <span class="text-xs text-gray-600">{strengthLabel}</span>
          </div>
        </div>
      {/if}
    </div>

    <div>
      <label for="register-confirm-password" class="block text-sm font-medium text-gray-700">
        Confirmer le mot de passe
      </label>
      <input
        id="register-confirm-password"
        type="password"
        autocomplete="new-password"
        required
        aria-required="true"
        aria-invalid={!passwordsMatch}
        bind:value={confirmPassword}
        class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 sm:text-sm"
        class:border-red-500={!passwordsMatch}
      />
      {#if !passwordsMatch}
        <p class="mt-1 text-sm text-red-600" role="alert">
          Les mots de passe ne correspondent pas
        </p>
      {/if}
    </div>

    <div class="flex items-start">
      <input
        id="register-terms"
        type="checkbox"
        bind:checked={acceptTerms}
        aria-required="true"
        class="mt-1 h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
      />
      <label for="register-terms" class="ms-2 block text-sm text-gray-700">
        J'accepte les conditions d'utilisation
      </label>
    </div>

    <button
      type="submit"
      disabled={!formValid || submitting}
      class="w-full rounded-md bg-blue-600 px-4 py-2 text-sm font-semibold text-white shadow-sm hover:bg-blue-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600 disabled:cursor-not-allowed disabled:opacity-50"
    >
      {#if submitting}
        Chargement...
      {:else}
        S'inscrire
      {/if}
    </button>
  </form>

  <p class="mt-6 text-center text-sm text-gray-600">
    Deja un compte ?
    <a
      href="/login"
      class="font-medium text-blue-600 hover:text-blue-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600"
    >
      Se connecter
    </a>
  </p>
</section>
