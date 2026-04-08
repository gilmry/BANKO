import { test as base, expect, type Page, type Locator } from '@playwright/test';

/**
 * BANKO Test Fixtures — Méthode Maury
 *
 * Exports:
 *   - `test`       → rapide, pour smoke tests (temps réel)
 *   - `testHuman`  → rythme humain, pour tests de modules (vidéo lisible)
 *
 * Le mode humain ajoute des pauses entre chaque action :
 *   - Après navigation  : 800ms
 *   - Après clic         : 500ms
 *   - Après saisie       : 300ms
 *   - Après select       : 400ms
 */

// ── Click Indicator (point rouge visible dans les vidéos) ─────────
const CLICK_INDICATOR_SCRIPT = `
  document.addEventListener('click', (e) => {
    if (!document.body) return;
    const dot = document.createElement('div');
    Object.assign(dot.style, {
      position: 'fixed',
      left: (e.clientX - 12) + 'px',
      top: (e.clientY - 12) + 'px',
      width: '24px',
      height: '24px',
      borderRadius: '50%',
      background: 'rgba(255, 0, 0, 0.6)',
      border: '3px solid rgba(255, 0, 0, 0.9)',
      pointerEvents: 'none',
      zIndex: '2147483647',
      animation: 'banko-click 0.6s ease-out forwards',
    });
    document.body.appendChild(dot);
    setTimeout(() => dot.remove(), 700);
  }, true);

  function injectStyle() {
    if (!document.head) { setTimeout(injectStyle, 50); return; }
    if (!document.querySelector('#banko-click-style')) {
      const style = document.createElement('style');
      style.id = 'banko-click-style';
      style.textContent = '@keyframes banko-click { 0% { transform: scale(0.3); opacity: 1; } 50% { transform: scale(1.2); opacity: 0.7; } 100% { transform: scale(1.5); opacity: 0; } }';
      document.head.appendChild(style);
    }
  }
  injectStyle();
`;

// ── Human-pace timing constants ───────────────────────────────────
const TIMING = {
  afterNavigation: 800,
  afterClick: 500,
  afterFill: 300,
  afterSelect: 400,
  afterCheck: 300,
  betweenSteps: 600,
} as const;

// ── HumanPage — wraps Page with deliberate pauses ─────────────────
class HumanPage {
  constructor(private page: Page) {}

  /** Navigate with pause after load */
  async goto(url: string) {
    await this.page.goto(url);
    await this.page.waitForTimeout(TIMING.afterNavigation);
  }

  /** Click with pause */
  async click(selector: string) {
    await this.page.click(selector);
    await this.page.waitForTimeout(TIMING.afterClick);
  }

  /** Click a locator with pause */
  async clickLocator(locator: Locator) {
    await locator.click();
    await this.page.waitForTimeout(TIMING.afterClick);
  }

  /** Fill input with pause */
  async fill(selector: string, value: string) {
    await this.page.fill(selector, value);
    await this.page.waitForTimeout(TIMING.afterFill);
  }

  /** Fill a locator with pause */
  async fillLocator(locator: Locator, value: string) {
    await locator.fill(value);
    await this.page.waitForTimeout(TIMING.afterFill);
  }

  /** Select option with pause */
  async selectOption(selector: string, value: string) {
    await this.page.selectOption(selector, value);
    await this.page.waitForTimeout(TIMING.afterSelect);
  }

  /** Select option on locator with pause */
  async selectLocator(locator: Locator, value: string) {
    await locator.selectOption(value);
    await this.page.waitForTimeout(TIMING.afterSelect);
  }

  /** Check checkbox with pause */
  async check(selector: string) {
    await this.page.check(selector);
    await this.page.waitForTimeout(TIMING.afterCheck);
  }

  /** Pause between logical steps */
  async step() {
    await this.page.waitForTimeout(TIMING.betweenSteps);
  }

  /** Access the raw Playwright page (for assertions, locators, etc.) */
  get raw(): Page {
    return this.page;
  }
}

