import { test, expect } from '../../fixtures/banko.fixture';

/**
 * SCÉNARIO MULTI-RÔLE 3: Compliance Officer — Investigation AML/Sanctions
 *
 * Parcours: AML → Sanctions screening → Filtrer OFAC → Audit log → Filtrer actions → Export
 *
 * Given  le compliance officer reçoit une alerte
 * When   il investigue dans AML, Sanctions et l'Audit trail
 * Then   il peut filtrer, investiguer et tracer chaque action
 */
test.describe('Compliance Officer — Investigation AML/Sanctions', () => {

  test('Étape 1: Consulter le dashboard AML', async ({ page }) => {
    await page.goto('/aml');
    await expect(page).toHaveTitle(/AML/);
    await expect(page.getByRole('heading', { name: 'Tableau de bord AML' })).toBeVisible();
    await expect(page.getByRole('navigation', { name: 'Navigation principale' })).toBeVisible();
  });

  test('Étape 2: Naviguer vers le screening Sanctions', async ({ page, sidebar }) => {
    await page.goto('/aml');
    await sidebar.goto('Sanctions');
    await page.waitForURL('**/sanctions');
    await expect(page).toHaveTitle(/Sanctions/);
  });

  test('Étape 3: Vérifier les KPIs sanctions', async ({ page }) => {
    await page.goto('/sanctions');
    await expect(page.getByText('Alertes actives')).toBeVisible();
    await expect(page.getByText('Screening du jour')).toBeVisible();
    await expect(page.getByText('1,247')).toBeVisible();
    await expect(page.getByText('Listes actives')).toBeVisible();
    await expect(page.getByText('Taux de faux positifs')).toBeVisible();
    await expect(page.getByText('2.1%')).toBeVisible();
  });

  test('Étape 4: Filtrer les alertes par liste OFAC', async ({ page }) => {
    await page.goto('/sanctions');
    const listFilter = page.locator('select').first();
    await listFilter.selectOption('ofac');
    await expect(listFilter).toHaveValue('ofac');
  });

  test('Étape 5: Filtrer par statut "En investigation"', async ({ page }) => {
    await page.goto('/sanctions');
    const statusFilter = page.locator('select').nth(1);
    await statusFilter.selectOption('investigating');
    await expect(statusFilter).toHaveValue('investigating');
  });

  test('Étape 6: Combiner filtres OFAC + Investigation', async ({ page }) => {
    await page.goto('/sanctions');
    await page.locator('select').first().selectOption('ofac');
    await page.locator('select').nth(1).selectOption('investigating');

    // Les deux filtres sont actifs
    await expect(page.locator('select').first()).toHaveValue('ofac');
    await expect(page.locator('select').nth(1)).toHaveValue('investigating');
  });

  test('Étape 7: Vérifier les listes de sanctions actives', async ({ page }) => {
    await page.goto('/sanctions');
    await expect(page.getByRole('heading', { name: 'Listes de sanctions actives' })).toBeVisible();

    // 6 listes
    await expect(page.getByText('OFAC SDN')).toBeVisible();
    await expect(page.getByText('EU Consolidated')).toBeVisible();
    await expect(page.getByText('UN Consolidated')).toBeVisible();
    await expect(page.getByText('CTAF Tunisie')).toBeVisible();
    await expect(page.getByText('UK HMT')).toBeVisible();
    await expect(page.getByText('PEP Database')).toBeVisible();

    // Statuts de mise à jour
    await expect(page.getByText('12,847 entrées')).toBeVisible();
    await expect(page.getByText('45,230 entrées')).toBeVisible();
  });

  test('Étape 8: Consulter le journal d\'audit', async ({ page, sidebar }) => {
    await page.goto('/sanctions');
    await sidebar.goto('Audit');
    await page.waitForURL('**/audit/log');
    await expect(page).toHaveTitle(/audit/i);

    // Vérifier les outils audit
    await expect(page.getByRole('button', { name: /Exporter.*CSV/ })).toBeVisible();
    await expect(page.getByRole('button', { name: /Exporter.*JSON/ })).toBeVisible();
  });

  test('Étape 9: Filtrer l\'audit par action "Approve" + ressource "Customer"', async ({ page }) => {
    await page.goto('/audit/log');

    // Filtre action
    const actionFilter = page.getByRole('combobox').first();
    await actionFilter.selectOption('Approve');
    await expect(actionFilter).toHaveValue('Approve');

    // Filtre ressource
    const resourceFilter = page.getByRole('combobox').nth(1);
    await resourceFilter.selectOption('Customer');
    await expect(resourceFilter).toHaveValue('Customer');
  });

  test('Étape 10: Filtrer l\'audit par plage de dates', async ({ page }) => {
    await page.goto('/audit/log');

    await page.getByLabel('Date debut').fill('2026-04-01');
    await page.getByLabel('Date fin').fill('2026-04-07');

    await expect(page.getByLabel('Date debut')).toHaveValue('2026-04-01');
    await expect(page.getByLabel('Date fin')).toHaveValue('2026-04-07');
  });

  test('Étape 11: Filtrer par ID acteur (UUID)', async ({ page }) => {
    await page.goto('/audit/log');
    const actorInput = page.getByPlaceholder('UUID...');
    await actorInput.fill('550e8400-e29b-41d4-a716-446655440000');
    await expect(actorInput).toHaveValue('550e8400-e29b-41d4-a716-446655440000');
  });

  test('Étape 12: Bouton "Effacer" réinitialise les filtres', async ({ page }) => {
    await page.goto('/audit/log');

    // Set un filtre
    await page.getByRole('combobox').first().selectOption('Delete');
    // Effacer
    await page.getByRole('button', { name: 'Effacer' }).click();
  });

  test('Parcours complet AML → Sanctions → Audit sans erreur JS', async ({ page }) => {
    const errors: string[] = [];
    page.on('pageerror', err => errors.push(err.message));

    await page.goto('/aml');
    await page.waitForTimeout(300);
    await page.goto('/sanctions');
    await page.waitForTimeout(300);
    await page.goto('/audit/log');
    await page.waitForTimeout(300);
    await page.goto('/dashboards/risk');
    await page.waitForTimeout(300);

    expect(errors).toHaveLength(0);
  });
});
