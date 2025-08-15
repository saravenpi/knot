<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { authStore } from '../lib/stores';

	$: user = $authStore.user;
	$: isLoggedIn = $authStore.isAuthenticated;
	$: initialized = $authStore.initialized;
	$: loading = $authStore.loading;
	$: currentPath = $page.url.pathname;

	// Pages that should use public layout even when authenticated
	$: isPublicPage = currentPath === '/docs' || currentPath === '/login' || currentPath === '/register' || currentPath === '/';

	onMount(async () => {
		await authStore.initialize();

		// Set up periodic token validation (every 5 minutes)
		const interval = setInterval(async () => {
			if ($authStore.isAuthenticated && !$authStore.loading) {
				try {
					await authStore.refresh();
				} catch (error) {
					console.warn('Token validation failed:', error);
					// Don't show error to user, just silently log out
				}
			}
		}, 5 * 60 * 1000); // 5 minutes

		// Cleanup interval on unmount
		return () => clearInterval(interval);
	});
</script>

<div class="min-h-screen bg-background">
	{#if !initialized}
		<!-- Loading screen during authentication initialization -->
		<div class="flex items-center justify-center min-h-screen">
			<div class="text-center">
				<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary mx-auto mb-4"></div>
				<p class="text-muted-foreground">Loading...</p>
			</div>
		</div>
	{:else}
		{#if isLoggedIn && !isPublicPage}
			<!-- Authenticated users get sidebar layout (except for public pages) -->
			<slot />
		{:else}
			<!-- Public layout for unauthenticated users OR public pages -->
			<nav class="border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
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
							<a href="/docs" class="text-sm font-medium hover:text-primary transition-colors">
								Docs
							</a>
							{#if isLoggedIn}
								<!-- Authenticated user options -->
								<a href="/packages" class="text-sm font-medium hover:text-primary transition-colors">
									Dashboard
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
			<main class="container mx-auto px-4 sm:px-6 lg:px-8 py-8">
				<slot />
			</main>

			<!-- Footer -->
			<footer class="border-t mt-auto">
				<div class="container mx-auto px-4 sm:px-6 lg:px-8 py-8">
					<div class="text-center text-muted-foreground">
						<p>&copy; 2025 Knot Space. Built with SvelteKit and ❤️</p>
					</div>
				</div>
			</footer>
		{/if}
	{/if}
</div>