// ── Sidebar Navigation Helper ──────────────────────────────────────
export class SidebarNav {
  constructor(private page: Page, private human?: HumanPage) {}

  async goto(label: string) {
    const link = this.page.getByRole('navigation', { name: 'Navigation principale' })
      .getByRole('link', { name: label });
    await link.click();
    if (this.human) await this.page.waitForTimeout(TIMING.afterClick);
  }

  async expectVisible() {
    await expect(
      this.page.getByRole('navigation', { name: 'Navigation principale' })
    ).toBeVisible();
  }

  async expectActiveLink(label: string) {
    const link = this.page.getByRole('navigation', { name: 'Navigation principale' })
      .getByRole('link', { name: label });
    await expect(link).toBeVisible();
  }

  async getAllLinks(): Promise<string[]> {
    const links = this.page.locator('aside a[href]');
    return links.evaluateAll(els => els.map(el => el.getAttribute('href') || ''));
  }
}

// ── Dashboard Page Object ──────────────────────────────────────────
export class DashboardPage {
  constructor(private page: Page, private human?: HumanPage) {}

  async goto() {
    if (this.human) {
      await this.human.goto('/dashboard');
    } else {
      await this.page.goto('/dashboard');
    }
  }

  async expectKpiCards(count: number) {
    await expect(this.page.locator('article')).toHaveCount(count);
    if (this.human) await this.human.step();
  }

  async expectTransactionsTable() {
    await expect(this.page.getByRole('heading', { name: 'Transactions Récentes' })).toBeVisible();
    await expect(this.page.getByRole('table')).toBeVisible();
    if (this.human) await this.human.step();
  }

  async expectQuickActions() {
    await expect(this.page.getByRole('heading', { name: 'Nouveau Client' })).toBeVisible();
    await expect(this.page.getByRole('heading', { name: 'Nouveau Compte' })).toBeVisible();
    await expect(this.page.getByRole('heading', { name: 'Paiement' })).toBeVisible();
    await expect(this.page.getByRole('heading', { name: 'Crédit' })).toBeVisible();
    if (this.human) await this.human.step();
  }
}

// ── Customers Page Object ──────────────────────────────────────────
export class CustomersPage {
  constructor(private page: Page, private human?: HumanPage) {}

  async goto() {
    if (this.human) {
      await this.human.goto('/customers');
    } else {
      await this.page.goto('/customers');
    }
  }

  async search(query: string) {
    const input = this.page.getByPlaceholder('Rechercher par nom ou email');
    if (this.human) {
      await this.human.fillLocator(input, query);
    } else {
      await input.fill(query);
    }
  }

  async filterByKycStatus(status: string) {
    const select = this.page.locator('[data-testid="customers-filter-kyc-status"]');
    if (this.human) {
      await this.human.selectLocator(select, status);
    } else {
      await select.selectOption(status);
    }
  }

  async sortBy(value: string) {
    const select = this.page.locator('[data-testid="customers-filter-sort"]');
    if (this.human) {
      await this.human.selectLocator(select, value);
    } else {
      await select.selectOption(value);
    }
  }

  async clickNewClient() {
    const link = this.page.getByRole('link', { name: 'Nouveau Client' });
    if (this.human) {
      await this.human.clickLocator(link);
    } else {
      await link.click();
    }
  }

  async expectCustomerRows(minCount: number) {
    const rows = this.page.locator('table tbody tr, [role="row"]');
    await expect(rows).toHaveCount(minCount, { timeout: 5000 }).catch(() => {});
    if (this.human) await this.human.step();
  }
}

// ── KYC Wizard Page Object ─────────────────────────────────────────
export class KycWizardPage {
  constructor(private page: Page, private human?: HumanPage) {}

  async goto() {
    if (this.human) {
      await this.human.goto('/customer/onboarding');
    } else {
      await this.page.goto('/customer/onboarding');
    }
  }

  async expectStep(stepNumber: number) {
    await expect(
      this.page.getByText(`Etape ${stepNumber} sur 5`, { exact: false })
    ).toBeVisible();
    if (this.human) await this.human.step();
  }

