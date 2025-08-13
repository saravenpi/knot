<script lang="ts">
	import { onMount } from 'svelte';
	import { packagesStore } from '../../../lib/stores';

	$: packages = $packagesStore.packages;
	$: loading = $packagesStore.loading;
	$: searchResults = $packagesStore.searchResults;
	let searchTerm = '';
	let filteredPackages: any[] = [];

	onMount(async () => {
		try {
			await packagesStore.fetchAll();
		} catch (error) {
			console.error('Failed to fetch packages:', error);
		}
	});

	// Filter packages based on search term
	$: {
		if (searchTerm.trim() === '') {
			filteredPackages = packages;
			packagesStore.clearSearch();
		} else {
			const term = searchTerm.toLowerCase();
			filteredPackages = packages.filter(pkg => 
				pkg.name.toLowerCase().includes(term) ||
				pkg.description?.toLowerCase().includes(term) ||
				pkg.owner.username.toLowerCase().includes(term) ||
				pkg.team?.name.toLowerCase().includes(term)
			);
		}
	}

	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric'
		});
	}
</script>

<svelte:head>
	<title>Packages - Knot Space</title>
	<meta name="description" content="Browse all packages in the Knot Space registry" />
</svelte:head>

<div class="space-y-6">
	<div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
		<div>
			<h1 class="text-3xl font-bold">Packages</h1>
			<p class="text-muted-foreground">Discover and explore packages in the registry</p>
		</div>
		
		<!-- Search -->
		<div class="relative max-w-sm">
			<svg class="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
				<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
			</svg>
			<input
				type="text"
				placeholder="Search packages..."
				bind:value={searchTerm}
				class="pl-10 pr-4 py-2 w-full border border-input rounded-md bg-background focus:outline-none focus:ring-2 focus:ring-ring focus:border-transparent"
			/>
		</div>
	</div>

	{#if loading}
		<div class="grid grid-cols-1 gap-4">
			{#each Array(8) as _}
				<div class="border rounded-lg p-6 animate-pulse">
					<div class="flex items-start justify-between mb-4">
						<div class="flex-1">
							<div class="h-5 bg-muted rounded w-1/3 mb-2"></div>
							<div class="h-4 bg-muted rounded w-1/4 mb-2"></div>
							<div class="h-4 bg-muted rounded w-2/3"></div>
						</div>
						<div class="h-6 bg-muted rounded w-16"></div>
					</div>
					<div class="flex justify-between">
						<div class="h-4 bg-muted rounded w-1/4"></div>
						<div class="h-4 bg-muted rounded w-1/6"></div>
					</div>
				</div>
			{/each}
		</div>
	{:else if filteredPackages.length === 0}
		<div class="text-center py-12">
			<div class="text-6xl mb-4">
				{searchTerm ? '🔍' : '📦'}
			</div>
			<h3 class="text-xl font-semibold mb-2">
				{searchTerm ? 'No packages found' : 'No packages yet'}
			</h3>
			<p class="text-muted-foreground mb-4">
				{searchTerm 
					? `No packages match "${searchTerm}". Try a different search term.`
					: 'Be the first to publish a package to Knot Space!'
				}
			</p>
			{#if searchTerm}
				<button 
					on:click={() => searchTerm = ''}
					class="bg-primary text-primary-foreground hover:bg-primary/90 px-6 py-2 rounded-md font-medium transition-colors"
				>
					Clear Search
				</button>
			{:else}
				<a 
					href="/login" 
					class="bg-primary text-primary-foreground hover:bg-primary/90 px-6 py-2 rounded-md font-medium transition-colors inline-block"
				>
					Get Started
				</a>
			{/if}
		</div>
	{:else}
		<div class="space-y-4">
			<!-- Results count -->
			<div class="text-sm text-muted-foreground">
				{#if searchTerm}
					{filteredPackages.length} package{filteredPackages.length === 1 ? '' : 's'} found for "{searchTerm}"
				{:else}
					{filteredPackages.length} package{filteredPackages.length === 1 ? '' : 's'} total
				{/if}
			</div>

			<!-- Package list -->
			<div class="grid grid-cols-1 gap-4">
				{#each filteredPackages as pkg}
					<div class="border rounded-lg p-6 hover:shadow-md transition-all duration-200">
						<div class="flex items-start justify-between mb-4">
							<div class="flex-1 min-w-0">
								<div class="flex items-center gap-2 mb-2">
									<h3 class="font-semibold text-lg truncate">
										<a href="/packages/{encodeURIComponent(pkg.name)}" class="hover:text-primary transition-colors">
											{pkg.name}
										</a>
									</h3>
									<span class="text-sm text-muted-foreground bg-secondary px-2 py-1 rounded flex-shrink-0">
										v{pkg.version}
									</span>
								</div>
								
								{#if pkg.description}
									<p class="text-muted-foreground mb-3 line-clamp-2">
										{pkg.description}
									</p>
								{/if}
								
								<div class="flex items-center gap-4 text-sm text-muted-foreground">
									<span>by {pkg.owner.username}</span>
									{#if pkg.team}
										<span class="flex items-center gap-1">
											<span class="w-2 h-2 bg-primary rounded-full"></span>
											{pkg.team.name}
										</span>
									{/if}
									<span>{formatDate(pkg.published_at)}</span>
								</div>
							</div>
							
							<div class="flex flex-col items-end gap-2 text-sm text-muted-foreground ml-4">
								<div class="flex items-center gap-1">
									<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M9 19l3 3m0 0l3-3m-3 3V10" />
									</svg>
									{(pkg.downloads_count || 0).toLocaleString()}
								</div>
								<div class="text-xs">
									{(pkg.file_size / 1024).toFixed(1)} KB
								</div>
							</div>
						</div>
					</div>
				{/each}
			</div>
		</div>
	{/if}
</div>