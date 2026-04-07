import { test, expect } from '../../fixtures/banko.fixture';

/**
 * SCÉNARIO MULTI-RÔLE 2: Agent KYC — Onboarding nouveau client
 *
 * Parcours: Clients → Recherche → Filtre KYC → Nouveau Client → KYC Wizard 5 étapes
 *
 * Given  l'agent KYC reçoit une demande d'ouverture de compte
 * When   il vérifie la liste des clients existants
 * And    il lance le formulaire d'onboarding KYC
 * Then   il peut remplir les 5 étapes du wizard
 */
test.describe('Agent KYC — Onboarding nouveau client', () => {

  test('Étape 1: Consulter la liste des clients', async ({ customersPage, page }) => {
    await customersPage.goto();
    await expect(page).toHaveTitle(/Clients/);

    // Vérifier les outils de l'agent
    await expect(page.getByPlaceholder('Rechercher par nom ou email')).toBeVisible();
    await expect(page.getByRole('link', { name: 'Nouveau Client' })).toBeVisible();
  });

  test('Étape 2: Rechercher un client existant', async ({ customersPage, page }) => {
    await customersPage.goto();
    const searchInput = page.getByPlaceholder('Rechercher par nom ou email');
    await searchInput.fill('Ahmed Ben Ali');
    await expect(searchInput).toHaveValue('Ahmed Ben Ali');
  });

  test('Étape 3: Filtrer par statut KYC "En attente"', async ({ customersPage, page }) => {
    await customersPage.goto();
    await customersPage.filterByKycStatus('pending');
    const kycFilter = page.getByRole('combobox').first();
    await expect(kycFilter).toHaveValue('pending');
  });

  test('Étape 4: Trier les clients par nom A→Z', async ({ customersPage, page }) => {
    await customersPage.goto();
    await customersPage.sortBy('name');
    const sortSelect = page.getByRole('combobox').nth(1);
    await expect(sortSelect).toHaveValue('name');
  });

  test('Étape 5: Lancer l\'onboarding — accéder au wizard KYC', async ({ customersPage, page }) => {
    await customersPage.goto();
    await customersPage.clickNewClient();
    await page.waitForURL('**/customer/onboarding');
    await expect(page).toHaveTitle(/KYC/);
  });

  test('Étape 6: KYC Wizard — Remplir les informations personnelles (étape 1/5)', async ({ kycWizard, page }) => {
    await kycWizard.goto();
    await kycWizard.expectStep(1);

    // Remplir avec Playwright fill() (keyboard events → Svelte 5 $state OK)
    await page.getByLabel('Prenom', { exact: false }).fill('Fatima');
    await page.getByLabel('Nom', { exact: false }).fill('Kefi');
    await page.getByLabel('Date de naissance', { exact: false }).fill('1990-05-15');
    await page.getByRole('combobox').selectOption('female');
    await page.getByLabel('Numero CIN', { exact: false }).fill('12345678');
    await page.getByLabel('Nationalite', { exact: false }).fill('Tunisienne');

    // Vérifier les valeurs
    await expect(page.getByLabel('Prenom', { exact: false })).toHaveValue('Fatima');
    await expect(page.getByLabel('Nom', { exact: false })).toHaveValue('Kefi');
  });

  test('Étape 7: KYC Wizard — Avancer à l\'étape 2 (profession)', async ({ kycWizard, page }) => {
    await kycWizard.goto();

    // Remplir étape 1
    await page.getByLabel('Prenom', { exact: false }).fill('Fatima');
    await page.getByLabel('Nom', { exact: false }).fill('Kefi');
    await page.getByLabel('Date de naissance', { exact: false }).fill('1990-05-15');
    await page.getByRole('combobox').selectOption('female');
    await page.getByLabel('Numero CIN', { exact: false }).fill('12345678');
    await page.getByLabel('Nationalite', { exact: false }).fill('Tunisienne');

    // Soumettre
    await kycWizard.clickNext();

    // Vérifier qu'on est à l'étape 2
    await expect(page.getByLabel('Profession', { exact: false })).toBeVisible();
    await expect(page.getByLabel('Revenu mensuel', { exact: false })).toBeVisible();
  });

  test('Étape 8: KYC Wizard — Remplir profession et avancer', async ({ kycWizard, page }) => {
    await kycWizard.goto();

    // Étape 1
    await page.getByLabel('Prenom', { exact: false }).fill('Fatima');
    await page.getByLabel('Nom', { exact: false }).fill('Kefi');
    await page.getByLabel('Date de naissance', { exact: false }).fill('1990-05-15');
    await page.getByRole('combobox').selectOption('female');
    await page.getByLabel('Numero CIN', { exact: false }).fill('12345678');
    await page.getByLabel('Nationalite', { exact: false }).fill('Tunisienne');
    await kycWizard.clickNext();

    // Étape 2
    await page.getByLabel('Profession', { exact: false }).fill('Ingénieure');
    await page.getByLabel('Employeur', { exact: false }).fill('Aivacore SARL');
    await page.getByLabel('Revenu mensuel', { exact: false }).fill('3500');
    await page.getByRole('combobox').selectOption('salary');

    await expect(page.getByLabel('Profession', { exact: false })).toHaveValue('Ingénieure');
  });

  test('Étape 9: Bouton "Précédent" revient à l\'étape 1', async ({ kycWizard, page }) => {
    await kycWizard.goto();

    // Remplir et avancer
    await page.getByLabel('Prenom', { exact: false }).fill('Test');
    await page.getByLabel('Nom', { exact: false }).fill('User');
    await page.getByLabel('Date de naissance', { exact: false }).fill('1995-01-01');
    await page.getByRole('combobox').selectOption('male');
    await page.getByLabel('Numero CIN', { exact: false }).fill('99999999');
    await page.getByLabel('Nationalite', { exact: false }).fill('Tunisien');
    await kycWizard.clickNext();

    // Revenir
    await page.getByRole('button', { name: 'Precedent' }).click();

    // Vérifier qu'on est revenu à l'étape 1
    await kycWizard.expectStep(1);
    await expect(page.getByLabel('Prenom', { exact: false })).toBeVisible();
  });

  test('Parcours complet sans erreur JS', async ({ page }) => {
    const errors: string[] = [];
    page.on('pageerror', err => errors.push(err.message));

    await page.goto('/customers');
    await page.waitForTimeout(300);
    await page.goto('/customer/onboarding');
    await page.waitForTimeout(300);
    await page.goto('/customers/detail');
    await page.waitForTimeout(300);

    expect(errors).toHaveLength(0);
  });
});
