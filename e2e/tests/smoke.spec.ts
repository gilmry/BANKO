import { test, expect } from '../fixtures/banko.fixture';

/**
 * SMOKE TEST — Méthode Maury
 * Vérifie en <30s que toutes les 20 pages se chargent sans 404/500.
 * À exécuter AVANT les tests détaillés.
 *
 * BDD: Given l'application tourne sur localhost
 *      When j'accède à chaque route
 *      Then le status HTTP est 200 et le titre est correct
 */
test.describe('Smoke Test — Toutes les routes', () => {
  const routes = [
    { path: '/', titlePattern: /Accueil/ },
    { path: '/dashboard', titlePattern: /Tableau de bord/ },
    { path: '/customers', titlePattern: /Clients/ },
    { path: '/customers/detail', titlePattern: /Fiche client/ },
    { path: '/accounts', titlePattern: /Mes comptes/ },
    { path: '/accounts/list', titlePattern: /Comptes/ },
    { path: '/accounts/detail', titlePattern: /Détail du compte/ },
    { path: '/payments', titlePattern: /Paiements/ },
    { path: '/credit', titlePattern: /Crédit/ },
    { path: '/foreign-exchange', titlePattern: /Devises/ },
    { path: '/aml', titlePattern: /AML/ },
    { path: '/sanctions', titlePattern: /Sanctions/ },
    { path: '/audit/log', titlePattern: /audit/i },
    { path: '/dashboards/risk', titlePattern: /prudentiel/i },
    { path: '/settings', titlePattern: /Paramètres/ },
    { path: '/reporting', titlePattern: /Rapports/ },
    { path: '/login', titlePattern: /Connexion/ },
    { path: '/register', titlePattern: /Inscription/ },
    { path: '/customer/onboarding', titlePattern: /KYC/ },
    { path: '/admin/audit-bct', titlePattern: /BCT Audit/i },
  ];

  for (const route of routes) {
    test(`GET ${route.path} → 200 + titre correct`, async ({ page }) => {
      const response = await page.goto(route.path);

      // Vérifie HTTP 200
      expect(response?.status()).toBe(200);

      // Vérifie le titre
      await expect(page).toHaveTitle(route.titlePattern);

      // Vérifie pas d'erreur JS
      const errors: string[] = [];
      page.on('pageerror', err => errors.push(err.message));
      await page.waitForTimeout(300);
      expect(errors).toHaveLength(0);
    });
  }

  test('Aucune requête réseau en erreur (4xx/5xx)', async ({ page }) => {
    const failedRequests: string[] = [];
    page.on('response', response => {
      if (response.status() >= 400 && !response.url().includes('favicon')) {
        failedRequests.push(`${response.status()} ${response.url()}`);
      }
    });

    // Visite les pages principales
    for (const route of ['/', '/dashboard', '/customers', '/accounts', '/payments']) {
      await page.goto(route);
      await page.waitForTimeout(200);
    }

    expect(failedRequests).toHaveLength(0);
  });
});
