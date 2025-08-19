<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { authStore } from '../../lib/stores';
	import Icon from '@iconify/svelte';
	import Input from '../../lib/components/ui/input.svelte';
	import Button from '../../lib/components/ui/button.svelte';

	let username = '';
	let email = '';
	let password = '';
	let confirmPassword = '';
	let error = '';

	$: loading = $authStore.loading;

	onMount(async () => {
		// Wait for auth to initialize before checking authentication status
		if (!$authStore.initialized) {
			await authStore.initialize();
		}
		
		// Redirect if already logged in
		if ($authStore.isAuthenticated) {
			goto('/packages');
		}
	});

	async function handleRegister() {
		// Validation
		if (!username || !email || !password || !confirmPassword) {
			error = 'Please fill in all fields';
			return;
		}

		if (password !== confirmPassword) {
			error = 'Passwords do not match';
			return;
		}

		if (password.length < 6) {
			error = 'Password must be at least 6 characters';
			return;
		}

		// Username validation
		if (!/^[a-zA-Z0-9][a-zA-Z0-9\-_]*$/.test(username)) {
			error = 'Username must start with alphanumeric character and contain only letters, numbers, hyphens, and underscores';
			return;
		}

		// Email validation
		if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email)) {
			error = 'Please enter a valid email address';
			return;
		}

		error = '';

		try {
			await authStore.register(username.trim(), email.trim().toLowerCase(), password);
			// Redirect to packages page after successful registration
			goto('/packages');
		} catch (err) {
			error = err instanceof Error ? err.message : 'Registration failed';
		}
	}

	function handleKeyPress(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			handleRegister();
		}
	}
</script>

<svelte:head>
	<title>Sign Up - Knot Space</title>
</svelte:head>

<div class="max-w-md mx-auto">
	<div class="text-center mb-8">
		<h1 class="text-3xl font-bold mb-2" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">Create Account</h1>
		<p class="text-muted-foreground">Join Knot Space and start publishing packages</p>
	</div>

	<div class="border rounded-lg p-6">
		{#if error}
			<div class="bg-destructive/10 text-destructive border border-destructive/20 rounded-md p-3 mb-4">
				{error}
			</div>
		{/if}

		<form on:submit|preventDefault={handleRegister}>
			<div class="space-y-4">
				<div>
					<label for="username" class="block text-sm font-medium mb-2">
						Username
					</label>
					<Input
						id="username"
						type="text"
						bind:value={username}
						on:keypress={handleKeyPress}
						placeholder="Choose a username"
						disabled={loading}
						required
					/>
					<p class="text-xs text-muted-foreground mt-1">
						Letters, numbers, hyphens, and underscores only
					</p>
				</div>

				<div>
					<label for="email" class="block text-sm font-medium mb-2">
						Email
					</label>
					<Input
						id="email"
						type="email"
						bind:value={email}
						on:keypress={handleKeyPress}
						placeholder="Enter your email"
						disabled={loading}
						required
					/>
				</div>

				<div>
					<label for="password" class="block text-sm font-medium mb-2">
						Password
					</label>
					<Input
						id="password"
						type="password"
						bind:value={password}
						on:keypress={handleKeyPress}
						placeholder="Create a password"
						disabled={loading}
						required
					/>
					<p class="text-xs text-muted-foreground mt-1">
						At least 6 characters
					</p>
				</div>

				<div>
					<label for="confirmPassword" class="block text-sm font-medium mb-2">
						Confirm Password
					</label>
					<Input
						id="confirmPassword"
						type="password"
						bind:value={confirmPassword}
						on:keypress={handleKeyPress}
						placeholder="Confirm your password"
						disabled={loading}
						required
					/>
				</div>

				<Button
					type="submit"
					disabled={loading}
					class="w-full"
				>
					{#if loading}
						<Icon icon="solar:refresh-bold" class="animate-spin mr-2 h-4 w-4" />
						Creating Account...
					{:else}
						<Icon icon="solar:user-plus-bold" class="mr-2 h-4 w-4" />
						Create Account
					{/if}
				</Button>
			</div>
		</form>
	</div>

	<div class="text-center mt-6">
		<p class="text-muted-foreground">
			Already have an account?
			<a href="/login" class="text-primary hover:text-primary/80 font-medium">
				Sign in
			</a>
		</p>
	</div>
</div>