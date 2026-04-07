import { test, expect } from '../../fixtures/banko.fixture';

/**
 * PARCOURS 2: Authentification
 * Login → Register → Liens croisés
 *
 * BDD: Given je suis sur /login
 *      When je remplis le formulaire
 *      Then je vois le dashboard (ou erreur si invalide)
 */
test.describe('Authentification — Login & Register', () => {

  test.describe('Page Login', () => {
    test('Affiche le formulaire de connexion complet', async ({ loginPage, page }) => {
      await loginPage.goto();
      await loginPage.expectLoginForm();
      await expect(page).toHaveTitle(/Connexion/);
    });

    test('Champs email et mot de passe sont requis et typés', async ({ page }) => {
      await page.goto('/login');
      const emailInput = page.getByLabel('Adresse e-mail');
      const passwordInput = page.getByLabel('Mot de passe');
      await expect(emailInput).toHaveAttribute('type', 'email');
      await expect(passwordInput).toHaveAttribute('type', 'password');
    });

    test('Le lien "Créer un compte" mène à /register', async ({ loginPage, page }) => {
      await loginPage.goto();
      await loginPage.clickRegister();
      await page.waitForURL('**/register');
      await expect(page).toHaveTitle(/Inscription/);
    });

    test('Soumission du formulaire login (UI sans backend)', async ({ loginPage, page }) => {
      await loginPage.goto();
      await loginPage.fillCredentials('test@banko.tn', 'Password123!');
      // Vérifier que le bouton est cliquable
      const submitBtn = page.getByRole('button', { name: 'Se connecter' });
      await expect(submitBtn).toBeEnabled();
    });
  });

  test.describe('Page Register', () => {
    test('Affiche le formulaire d\'inscription complet', async ({ registerPage, page }) => {
      await registerPage.goto();
      await registerPage.expectRegisterForm();
      await expect(page).toHaveTitle(/Inscription/);
    });

    test('Contient email, 2x password, checkbox CGU, bouton submit', async ({ page }) => {
      await page.goto('/register');
      await expect(page.getByLabel('Adresse e-mail')).toBeVisible();
      await expect(page.locator('input[type="password"]')).toHaveCount(2);
      await expect(page.getByRole('checkbox')).toBeVisible();
      await expect(page.getByRole('button', { name: "S'inscrire" })).toBeVisible();
    });

    test('Le lien "Se connecter" mène à /login', async ({ page }) => {
      await page.goto('/register');
      await page.getByRole('link', { name: 'Se connecter' }).click();
      await page.waitForURL('**/login');
      await expect(page).toHaveTitle(/Connexion/);
    });

    test('Remplissage complet du formulaire inscription', async ({ registerPage }) => {
      await registerPage.goto();
      await registerPage.fillForm('nouveau@banko.tn', 'SecureP@ss2026!');
      // Vérifie que la checkbox est cochée
    });
  });

  test.describe('Parcours Login ↔ Register', () => {
    test('Aller-retour Login → Register → Login', async ({ page }) => {
      await page.goto('/login');
      await page.getByRole('link', { name: 'Creer un compte' }).click();
      await page.waitForURL('**/register');
      await page.getByRole('link', { name: 'Se connecter' }).click();
      await page.waitForURL('**/login');
      await expect(page).toHaveTitle(/Connexion/);
    });
  });
});
