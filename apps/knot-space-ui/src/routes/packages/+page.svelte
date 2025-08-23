<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { packagesStore } from '$lib/stores';
	import { formatDownloadCount, formatFileSize, formatDate, formatTimeAgo } from '$lib/utils/format';
	import PackageCard from '$lib/components/PackageCard.svelte';

	$: packages = $packagesStore.packages;
	$: loading = $packagesStore.loading;
	$: searchResults = $packagesStore.searchResults;
	let searchTerm = '';
	let selectedTags: string[] = [];
	let filteredPackages: any[] = [];
	let availableTags: string[] = [];
	let popularTags: { tag: string; count: number }[] = [];
	let tagSearchTerm = '';
	let showAllTags = false;

	onMount(async () => {
		try {
			await packagesStore.fetchAll();
		} catch (error) {
			console.error('Failed to fetch packages:', error);
		}
	});

	// Extract all available tags from packages and calculate popularity
	$: {
		const tagCount = new Map<string, number>();
		packages.forEach(pkg => {
			pkg.tags?.forEach(tag => {
				tagCount.set(tag, (tagCount.get(tag) || 0) + 1);
			});
		});
		
		// Sort by popularity (count) then alphabetically
		popularTags = Array.from(tagCount.entries())
			.map(([tag, count]) => ({ tag, count }))
			.sort((a, b) => b.count - a.count || a.tag.localeCompare(b.tag));
		
		availableTags = popularTags.map(t => t.tag);
	}

	// Filter packages based on search term and selected tags
	$: {
		let filtered = packages;

		// Apply text search filter
		if (searchTerm.trim() !== '') {
			const term = searchTerm.toLowerCase();
			filtered = filtered.filter(pkg => 
				pkg.name.toLowerCase().includes(term) ||
				pkg.description?.toLowerCase().includes(term) ||
				pkg.owner.username.toLowerCase().includes(term) ||
				pkg.team?.name.toLowerCase().includes(term) ||
				pkg.tags?.some(tag => tag.toLowerCase().includes(term))
			);
		}

		// Apply tag filter
		if (selectedTags.length > 0) {
			filtered = filtered.filter(pkg => 
				selectedTags.every(selectedTag => 
					pkg.tags?.includes(selectedTag)
				)
			);
		}

		filteredPackages = filtered;

		// Clear search results from store if using local filtering
		if (searchTerm.trim() === '' && selectedTags.length === 0) {
			packagesStore.clearSearch();
		}
	}

	function toggleTag(tag: string) {
		if (selectedTags.includes(tag)) {
			selectedTags = selectedTags.filter(t => t !== tag);
		} else {
			selectedTags = [...selectedTags, tag];
		}
	}

	function clearAllFilters() {
		searchTerm = '';
		selectedTags = [];
		tagSearchTerm = '';
		showAllTags = false;
	}

	// Filter tags based on search term
	$: filteredTagsForSearch = popularTags.filter(({ tag }) => 
		tagSearchTerm.trim() === '' || 
		tag.toLowerCase().includes(tagSearchTerm.toLowerCase())
	);

	// Get tags to display: popular ones first, then filtered by search
	$: displayTags = showAllTags ? filteredTagsForSearch : filteredTagsForSearch.slice(0, 12);

</script>

