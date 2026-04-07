<script lang="ts">
  import { onMount } from 'svelte';

  let isMobileOpen = $state(false);
  const currentPath = new URL(globalThis.location?.href || '').pathname;

  const navigationItems = [
    {
      section: 'Principal',
      items: [
        { icon: '📊', label: 'Tableau de bord', href: '/' },
        { icon: '👥', label: 'Clients', href: '/customer/onboarding' },
        { icon: '💳', label: 'Comptes', href: '/accounts' },
      ],
    },
    {
      section: 'Opérations',
      items: [
        { icon: '💰', label: 'Paiements', href: '/payments' },
        { icon: '📈', label: 'Crédit', href: '/credit' },
        { icon: '🔄', label: 'Devises', href: '/foreign-exchange' },
      ],
    },
    {
      section: 'Conformité',
      items: [
        { icon: '⚠️', label: 'AML', href: '/aml' },
        { icon: '🛡️', label: 'Sanctions', href: '/sanctions' },
        { icon: '📋', label: 'Audit', href: '/audit/log' },
        { icon: '📊', label: 'Risques', href: '/dashboards/risk' },
      ],
    },
    {
      section: 'Administration',
      items: [
        { icon: '⚙️', label: 'Paramètres', href: '/settings' },
        { icon: '📝', label: 'Rapports', href: '/reporting' },
      ],
    },
  ];

  function isActive(href: string): boolean {
    return currentPath === href || currentPath.startsWith(href + '/');
  }

  onMount(() => {
    const btn = document.getElementById('mobile-menu-btn');
    if (btn) {
      btn.addEventListener('click', () => {
        isMobileOpen = !isMobileOpen;
      });
    }
  });
</script>

<aside
  class={`fixed left-0 top-0 z-20 h-full w-64 border-e border-gray-200 bg-white shadow-lg transition-transform lg:sticky lg:shadow-none ${
    isMobileOpen ? 'translate-x-0' : '-translate-x-full lg:translate-x-0'
  }`}
  role="navigation"
  aria-label="Navigation principale"
>
  <!-- Logo -->
  <div class="flex h-16 items-center border-b border-gray-200 px-6">
    <a href="/" class="text-xl font-bold text-gray-900" aria-label="BANKO - Accueil">
      BANKO
    </a>
  </div>

  <!-- Navigation sections -->
  <nav class="space-y-8 overflow-y-auto px-3 py-6 h-[calc(100vh-64px)]">
    {#each navigationItems as section}
      <div>
        <p class="px-3 text-xs font-semibold uppercase tracking-wider text-gray-400">
          {section.section}
        </p>
        <div class="mt-3 space-y-1">
          {#each section.items as item}
            <a
              href={item.href}
              class={`nav-item flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600 ${
                isActive(item.href)
                  ? 'bg-blue-50 text-blue-700 border-l-4 border-blue-600'
                  : 'text-gray-700 hover:bg-gray-100 hover:text-gray-900'
              }`}
            >
              <span class="text-lg" aria-hidden="true">{item.icon}</span>
              {item.label}
            </a>
          {/each}
        </div>
      </div>
    {/each}
  </nav>
</aside>

<!-- Mobile overlay -->
{#if isMobileOpen}
  <div
    class="fixed inset-0 z-10 bg-black/50 lg:hidden"
    onclick={() => {
      isMobileOpen = false;
    }}
  ></div>
{/if}

<style>
  aside {
    scrollbar-width: thin;
    scrollbar-color: rgba(156, 163, 175, 0.5) transparent;
  }

  aside::-webkit-scrollbar {
    width: 6px;
  }

  aside::-webkit-scrollbar-track {
    background: transparent;
  }

  aside::-webkit-scrollbar-thumb {
    background-color: rgba(156, 163, 175, 0.5);
    border-radius: 3px;
  }

  aside::-webkit-scrollbar-thumb:hover {
    background-color: rgba(107, 114, 128, 0.7);
  }
</style>
