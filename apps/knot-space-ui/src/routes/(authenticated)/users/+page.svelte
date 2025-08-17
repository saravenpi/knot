<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { usersApi, type User } from '../../../lib/api';
	import { formatDate } from '../../../lib/utils/format';
	import Icon from '@iconify/svelte';

	let allUsers: User[] = [];
	let filteredUsers: User[] = [];
	let loading = true;
	let error = '';
	let searchTerm = '';

	onMount(async () => {
		await loadUsers();
	});

	async function loadUsers() {
		loading = true;
		error = '';
		
		try {
			allUsers = await usersApi.getAllUsers();
			filteredUsers = allUsers;
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load users';
			console.error('Failed to load users:', err);
		} finally {
			loading = false;
		}
	}

	// Filter users based on search term
	$: {
		if (searchTerm.trim() === '') {
			filteredUsers = allUsers;
		} else {
			const term = searchTerm.toLowerCase();
			filteredUsers = allUsers.filter(user => 
				user.username.toLowerCase().includes(term)
			);
		}
	}

	function navigateToUser(username: string) {
		goto(`/users/${encodeURIComponent(username)}`);
	}
</script>

<svelte:head>
	<title>Users - Knot Space</title>
	<meta name="description" content="Browse all users in the Knot Space community" />
</svelte:head>

<div class="space-y-6">
	<div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
		<div>
			<h1 class="text-3xl font-bold" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">Users</h1>
			<p class="text-muted-foreground">Discover and explore users in the community</p>
		</div>
		
		<!-- Search -->
		<div class="relative max-w-sm">
			<svg class="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
				<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
			</svg>
			<input
				type="text"
				placeholder="Search users..."
				bind:value={searchTerm}
				class="pl-10 pr-4 py-2 w-full border border-input rounded-md bg-background focus:outline-none focus:ring-2 focus:ring-ring focus:border-transparent"
			/>
		</div>
	</div>

	{#if loading}
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
			{#each Array(9) as _}
				<div class="border rounded-lg p-6 animate-pulse">
					<div class="flex items-center gap-4 mb-4">
						<div class="w-12 h-12 bg-muted rounded-lg"></div>
						<div class="flex-1">
							<div class="h-4 bg-muted rounded w-2/3 mb-2"></div>
							<div class="h-3 bg-muted rounded w-1/2"></div>
						</div>
					</div>
					<div class="space-y-2">
						<div class="h-3 bg-muted rounded w-1/3"></div>
						<div class="h-3 bg-muted rounded w-1/4"></div>
					</div>
				</div>
			{/each}
		</div>
	{:else if error}
		<div class="text-center py-12">
			<div class="text-6xl mb-4">❌</div>
			<h3 class="text-xl font-semibold mb-2">Failed to load users</h3>
			<p class="text-muted-foreground mb-6">{error}</p>
			<button 
				on:click={loadUsers}
				class="bg-primary text-primary-foreground hover:bg-primary/90 px-6 py-2 rounded-md font-medium transition-colors"
			>
				Try Again
			</button>
		</div>
	{:else if filteredUsers.length === 0}
		<div class="text-center py-12">
			<div class="text-6xl mb-4">
				{searchTerm ? '🔍' : '👥'}
			</div>
			<h3 class="text-xl font-semibold mb-2">
				{searchTerm ? 'No users found' : 'No users yet'}
			</h3>
			<p class="text-muted-foreground mb-4">
				{searchTerm 
					? `No users match "${searchTerm}". Try a different search term.`
					: 'Be the first to join the Knot Space community!'
				}
			</p>
			{#if searchTerm}
				<button 
					on:click={() => searchTerm = ''}
					class="bg-primary text-primary-foreground hover:bg-primary/90 px-6 py-2 rounded-md font-medium transition-colors"
				>
					Clear Search
				</button>
			{/if}
		</div>
	{:else}
		<div class="space-y-4">
			<!-- Results count -->
			<div class="text-sm text-muted-foreground">
				{#if searchTerm}
					{filteredUsers.length} user{filteredUsers.length === 1 ? '' : 's'} found for "{searchTerm}"
				{:else}
					{filteredUsers.length} user{filteredUsers.length === 1 ? '' : 's'} total
				{/if}
			</div>

			<!-- Users grid -->
			<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
				{#each filteredUsers as user}
					<button 
						on:click={() => navigateToUser(user.username)}
						class="border rounded-lg p-6 hover:shadow-md transition-all duration-200 hover:border-primary/50 cursor-pointer text-left"
					>
						<div class="flex items-center gap-4 mb-4">
							<!-- Profile avatar matching the profile page design -->
							<div class="w-12 h-12 bg-gradient-to-br from-primary to-primary/80 rounded-xl flex items-center justify-center text-primary-foreground text-lg font-bold">
								{user.username.charAt(0).toUpperCase()}
							</div>
							<div class="flex-1 min-w-0">
								<h3 class="font-semibold text-lg truncate hover:text-primary transition-colors">
									{user.username}
								</h3>
								<p class="text-sm text-muted-foreground">
									Joined {formatDate(user.createdAt)}
								</p>
							</div>
						</div>
						
						<div class="space-y-1 text-sm text-muted-foreground">
							<div class="flex items-center justify-between">
								<span class="flex items-center gap-1">
									<Icon icon="solar:box-bold" class="w-4 h-4" />
									Packages
								</span>
								<span class="font-medium">{user._count?.ownedPackages || 0}</span>
							</div>
						</div>
					</button>
				{/each}
			</div>
		</div>
	{/if}
</div>