<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { authStore } from '../../lib/stores';
  import Icon from '@iconify/svelte';

  $: user = $authStore.user;
  $: isAuthenticated = $authStore.isAuthenticated;

  onMount(async () => {
    await authStore.initialize();
    
    const unsubscribe = authStore.subscribe(({ initialized, isAuthenticated }) => {
      if (initialized && !isAuthenticated) {
        goto('/login');
      }
    });

    // Check immediately after initialization
    if ($authStore.initialized && !$authStore.isAuthenticated) {
      goto('/login');
    }

    return unsubscribe;
  });

  async function logout() {
    await authStore.logout();
    goto('/');
  }

  $: currentPath = $page.url.pathname;
</script>

<div class="flex min-h-screen bg-background">
  <!-- Sidebar -->
  <aside class="w-64 bg-card border-r border-border flex-shrink-0 relative">
    <div class="p-6">
      <!-- Logo -->
      <div class="flex items-center space-x-2 mb-8">
        <div class="w-8 h-8 bg-black rounded-md flex items-center justify-center">
          <span class="text-white font-bold text-sm">K</span>
        </div>
        <span class="font-semibold text-xl">Knot Space</span>
      </div>

      <!-- User info -->
      {#if user}
        <div class="mb-8 p-3 bg-muted/50 rounded-lg">
          <div class="text-sm font-medium">{user.username}</div>
          <div class="text-xs text-muted-foreground">{user.email}</div>
        </div>
      {/if}

      <!-- Navigation -->
      <nav class="space-y-2">
        <a 
          href="/packages" 
          class="flex items-center space-x-3 px-3 py-2 rounded-lg text-sm font-medium transition-colors {currentPath === '/packages' ? 'bg-primary text-primary-foreground' : 'hover:bg-muted'}"
        >
          <Icon icon="solar:box-bold" class="w-5 h-5" />
          <span>Packages</span>
        </a>

        <a 
          href="/teams" 
          class="flex items-center space-x-3 px-3 py-2 rounded-lg text-sm font-medium transition-colors {currentPath === '/teams' ? 'bg-primary text-primary-foreground' : 'hover:bg-muted'}"
        >
          <Icon icon="solar:users-group-two-rounded-bold" class="w-5 h-5" />
          <span>Teams</span>
        </a>

        <a 
          href="/settings" 
          class="flex items-center space-x-3 px-3 py-2 rounded-lg text-sm font-medium transition-colors {currentPath === '/settings' ? 'bg-primary text-primary-foreground' : 'hover:bg-muted'}"
        >
          <Icon icon="solar:settings-bold" class="w-5 h-5" />
          <span>Settings</span>
        </a>

        <a 
          href="/docs" 
          class="flex items-center space-x-3 px-3 py-2 rounded-lg text-sm font-medium transition-colors {currentPath === '/docs' ? 'bg-primary text-primary-foreground' : 'hover:bg-muted'}"
        >
          <Icon icon="solar:book-bold" class="w-5 h-5" />
          <span>Documentation</span>
        </a>
      </nav>
    </div>

    <!-- Logout button at bottom -->
    <div class="absolute bottom-6 left-6 right-6">
      <button
        on:click={logout}
        class="w-full flex items-center space-x-3 px-3 py-2 rounded-lg text-sm font-medium text-destructive hover:bg-destructive/10 transition-colors"
      >
        <Icon icon="solar:logout-3-bold" class="w-5 h-5" />
        <span>Sign Out</span>
      </button>
    </div>
  </aside>

  <!-- Main content -->
  <main class="flex-1 p-8">
    <slot />
  </main>
</div>