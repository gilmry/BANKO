<script lang="ts">
  import { authApi } from '../../lib/api/auth';

  // Svelte 5 runes
  let email = $state('');
  let password = $state('');
  let error = $state('');
  let submitting = $state(false);

  let emailValid = $derived(email.length === 0 || /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email));
  let formValid = $derived(email.length > 0 && password.length > 0 && emailValid);

  async function handleSubmit(e: Event) {
    e.preventDefault();
    if (!formValid || submitting) return;

    submitting = true;
    error = '';

    try {
      await authApi.login({ email, password });
      // Server sets httpOnly cookie; redirect to accounts
      window.location.href = '/accounts';
    } catch (err: unknown) {
      // TODO: use t('auth.login.error')
      error = err instanceof Error ? err.message : 'Identifiants invalides';
    } finally {
      submitting = false;
    }
  }
</script>

<section class="mx-auto w-full max-w-md" aria-labelledby="login-heading">
  <h1 id="login-heading" data-testid="login-heading" class="mb-8 text-center text-3xl font-bold text-gray-900">
    <!-- TODO: use t('auth.login.title') -->
    Connexion
  </h1>

  {#if error}
    <div
      role="alert"
      aria-live="polite"
      data-testid="login-error"
      class="mb-4 rounded-md border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-700"
    >
      {error}
    </div>
  {/if}

  <form onsubmit={handleSubmit} class="space-y-6" novalidate data-testid="login-form">
    <div>
      <label for="login-email" class="block text-sm font-medium text-gray-700">
        <!-- TODO: use t('auth.login.email') -->
        Adresse e-mail
      </label>
      <input
        id="login-email"
        data-testid="login-email-input"
        type="email"
        autocomplete="email"
        required
        aria-required="true"
        aria-invalid={!emailValid}
        aria-describedby={!emailValid ? 'email-error' : undefined}
        bind:value={email}
        class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 sm:text-sm"
        class:border-red-500={!emailValid}
      />
      {#if !emailValid}
        <p id="email-error" class="mt-1 text-sm text-red-600" role="alert">
          Format d'e-mail invalide
        </p>
      {/if}
    </div>

    <div>
      <label for="login-password" class="block text-sm font-medium text-gray-700">
        <!-- TODO: use t('auth.login.password') -->
        Mot de passe
      </label>
      <input
        id="login-password"
        data-testid="login-password-input"
        type="password"
        autocomplete="current-password"
        required
        aria-required="true"
        bind:value={password}
        class="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 sm:text-sm"
      />
    </div>

    <button
      type="submit"
      disabled={!formValid || submitting}
      data-testid="login-submit-btn"
      class="w-full rounded-md bg-blue-600 px-4 py-2 text-sm font-semibold text-white shadow-sm hover:bg-blue-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600 disabled:cursor-not-allowed disabled:opacity-50"
    >
      {#if submitting}
        <!-- TODO: use t('common.loading') -->
        Chargement...
      {:else}
        <!-- TODO: use t('auth.login.submit') -->
        Se connecter
      {/if}
    </button>
  </form>

  <p class="mt-6 text-center text-sm text-gray-600">
    <!-- TODO: use t('auth.login.noAccount') -->
    Pas encore de compte ?
    <a
      href="/register"
      data-testid="login-register-link"
      class="font-medium text-blue-600 hover:text-blue-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600"
    >
      <!-- TODO: use t('auth.login.register') -->
      Creer un compte
    </a>
  </p>
</section>
