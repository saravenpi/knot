<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { packagesStore, authStore } from '../lib/stores';
	import { packagesApi } from '../lib/api';
	import Icon from '@iconify/svelte';

	$: packages = $packagesStore.packages;
	$: loading = $packagesStore.loading;
	$: isAuthenticated = $authStore.isAuthenticated;
	$: initialized = $authStore.initialized;
	
	let stats = {
		totalPackages: 0,
		totalDownloads: 0,
		totalUsers: 0
	};
	let statsLoading = true;

	onMount(async () => {
		// Initialize auth first to check if user is logged in
		await authStore.initialize();
		
		// If user is authenticated, redirect to packages
		if ($authStore.isAuthenticated) {
			goto('/packages');
			return;
		}

		// Load packages for public home page and global stats in parallel
		try {
			const [packagesResult, statsResult] = await Promise.all([
				packagesStore.fetchAll(),
				packagesApi.getGlobalStats()
			]);
			
			stats = statsResult;
			statsLoading = false;
		} catch (error) {
			console.error('Failed to fetch data:', error);
			statsLoading = false;
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
	<h1 class="text-4xl md:text-6xl font-bold tracking-tight mb-6" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
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
		{#if statsLoading}
			<div class="text-3xl font-bold text-black mb-2">
				<div class="animate-pulse bg-muted rounded w-16 h-8 mx-auto"></div>
			</div>
		{:else}
			<div class="text-3xl font-bold text-black mb-2">{stats.totalPackages.toLocaleString()}</div>
		{/if}
		<div class="text-muted-foreground">Total Packages</div>
	</div>
	<div class="text-center">
		{#if statsLoading}
			<div class="text-3xl font-bold text-black mb-2">
				<div class="animate-pulse bg-muted rounded w-16 h-8 mx-auto"></div>
			</div>
		{:else}
			<div class="text-3xl font-bold text-black mb-2">{stats.totalDownloads.toLocaleString()}</div>
		{/if}
		<div class="text-muted-foreground">Total Downloads</div>
	</div>
	<div class="text-center">
		{#if statsLoading}
			<div class="text-3xl font-bold text-black mb-2">
				<div class="animate-pulse bg-muted rounded w-16 h-8 mx-auto"></div>
			</div>
		{:else}
			<div class="text-3xl font-bold text-black mb-2">{stats.totalUsers.toLocaleString()}</div>
		{/if}
		<div class="text-muted-foreground">Total Users</div>
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
						<span>{pkg.downloadsCount || 0} downloads</span>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>

<!-- Quick Start -->
<div class="py-12 border-t">
	<div class="max-w-2xl mx-auto text-center">
		<div class="bg-black/5 border border-border rounded-lg p-8">
			<h3 class="text-xl font-semibold mb-4">Ready to get started?</h3>
			<p class="text-muted-foreground mb-6">
				Install the Knot CLI and start building your monorepo today.
			</p>
			<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded mb-6 text-left overflow-x-auto relative">
				<code>curl -fsSL https://raw.githubusercontent.com/saravenpi/knot/main/install.sh | bash</code>
				<button 
					class="absolute top-2 right-2 p-1.5 rounded bg-white/10 hover:bg-white/20 transition-colors"
					onclick="navigator.clipboard.writeText('curl -fsSL https://raw.githubusercontent.com/saravenpi/knot/main/install.sh | bash')"
					title="Copy to clipboard"
				>
					<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"></path>
					</svg>
				</button>
			</div>
			<div class="flex flex-col sm:flex-row gap-3 justify-center">
				<a 
					href="/docs" 
					class="border border-border hover:bg-accent hover:text-accent-foreground px-6 py-3 rounded-lg font-medium transition-colors flex items-center justify-center space-x-2"
				>
					<Icon icon="solar:book-bold" class="w-4 h-4" />
					<span>Read the Docs</span>
				</a>
				<a 
					href="/register" 
					class="bg-black text-white hover:bg-black/90 px-6 py-3 rounded-lg font-medium transition-colors flex items-center justify-center space-x-2"
				>
					<Icon icon="solar:rocket-2-bold" class="w-4 h-4" />
					<span>Get Started</span>
				</a>
			</div>
		</div>
	</div>
</div>

<!-- How It Works -->
<div class="py-12 border-t">
	<h2 class="text-3xl font-bold text-center mb-12">How It Works</h2>
	
	<!-- Configuration Examples -->
	<div class="max-w-6xl mx-auto mb-16">
		<div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
			<!-- Project Setup -->
			<div class="space-y-4">
				<div class="flex items-center space-x-3 mb-4">
					<div class="w-8 h-8 bg-black/10 rounded-lg flex items-center justify-center">
						<Icon icon="solar:settings-bold" class="w-5 h-5 text-black" />
					</div>
					<h3 class="text-xl font-semibold">1. Initialize Your Project</h3>
				</div>
				<div class="bg-muted/30 rounded-lg p-4">
					<div class="text-sm text-muted-foreground mb-2">Terminal</div>
					<pre class="text-sm font-mono bg-black/90 text-green-400 p-3 rounded overflow-x-auto"><code>$ knot init MyProject
✅ Initialized new Knot project: MyProject
📁 Created directories: packages/, apps/
💡 Use 'knot init:package &lt;name&gt;' to create packages</code></pre>
				</div>
				<div class="bg-muted/30 rounded-lg p-4">
					<div class="text-sm text-muted-foreground mb-2">knot.yml</div>
					<pre class="text-sm font-mono bg-black/90 text-white p-3 rounded overflow-x-auto"><code><span class="text-blue-400">name:</span> <span class="text-green-400">MyProject</span>
<span class="text-blue-400">description:</span> <span class="text-green-400">A modern TypeScript monorepo</span>
<span class="text-blue-400">apps:</span>
  <span class="text-blue-400">frontend:</span>
    <span class="text-blue-400">tsAlias:</span> <span class="text-green-400">"#"</span>
    <span class="text-blue-400">packages:</span> <span class="text-yellow-400">[types, utils]</span>
<span class="text-blue-400">scripts:</span>
  <span class="text-blue-400">setup:</span> <span class="text-green-400">"npm install"</span>
  <span class="text-blue-400">test-all:</span> <span class="text-green-400">"knot run test --all"</span></code></pre>
				</div>
			</div>

			<!-- Package Creation -->
			<div class="space-y-4">
				<div class="flex items-center space-x-3 mb-4">
					<div class="w-8 h-8 bg-black/10 rounded-lg flex items-center justify-center">
						<Icon icon="solar:box-bold" class="w-5 h-5 text-black" />
					</div>
					<h3 class="text-xl font-semibold">2. Create Packages</h3>
				</div>
				<div class="bg-muted/30 rounded-lg p-4">
					<div class="text-sm text-muted-foreground mb-2">Terminal</div>
					<pre class="text-sm font-mono bg-black/90 text-green-400 p-3 rounded overflow-x-auto"><code>$ knot init:package utils --team myteam
📦 Initialized new package: utils</code></pre>
				</div>
				<div class="bg-muted/30 rounded-lg p-4">
					<div class="text-sm text-muted-foreground mb-2">packages/utils/package.yml</div>
					<pre class="text-sm font-mono bg-black/90 text-white p-3 rounded overflow-x-auto"><code><span class="text-blue-400">name:</span> <span class="text-green-400">utils</span>
<span class="text-blue-400">team:</span> <span class="text-green-400">myteam</span>
<span class="text-blue-400">version:</span> <span class="text-green-400">1.0.0</span>
<span class="text-blue-400">description:</span> <span class="text-green-400">Shared utility functions</span>
<span class="text-blue-400">tags:</span> <span class="text-yellow-400">[utilities, helpers]</span>
<span class="text-blue-400">scripts:</span>
  <span class="text-blue-400">build:</span> <span class="text-green-400">"tsc && rollup -c"</span>
  <span class="text-blue-400">test:</span> <span class="text-green-400">"vitest run"</span></code></pre>
				</div>
			</div>
		</div>
	</div>

	<!-- CLI Commands -->
	<div class="max-w-4xl mx-auto mb-16">
		<div class="flex items-center justify-center space-x-3 mb-8">
			<div class="w-8 h-8 bg-black/10 rounded-lg flex items-center justify-center">
				<Icon icon="solar:terminal-bold" class="w-5 h-5 text-black" />
			</div>
			<h3 class="text-2xl font-semibold">Essential Commands</h3>
		</div>
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<div class="bg-muted/30 rounded-lg p-6">
				<div class="flex items-center space-x-2 mb-3">
					<Icon icon="solar:link-bold" class="w-4 h-4 text-black" />
					<span class="font-semibold">Link Packages</span>
				</div>
				<pre class="text-sm font-mono bg-black/90 text-green-400 p-3 rounded overflow-x-auto"><code>$ knot link
🔗 Successfully copied all packages and 
   updated TypeScript configurations</code></pre>
			</div>
			<div class="bg-muted/30 rounded-lg p-6">
				<div class="flex items-center space-x-2 mb-3">
					<Icon icon="solar:rocket-2-bold" class="w-4 h-4 text-black" />
					<span class="font-semibold">Publish Package</span>
				</div>
				<pre class="text-sm font-mono bg-black/90 text-green-400 p-3 rounded overflow-x-auto"><code>$ knot publish --team myteam
📦 Successfully published utils v1.0.0
   Team: myteam</code></pre>
			</div>
			<div class="bg-muted/30 rounded-lg p-6">
				<div class="flex items-center space-x-2 mb-3">
					<Icon icon="solar:play-bold" class="w-4 h-4 text-black" />
					<span class="font-semibold">Run Scripts</span>
				</div>
				<pre class="text-sm font-mono bg-black/90 text-green-400 p-3 rounded overflow-x-auto"><code>$ knot run dev
🚀 Running app script 'dev'...
📝 Command: vite dev --port 3000</code></pre>
			</div>
			<div class="bg-muted/30 rounded-lg p-6">
				<div class="flex items-center space-x-2 mb-3">
					<Icon icon="solar:hammer-bold" class="w-4 h-4 text-black" />
					<span class="font-semibold">Build Apps</span>
				</div>
				<pre class="text-sm font-mono bg-black/90 text-green-400 p-3 rounded overflow-x-auto"><code>$ knot build
🔨 Building all apps...
✅ Successfully built: 2/2 apps</code></pre>
			</div>
		</div>
	</div>

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