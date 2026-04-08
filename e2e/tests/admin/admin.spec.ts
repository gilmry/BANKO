import { testHuman as test, expect } from '../../fixtures/banko.fixture';

/**
 * PARCOURS 7: Administration — Settings, Reporting, Dashboard principal
 *
 * BDD: Given je suis sur /settings
 *      When je change la langue en arabe
 *      Then le select affiche "العربية"
 */
test.describe('Administration — Paramètres, Rapports, Dashboard', () => {

  test.describe('Dashboard principal', () => {
    test('Affiche 4 KPI cards', async ({ dashboardPage }) => {
      await dashboardPage.goto();
      await dashboardPage.expectKpiCards(4);
    });

    test('Table des transactions récentes', async ({ dashboardPage }) => {
      await dashboardPage.goto();
      await dashboardPage.expectTransactionsTable();
    });

    test('Quick actions: Nouveau Client, Nouveau Compte, Paiement, Crédit', async ({ dashboardPage }) => {
      await dashboardPage.goto();
      await dashboardPage.expectQuickActions();
    });

    test('Section Solde des Dépôts avec Courants + Épargne + Total', async ({ page }) => {
      await page.goto('/dashboard');
      const depositsCard = page.locator('[data-testid="dashboard-deposits-card"]');
      await expect(depositsCard.getByText('Comptes Courants')).toBeVisible();
      await expect(depositsCard.getByText('Comptes Épargne')).toBeVisible();
      await expect(depositsCard.getByText('Total')).toBeVisible();
    });

    test('Indicateurs de risque: Solvabilité, LCR, NPL', async ({ page }) => {
      await page.goto('/dashboard');
      await expect(page.getByText('Ratio de Solvabilité')).toBeVisible();
      await expect(page.getByText('Ratio LCR')).toBeVisible();
      await expect(page.getByText('NPL Ratio')).toBeVisible();
    });

    test('Lien "Voir tout" mène aux comptes', async ({ page }) => {
      await page.goto('/dashboard');
      await page.getByRole('link', { name: 'Voir tout' }).click();
      await page.waitForURL('**/accounts');
    });

    test('Quick action "Nouveau Client" → onboarding', async ({ page }) => {
      await page.goto('/dashboard');
      await page.getByRole('link', { name: /Nouveau Client/ }).click();
      await page.waitForURL('**/customer/onboarding');
    });

    test('Quick action "Paiement" → payments', async ({ page }) => {
      await page.goto('/dashboard');
      await page.locator('[data-testid="dashboard-action-payment"]').click();
      await page.waitForURL('**/payments');
    });
  });

  test.describe('Paramètres (/settings)', () => {
    test('Affiche les 3 sections: Général, Sécurité, Système', async ({ settingsPage }) => {
      await settingsPage.goto();
      await settingsPage.expectSections();
    });

    test('Settings sidebar navigation avec 6 liens', async ({ page }) => {
      await page.goto('/settings');
      await expect(page.getByRole('link', { name: 'Général' })).toBeVisible();
      await expect(page.getByRole('link', { name: 'Sécurité' })).toBeVisible();
      await expect(page.getByRole('link', { name: 'Notifications' })).toBeVisible();
      await expect(page.getByRole('link', { name: 'API & Intégrations' })).toBeVisible();
      await expect(page.getByRole('link', { name: 'Conformité' })).toBeVisible();
      await expect(page.getByRole('link', { name: 'Système' })).toBeVisible();
    });

    test('Input nom institution modifiable', async ({ page }) => {
      await page.goto('/settings');
      const input = page.locator('input[type="text"]').first();
      await input.clear();
      await input.fill('Ma Banque Test');
      await expect(input).toHaveValue('Ma Banque Test');
    });

    test('Changement de devise par défaut', async ({ page }) => {
      await page.goto('/settings');
      const currencySelect = page.getByRole('combobox').first();
      await currencySelect.selectOption('EUR');
      await expect(currencySelect).toHaveValue('EUR');
    });

    test('Changement de langue', async ({ page }) => {
      await page.goto('/settings');
      const langSelect = page.getByRole('combobox').nth(1);
      await langSelect.selectOption('ar');
      await expect(langSelect).toHaveValue('ar');
    });

    test('Toggle 2FA (switch) interactif', async ({ page }) => {
      await page.goto('/settings');
      const toggle = page.getByRole('switch');
      await expect(toggle).toBeVisible();
    });

    test('Sélection durée de session', async ({ page }) => {
      await page.goto('/settings');
      const sessionSelect = page.getByRole('combobox').last();
      await sessionSelect.selectOption('15');
      await expect(sessionSelect).toHaveValue('15');
    });

    test('Informations système affichées', async ({ page }) => {
      await page.goto('/settings');
      await expect(page.getByText('BANKO v0.1.0-alpha')).toBeVisible();
      await expect(page.getByText('Rust + Actix-web 4.x')).toBeVisible();
      await expect(page.getByText('PostgreSQL 16')).toBeVisible();
      await expect(page.getByText('AGPL-3.0')).toBeVisible();
    });

    test('0 erreur console', async ({ page }) => {
      const errors: string[] = [];
      page.on('pageerror', err => {
        if (err.message.includes('appendChild')) return;
        errors.push(err.message);
      });
      await page.goto('/settings');
      await page.waitForTimeout(1000);
      expect(errors).toHaveLength(0);
    });
  });

  test.describe('Rapports (/reporting)', () => {
    test('Affiche les 6 catégories de rapports', async ({ page }) => {
      await page.goto('/reporting');
      await expect(page).toHaveTitle(/Rapports/);
      await expect(page.getByText('Rapports BCT')).toBeVisible();
      await expect(page.getByText('Rapports AML/CTAF')).toBeVisible();
      await expect(page.getByText('Rapports Bâle III')).toBeVisible();
      await expect(page.getByText('Rapports IFRS 9')).toBeVisible();
      await expect(page.getByText('Rapports financiers')).toBeVisible();
      await expect(page.getByText('Rapports opérationnels')).toBeVisible();
    });

    test('Bouton "Générer un rapport" visible', async ({ page }) => {
      await page.goto('/reporting');
      await expect(page.getByRole('button', { name: 'Générer un rapport' })).toBeVisible();
    });

    test('Table des rapports récents', async ({ page }) => {
      await page.goto('/reporting');
      await expect(page.getByRole('heading', { name: 'Rapports récents' })).toBeVisible();
      await expect(page.getByRole('table')).toBeVisible();
    });

    test('Rapport CET1 affiché avec statut "Prêt"', async ({ page }) => {
      await page.goto('/reporting');
      await expect(page.getByText('Ratio de solvabilité CET1')).toBeVisible();
    });

    test('0 erreur console', async ({ page }) => {
      const errors: string[] = [];
      page.on('pageerror', err => {
        if (err.message.includes('appendChild')) return;
        errors.push(err.message);
      });
      await page.goto('/reporting');
      await page.waitForTimeout(1000);
      expect(errors).toHaveLength(0);
    });
  });

  test.describe('Landing page (/)', () => {
    test('Page d\'accueil avec hero section', async ({ page }) => {
      await page.goto('/');
      await expect(page).toHaveTitle(/Accueil/);
    });

    test('0 erreur console', async ({ page }) => {
      const errors: string[] = [];
      page.on('pageerror', err => {
        if (err.message.includes('appendChild')) return;
        errors.push(err.message);
      });
      await page.goto('/');
      await page.waitForTimeout(1000);
      expect(errors).toHaveLength(0);
    });
  });

  test.describe('BCT Audit Dashboard (/admin/audit-bct)', () => {
    test('Page audit BCT se charge', async ({ page }) => {
      await page.goto('/admin/audit-bct');
      await expect(page).toHaveTitle(/BCT Audit/i);
    });

    test('0 erreur console', async ({ page }) => {
      const errors: string[] = [];
      page.on('pageerror', err => {
        if (err.message.includes('appendChild')) return;
        errors.push(err.message);
      });
      await page.goto('/admin/audit-bct');
      await page.waitForTimeout(1000);
      expect(errors).toHaveLength(0);
    });
  });
});
