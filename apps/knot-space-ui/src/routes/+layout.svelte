<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { authStore } from '../lib/stores';

	$: user = $authStore.user;
	$: isLoggedIn = $authStore.isAuthenticated;

	onMount(() => {
		authStore.initialize();
	});

	async function logout() {
		await authStore.logout();
		goto('/');
	}
</script>

<div class="min-h-screen bg-background">
	<!-- Navigation -->
	<nav class="border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
		<div class="container mx-auto px-4 sm:px-6 lg:px-8">
			<div class="flex justify-between h-16">
				<div class="flex items-center">
					<a href="/" class="flex items-center space-x-2">
						<div class="w-8 h-8 bg-primary rounded-md flex items-center justify-center">
							<span class="text-primary-foreground font-bold text-sm">🪢</span>
						</div>
						<span class="font-semibold text-xl">Knot Space</span>
					</a>
				</div>

				<div class="flex items-center space-x-4">
					<a href="/packages" class="text-sm font-medium hover:text-primary transition-colors">
						Packages
					</a>
					<a href="/teams" class="text-sm font-medium hover:text-primary transition-colors">
						Teams
					</a>

					{#if isLoggedIn && user}
						<div class="flex items-center space-x-2">
							<span class="text-sm text-muted-foreground">Welcome, {user.username}</span>
							<button
								on:click={logout}
								class="text-sm font-medium hover:text-primary transition-colors"
							>
								Logout
							</button>
						</div>
					{:else}
						<div class="flex items-center space-x-2">
							<a href="/login" class="text-sm font-medium hover:text-primary transition-colors">
								Login
							</a>
							<a
								href="/register"
								class="bg-primary text-primary-foreground hover:bg-primary/90 px-4 py-2 rounded-md text-sm font-medium transition-colors"
							>
								Sign Up
							</a>
						</div>
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
				<p>&copy; 2024 Knot Space. Built with SvelteKit and ❤️</p>
			</div>
		</div>
	</footer>
</div>
