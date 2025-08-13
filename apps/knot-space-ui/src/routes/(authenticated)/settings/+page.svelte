<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';
	import { authStore } from '../../../lib/stores';
	import Icon from '@iconify/svelte';

	$: user = $authStore.user;
	$: loading = $authStore.loading;
	$: initialized = $authStore.initialized;

	let token = '';
	let showToken = false;
	let copied = false;
	let pageLoading = true;

	onMount(async () => {
		// Get the token from localStorage - no need to call getProfile() again
		// since we're already in an authenticated layout
		if (browser) {
			token = localStorage.getItem('knot_token') || '';
		}
		pageLoading = false;
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

<div class="max-w-4xl mx-auto">
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight">Settings</h1>
		<p class="text-muted-foreground mt-2">Manage your account and CLI authentication</p>
	</div>

	{#if pageLoading}
		<!-- Loading state -->
		<div class="flex items-center justify-center py-16">
			<div class="text-center">
				<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary mx-auto mb-4"></div>
				<p class="text-muted-foreground">Loading your settings...</p>
			</div>
		</div>
	{:else}
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
						{user?.createdAt ? new Date(user.createdAt).toLocaleDateString() : 'N/A'}
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
							aria-label={showToken ? 'Hide token' : 'Show token'}
						>
							<Icon icon={showToken ? "solar:eye-closed-bold" : "solar:eye-bold"} class="w-4 h-4" />
						</button>
						<button
							on:click={copyToken}
							class="px-3 py-2 bg-primary text-primary-foreground hover:bg-primary/90 rounded-md text-sm"
							disabled={!token}
							aria-label="Copy token to clipboard"
							title={copied ? 'Copied!' : 'Copy token'}
						>
							<Icon icon={copied ? "solar:check-circle-bold" : "solar:copy-bold"} class="w-4 h-4" />
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
						class="px-4 py-2 border border-input bg-background hover:bg-accent hover:text-accent-foreground rounded-md text-sm flex items-center space-x-2"
					>
						<Icon icon="solar:refresh-bold" class="w-4 h-4" />
						<span>Regenerate Token</span>
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
						class="px-4 py-2 bg-destructive text-destructive-foreground hover:bg-destructive/90 rounded-md text-sm flex items-center space-x-2"
					>
						<Icon icon="solar:logout-3-bold" class="w-4 h-4" />
						<span>Sign Out</span>
					</button>
				</div>
			</div>
		</div>
	</div>
	{/if}
</div>

<style>
	/* Custom styles for better token visibility */
	input[type="password"] {
		font-family: monospace;
		letter-spacing: 0.1em;
	}
</style>