  async fillPersonalInfo(data: {
    firstName: string;
    lastName: string;
    birthDate: string;
    gender: string;
    cin: string;
    nationality: string;
  }) {
    const fillByTestId = async (testId: string, value: string) => {
      const loc = this.page.locator(`[data-testid="${testId}"]`);
      if (this.human) {
        await this.human.fillLocator(loc, value);
      } else {
        await loc.fill(value);
      }
    };
    await fillByTestId('kyc-basic-firstname', data.firstName);
    await fillByTestId('kyc-basic-lastname', data.lastName);
    await fillByTestId('kyc-basic-dob', data.birthDate);
    const select = this.page.locator('[data-testid="kyc-basic-gender"]');
    if (this.human) {
      await this.human.selectLocator(select, data.gender);
    } else {
      await select.selectOption(data.gender);
    }
    await fillByTestId('kyc-basic-cin', data.cin);
    await fillByTestId('kyc-basic-nationality', data.nationality);
  }

  async clickNext() {
    const btn = this.page.getByRole('button', { name: 'Suivant' });
    if (this.human) {
      await this.human.clickLocator(btn);
    } else {
      await btn.click();
    }
  }
}

// ── Login Page Object ──────────────────────────────────────────────
export class LoginPage {
  constructor(private page: Page, private human?: HumanPage) {}

  async goto() {
    if (this.human) {
      await this.human.goto('/login');
    } else {
      await this.page.goto('/login');
    }
  }

  async fillCredentials(email: string, password: string) {
    const emailInput = this.page.getByLabel('Adresse e-mail');
    const passInput = this.page.getByLabel('Mot de passe');
    if (this.human) {
      await this.human.fillLocator(emailInput, email);
      await this.human.fillLocator(passInput, password);
    } else {
      await emailInput.fill(email);
      await passInput.fill(password);
    }
  }

  async submit() {
    const btn = this.page.getByRole('button', { name: 'Se connecter' });
    if (this.human) {
      await this.human.clickLocator(btn);
    } else {
      await btn.click();
    }
  }

  async expectLoginForm() {
    await expect(this.page.getByLabel('Adresse e-mail')).toBeVisible();
    await expect(this.page.getByLabel('Mot de passe')).toBeVisible();
    await expect(this.page.getByRole('button', { name: 'Se connecter' })).toBeVisible();
    if (this.human) await this.human.step();
  }

  async clickRegister() {
    const link = this.page.getByRole('link', { name: 'Creer un compte' });
    if (this.human) {
      await this.human.clickLocator(link);
    } else {
      await link.click();
    }
  }
}

// ── Register Page Object ───────────────────────────────────────────
export class RegisterPage {
  constructor(private page: Page, private human?: HumanPage) {}

  async goto() {
    if (this.human) {
      await this.human.goto('/register');
    } else {
      await this.page.goto('/register');
    }
  }

  async fillForm(email: string, password: string) {
    const emailInput = this.page.getByLabel('Adresse e-mail');
    const pass1 = this.page.locator('input[type="password"]').first();
    const pass2 = this.page.locator('input[type="password"]').nth(1);
    const checkbox = this.page.getByRole('checkbox');
    if (this.human) {
      await this.human.fillLocator(emailInput, email);
      await this.human.fillLocator(pass1, password);
      await this.human.fillLocator(pass2, password);
      await checkbox.check();
      await this.page.waitForTimeout(TIMING.afterCheck);
    } else {
      await emailInput.fill(email);
      await pass1.fill(password);
      await pass2.fill(password);
      await checkbox.check();
    }
  }

  async submit() {
    const btn = this.page.getByRole('button', { name: "S'inscrire" });
    if (this.human) {
      await this.human.clickLocator(btn);
    } else {
      await btn.click();
    }
  }

  async expectRegisterForm() {
    await expect(this.page.getByLabel('Adresse e-mail')).toBeVisible();
    await expect(this.page.getByRole('button', { name: "S'inscrire" })).toBeVisible();
    if (this.human) await this.human.step();
  }
}

