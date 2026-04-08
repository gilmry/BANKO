import { defineConfig, devices } from '@playwright/test';

/**
 * BANKO E2E Test Configuration
 * Méthode Maury — Software Craftsmanship
 *
 * Features:
 * - Trace ON pour chaque test (debug complet)
 * - Screenshots on failure
 * - Video recording on first retry
 * - HTML reporter avec détails
 * - Retry automatique (CI: 2, local: 0)
 */
export default defineConfig({
  testDir: './tests',
  outputDir: './test-results',

  /* Timeout global par test: 30s */
  timeout: 30_000,

  /* Expect timeout: 5s (cohérent avec P99 < 5ms backend) */
  expect: {
    timeout: 5_000,
  },

  /* Fully parallel */
  fullyParallel: true,

  /* Forbid test.only in CI */
  forbidOnly: !!process.env.CI,

  /* Retries: 2 in CI, 0 local */
  retries: process.env.CI ? 2 : 0,

  /* Workers: 50% in CI, default local */
  workers: process.env.CI ? '50%' : undefined,

  /* Reporters */
  reporter: [
    ['html', { outputFolder: './playwright-report', open: 'never' }],
    ['json', { outputFile: './test-results/results.json' }],
    ['list'],
  ],

  /* Global settings */
  use: {
    /* Base URL — Traefik on port 80 (Docker: traefik:80 with Host header) */
    baseURL: process.env.PLAYWRIGHT_BASE_URL || 'http://localhost',


    /* TRACE: on pour CHAQUE test (mode debug/trace complet) */
    trace: 'on',

    /* Screenshot on failure */
    screenshot: 'only-on-failure',

    /* Video: enregistrement systématique */
    video: 'on',

    /* Locale français */
    locale: 'fr-FR',

    /* Timezone Tunis */
    timezoneId: 'Africa/Tunis',

    /* Viewport standard */
    viewport: { width: 1536, height: 776 },

    /* Action timeout */
    actionTimeout: 10_000,

    /* Navigation timeout */
    navigationTimeout: 15_000,
  },

  /* Projects: multi-browser */
  projects: [
    {
      name: 'chromium',
      use: {
        ...devices['Desktop Chrome'],
        // Utilise Chrome système si disponible (pas en CI/Docker)
        ...(process.env.CI ? {} : { channel: 'chrome' }),
      },
    },
    {
      name: 'firefox',
      use: { ...devices['Desktop Firefox'] },
    },
    {
      name: 'webkit',
      use: { ...devices['Desktop Safari'] },
    },
    /* Mobile viewports */
    {
      name: 'mobile-chrome',
      use: { ...devices['Pixel 5'] },
    },
    {
      name: 'mobile-safari',
      use: { ...devices['iPhone 13'] },
    },
  ],

  /* Web server — optionnel, si lancé hors Docker */
  // webServer: {
  //   command: 'cd ../frontend && npm run dev',
  //   url: 'http://localhost:3000',
  //   reuseExistingServer: !process.env.CI,
  // },
});
