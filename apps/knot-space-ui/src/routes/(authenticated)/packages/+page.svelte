<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { packagesStore } from '../../../lib/stores';
	import { formatDownloadCount, formatFileSize, formatDate, formatTimeAgo } from '../../../lib/utils/format';

	$: packages = $packagesStore.packages;
	$: loading = $packagesStore.loading;
	$: searchResults = $packagesStore.searchResults;
	let searchTerm = '';
	let selectedTags: string[] = [];
	let filteredPackages: any[] = [];
	let availableTags: string[] = [];

	onMount(async () => {
		try {
			await packagesStore.fetchAll();
		} catch (error) {
			console.error('Failed to fetch packages:', error);
		}
	});

	// Extract all available tags from packages
	$: {
		const tagSet = new Set<string>();
		packages.forEach(pkg => {
			pkg.tags?.forEach(tag => tagSet.add(tag));
		});
		availableTags = Array.from(tagSet).sort();
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
	}

</script>

<svelte:head>
	<title>Packages - Knot Space</title>
	<meta name="description" content="Browse all packages in the Knot Space registry" />
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

		<!-- Tag Filters -->
		{#if availableTags.length > 0}
			<div class="space-y-3">
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
				<div class="flex flex-wrap gap-2">
					{#each availableTags as tag}
						<button
							on:click={() => toggleTag(tag)}
							class="px-3 py-1 text-xs rounded-full border transition-all duration-200 {selectedTags.includes(tag) 
								? 'bg-primary text-primary-foreground border-primary' 
								: 'bg-secondary hover:bg-secondary/80 text-secondary-foreground border-input hover:border-primary/50'}"
						>
							{tag}
						</button>
					{/each}
				</div>
				{#if selectedTags.length > 0}
					<div class="flex items-center gap-2 text-xs text-muted-foreground">
						<span>Selected tags:</span>
						<div class="flex flex-wrap gap-1">
							{#each selectedTags as tag}
								<span class="px-2 py-1 bg-primary/10 text-primary rounded">
									{tag}
									<button 
										on:click={() => toggleTag(tag)}
										class="ml-1 hover:text-primary-foreground transition-colors"
									>×</button>
								</span>
							{/each}
						</div>
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
					<a 
						href="/packages/{encodeURIComponent(pkg.name)}"
						class="block border rounded-lg p-6 hover:shadow-md transition-all duration-200 hover:border-primary/50 cursor-pointer"
					>
						<div class="flex items-start justify-between mb-4">
							<div class="flex-1 min-w-0">
								<div class="flex items-center gap-2 mb-2">
									<h3 class="font-semibold text-lg truncate hover:text-primary transition-colors">
										{pkg.name}
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

								<!-- Tags -->
								{#if pkg.tags && pkg.tags.length > 0}
									<div class="flex flex-wrap gap-1 mb-3">
										{#each pkg.tags as tag}
											<span class="px-2 py-1 text-xs bg-secondary text-secondary-foreground rounded border">
												{tag}
											</span>
										{/each}
									</div>
								{/if}
								
								<div class="flex items-center gap-4 text-sm text-muted-foreground">
									<span>by 
										<button 
											class="hover:text-primary transition-colors font-medium underline"
											on:click={(e) => {
												e.stopPropagation();
												e.preventDefault();
												goto(`/users/${encodeURIComponent(pkg.owner.username)}`);
											}}
										>{pkg.owner.username}</button>
									</span>
									{#if pkg.team}
										<span class="flex items-center gap-1">
											<span class="w-2 h-2 bg-primary rounded-full"></span>
											{pkg.team.name}
										</span>
									{/if}
									<span>{formatDate(pkg.publishedAt || pkg.createdAt)}</span>
									{#if pkg.updatedAt && pkg.updatedAt !== pkg.createdAt}
										<span class="flex items-center gap-1 text-xs">
											<svg class="h-3 w-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
												<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
											</svg>
											Updated {formatTimeAgo(pkg.updatedAt)}
										</span>
									{/if}
								</div>
							</div>
							
							<div class="flex flex-col items-end gap-2 text-sm text-muted-foreground ml-4">
								<div class="flex items-center gap-1">
									<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M9 19l3 3m0 0l3-3m-3 3V10" />
									</svg>
									{formatDownloadCount(pkg.totalDownloadsCount || pkg.downloadsCount)}
								</div>
								<div class="text-xs">
									{formatFileSize(pkg.fileSize)}
								</div>
							</div>
						</div>
					</a>
				{/each}
			</div>
		</div>
	{/if}
</div>