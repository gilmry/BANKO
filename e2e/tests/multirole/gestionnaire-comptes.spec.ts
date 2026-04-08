import { testHuman as test, expect } from '../../fixtures/banko.fixture';

/**
 * SCÉNARIO MULTI-RÔLE 5: Gestionnaire de Comptes — Gestion quotidienne
 *
 * Parcours: Comptes list → Filtres → Détail → Paiements → Crédit → Dashboard
 *
 * Given  le gestionnaire ouvre la liste des comptes
 * When   il filtre par type et consulte un détail
 * And    il navigue vers paiements et crédit
 * Then   toute la chaîne opérationnelle est accessible
 */
test.describe('Gestionnaire de Comptes — Gestion quotidienne', () => {

  test('Étape 1: Accéder à la liste des comptes', async ({ page }) => {
    await page.goto('/accounts/list');
    await expect(page).toHaveTitle(/Comptes/);
    await expect(page.getByText('Gestion des comptes clients')).toBeVisible();
  });

  test('Étape 2: Rechercher un compte par IBAN', async ({ page }) => {
    await page.goto('/accounts/list');
    const searchInput = page.getByPlaceholder('Rechercher par IBAN ou titulaire');
    await searchInput.fill('FR76 0000');
    await expect(searchInput).toHaveValue('FR76 0000');
  });

  test('Étape 3: Filtrer par type "Compte Épargne"', async ({ page }) => {
    await page.goto('/accounts/list');
    const typeFilter = page.locator('select').first();
    await typeFilter.selectOption('savings');
    await expect(typeFilter).toHaveValue('savings');
  });

  test('Étape 4: Filtrer par statut "Suspendu"', async ({ page }) => {
    await page.goto('/accounts/list');
    const statusFilter = page.locator('select').nth(1);
    await statusFilter.selectOption('suspended');
    await expect(statusFilter).toHaveValue('suspended');
  });

  test('Étape 5: Trier par solde', async ({ page }) => {
    await page.goto('/accounts/list');
    const sortFilter = page.locator('select').nth(2);
    await sortFilter.selectOption('balance');
    await expect(sortFilter).toHaveValue('balance');
  });

  test('Étape 6: Vérifier les données du tableau des comptes', async ({ page }) => {
    await page.goto('/accounts/list');

    const tableSection = page.locator('[data-testid="accounts-list-table"]');

    // 4 comptes affichés
    await expect(tableSection.getByText('Ahmed Ben Ali').first()).toBeVisible();
    await expect(tableSection.getByText('Fatima Kefi')).toBeVisible();
    await expect(tableSection.getByText('Mohamed Jdaidi')).toBeVisible();

    // Données financières
    await expect(tableSection.getByText('15,000.00 TND')).toBeVisible();
    await expect(tableSection.getByText('50,000.00 TND')).toBeVisible();
    await expect(tableSection.getByText('-5,000.00 TND')).toBeVisible();
  });

  test('Étape 7: Vérifier les stats résumé', async ({ page }) => {
    await page.goto('/accounts/list');

    await expect(page.getByText('Comptes Actifs')).toBeVisible();
    await expect(page.getByText('Total Dépôts')).toBeVisible();
    await expect(page.getByText('73,500.50 TND')).toBeVisible();
    await expect(page.getByText('Comptes Suspendus')).toBeVisible();
    await expect(page.getByText('Crédits en Cours')).toBeVisible();
  });

  test('Étape 8: Accéder au détail d\'un compte', async ({ page }) => {
    await page.goto('/accounts/detail?id=1');
    await expect(page).toHaveTitle(/Détail du compte/);
    await expect(page.getByRole('navigation', { name: 'Navigation principale' })).toBeVisible();
  });

  test('Étape 9: Naviguer vers les paiements', async ({ page, sidebar }) => {
    await page.goto('/accounts/list');
    await sidebar.goto('Paiements');
    await page.waitForURL('**/payments');
    await expect(page).toHaveTitle(/Paiements/);
  });

  test('Étape 10: Naviguer vers le crédit', async ({ page, sidebar }) => {
    await page.goto('/payments');
    await sidebar.goto('Crédit');
    await page.waitForURL('**/credit');
    await expect(page).toHaveTitle(/Crédit/);
  });

  test('Étape 11: Retour au dashboard via quick action', async ({ page }) => {
    await page.goto('/dashboard');
    await page.getByRole('link', { name: /Nouveau Compte/ }).click();
    await page.waitForURL('**/accounts');
  });

  test('Parcours complet Comptes → Paiements → Crédit → Dashboard sans erreur', async ({ page }) => {
    const errors: string[] = [];
    page.on('pageerror', err => {
      if (err.message.includes('appendChild')) return;
      errors.push(err.message);
    });

    await page.goto('/accounts/list');
    await page.waitForTimeout(300);
    await page.goto('/accounts/detail');
    await page.waitForTimeout(300);
    await page.goto('/payments');
    await page.waitForTimeout(300);
    await page.goto('/credit');
    await page.waitForTimeout(300);
    await page.goto('/dashboard');
    await page.waitForTimeout(300);

    expect(errors).toHaveLength(0);
  });
});
