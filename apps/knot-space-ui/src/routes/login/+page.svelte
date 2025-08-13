<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { authStore } from '../../lib/stores';

	let username = '';
	let password = '';
	let error = '';

	$: loading = $authStore.loading;

	onMount(async () => {
		// Wait for auth to initialize before checking authentication status
		await authStore.initialize();
		
		// Redirect if already logged in
		if ($authStore.isAuthenticated) {
			goto('/packages');
		}
	});

	async function handleLogin() {
		if (!username || !password) {
			error = 'Please fill in all fields';
			return;
		}

		error = '';

		try {
			await authStore.login(username.trim(), password);
			// Check for redirect URL in query params, otherwise go to packages
			const redirectTo = $page.url.searchParams.get('redirectTo') || '/packages';
			goto(redirectTo);
		} catch (err) {
			error = err instanceof Error ? err.message : 'Login failed';
		}
	}

	function handleKeyPress(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			handleLogin();
		}
	}
</script>

<svelte:head>
	<title>Login - Knot Space</title>
</svelte:head>

<div class="max-w-md mx-auto">
	<div class="text-center mb-8">
		<h1 class="text-3xl font-bold mb-2">Welcome Back</h1>
		<p class="text-muted-foreground">Sign in to your Knot Space account</p>
	</div>

	<div class="border rounded-lg p-6">
		{#if error}
			<div class="bg-destructive/10 text-destructive border border-destructive/20 rounded-md p-3 mb-4">
				{error}
			</div>
		{/if}

		<form on:submit|preventDefault={handleLogin}>
			<div class="space-y-4">
				<div>
					<label for="username" class="block text-sm font-medium mb-2">
						Username
					</label>
					<input
						id="username"
						type="text"
						bind:value={username}
						on:keypress={handleKeyPress}
						placeholder="Enter your username"
						class="w-full px-3 py-2 border border-input rounded-md bg-background focus:outline-none focus:ring-2 focus:ring-ring focus:border-transparent"
						disabled={loading}
						required
					/>
				</div>

				<div>
					<label for="password" class="block text-sm font-medium mb-2">
						Password
					</label>
					<input
						id="password"
						type="password"
						bind:value={password}
						on:keypress={handleKeyPress}
						placeholder="Enter your password"
						class="w-full px-3 py-2 border border-input rounded-md bg-background focus:outline-none focus:ring-2 focus:ring-ring focus:border-transparent"
						disabled={loading}
						required
					/>
				</div>

				<button
					type="submit"
					disabled={loading}
					class="w-full bg-primary text-primary-foreground hover:bg-primary/90 disabled:opacity-50 disabled:cursor-not-allowed px-4 py-2 rounded-md font-medium transition-colors"
				>
					{#if loading}
						<span class="flex items-center justify-center">
							<svg class="animate-spin -ml-1 mr-3 h-4 w-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
								<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
								<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
							</svg>
							Signing In...
						</span>
					{:else}
						Sign In
					{/if}
				</button>
			</div>
		</form>
	</div>

	<div class="text-center mt-6">
		<p class="text-muted-foreground">
			Don't have an account?
			<a href="/register" class="text-primary hover:text-primary/80 font-medium">
				Sign up
			</a>
		</p>
	</div>
</div>