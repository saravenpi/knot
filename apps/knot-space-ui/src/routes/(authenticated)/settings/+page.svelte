<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';
	import { authStore } from '../../../lib/stores';
	import Icon from '@iconify/svelte';
	import Drawer from '$lib/components/ui/drawer.svelte';

	$: user = $authStore.user;
	$: loading = $authStore.loading;
	$: initialized = $authStore.initialized;

	let token = '';
	let showToken = false;
	let copied = false;
	let pageLoading = true;
	let showDeleteConfirm = false;
	let deleteConfirmText = '';
	let isDeleting = false;
	let showEditProfile = false;
	let editForm = { username: '', email: '' };
	let editError = '';
	let isUpdating = false;

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

	function showDeleteDialog() {
		showDeleteConfirm = true;
		deleteConfirmText = '';
	}

	function cancelDelete() {
		showDeleteConfirm = false;
		deleteConfirmText = '';
		isDeleting = false;
	}

	async function deleteAccount() {
		if (deleteConfirmText.toLowerCase() !== 'delete my account') {
			return;
		}

		isDeleting = true;

		try {
			const response = await fetch('/api/auth/account', {
				method: 'DELETE',
				headers: {
					'Authorization': `Bearer ${token}`,
					'Content-Type': 'application/json'
				}
			});

			if (response.ok) {
				// Account deleted successfully
				await authStore.logout();
				goto('/', { replaceState: true });
			} else {
				const error = await response.text();
				alert(`Failed to delete account: ${error}`);
				isDeleting = false;
			}
		} catch (error) {
			console.error('Error deleting account:', error);
			alert('An error occurred while deleting your account. Please try again.');
			isDeleting = false;
		}
	}

	function showEditDialog() {
		editForm.username = user?.username || '';
		editForm.email = user?.email || '';
		editError = '';
		showEditProfile = true;
	}

	function cancelEdit() {
		showEditProfile = false;
		editForm = { username: '', email: '' };
		editError = '';
		isUpdating = false;
	}

	async function updateProfile() {
		if (!editForm.username.trim() || !editForm.email.trim()) {
			editError = 'Username and email are required';
			return;
		}

		isUpdating = true;
		editError = '';

		try {
			const response = await fetch('/api/auth/profile', {
				method: 'PUT',
				headers: {
					'Authorization': `Bearer ${token}`,
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					username: editForm.username.trim(),
					email: editForm.email.trim()
				})
			});

			const result = await response.json();

			if (response.ok && result.success) {
				// Update the auth store with new user data
				await authStore.updateUser(result.data.user);
				// Update token in localStorage if provided
				if (result.data.token) {
					localStorage.setItem('knot_token', result.data.token);
					token = result.data.token;
				}
				showEditProfile = false;
			} else {
				editError = result.error || 'Failed to update profile';
				isUpdating = false;
			}
		} catch (error) {
			console.error('Error updating profile:', error);
			editError = 'An error occurred while updating your profile. Please try again.';
			isUpdating = false;
		}
	}
</script>

<svelte:head>
	<title>Settings - Knot Space</title>
</svelte:head>

<div class="max-w-4xl mx-auto">
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">Settings</h1>
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
			<div class="flex justify-between items-center mb-4">
				<h2 class="text-xl font-semibold">Profile Information</h2>
				<button
					on:click={showEditDialog}
					class="px-4 py-2 bg-primary text-primary-foreground hover:bg-primary/90 rounded-md text-sm font-medium transition-colors flex items-center gap-2"
				>
					<Icon icon="solar:pen-bold" class="w-4 h-4" />
					Edit Profile
				</button>
			</div>
			<div class="space-y-3">
				<div class="flex justify-between items-center py-2">
					<span class="font-medium">Username</span>
					<span class="text-muted-foreground">{user?.username || 'N/A'}</span>
				</div>
				<div class="flex justify-between items-center py-2">
					<span class="font-medium">Email</span>
					<span class="text-muted-foreground">{user?.email || 'N/A'}</span>
				</div>
				<div class="flex justify-between items-center py-2">
					<span class="font-medium">Member since</span>
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
							class="flex-1 px-3 py-2 border border-input bg-background rounded-md font-mono text-sm overflow-hidden"
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
						<code class="block bg-background px-3 py-1 rounded font-mono text-xs overflow-x-auto whitespace-nowrap">
							curl -fsSL https://raw.githubusercontent.com/saravenpi/knot/main/install.sh | bash
						</code>
						
						<p class="pt-2"><strong>2. Set your authentication token:</strong></p>
						<code class="block bg-background px-3 py-1 rounded font-mono text-xs overflow-x-auto whitespace-nowrap">
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

				<div class="border-t border-destructive/20 pt-4">
					<div class="flex justify-between items-center">
						<div>
							<h3 class="font-medium text-destructive">Delete Account</h3>
							<p class="text-sm text-muted-foreground">Permanently delete your account and all associated data</p>
						</div>
						<button
							on:click={showDeleteDialog}
							class="px-4 py-2 bg-destructive text-destructive-foreground hover:bg-destructive/90 rounded-md text-sm flex items-center space-x-2"
						>
							<Icon icon="solar:trash-bin-trash-bold" class="w-4 h-4" />
							<span>Delete Account</span>
						</button>
					</div>
				</div>
			</div>
		</div>
	</div>
	{/if}
