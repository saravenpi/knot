<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { authStore } from '../../lib/stores';

	$: user = $authStore.user;
	$: isLoggedIn = $authStore.isAuthenticated;

	let token = '';
	let showToken = false;
	let copied = false;

	onMount(() => {
		// Redirect if not authenticated
		if (!$authStore.isAuthenticated) {
			goto('/login');
			return;
		}
		
		// Get token from localStorage
		token = localStorage.getItem('knot_token') || '';
	});

	async function logout() {
		await authStore.logout();
		goto('/');
	}

	async function copyToken() {
		try {
			await navigator.clipboard.writeText(token);
			copied = true;
			setTimeout(() => {
				copied = false;
			}, 2000);
		} catch (err) {
			console.error('Failed to copy token:', err);
		}
	}

	function toggleTokenVisibility() {
		showToken = !showToken;
	}

	function regenerateToken() {
		// This would typically call an API to regenerate the token
		alert('Token regeneration not implemented yet. Please log out and log in again to get a new token.');
	}
</script>

<svelte:head>
	<title>Settings - Knot Space</title>
</svelte:head>

{#if !isLoggedIn}
	<div class="text-center">
		<p class="text-muted-foreground">Please log in to access settings.</p>
	</div>
{:else}
	<div class="max-w-4xl mx-auto">
		<div class="mb-8">
			<h1 class="text-3xl font-bold tracking-tight">Settings</h1>
			<p class="text-muted-foreground mt-2">Manage your account and CLI authentication</p>
		</div>

		<div class="space-y-6">
			<!-- Profile Section -->
			<div class="bg-card text-card-foreground rounded-lg border p-6">
				<h2 class="text-xl font-semibold mb-4">Profile Information</h2>
				<div class="space-y-3">
					<div class="flex justify-between items-center py-2">
						<span class="font-medium">Username:</span>
						<span class="text-muted-foreground">{user?.username || 'N/A'}</span>
					</div>
					<div class="flex justify-between items-center py-2">
						<span class="font-medium">Email:</span>
						<span class="text-muted-foreground">{user?.email || 'N/A'}</span>
					</div>
					<div class="flex justify-between items-center py-2">
						<span class="font-medium">Member since:</span>
						<span class="text-muted-foreground">
							{user?.created_at ? new Date(user.created_at).toLocaleDateString() : 'N/A'}
						</span>
					</div>
				</div>
			</div>

			<!-- CLI Authentication Section -->
			<div class="bg-card text-card-foreground rounded-lg border p-6">
				<h2 class="text-xl font-semibold mb-4">CLI Authentication</h2>
				<p class="text-muted-foreground mb-6">
					Use this token to authenticate with the Knot CLI for publishing packages and managing your account.
				</p>
				
				<div class="space-y-4">
					<div>
						<label for="token" class="block text-sm font-medium mb-2">Authentication Token</label>
						<div class="flex space-x-2">
							<input
								id="token"
								type={showToken ? 'text' : 'password'}
								value={token}
								readonly
								class="flex-1 px-3 py-2 border border-input bg-background rounded-md font-mono text-sm"
								placeholder="Token will appear here after login"
							/>
							<button
								on:click={toggleTokenVisibility}
								class="px-3 py-2 border border-input bg-background hover:bg-accent hover:text-accent-foreground rounded-md text-sm"
								title={showToken ? 'Hide token' : 'Show token'}
							>
								{#if showToken}
									<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.878 9.878L8.464 8.464M9.878 9.878l1.414-1.414" />
									</svg>
								{:else}
									<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.543 7-1.275 4.057-5.065 7-9.543 7-4.477 0-8.268-2.943-9.542-7z" />
									</svg>
								{/if}
							</button>
							<button
								on:click={copyToken}
								class="px-3 py-2 bg-primary text-primary-foreground hover:bg-primary/90 rounded-md text-sm"
								disabled={!token}
							>
								{#if copied}
									<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
									</svg>
								{:else}
									<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
									</svg>
								{/if}
							</button>
						</div>
					</div>

					<!-- CLI Setup Instructions -->
					<div class="bg-muted/50 rounded-md p-4">
						<h3 class="font-medium mb-2">CLI Setup Instructions</h3>
						<div class="space-y-2 text-sm text-muted-foreground">
							<p><strong>1. Install the Knot CLI:</strong></p>
							<code class="block bg-background px-3 py-1 rounded font-mono text-xs">
								curl -fsSL https://raw.githubusercontent.com/saravenpi/knot/main/install.sh | bash
							</code>
							
							<p class="pt-2"><strong>2. Set your authentication token:</strong></p>
							<code class="block bg-background px-3 py-1 rounded font-mono text-xs">
								export KNOT_TOKEN={token || 'your-token-here'}
							</code>
							
							<p class="pt-2"><strong>3. Publish a package:</strong></p>
							<code class="block bg-background px-3 py-1 rounded font-mono text-xs">
								knot publish --description "My awesome package"
							</code>
						</div>
					</div>

					<div class="flex space-x-2 pt-2">
						<button
							on:click={regenerateToken}
							class="px-4 py-2 border border-input bg-background hover:bg-accent hover:text-accent-foreground rounded-md text-sm"
						>
							Regenerate Token
						</button>
					</div>
				</div>
			</div>

			<!-- Danger Zone -->
			<div class="bg-card text-card-foreground rounded-lg border border-destructive/20 p-6">
				<h2 class="text-xl font-semibold mb-4 text-destructive">Danger Zone</h2>
				<div class="space-y-4">
					<div class="flex justify-between items-center">
						<div>
							<h3 class="font-medium">Sign out</h3>
							<p class="text-sm text-muted-foreground">Sign out from your account on this device</p>
						</div>
						<button
							on:click={logout}
							class="px-4 py-2 bg-destructive text-destructive-foreground hover:bg-destructive/90 rounded-md text-sm"
						>
							Sign Out
						</button>
					</div>
				</div>
			</div>
		</div>
	</div>
{/if}

<style>
	/* Custom styles for better token visibility */
	input[type="password"] {
		font-family: monospace;
		letter-spacing: 0.1em;
	}
</style>