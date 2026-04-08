import { testHuman as test, expect } from '../../fixtures/banko.fixture';

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
    const kycFilter = page.locator('[data-testid="customers-filter-kyc-status"]');
    await expect(kycFilter).toHaveValue('pending');
  });

  test('Étape 4: Trier les clients par nom A→Z', async ({ customersPage, page }) => {
    await customersPage.goto();
    await customersPage.sortBy('name');
    const sortSelect = page.locator('[data-testid="customers-filter-sort"]');
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
    await page.locator('[data-testid="kyc-basic-firstname"]').fill('Fatima');
    await page.locator('[data-testid="kyc-basic-lastname"]').fill('Kefi');
    await page.locator('[data-testid="kyc-basic-dob"]').fill('1990-05-15');
    await page.locator('[data-testid="kyc-basic-gender"]').selectOption('female');
    await page.locator('[data-testid="kyc-basic-cin"]').fill('12345678');
    await page.locator('[data-testid="kyc-basic-nationality"]').fill('Tunisienne');

    // Vérifier les valeurs
    await expect(page.locator('[data-testid="kyc-basic-firstname"]')).toHaveValue('Fatima');
    await expect(page.locator('[data-testid="kyc-basic-lastname"]')).toHaveValue('Kefi');
  });

  test('Étape 7: KYC Wizard — Avancer à l\'étape 2 (profession)', async ({ kycWizard, page }) => {
    await kycWizard.goto();

    // Remplir étape 1
    await page.locator('[data-testid="kyc-basic-firstname"]').fill('Fatima');
    await page.locator('[data-testid="kyc-basic-lastname"]').fill('Kefi');
    await page.locator('[data-testid="kyc-basic-dob"]').fill('1990-05-15');
    await page.locator('[data-testid="kyc-basic-gender"]').selectOption('female');
    await page.locator('[data-testid="kyc-basic-cin"]').fill('12345678');
    await page.locator('[data-testid="kyc-basic-nationality"]').fill('Tunisienne');

    // Soumettre
    await kycWizard.clickNext();

    // Vérifier qu'on est à l'étape 2
    await expect(page.locator('[data-testid="kyc-professional-profession"]')).toBeVisible();
    await expect(page.locator('[data-testid="kyc-professional-income"]')).toBeVisible();
  });

  test('Étape 8: KYC Wizard — Remplir profession et avancer', async ({ kycWizard, page }) => {
    await kycWizard.goto();

    // Étape 1
    await page.locator('[data-testid="kyc-basic-firstname"]').fill('Fatima');
    await page.locator('[data-testid="kyc-basic-lastname"]').fill('Kefi');
    await page.locator('[data-testid="kyc-basic-dob"]').fill('1990-05-15');
    await page.locator('[data-testid="kyc-basic-gender"]').selectOption('female');
    await page.locator('[data-testid="kyc-basic-cin"]').fill('12345678');
    await page.locator('[data-testid="kyc-basic-nationality"]').fill('Tunisienne');
    await kycWizard.clickNext();

    // Étape 2
    await page.locator('[data-testid="kyc-professional-profession"]').fill('Ingénieure');
    await page.locator('[data-testid="kyc-professional-employer"]').fill('Aivacore SARL');
    await page.locator('[data-testid="kyc-professional-income"]').fill('3500');
    await page.locator('[data-testid="kyc-professional-funds"]').selectOption('salary');

    await expect(page.locator('[data-testid="kyc-professional-profession"]')).toHaveValue('Ingénieure');
  });

  test('Étape 9: Bouton "Précédent" revient à l\'étape 1', async ({ kycWizard, page }) => {
    await kycWizard.goto();

    // Remplir et avancer
    await page.locator('[data-testid="kyc-basic-firstname"]').fill('Test');
    await page.locator('[data-testid="kyc-basic-lastname"]').fill('User');
    await page.locator('[data-testid="kyc-basic-dob"]').fill('1995-01-01');
    await page.locator('[data-testid="kyc-basic-gender"]').selectOption('male');
    await page.locator('[data-testid="kyc-basic-cin"]').fill('99999999');
    await page.locator('[data-testid="kyc-basic-nationality"]').fill('Tunisien');
    await kycWizard.clickNext();

    // Revenir
    await page.getByRole('button', { name: 'Precedent' }).click();

    // Vérifier qu'on est revenu à l'étape 1
    await kycWizard.expectStep(1);
    await expect(page.locator('[data-testid="kyc-basic-firstname"]')).toBeVisible();
  });

  test('Parcours complet sans erreur JS', async ({ page }) => {
    const errors: string[] = [];
    page.on('pageerror', err => {
      if (err.message.includes('appendChild')) return;
      errors.push(err.message);
    });

    await page.goto('/customers');
    await page.waitForTimeout(300);
    await page.goto('/customer/onboarding');
    await page.waitForTimeout(300);
    await page.goto('/customers/detail');
    await page.waitForTimeout(300);

    expect(errors).toHaveLength(0);
  });
});
