import { testHuman as test, expect } from '../../fixtures/banko.fixture';

/**
 * SCÉNARIO MULTI-RÔLE 4: Trader FX — Consultation taux et opérations de change
 *
 * Parcours: FX → Taux BCT → Opérations récentes → Nouvelle opération → Dashboard
 *
 * Given  le trader ouvre la page devises
 * When   il consulte les taux BCT du jour
 * And    il vérifie les opérations en cours
 * Then   il peut initier une nouvelle opération de change
 */
test.describe('Trader FX — Consultation taux et opérations', () => {

  test('Étape 1: Accéder à la page Devises & Change', async ({ page }) => {
    await page.goto('/foreign-exchange');
    await expect(page).toHaveTitle(/Devises/);
    await expect(page.getByRole('heading', { name: 'Devises & Change' })).toBeVisible();
  });

  test('Étape 2: Consulter les taux live (KPI cards)', async ({ page }) => {
    await page.goto('/foreign-exchange');

    // USD/TND
    await expect(page.getByText('USD/TND')).toBeVisible();
    await expect(page.getByText('3.1245').first()).toBeVisible();
    await expect(page.getByText('+0.12% aujourd\'hui')).toBeVisible();

    // EUR/TND
    await expect(page.getByText('EUR/TND')).toBeVisible();
    await expect(page.getByText('3.3890').first()).toBeVisible();

    // GBP/TND
    await expect(page.getByText('GBP/TND')).toBeVisible();
    await expect(page.getByText('3.9510').first()).toBeVisible();

    // Volume
    await expect(page.getByText('Opérations du jour')).toBeVisible();
    await expect(page.getByText('Volume: 2.3M TND')).toBeVisible();
  });

  test('Étape 3: Consulter la table des taux BCT', async ({ page }) => {
    await page.goto('/foreign-exchange');

    await expect(page.getByRole('heading', { name: 'Taux de change BCT' })).toBeVisible();
    await expect(page.getByText('Dernière mise à jour: 07/04/2026 09:30')).toBeVisible();

    // 6 devises
    const devises = ['Dollar américain', 'Euro', 'Livre sterling', 'Yen japonais (100)', 'Dirham marocain', 'Dinar algérien'];
    for (const devise of devises) {
      await expect(page.getByText(devise)).toBeVisible();
    }

    // Vérifier les colonnes Achat/Vente
    await expect(page.getByText('Achat').first()).toBeVisible();
    await expect(page.getByText('Vente').first()).toBeVisible();
    await expect(page.getByText('Variation').first()).toBeVisible();
  });

  test('Étape 4: Vérifier le spread Achat/Vente USD', async ({ page }) => {
    await page.goto('/foreign-exchange');

    // USD: Achat 3.1200, Vente 3.1290 → spread 0.0090
    await expect(page.getByText('3.1200')).toBeVisible();
    await expect(page.getByText('3.1290')).toBeVisible();
  });

  test('Étape 5: Consulter les opérations de change récentes', async ({ page }) => {
    await page.goto('/foreign-exchange');

    await expect(page.getByRole('heading', { name: 'Opérations de change récentes' })).toBeVisible();

    // Opération 1: Ahmed Ben Ali — Achat EUR 10,000
    await expect(page.getByText('Ahmed Ben Ali')).toBeVisible();
    await expect(page.getByText('Achat EUR')).toBeVisible();
    await expect(page.getByText('10,000.00 EUR')).toBeVisible();

    // Opération 2: TunisExport — Vente USD en cours
    await expect(page.getByText('Société TunisExport')).toBeVisible();
    await expect(page.getByText('50,000.00 USD')).toBeVisible();
    await expect(page.getByText('En cours')).toBeVisible();

    // Opération 3: Leïla Saïdi — Achat GBP
    await expect(page.getByText('Leïla Saïdi')).toBeVisible();
    await expect(page.getByText('5,000.00 GBP')).toBeVisible();
  });

  test('Étape 6: Bouton "Nouvelle opération" disponible', async ({ page }) => {
    await page.goto('/foreign-exchange');
    const btn = page.getByRole('button', { name: 'Nouvelle opération' });
    await expect(btn).toBeVisible();
    await expect(btn).toBeEnabled();
  });

  test('Étape 7: Retour au dashboard depuis FX via sidebar', async ({ page, sidebar }) => {
    await page.goto('/foreign-exchange');
    await sidebar.goto('Tableau de bord');
    await page.waitForURL('**/dashboard');
    await expect(page).toHaveTitle(/Tableau de bord/);
  });

  test('Parcours complet FX sans erreur JS', async ({ page }) => {
    const errors: string[] = [];
    page.on('pageerror', err => {
      if (err.message.includes('appendChild')) return;
      errors.push(err.message);
    });

    await page.goto('/foreign-exchange');
    await page.waitForTimeout(500);

    expect(errors).toHaveLength(0);
  });
});
