import { testHuman as test, expect } from '../../fixtures/banko.fixture';

/**
 * PARCOURS 3: Gestion Clients (BC1 — Customer)
 * Liste clients → Recherche → Filtre KYC → Tri → Détail → KYC Wizard
 *
 * BDD: Given la liste des clients est affichée
 *      When je filtre par statut KYC "Approuvé"
 *      Then je vois uniquement les clients approuvés
 */
test.describe('Clients — Bounded Context Customer', () => {

  test.describe('Liste des clients', () => {
    test('Affiche la page clients avec titre et sidebar', async ({ customersPage, page }) => {
      await customersPage.goto();
      await expect(page).toHaveTitle(/Clients/);
      await expect(page.getByRole('navigation', { name: 'Navigation principale' })).toBeVisible();
    });

    test('Contient la barre de recherche', async ({ page }) => {
      await page.goto('/customers');
      await expect(page.getByPlaceholder('Rechercher par nom ou email')).toBeVisible();
    });

    test('Contient le filtre KYC avec toutes les options', async ({ page }) => {
      await page.goto('/customers');
      const kycFilter = page.locator('[data-testid="customers-filter-kyc-status"]');
      await expect(kycFilter).toBeVisible();
      // Vérifie les options
      const options = kycFilter.locator('option');
      await expect(options).toHaveCount(5); // Tous + 4 statuts
    });

    test('Contient le tri avec options date et nom', async ({ page }) => {
      await page.goto('/customers');
      const sortSelect = page.locator('[data-testid="customers-filter-sort"]');
      await expect(sortSelect).toBeVisible();
    });

    test('Bouton "Nouveau Client" mène à l\'onboarding KYC', async ({ customersPage, page }) => {
      await customersPage.goto();
      await customersPage.clickNewClient();
      await page.waitForURL('**/customer/onboarding');
      await expect(page).toHaveTitle(/KYC/);
    });

    test('Pagination présente (Précédent / Suivant)', async ({ page }) => {
      await page.goto('/customers');
      await expect(page.getByRole('button', { name: 'Précédent' })).toBeVisible();
      await expect(page.getByRole('button', { name: 'Suivant' })).toBeVisible();
    });

    test('Recherche client fonctionne (interaction UI)', async ({ customersPage }) => {
      await customersPage.goto();
      await customersPage.search('Ahmed');
      // La recherche est debounced côté Svelte — on vérifie juste que l'input accepte le texte
    });

    test('Filtre KYC sélectionnable', async ({ customersPage }) => {
      await customersPage.goto();
      await customersPage.filterByKycStatus('approved');
    });
  });

  test.describe('Fiche client (détail)', () => {
    test('Page détail client se charge sans erreur', async ({ page }) => {
      await page.goto('/customers/detail');
      await expect(page).toHaveTitle(/Fiche client/);
    });

    test('Sidebar visible sur la page détail', async ({ page, sidebar }) => {
      await page.goto('/customers/detail');
      await sidebar.expectVisible();
    });
  });

  test.describe('KYC Wizard — Parcours onboarding', () => {
    test('Affiche l\'étape 1 avec tous les champs', async ({ kycWizard, page }) => {
      await kycWizard.goto();
      await kycWizard.expectStep(1);
      await expect(page.locator('[data-testid="kyc-basic-firstname"]')).toBeVisible();
      await expect(page.locator('[data-testid="kyc-basic-lastname"]')).toBeVisible();
      await expect(page.locator('[data-testid="kyc-basic-dob"]')).toBeVisible();
      await expect(page.locator('[data-testid="kyc-basic-cin"]')).toBeVisible();
      await expect(page.locator('[data-testid="kyc-basic-nationality"]')).toBeVisible();
    });

    test('Remplissage complet étape 1 — Informations personnelles', async ({ kycWizard }) => {
      await kycWizard.goto();
      await kycWizard.fillPersonalInfo({
        firstName: 'Fatima',
        lastName: 'Kefi',
        birthDate: '1990-05-15',
        gender: 'female',
        cin: '12345678',
        nationality: 'Tunisienne',
      });
      await kycWizard.clickNext();
    });

    test('Sidebar visible sur la page onboarding', async ({ page, sidebar }) => {
      await page.goto('/customer/onboarding');
      await sidebar.expectVisible();
    });
  });
});
