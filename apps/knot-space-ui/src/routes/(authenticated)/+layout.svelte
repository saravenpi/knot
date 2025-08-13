<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { authStore } from '../../lib/stores';

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
  <aside class="w-64 bg-card border-r border-border flex-shrink-0">
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
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
          </svg>
          <span>Packages</span>
        </a>

        <a 
          href="/teams" 
          class="flex items-center space-x-3 px-3 py-2 rounded-lg text-sm font-medium transition-colors {currentPath === '/teams' ? 'bg-primary text-primary-foreground' : 'hover:bg-muted'}"
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
          </svg>
          <span>Teams</span>
        </a>

        <a 
          href="/settings" 
          class="flex items-center space-x-3 px-3 py-2 rounded-lg text-sm font-medium transition-colors {currentPath === '/settings' ? 'bg-primary text-primary-foreground' : 'hover:bg-muted'}"
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
          </svg>
          <span>Settings</span>
        </a>
      </nav>
    </div>

    <!-- Logout button at bottom -->
    <div class="absolute bottom-6 left-6 right-6">
      <button
        on:click={logout}
        class="w-full flex items-center space-x-3 px-3 py-2 rounded-lg text-sm font-medium text-destructive hover:bg-destructive/10 transition-colors"
      >
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" />
        </svg>
        <span>Sign Out</span>
      </button>
    </div>
  </aside>

  <!-- Main content -->
  <main class="flex-1 p-8">
    <slot />
  </main>
</div>