import { test, expect } from '../../fixtures/banko.fixture';

/**
 * PARCOURS 5: Opérations (BC9 Payment, BC3 Credit, BC10 ForeignExchange)
 * Paiements → Crédit → Devises
 *
 * BDD: Given je suis sur la page paiements
 *      When la page se charge
 *      Then je vois le tableau de bord paiements avec le Sidebar complet
 */
test.describe('Opérations — Paiements, Crédit, Devises', () => {

  test.describe('Paiements (BC9)', () => {
    test('Page paiements avec DashboardLayout', async ({ page }) => {
      await page.goto('/payments');
      await expect(page).toHaveTitle(/Paiements/);
      await expect(page.getByRole('navigation', { name: 'Navigation principale' })).toBeVisible();
    });

    test('0 erreur console', async ({ page }) => {
      const errors: string[] = [];
      page.on('pageerror', err => errors.push(err.message));
      await page.goto('/payments');
      await page.waitForTimeout(1000);
      expect(errors).toHaveLength(0);
    });
  });

  test.describe('Crédit (BC3)', () => {
    test('Page crédit avec DashboardLayout', async ({ page }) => {
      await page.goto('/credit');
      await expect(page).toHaveTitle(/Crédit/);
      await expect(page.getByRole('navigation', { name: 'Navigation principale' })).toBeVisible();
    });

    test('0 erreur console', async ({ page }) => {
      const errors: string[] = [];
      page.on('pageerror', err => errors.push(err.message));
      await page.goto('/credit');
      await page.waitForTimeout(1000);
      expect(errors).toHaveLength(0);
    });
  });

  test.describe('Devises & Change (BC10)', () => {
    test('Affiche les taux de change KPI', async ({ page }) => {
      await page.goto('/foreign-exchange');
      await expect(page).toHaveTitle(/Devises/);
      await expect(page.getByText('USD/TND')).toBeVisible();
      await expect(page.getByText('EUR/TND')).toBeVisible();
      await expect(page.getByText('GBP/TND')).toBeVisible();
    });

    test('Table des taux BCT présente', async ({ page }) => {
      await page.goto('/foreign-exchange');
      await expect(page.getByRole('heading', { name: 'Taux de change BCT' })).toBeVisible();
      await expect(page.getByRole('table').first()).toBeVisible();
    });

    test('Devises affichées: USD, EUR, GBP, JPY, MAD, DZD', async ({ page }) => {
      await page.goto('/foreign-exchange');
      await expect(page.getByText('Dollar américain')).toBeVisible();
      await expect(page.getByText('Euro')).toBeVisible();
      await expect(page.getByText('Livre sterling')).toBeVisible();
      await expect(page.getByText('Yen japonais')).toBeVisible();
      await expect(page.getByText('Dirham marocain')).toBeVisible();
      await expect(page.getByText('Dinar algérien')).toBeVisible();
    });

    test('Opérations de change récentes avec bouton nouvelle opération', async ({ page }) => {
      await page.goto('/foreign-exchange');
      await expect(page.getByRole('heading', { name: 'Opérations de change récentes' })).toBeVisible();
      await expect(page.getByRole('button', { name: 'Nouvelle opération' })).toBeVisible();
    });

    test('0 erreur console', async ({ page }) => {
      const errors: string[] = [];
      page.on('pageerror', err => errors.push(err.message));
      await page.goto('/foreign-exchange');
      await page.waitForTimeout(1000);
      expect(errors).toHaveLength(0);
    });
  });
});
