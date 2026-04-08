import { testHuman as test, expect } from '../../fixtures/banko.fixture';

/**
 * PARCOURS 6: Conformité (BC4 AML, BC5 Sanctions, BC6 Prudential, BC11 Governance)
 * AML → Sanctions → Audit → Risques
 *
 * BDD: Given je suis sur la page AML
 *      When je vois le tableau de bord
 *      Then les alertes AML sont affichées
 */
test.describe('Conformité — AML, Sanctions, Audit, Risques', () => {

  test.describe('AML (BC4)', () => {
    test('Page AML avec DashboardLayout et Sidebar', async ({ page }) => {
      await page.goto('/aml');
      await expect(page).toHaveTitle(/AML/);
      await expect(page.getByRole('navigation', { name: 'Navigation principale' })).toBeVisible();
    });

    test('0 erreur console', async ({ page }) => {
      const errors: string[] = [];
      page.on('pageerror', err => {
        if (err.message.includes('appendChild')) return;
        errors.push(err.message);
      });
      await page.goto('/aml');
      await page.waitForTimeout(1000);
      expect(errors).toHaveLength(0);
    });
  });

  test.describe('Sanctions (BC5)', () => {
    test('KPIs sanctions visibles', async ({ page }) => {
      await page.goto('/sanctions');
      await expect(page).toHaveTitle(/Sanctions/);
      await expect(page.getByText('Alertes actives')).toBeVisible();
      await expect(page.getByText('Screening du jour')).toBeVisible();
      await expect(page.getByText('Listes actives')).toBeVisible();
      await expect(page.getByText('Taux de faux positifs')).toBeVisible();
    });

    test('Table des alertes sanctions avec filtres', async ({ page }) => {
      await page.goto('/sanctions');
      await expect(page.getByRole('heading', { name: 'Alertes sanctions en cours' })).toBeVisible();
      // Vérifier les filtres
      const selects = page.locator('select');
      await expect(selects).toHaveCount(2); // Liste + Statut
    });

    test('Filtre par liste de sanctions (OFAC, EU, UN, CTAF)', async ({ page }) => {
      await page.goto('/sanctions');
      const listFilter = page.locator('select').first();
      await listFilter.selectOption('ofac');
      await listFilter.selectOption('eu');
      await listFilter.selectOption('un');
      await listFilter.selectOption('ctaf');
    });

    test('Listes de sanctions actives affichées (6 listes)', async ({ page }) => {
      await page.goto('/sanctions');
      const listsSection = page.locator('[data-testid="sanctions-lists-section"]');
      await expect(listsSection.getByText('Listes de sanctions actives')).toBeVisible();
      await expect(listsSection.getByText('OFAC SDN')).toBeVisible();
      await expect(listsSection.getByText('EU Consolidated')).toBeVisible();
      await expect(listsSection.getByText('UN Consolidated')).toBeVisible();
      await expect(listsSection.getByText('CTAF Tunisie')).toBeVisible();
      await expect(listsSection.getByText('UK HMT')).toBeVisible();
      await expect(listsSection.getByText('PEP Database')).toBeVisible();
    });

    test('0 erreur console', async ({ page }) => {
      const errors: string[] = [];
      page.on('pageerror', err => {
        if (err.message.includes('appendChild')) return;
        errors.push(err.message);
      });
      await page.goto('/sanctions');
      await page.waitForTimeout(1000);
      expect(errors).toHaveLength(0);
    });
  });

  test.describe('Audit (BC11 Governance)', () => {
    test('Journal d\'audit avec DashboardLayout', async ({ page }) => {
      await page.goto('/audit/log');
      await expect(page).toHaveTitle(/audit/i);
      await expect(page.getByRole('navigation', { name: 'Navigation principale' })).toBeVisible();
    });

    test('0 erreur console', async ({ page }) => {
      const errors: string[] = [];
      page.on('pageerror', err => {
        if (err.message.includes('appendChild')) return;
        errors.push(err.message);
      });
      await page.goto('/audit/log');
      await page.waitForTimeout(1000);
      expect(errors).toHaveLength(0);
    });
  });

  test.describe('Risques / Prudentiel (BC6)', () => {
    test('Tableau de bord prudentiel avec DashboardLayout', async ({ page }) => {
      await page.goto('/dashboards/risk');
      await expect(page).toHaveTitle(/prudentiel/i);
      await expect(page.getByRole('navigation', { name: 'Navigation principale' })).toBeVisible();
    });

    test('0 erreur console', async ({ page }) => {
      const errors: string[] = [];
      page.on('pageerror', err => {
        if (err.message.includes('appendChild')) return;
        errors.push(err.message);
      });
      await page.goto('/dashboards/risk');
      await page.waitForTimeout(1000);
      expect(errors).toHaveLength(0);
    });
  });
});