<svelte:head>
	<title>Browse Packages - Knot Space Package Registry</title>
	<meta name="description" content="Discover and explore TypeScript/JavaScript packages in the Knot Space registry. Find packages by tags, search by name or description, and explore the growing ecosystem of monorepo packages." />
	<meta name="keywords" content="package registry, browse packages, typescript packages, javascript packages, monorepo packages, knot packages, npm alternative" />
	<meta name="author" content="Knot Space" />
	
	<!-- Open Graph / Facebook -->
	<meta property="og:type" content="website" />
	<meta property="og:url" content="https://knot.klysium.com/packages" />
	<meta property="og:title" content="Browse Packages - Knot Space Package Registry" />
	<meta property="og:description" content="Discover and explore TypeScript/JavaScript packages in the Knot Space registry. Find packages by tags, search by name or description, and explore the growing ecosystem." />
	<meta property="og:image" content="https://knot.klysium.com/images/og/packages.png" />
	<meta property="og:image:alt" content="Browse Packages in Knot Space Registry" />
	<meta property="og:site_name" content="Knot Space" />

	<!-- Twitter -->
	<meta name="twitter:card" content="summary_large_image" />
	<meta name="twitter:url" content="https://knot.klysium.com/packages" />
	<meta name="twitter:title" content="Browse Packages - Knot Space Package Registry" />
	<meta name="twitter:description" content="Discover and explore TypeScript/JavaScript packages in the Knot Space registry. Find packages by tags and search functionality." />
	<meta name="twitter:image" content="https://knot.klysium.com/images/og/packages.png" />
	<meta name="twitter:image:alt" content="Browse Packages in Knot Space Registry" />
	<meta name="twitter:creator" content="@knotspace" />
	<meta name="twitter:site" content="@knotspace" />

	<!-- Additional SEO -->
	<meta name="robots" content="index, follow" />
	<link rel="canonical" href="https://knot.klysium.com/packages" />
	
	<!-- Schema.org structured data -->
	<script type="application/ld+json">
	{
		"@context": "https://schema.org",
		"@type": "CollectionPage",
		"name": "Packages - Knot Space",
		"description": "Discover and explore TypeScript/JavaScript packages in the Knot Space registry",
		"url": "https://knot.klysium.com/packages",
		"mainEntity": {
			"@type": "ItemList",
			"name": "Package Registry",
			"description": "Collection of TypeScript/JavaScript packages for monorepo development"
		},
		"breadcrumb": {
			"@type": "BreadcrumbList",
			"itemListElement": [
				{
					"@type": "ListItem",
					"position": 1,
					"name": "Home",
					"item": "https://knot.klysium.com"
				},
				{
					"@type": "ListItem",
					"position": 2,
					"name": "Packages",
					"item": "https://knot.klysium.com/packages"
				}
			]
		},
		"publisher": {
			"@type": "Organization",
			"name": "Knot Space",
			"url": "https://knot.klysium.com"
		}
	}
	</script>
</svelte:head>

