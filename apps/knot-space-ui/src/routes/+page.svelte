<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { packagesStore, authStore } from '../lib/stores';
	import Icon from '@iconify/svelte';

	$: packages = $packagesStore.packages;
	$: loading = $packagesStore.loading;
	$: isAuthenticated = $authStore.isAuthenticated;
	$: initialized = $authStore.initialized;
	$: stats = {
		totalPackages: packages.length,
		totalDownloads: packages.reduce((sum, pkg) => sum + (pkg.downloads_count || 0), 0),
		totalUsers: 0
	};

	onMount(async () => {
		// Initialize auth first to check if user is logged in
		await authStore.initialize();
		
		// If user is authenticated, redirect to packages
		if ($authStore.isAuthenticated) {
			goto('/packages');
			return;
		}

		// Load packages for public home page
		try {
			await packagesStore.fetchAll();
		} catch (error) {
			console.error('Failed to fetch packages:', error);
		}
	});
</script>

<svelte:head>
	<title>Knot Space - Package Registry for TypeScript/JavaScript</title>
	<meta name="description" content="A modern package registry for Knot monorepo packages" />
</svelte:head>

{#if !initialized}
	<!-- Loading state while checking authentication -->
	<div class="flex items-center justify-center min-h-[50vh]">
		<div class="text-center">
			<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-black mx-auto mb-4"></div>
			<p class="text-muted-foreground">Loading...</p>
		</div>
	</div>
{:else if !isAuthenticated}
	<!-- Show home page only for unauthenticated users -->
	
<!-- Hero Section -->
<div class="text-center py-12 md:py-20">
	<h1 class="text-4xl md:text-6xl font-bold tracking-tight mb-6" style="font-family: 'Gambarino', serif;">
		Welcome to <span class="text-black">Knot Space</span>
	</h1>
	<p class="text-xl text-muted-foreground max-w-3xl mx-auto mb-8">
		The modern package registry for Knot monorepo packages. Publish, share, and manage your TypeScript/JavaScript packages with teams.
	</p>
	<div class="flex flex-col sm:flex-row gap-4 justify-center items-center">
		<a 
			href="/packages" 
			class="bg-black text-white hover:bg-black/90 px-8 py-3 rounded-lg font-medium transition-colors flex items-center space-x-2"
		>
			<Icon icon="solar:box-bold" class="w-5 h-5" />
			<span>Browse Packages</span>
		</a>
		<a 
			href="/register" 
			class="border border-border hover:bg-accent hover:text-accent-foreground px-8 py-3 rounded-lg font-medium transition-colors flex items-center space-x-2"
		>
			<Icon icon="solar:rocket-2-bold" class="w-5 h-5" />
			<span>Get Started</span>
		</a>
	</div>
</div>

<!-- Stats -->
<div class="grid grid-cols-1 md:grid-cols-3 gap-6 py-12 border-t border-b">
	<div class="text-center">
		<div class="text-3xl font-bold text-black mb-2">{stats.totalPackages.toLocaleString()}</div>
		<div class="text-muted-foreground">Total Packages</div>
	</div>
	<div class="text-center">
		<div class="text-3xl font-bold text-black mb-2">{stats.totalDownloads.toLocaleString()}</div>
		<div class="text-muted-foreground">Total Downloads</div>
	</div>
	<div class="text-center">
		<div class="text-3xl font-bold text-black mb-2">∞</div>
		<div class="text-muted-foreground">Possibilities</div>
	</div>
</div>

<!-- Recent Packages -->
<div class="py-12">
	<div class="flex justify-between items-center mb-8">
		<h2 class="text-3xl font-bold">Recent Packages</h2>
		<a 
			href="/packages" 
			class="text-black hover:text-black/80 font-medium"
		>
			View All →
		</a>
	</div>

	{#if loading}
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
			{#each Array(6) as _}
				<div class="border rounded-lg p-6 animate-pulse">
					<div class="h-4 bg-muted rounded w-3/4 mb-2"></div>
					<div class="h-3 bg-muted rounded w-1/2 mb-4"></div>
					<div class="h-3 bg-muted rounded w-full"></div>
				</div>
			{/each}
		</div>
	{:else if packages.length === 0}
		<div class="text-center py-12">
			<div class="mb-4">
				<Icon icon="solar:box-bold" class="w-16 h-16 text-muted-foreground mx-auto" />
			</div>
			<h3 class="text-xl font-semibold mb-2">No packages yet</h3>
			<p class="text-muted-foreground mb-4">Be the first to publish a package to Knot Space!</p>
			<a 
				href="/login" 
				class="bg-black text-white hover:bg-black/90 px-6 py-2 rounded-md font-medium transition-colors inline-flex items-center space-x-2"
			>
				<Icon icon="solar:rocket-2-bold" class="w-4 h-4" />
				<span>Get Started</span>
			</a>
		</div>
	{:else}
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
			{#each packages.slice(0, 6) as pkg}
				<div class="border rounded-lg p-6 hover:shadow-md transition-shadow">
					<div class="flex items-start justify-between mb-3">
						<div>
							<h3 class="font-semibold text-lg leading-none mb-1">
								<a href="/packages/{pkg.name}" class="hover:text-black transition-colors">
									{pkg.name}
								</a>
							</h3>
							<p class="text-sm text-muted-foreground">v{pkg.version}</p>
						</div>
						{#if pkg.team}
							<span class="bg-secondary text-secondary-foreground px-2 py-1 rounded text-xs">
								{pkg.team.name}
							</span>
						{/if}
					</div>
					
					{#if pkg.description}
						<p class="text-muted-foreground text-sm mb-4 line-clamp-2">
							{pkg.description}
						</p>
					{/if}
					
					<div class="flex items-center justify-between text-xs text-muted-foreground">
						<span>by {pkg.owner.username}</span>
						<span>{pkg.downloads_count || 0} downloads</span>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>

<!-- Features -->
<div class="py-12 border-t">
	<h2 class="text-3xl font-bold text-center mb-12">Why Knot Space?</h2>
	<div class="grid grid-cols-1 md:grid-cols-3 gap-8">
		<div class="text-center">
			<div class="w-12 h-12 bg-black/10 rounded-lg flex items-center justify-center mx-auto mb-4">
				<Icon icon="solar:rocket-2-bold" class="w-6 h-6 text-black" />
			</div>
			<h3 class="font-semibold text-lg mb-2">Fast Publishing</h3>
			<p class="text-muted-foreground">
				Publish packages instantly with the Knot CLI. Simple, secure, and integrated with your workflow.
			</p>
		</div>
		<div class="text-center">
			<div class="w-12 h-12 bg-black/10 rounded-lg flex items-center justify-center mx-auto mb-4">
				<Icon icon="solar:users-group-two-rounded-bold" class="w-6 h-6 text-black" />
			</div>
			<h3 class="font-semibold text-lg mb-2">Team Collaboration</h3>
			<p class="text-muted-foreground">
				Create teams, manage permissions, and collaborate on packages with role-based access control.
			</p>
		</div>
		<div class="text-center">
			<div class="w-12 h-12 bg-black/10 rounded-lg flex items-center justify-center mx-auto mb-4">
				<Icon icon="solar:shield-check-bold" class="w-6 h-6 text-black" />
			</div>
			<h3 class="font-semibold text-lg mb-2">Secure & Reliable</h3>
			<p class="text-muted-foreground">
				Built with security first. JWT authentication, checksums, and secure file storage.
			</p>
		</div>
	</div>
</div>

{/if}