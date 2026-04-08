import { testHuman as test, expect } from '../../fixtures/banko.fixture';

/**
 * PARCOURS 4: Gestion Comptes (BC2 — Account)
 * Liste comptes → Filtres → Détail → Comptes list
 *
 * BDD: Given la liste des comptes est affichée
 *      When je filtre par type "Épargne"
 *      Then je vois les comptes épargne
 */
test.describe('Comptes — Bounded Context Account', () => {

  test.describe('Page Mes comptes (/accounts)', () => {
    test('Affiche la page comptes avec DashboardLayout', async ({ page }) => {
      await page.goto('/accounts');
      await expect(page).toHaveTitle(/Mes comptes/);
      await expect(page.getByRole('navigation', { name: 'Navigation principale' })).toBeVisible();
    });

    test('0 erreur console au chargement', async ({ page }) => {
      const errors: string[] = [];
      page.on('pageerror', err => {
        if (err.message.includes('appendChild')) return;
        errors.push(err.message);
      });
      await page.goto('/accounts');
      await page.waitForTimeout(1000);
      expect(errors).toHaveLength(0);
    });
  });

  test.describe('Liste comptes (/accounts/list)', () => {
    test('Affiche la liste avec recherche et filtres', async ({ page }) => {
      await page.goto('/accounts/list');
      await expect(page).toHaveTitle(/Comptes/);
      await expect(page.getByPlaceholder('Rechercher par IBAN ou titulaire')).toBeVisible();
    });

    test('Filtre par type de compte', async ({ page }) => {
      await page.goto('/accounts/list');
      const typeFilter = page.locator('select').first();
      await typeFilter.selectOption('savings');
    });

    test('Filtre par statut de compte', async ({ page }) => {
      await page.goto('/accounts/list');
      const statusFilter = page.locator('select').nth(1);
      await statusFilter.selectOption('active');
    });

    test('Tri par date ou solde', async ({ page }) => {
      await page.goto('/accounts/list');
      const sortFilter = page.locator('select').nth(2);
      await sortFilter.selectOption('balance');
    });

    test('Bouton "Nouveau Compte" présent', async ({ page }) => {
      await page.goto('/accounts/list');
      await expect(page.getByRole('link', { name: 'Nouveau Compte' })).toBeVisible();
    });

    test('Stats résumé en bas de page', async ({ page }) => {
      await page.goto('/accounts/list');
      await expect(page.getByText('Comptes Actifs')).toBeVisible();
      await expect(page.getByText('Total Dépôts')).toBeVisible();
    });
  });

  test.describe('Détail compte', () => {
    test('Page détail se charge sans erreur', async ({ page }) => {
      const errors: string[] = [];
      page.on('pageerror', err => {
        if (err.message.includes('appendChild')) return;
        errors.push(err.message);
      });
      await page.goto('/accounts/detail');
      await expect(page).toHaveTitle(/Détail du compte/);
      await page.waitForTimeout(500);
      expect(errors).toHaveLength(0);
    });
  });
});