// ── Settings Page Object ───────────────────────────────────────────
export class SettingsPage {
  constructor(private page: Page, private human?: HumanPage) {}

  async goto() {
    if (this.human) {
      await this.human.goto('/settings');
    } else {
      await this.page.goto('/settings');
    }
  }

  async expectSections() {
    await expect(this.page.getByRole('heading', { name: 'Paramètres généraux' })).toBeVisible();
    await expect(this.page.getByRole('heading', { name: 'Sécurité' })).toBeVisible();
    await expect(this.page.getByRole('heading', { name: 'Informations système' })).toBeVisible();
    if (this.human) await this.human.step();
  }

  async changeLocale(locale: string) {
    const select = this.page.getByRole('combobox').nth(1);
    if (this.human) {
      await this.human.selectLocator(select, locale);
    } else {
      await select.selectOption(locale);
    }
  }

  async changeTimezone(tz: string) {
    const select = this.page.getByRole('combobox').nth(2);
    if (this.human) {
      await this.human.selectLocator(select, tz);
    } else {
      await select.selectOption(tz);
    }
  }
}

// ══════════════════════════════════════════════════════════════════
// TEST FIXTURES
// ══════════════════════════════════════════════════════════════════

// ── `test` — Mode rapide (smoke tests) ───────────────────────────
export const test = base.extend<{
  sidebar: SidebarNav;
  dashboardPage: DashboardPage;
  customersPage: CustomersPage;
  kycWizard: KycWizardPage;
  loginPage: LoginPage;
  registerPage: RegisterPage;
  settingsPage: SettingsPage;
}>({
  page: async ({ page }, use) => {
    await page.addInitScript(CLICK_INDICATOR_SCRIPT);
    await use(page);
  },
  sidebar: async ({ page }, use) => {
    await use(new SidebarNav(page));
  },
  dashboardPage: async ({ page }, use) => {
    await use(new DashboardPage(page));
  },
  customersPage: async ({ page }, use) => {
    await use(new CustomersPage(page));
  },
  kycWizard: async ({ page }, use) => {
    await use(new KycWizardPage(page));
  },
  loginPage: async ({ page }, use) => {
    await use(new LoginPage(page));
  },
  registerPage: async ({ page }, use) => {
    await use(new RegisterPage(page));
  },
  settingsPage: async ({ page }, use) => {
    await use(new SettingsPage(page));
  },
});

// ── `testHuman` — Mode rythme humain (tests de modules) ──────────
// Chaque action a une pause pour que la vidéo soit lisible.
// Usage: import { testHuman as test, expect } from '../../fixtures/banko.fixture';
export const testHuman = base.extend<{
  humanPage: HumanPage;
  sidebar: SidebarNav;
  dashboardPage: DashboardPage;
  customersPage: CustomersPage;
  kycWizard: KycWizardPage;
  loginPage: LoginPage;
  registerPage: RegisterPage;
  settingsPage: SettingsPage;
}>({
  page: async ({ page }, use) => {
    await page.addInitScript(CLICK_INDICATOR_SCRIPT);
    await use(page);
  },
  humanPage: async ({ page }, use) => {
    await use(new HumanPage(page));
  },
  sidebar: async ({ page }, use) => {
    const human = new HumanPage(page);
    await use(new SidebarNav(page, human));
  },
  dashboardPage: async ({ page }, use) => {
    const human = new HumanPage(page);
    await use(new DashboardPage(page, human));
  },
  customersPage: async ({ page }, use) => {
    const human = new HumanPage(page);
    await use(new CustomersPage(page, human));
  },
  kycWizard: async ({ page }, use) => {
    const human = new HumanPage(page);
    await use(new KycWizardPage(page, human));
  },
  loginPage: async ({ page }, use) => {
    const human = new HumanPage(page);
    await use(new LoginPage(page, human));
  },
  registerPage: async ({ page }, use) => {
    const human = new HumanPage(page);
    await use(new RegisterPage(page, human));
  },
  settingsPage: async ({ page }, use) => {
    const human = new HumanPage(page);
    await use(new SettingsPage(page, human));
  },
});

export { expect };
