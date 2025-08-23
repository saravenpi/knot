<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { authStore } from '$lib/stores';
	import Icon from '@iconify/svelte';

	$: user = $authStore.user;
	$: isLoggedIn = $authStore.isAuthenticated;
	$: initialized = $authStore.initialized;
	$: loading = $authStore.loading;
	$: currentPath = $page.url.pathname;

	// Pages that should use public layout even when authenticated (docs has its own layout)
	$: isPublicPage = currentPath === '/login' || currentPath === '/register' || currentPath === '/';
	$: isDocsPage = currentPath.startsWith('/docs');
	$: isPackagesPage = currentPath.startsWith('/packages');

	onMount(async () => {
		await authStore.initialize();


		// Set up periodic token validation (every 15 minutes)
		const interval = setInterval(async () => {
			if ($authStore.isAuthenticated && !$authStore.loading) {
				try {
					await authStore.refresh();
				} catch (error) {
					console.warn('Token validation failed:', error);
					// Don't show error to user, just silently log out
				}
			}
		}, 15 * 60 * 1000); // 15 minutes

		// Cleanup interval on unmount
		return () => clearInterval(interval);
	});
</script>

<div class="min-h-screen bg-background flex flex-col">
	{#if !initialized}
		<!-- Loading screen during authentication initialization -->
		<div class="flex items-center justify-center min-h-screen">
			<div class="text-center">
				<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary mx-auto mb-4"></div>
				<p class="text-muted-foreground">Loading...</p>
			</div>
		</div>
	{:else}
		{#if isDocsPage}
			<!-- Docs pages get their own layout -->
			<slot />
		{:else if isLoggedIn && (!isPublicPage || isPackagesPage)}
			<div class="min-h-screen bg-background">
  <!-- Desktop Sidebar -->
  <aside class="hidden lg:block fixed left-0 top-0 w-64 h-screen bg-card border-r border-border overflow-y-auto z-10">
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

  <!-- Mobile Floating Docs Button -->
  <a 
    href="/docs"
    class="lg:hidden fixed bottom-20 right-4 z-40 bg-primary text-primary-foreground p-3 rounded-full shadow-lg hover:bg-primary/90 transition-all duration-200 {currentPath.startsWith('/docs') ? 'ring-2 ring-primary ring-offset-2' : ''}"
    title="Documentation"
  >
    <Icon icon="solar:book-2-bold" class="w-5 h-5" />
  </a>

  <!-- Main content -->
  <main class="lg:ml-64 p-4 lg:p-8 pb-20 lg:pb-8 pt-16 lg:pt-8">
    <slot />
  </main>

  <!-- Mobile Bottom Navigation -->
  <nav class="lg:hidden fixed bottom-0 left-0 right-0 bg-card border-t border-border z-50">
    <div class="flex items-center justify-around py-2">
      <a 
        href="/packages" 
        class="flex flex-col items-center py-2 px-4 min-w-0 flex-1 text-center transition-colors {currentPath === '/packages' ? 'text-primary' : 'text-muted-foreground hover:text-foreground'}"
      >
        <Icon icon="solar:box-bold" class="w-6 h-6 mb-1" />
        <span class="text-xs font-medium truncate">Packages</span>
      </a>

      <a 
        href="/users" 
        class="flex flex-col items-center py-2 px-4 min-w-0 flex-1 text-center transition-colors {currentPath === '/users' ? 'text-primary' : 'text-muted-foreground hover:text-foreground'}"
      >
        <Icon icon="solar:user-bold" class="w-6 h-6 mb-1" />
        <span class="text-xs font-medium truncate">Users</span>
      </a>

      <a 
        href="/teams" 
        class="flex flex-col items-center py-2 px-4 min-w-0 flex-1 text-center transition-colors {currentPath === '/teams' ? 'text-primary' : 'text-muted-foreground hover:text-foreground'}"
      >
        <Icon icon="solar:users-group-two-rounded-bold" class="w-6 h-6 mb-1" />
        <span class="text-xs font-medium truncate">Teams</span>
      </a>

      <a 
        href="/settings" 
        class="flex flex-col items-center py-2 px-4 min-w-0 flex-1 text-center transition-colors {currentPath === '/settings' ? 'text-primary' : 'text-muted-foreground hover:text-foreground'}"
      >
        <Icon icon="solar:settings-bold" class="w-6 h-6 mb-1" />
        <span class="text-xs font-medium truncate">Settings</span>
      </a>
    </div>
  </nav>
</div>
		{:else}
			<!-- Public layout for unauthenticated users OR public pages -->
			<nav class="border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60 flex-shrink-0">
				<div class="container mx-auto px-4 sm:px-6 lg:px-8">
					<div class="flex justify-between h-16">
						<div class="flex items-center">
							<a href="/" class="flex items-center space-x-2">
								<div class="w-8 h-8 bg-black rounded-md flex items-center justify-center">
									<span class="text-white font-bold text-sm">K</span>
								</div>
								<span class="font-semibold text-xl">Knot Space</span>
							</a>
						</div>

						<div class="flex items-center space-x-4">
							<a href="/packages" class="text-sm font-medium hover:text-primary transition-colors">
								Browse Packages
							</a>
							<a href="/docs" class="text-sm font-medium hover:text-primary transition-colors">
								Docs
							</a>
							{#if isLoggedIn}
								<!-- Authenticated user options on public pages -->
								{#if currentPath !== '/'}
									<a href="/packages" class="text-sm font-medium hover:text-primary transition-colors">
										Packages
									</a>
									<button
										on:click={async () => {
											await authStore.logout();
											window.location.href = '/';
										}}
										class="text-sm font-medium hover:text-primary transition-colors"
									>
										Sign Out
									</button>
								{/if}
							{:else}
								<!-- Unauthenticated user options -->
								<a href="/login" class="text-sm font-medium hover:text-primary transition-colors">
									Login
								</a>
								<a
									href="/register"
									class="bg-primary text-primary-foreground hover:bg-primary/90 px-4 py-2 rounded-md text-sm font-medium transition-colors"
								>
									Sign Up
								</a>
							{/if}
						</div>
					</div>
				</div>
			</nav>

			<!-- Main content -->
			<main class="container mx-auto px-4 sm:px-6 lg:px-8 py-8 flex-grow">
				<slot />
			</main>

			<!-- Footer -->
			<footer class="bg-black text-white flex-shrink-0">
				<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
					<div class="grid grid-cols-1 md:grid-cols-4 gap-8">
						<!-- Brand Section -->
						<div class="space-y-4">
							<div class="flex items-center space-x-2">
								<div class="w-8 h-8 bg-white rounded-lg flex items-center justify-center">
									<span class="text-black font-bold text-sm">K</span>
								</div>
								<span class="text-xl font-bold">Knot Space</span>
							</div>
							<p class="text-gray-400 text-sm leading-relaxed">
								The modern package registry for Knot monorepo packages. Build, publish, and manage your TypeScript/JavaScript packages with ease.
							</p>
							<div class="flex space-x-4">
								<a href="https://github.com/saravenpi/knot" class="text-gray-400 hover:text-white transition-colors" aria-label="GitHub">
									<svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
										<path d="M12 0C5.374 0 0 5.373 0 12 0 17.302 3.438 21.8 8.207 23.387c.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23A11.509 11.509 0 0112 5.803c1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576C20.566 21.797 24 17.3 24 12c0-6.627-5.373-12-12-12z"/>
									</svg>
								</a>
								<a href="https://twitter.com/knotspace" class="text-gray-400 hover:text-white transition-colors" aria-label="Twitter">
									<svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
										<path d="M23.953 4.57a10 10 0 01-2.825.775 4.958 4.958 0 002.163-2.723c-.951.555-2.005.959-3.127 1.184a4.92 4.92 0 00-8.384 4.482C7.69 8.095 4.067 6.13 1.64 3.162a4.822 4.822 0 00-.666 2.475c0 1.71.87 3.213 2.188 4.096a4.904 4.904 0 01-2.228-.616v.06a4.923 4.923 0 003.946 4.827 4.996 4.996 0 01-2.212.085 4.936 4.936 0 004.604 3.417 9.867 9.867 0 01-6.102 2.105c-.39 0-.779-.023-1.17-.067a13.995 13.995 0 007.557 2.209c9.053 0 13.998-7.496 13.998-13.985 0-.21 0-.42-.015-.63A9.935 9.935 0 0024 4.59z"/>
									</svg>
								</a>
							</div>
						</div>

						<!-- Product -->
						<div class="space-y-4">
							<h3 class="text-lg font-semibold">Product</h3>
							<div class="space-y-3">
								<a href="/packages" class="block text-gray-400 hover:text-white transition-colors text-sm">Browse Packages</a>
								<a href="/teams" class="block text-gray-400 hover:text-white transition-colors text-sm">Teams</a>
								<a href="/docs" class="block text-gray-400 hover:text-white transition-colors text-sm">Documentation</a>
								<a href="/docs/cli-commands" class="block text-gray-400 hover:text-white transition-colors text-sm">CLI Reference</a>
							</div>
						</div>

						<!-- Resources -->
						<div class="space-y-4">
							<h3 class="text-lg font-semibold">Resources</h3>
							<div class="space-y-3">
								<a href="/docs/quick-start" class="block text-gray-400 hover:text-white transition-colors text-sm">Quick Start</a>
								<a href="/docs/installation" class="block text-gray-400 hover:text-white transition-colors text-sm">Installation</a>
								<a href="/docs/publishing" class="block text-gray-400 hover:text-white transition-colors text-sm">Publishing Guide</a>
								<a href="/docs/troubleshooting" class="block text-gray-400 hover:text-white transition-colors text-sm">Troubleshooting</a>
							</div>
						</div>

						<!-- Legal -->
						<div class="space-y-4">
							<h3 class="text-lg font-semibold">Legal</h3>
							<div class="space-y-3">
								<a href="/legal/privacy" class="block text-gray-400 hover:text-white transition-colors text-sm">Privacy Policy</a>
								<a href="/legal/terms" class="block text-gray-400 hover:text-white transition-colors text-sm">Terms of Service</a>
								<a href="/legal/cookies" class="block text-gray-400 hover:text-white transition-colors text-sm">Cookie Policy</a>
								<a href="/legal/security" class="block text-gray-400 hover:text-white transition-colors text-sm">Security</a>
							</div>
						</div>
					</div>

					<!-- Bottom Bar -->
					<div class="border-t border-gray-800 mt-12 pt-8 flex flex-col md:flex-row justify-between items-center space-y-4 md:space-y-0">
						<div class="text-gray-400 text-sm">
							© 2025 Knot Space. All rights reserved.
						</div>
						<div class="flex items-center space-x-6 text-sm">
							<a href="/legal/privacy" class="text-gray-400 hover:text-white transition-colors">Privacy</a>
							<a href="/legal/terms" class="text-gray-400 hover:text-white transition-colors">Terms</a>
							<a href="https://github.com/saravenpi/knot" class="text-gray-400 hover:text-white transition-colors">GitHub</a>
						</div>
					</div>
				</div>
			</footer>
		{/if}
	{/if}
</div>
