import { testHuman as test, expect } from '../../fixtures/banko.fixture';

/**
 * SCÉNARIO MULTI-RÔLE 1: Directeur d'Agence — Morning Briefing
 *
 * Parcours: Dashboard → Vérifier KPIs → Consulter risques → Générer rapports BCT
 *
 * Given  le directeur ouvre BANKO le matin
 * When   il consulte le tableau de bord
 * Then   il voit les KPIs, les dépôts, les risques et les transactions récentes
 * And    il peut naviguer vers les risques et les rapports réglementaires
 */
test.describe('Directeur d\'Agence — Morning Briefing', () => {

  test('Étape 1: Consulter le dashboard — vue d\'ensemble', async ({ page, dashboardPage }) => {
    await dashboardPage.goto();
    await expect(page).toHaveTitle(/Tableau de bord/);

    // 4 KPI cards visibles
    await dashboardPage.expectKpiCards(4);

    // KPI: Total Clients
    const kpiSection = page.locator('[data-testid="dashboard-kpi-section"]');
    await expect(kpiSection.getByText('Total Clients')).toBeVisible();
    await expect(kpiSection.getByText('2,847')).toBeVisible();

    // KPI: Comptes Actifs
    await expect(kpiSection.getByText('Comptes Actifs')).toBeVisible();
    await expect(kpiSection.getByText('5,432')).toBeVisible();

    // KPI: Prêts en Cours
    await expect(kpiSection.getByText('Prêts en Cours')).toBeVisible();
    await expect(kpiSection.getByText('1,234')).toBeVisible();

    // KPI: Alertes AML
    await expect(kpiSection.getByText('Alertes AML')).toBeVisible();
  });

  test('Étape 2: Vérifier les dépôts et indicateurs de risque', async ({ page }) => {
    await page.goto('/dashboard');

    // Solde des Dépôts
    await expect(page.getByText('Solde des Dépôts')).toBeVisible();
    await expect(page.getByText('Comptes Courants')).toBeVisible();
    await expect(page.getByText('8,523,450.75 TND')).toBeVisible();
    await expect(page.getByText('Comptes Épargne')).toBeVisible();
    await expect(page.getByText('12,847,921.50 TND')).toBeVisible();
    await expect(page.getByText('21,371,372.25 TND')).toBeVisible();

    // Indicateurs de Risque (Bâle III)
    await expect(page.getByText('Ratio de Solvabilité')).toBeVisible();
    await expect(page.getByText('18.5%')).toBeVisible();
    await expect(page.getByText('Ratio LCR')).toBeVisible();
    await expect(page.getByText('142%')).toBeVisible();
    await expect(page.getByText('NPL Ratio')).toBeVisible();
    await expect(page.getByText('4.2%')).toBeVisible();
  });

  test('Étape 3: Consulter les transactions récentes', async ({ page }) => {
    await page.goto('/dashboard');
    const txSection = page.locator('[data-testid="dashboard-transactions-section"]');
    await expect(txSection.getByRole('heading', { name: 'Transactions Récentes' })).toBeVisible();
    await expect(txSection.getByRole('table')).toBeVisible();

    // Vérifier les colonnes du tableau
    await expect(txSection.getByText('Date/Heure')).toBeVisible();
    await expect(txSection.getByText('Description')).toBeVisible();
    await expect(txSection.getByText('Montant')).toBeVisible();
    await expect(txSection.getByText('Statut')).toBeVisible();

    // Vérifier une transaction
    await expect(txSection.getByText('Virement vers Compte Épargne')).toBeVisible();
    await expect(txSection.getByText('+5,000.00 TND')).toBeVisible();
  });

  test('Étape 4: Naviguer vers le tableau de bord prudentiel', async ({ page, sidebar }) => {
    await page.goto('/dashboard');
    await sidebar.goto('Risques');
    await page.waitForURL('**/dashboards/risk');
    await expect(page).toHaveTitle(/prudentiel/i);
  });

  test('Étape 5: Consulter les rapports réglementaires', async ({ page, sidebar }) => {
    await page.goto('/dashboard');
    await sidebar.goto('Rapports');
    await page.waitForURL('**/reporting');
    await expect(page).toHaveTitle(/Rapports/);

    // Vérifier les 6 catégories de rapports
    await expect(page.getByText('Rapports BCT')).toBeVisible();
    await expect(page.getByText('Rapports AML/CTAF')).toBeVisible();
    await expect(page.getByText('Rapports Bâle III')).toBeVisible();
    await expect(page.getByText('Rapports IFRS 9')).toBeVisible();
    await expect(page.getByText('Rapports financiers')).toBeVisible();
    await expect(page.getByText('Rapports opérationnels')).toBeVisible();

    // Bouton de génération
    await expect(page.getByRole('button', { name: 'Générer un rapport' })).toBeVisible();

    // Table rapports récents
    await expect(page.getByText('Ratio de solvabilité CET1')).toBeVisible();
  });

  test('Étape 6: Parcours complet Dashboard → Risques → Rapports sans erreur', async ({ page }) => {
    const errors: string[] = [];
    page.on('pageerror', err => {
      if (err.message.includes('appendChild')) return;
      errors.push(err.message);
    });

    await page.goto('/dashboard');
    await page.waitForTimeout(500);
    await page.goto('/dashboards/risk');
    await page.waitForTimeout(500);
    await page.goto('/reporting');
    await page.waitForTimeout(500);

    expect(errors).toHaveLength(0);
  });
});