</div>

<!-- Delete Account Confirmation Drawer -->
<Drawer 
	bind:open={showDeleteConfirm} 
	title="Delete Account" 
	description="This action will permanently delete your account and all associated data"
	side="bottom"
>
	<div class="space-y-4">
		<div class="flex items-center space-x-3 mb-4">
			<Icon icon="solar:danger-bold" class="w-8 h-8 text-destructive" />
			<div>
				<h3 class="text-lg font-semibold text-destructive">Delete Account</h3>
				<p class="text-sm text-muted-foreground">This action cannot be undone!</p>
			</div>
		</div>
		
		<p class="text-sm text-muted-foreground mb-4">
			This action will permanently delete your account and all associated data including:
		</p>
		
		<ul class="text-sm text-muted-foreground mb-6 pl-4 space-y-1">
			<li>• All your published packages</li>
			<li>• Team memberships and owned teams</li>
			<li>• Download statistics</li>
			<li>• Account information</li>
		</ul>
		
		<div class="bg-destructive/10 border border-destructive/20 rounded-md p-3 mb-4">
			<p class="text-sm text-destructive font-medium mb-2">
				This action cannot be undone!
			</p>
			<p class="text-sm text-muted-foreground">
				To confirm, please type "<strong>delete my account</strong>" below:
			</p>
		</div>
		
		<input
			bind:value={deleteConfirmText}
			type="text"
			placeholder="Type 'delete my account' to confirm"
			class="w-full px-3 py-2 border border-input bg-background rounded-md text-sm mb-4"
			disabled={isDeleting}
		/>
		
		<div class="flex justify-end space-x-2">
			<button
				on:click={cancelDelete}
				class="px-4 py-2 border border-input bg-background hover:bg-accent hover:text-accent-foreground rounded-md text-sm"
				disabled={isDeleting}
			>
				Cancel
			</button>
			<button
				on:click={deleteAccount}
				class="px-4 py-2 bg-destructive text-destructive-foreground hover:bg-destructive/90 rounded-md text-sm flex items-center space-x-2"
				disabled={isDeleting || deleteConfirmText.toLowerCase() !== 'delete my account'}
			>
				{#if isDeleting}
					<Icon icon="solar:refresh-bold" class="animate-spin w-4 h-4" />
					<span>Deleting...</span>
				{:else}
					<Icon icon="solar:trash-bin-trash-bold" class="w-4 h-4" />
					<span>Delete Account</span>
				{/if}
			</button>
		</div>
	</div>
</Drawer>

<!-- Edit Profile Drawer -->
<Drawer 
	bind:open={showEditProfile} 
	title="Edit Profile" 
	description="Update your profile information"
	side="right"
>
	<form on:submit|preventDefault={updateProfile} class="space-y-4">
		<div>
			<label for="edit-username" class="block text-sm font-medium mb-2">Username</label>
			<input
				id="edit-username"
				bind:value={editForm.username}
				type="text"
				placeholder="Enter username"
				class="w-full px-3 py-2 border border-input bg-background rounded-md text-sm"
				disabled={isUpdating}
				required
			/>
		</div>

		<div>
			<label for="edit-email" class="block text-sm font-medium mb-2">Email</label>
			<input
				id="edit-email"
				bind:value={editForm.email}
				type="email"
				placeholder="Enter email"
				class="w-full px-3 py-2 border border-input bg-background rounded-md text-sm"
				disabled={isUpdating}
				required
			/>
		</div>

		{#if editError}
			<div class="bg-destructive/10 border border-destructive/20 rounded-md p-3">
				<p class="text-sm text-destructive">{editError}</p>
			</div>
		{/if}
		
		<div class="flex justify-end space-x-2 pt-4 mt-auto">
			<button
				type="button"
				on:click={cancelEdit}
				class="px-4 py-2 border border-input bg-background hover:bg-accent hover:text-accent-foreground rounded-md text-sm"
				disabled={isUpdating}
			>
				Cancel
			</button>
			<button
				type="submit"
				class="px-4 py-2 bg-primary text-primary-foreground hover:bg-primary/90 rounded-md text-sm flex items-center space-x-2"
				disabled={isUpdating}
			>
				{#if isUpdating}
					<Icon icon="solar:refresh-bold" class="animate-spin w-4 h-4" />
					<span>Updating...</span>
				{:else}
					<Icon icon="solar:check-circle-bold" class="w-4 h-4" />
					<span>Update Profile</span>
				{/if}
			</button>
		</div>
	</form>
</Drawer>

<style>
	/* Custom styles for better token visibility */
	input[type="password"] {
		font-family: monospace;
		letter-spacing: 0.1em;
	}
</style>