<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { authStore } from '../../lib/stores';
  import Icon from '@iconify/svelte';

  $: user = $authStore.user;
  $: isAuthenticated = $authStore.isAuthenticated;

  onMount(async () => {
    // Don't initialize again if already initialized
    if (!$authStore.initialized) {
      await authStore.initialize();
    }
    
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


  $: currentPath = $page.url.pathname;
</script>

<div class="flex min-h-screen bg-background">
  <!-- Sidebar -->
  <aside class="w-64 h-screen bg-card border-r border-border flex-shrink-0 relative">
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
          <div class="flex items-center gap-3">
            <!-- Profile avatar matching the profile page design -->
            <div class="w-10 h-10 bg-gradient-to-br from-primary to-primary/80 rounded-xl flex items-center justify-center text-primary-foreground text-sm font-bold flex-shrink-0">
              {user.username.charAt(0).toUpperCase()}
            </div>
            <div class="flex-1 min-w-0">
              <div class="text-sm font-medium truncate">{user.username}</div>
              <div class="text-xs text-muted-foreground truncate">{user.email}</div>
            </div>
          </div>
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
          href="/users" 
          class="flex items-center space-x-3 px-3 py-2 rounded-lg text-sm font-medium transition-colors {currentPath === '/users' ? 'bg-primary text-primary-foreground' : 'hover:bg-muted'}"
        >
          <Icon icon="solar:user-bold" class="w-5 h-5" />
          <span>Users</span>
        </a>

        <a 
          href="/teams" 
          class="flex items-center space-x-3 px-3 py-2 rounded-lg text-sm font-medium transition-colors {currentPath === '/teams' ? 'bg-primary text-primary-foreground' : 'hover:bg-muted'}"
        >
          <Icon icon="solar:users-group-two-rounded-bold" class="w-5 h-5" />
          <span>Teams</span>
        </a>
      </nav>
    </div>

    <!-- Bottom actions -->
    <div class="absolute bottom-6 left-6 right-6 space-y-2">
      <a 
        href="/docs" 
        class="w-full flex items-center space-x-3 px-3 py-2 rounded-lg text-xs font-medium text-muted-foreground hover:text-foreground hover:bg-muted/50 transition-colors"
      >
        <Icon icon="solar:book-2-bold" class="w-4 h-4" />
        <span>Documentation</span>
      </a>
      
      <a 
        href="/settings" 
        class="w-full flex items-center space-x-3 px-3 py-2 rounded-lg text-sm font-medium transition-colors {currentPath === '/settings' ? 'bg-primary text-primary-foreground' : 'hover:bg-muted'}"
      >
        <Icon icon="solar:settings-bold" class="w-5 h-5" />
        <span>Settings</span>
      </a>
    </div>
  </aside>

  <!-- Main content -->
  <main class="flex-1 p-8">
    <slot />
  </main>
</div>