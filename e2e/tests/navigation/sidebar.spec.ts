import { testHuman as test, expect } from '../../fixtures/banko.fixture';

/**
 * PARCOURS 1: Navigation Sidebar
 * Vérifie que tous les liens du Sidebar sont accessibles et mènent
 * aux bonnes pages (0 dead links, 0 404).
 *
 * Méthode Maury: Chaque lien du Sidebar = 1 test atomique.
 * BDD: Given je suis sur le dashboard / When je clique sur X / Then je vois la page X
 */
test.describe('Sidebar Navigation — Parcours complet', () => {
  const sidebarLinks = [
    { label: 'Tableau de bord', path: '/dashboard', title: 'Tableau de bord' },
    { label: 'Clients', path: '/customers', title: 'Clients' },
    { label: 'Comptes', path: '/accounts', title: 'Mes comptes' },
    { label: 'Paiements', path: '/payments', title: 'Tableau de bord Paiements' },
    { label: 'Crédit', path: '/credit', title: 'Tableau de bord Crédit' },
    { label: 'Devises', path: '/foreign-exchange', title: 'Devises & Change' },
    { label: 'AML', path: '/aml', title: 'Tableau de bord AML' },
    { label: 'Sanctions', path: '/sanctions', title: 'Sanctions' },
    { label: 'Audit', path: '/audit/log', title: "Journal d'audit" },
    { label: 'Risques', path: '/dashboards/risk', title: 'Tableau de bord prudentiel' },
    { label: 'Paramètres', path: '/settings', title: 'Paramètres' },
    { label: 'Rapports', path: '/reporting', title: 'Rapports' },
  ];

  test.beforeEach(async ({ page }) => {
    await page.goto('/dashboard');
  });

  test('Le Sidebar affiche toutes les sections de navigation', async ({ sidebar }) => {
    await sidebar.expectVisible();
    const links = await sidebar.getAllLinks();
    expect(links).toContain('/dashboard');
    expect(links).toContain('/customers');
    expect(links).toContain('/accounts');
    expect(links).toContain('/payments');
    expect(links).toContain('/credit');
    expect(links).toContain('/foreign-exchange');
    expect(links).toContain('/aml');
    expect(links).toContain('/sanctions');
    expect(links).toContain('/audit/log');
    expect(links).toContain('/dashboards/risk');
    expect(links).toContain('/settings');
    expect(links).toContain('/reporting');
  });

  for (const link of sidebarLinks) {
    test(`Navigation: ${link.label} → ${link.path}`, async ({ page, sidebar }) => {
      await sidebar.goto(link.label);
      await page.waitForURL(`**${link.path}`);
      await expect(page).toHaveTitle(new RegExp(link.title));
      // Vérifie pas d'erreur console
      const errors: string[] = [];
      page.on('pageerror', err => errors.push(err.message));
      await page.waitForTimeout(500);
      expect(errors).toHaveLength(0);
    });
  }

  test('Le logo BANKO renvoie à la landing page', async ({ page }) => {
    await page.getByRole('link', { name: 'BANKO - Accueil' }).click();
    await page.waitForURL('**/');
    await expect(page).toHaveTitle(/Accueil/);
  });

  test('Breadcrumb affiche le titre de la page courante', async ({ page }) => {
    await page.goto('/dashboard');
    const breadcrumb = page.locator('[data-testid="header-breadcrumb-current"]');
    await expect(breadcrumb).toBeVisible();
    await expect(breadcrumb).toHaveText('Tableau de bord');
  });
});