<div class="space-y-6">
	<div class="flex flex-col gap-4">
		<div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
			<div>
				<h1 class="text-3xl font-bold" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">Packages</h1>
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

		<!-- Enhanced Tag Filters -->
		{#if availableTags.length > 0}
			<div class="space-y-4">
				<div class="flex items-center justify-between">
					<h3 class="text-sm font-medium text-foreground">Filter by tags</h3>
					{#if selectedTags.length > 0 || searchTerm.trim() !== ''}
						<button 
							on:click={clearAllFilters}
							class="text-xs text-muted-foreground hover:text-primary transition-colors"
						>
							Clear all filters
						</button>
					{/if}
				</div>

				<!-- Selected tags display -->
				{#if selectedTags.length > 0}
					<div class="space-y-2">
						<span class="text-xs font-medium text-muted-foreground">Selected tags:</span>
						<div class="flex flex-wrap gap-1">
							{#each selectedTags as tag}
								<span class="px-2 py-1 bg-primary text-primary-foreground rounded-full text-xs flex items-center gap-1">
									{tag}
									<button 
										on:click={() => toggleTag(tag)}
										class="hover:bg-primary-foreground/20 rounded-full p-0.5 transition-colors"
										aria-label="Remove {tag} filter"
									>
										<svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
											<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
										</svg>
									</button>
								</span>
							{/each}
						</div>
					</div>
				{/if}

				<!-- Tag search -->
				<div class="relative">
					<svg class="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
					</svg>
					<input
						type="text"
						placeholder="Search tags..."
						bind:value={tagSearchTerm}
						class="pl-10 pr-4 py-2 w-full text-sm border border-input rounded-lg bg-background focus:outline-none focus:ring-2 focus:ring-ring focus:border-transparent"
					/>
					{#if tagSearchTerm.trim() !== ''}
						<button
							on:click={() => tagSearchTerm = ''}
							class="absolute right-3 top-1/2 transform -translate-y-1/2 text-muted-foreground hover:text-foreground transition-colors"
							aria-label="Clear tag search"
						>
							<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
							</svg>
						</button>
					{/if}
				</div>

				<!-- Available tags -->
				{#if displayTags.length > 0}
					<div class="space-y-3">
						{#if tagSearchTerm.trim() === ''}
							<span class="text-xs text-muted-foreground">
								{showAllTags ? 'All tags' : 'Popular tags'} 
								({displayTags.length}{showAllTags ? '' : ` of ${popularTags.length}`})
							</span>
						{:else}
							<span class="text-xs text-muted-foreground">
								{displayTags.length} tag{displayTags.length === 1 ? '' : 's'} found
							</span>
						{/if}
						
						<div class="flex flex-wrap gap-2">
							{#each displayTags as { tag, count }}
								<button
									on:click={() => toggleTag(tag)}
									class="group px-3 py-1.5 text-xs rounded-full border transition-all duration-200 {selectedTags.includes(tag) 
										? 'bg-primary text-primary-foreground border-primary shadow-sm' 
										: 'bg-secondary hover:bg-secondary/80 text-secondary-foreground border-input hover:border-primary/50 hover:shadow-sm'}"
									title="{count} package{count === 1 ? '' : 's'}"
								>
									<span class="font-medium">{tag}</span>
									<span class="ml-1 opacity-60 text-xs">({count})</span>
								</button>
							{/each}
						</div>

						<!-- Show more/less button -->
						{#if tagSearchTerm.trim() === '' && popularTags.length > 12}
							<div class="text-center">
								<button
									on:click={() => showAllTags = !showAllTags}
									class="text-xs text-muted-foreground hover:text-primary transition-colors underline"
								>
									{showAllTags ? 'Show less tags' : `Show all ${popularTags.length} tags`}
								</button>
							</div>
						{/if}
					</div>
				{:else if tagSearchTerm.trim() !== ''}
					<div class="text-center py-4 text-muted-foreground text-sm">
						No tags found matching "{tagSearchTerm}"
					</div>
				{/if}
			</div>
		{/if}
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
				{searchTerm || selectedTags.length > 0 ? '🔍' : '📦'}
			</div>
			<h3 class="text-xl font-semibold mb-2">
				{searchTerm || selectedTags.length > 0 ? 'No packages found' : 'No packages yet'}
			</h3>
			<p class="text-muted-foreground mb-4">
				{searchTerm || selectedTags.length > 0
					? `No packages match your current filters. Try adjusting your search or selected tags.`
					: 'Be the first to publish a package to Knot Space!'
				}
			</p>
			{#if searchTerm || selectedTags.length > 0}
				<button 
					on:click={clearAllFilters}
					class="bg-primary text-primary-foreground hover:bg-primary/90 px-6 py-2 rounded-md font-medium transition-colors"
				>
					Clear All Filters
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
				{#if searchTerm.trim() !== '' || selectedTags.length > 0}
					{filteredPackages.length} package{filteredPackages.length === 1 ? '' : 's'} found
					{#if searchTerm.trim() !== ''}
						for "{searchTerm}"
					{/if}
					{#if selectedTags.length > 0}
						with tag{selectedTags.length === 1 ? '' : 's'}: {selectedTags.join(', ')}
					{/if}
				{:else}
					{filteredPackages.length} package{filteredPackages.length === 1 ? '' : 's'} total
				{/if}
			</div>

			<!-- Package list -->
			<div class="grid grid-cols-1 gap-4">
				{#each filteredPackages as pkg}
					<PackageCard {pkg} />
				{/each}
			</div>
		</div>
	{/if}
</div>