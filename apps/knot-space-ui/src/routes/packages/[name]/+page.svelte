<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { packagesStore, authStore } from '$lib/stores';
	import { formatDownloadCount, formatFileSize, formatDateTime, formatTimeAgo, formatDate } from '../../../lib/utils/format';
	import Icon from '@iconify/svelte';
	import Chart from '../../../lib/components/ui/chart.svelte';
	import Drawer from '../../../lib/components/ui/drawer.svelte';
	import FileBrowser from '../../../lib/components/FileBrowser.svelte';
	import { requestApi } from '../../../lib/api';

	$: packageName = $page.params.name;
	$: selectedPackage = $packagesStore.selectedPackage;
	$: loading = $packagesStore.loading;
	$: currentUser = $authStore.user;

	let showDeleteConfirm = false;
	let showOptionsMenu = false;
	let deleteError = '';
	let downloadStats: { dailyStats: Array<{ date: string; downloads: number }> } | null = null;
	let loadingStats = false;
	let statsLoaded = false;
	let copySuccess = false;
	let availableVersions: Package[] = [];
	let loadingVersions = false;
	let showVersionSelector = false;
	let switchingVersion = false;

	$: isOwner = currentUser && selectedPackage && (
		selectedPackage.owner.id === currentUser.id ||
		(selectedPackage.team && selectedPackage.team.members.some(member => 
			member.user.id === currentUser.id && (member.role === 'owner' || member.role === 'admin')
		))
	);

	onMount(async () => {
		if (packageName) {
			try {
				// Check if version is specified in URL params
				const urlParams = new URLSearchParams(window.location.search);
				const versionParam = urlParams.get('version');
				
				if (versionParam) {
					await packagesStore.fetchByName(decodeURIComponent(packageName), versionParam);
				} else {
					await packagesStore.fetchByName(decodeURIComponent(packageName));
				}
			} catch (error) {
				console.error('Failed to fetch package:', error);
			}
		}
	});

	// Reset all data when package changes (version switching)
	$: if (selectedPackage) {
		statsLoaded = false;
		downloadStats = null;
		// Reset versions so they get refetched for the new package version
		if (availableVersions.length === 0) {
			loadingVersions = false;
		}
	}

	// Fetch download statistics when package is loaded
	$: if (selectedPackage && selectedPackage.version && !loadingStats && !statsLoaded) {
		fetchDownloadStats();
	}

	// Fetch available versions when package is loaded
	$: if (selectedPackage && selectedPackage.name && !loadingVersions && availableVersions.length === 0) {
		fetchAvailableVersions();
	}

	async function fetchDownloadStats() {
		if (!selectedPackage) return;
		
		loadingStats = true;
		try {
			const response = await requestApi(
				'GET',
				`/api/packages/${encodeURIComponent(selectedPackage.name)}/${encodeURIComponent(selectedPackage.version)}/stats`
			);
			
			// Handle both success response formats
			const statsData = response.data || response;
			
			// Ensure the data has the expected structure
			if (statsData && Array.isArray(statsData.dailyStats)) {
				downloadStats = statsData;
			} else {
				// If no stats data or wrong format, show empty state
				downloadStats = { dailyStats: [] };
			}
		} catch (error) {
			console.error('Failed to fetch download stats:', error);
			downloadStats = { dailyStats: [] };
		} finally {
			loadingStats = false;
			statsLoaded = true;
		}
	}

	async function fetchAvailableVersions() {
		if (!selectedPackage || loadingVersions) return;
		
		loadingVersions = true;
		try {
			const versions = await requestApi<{ success: boolean; data: Package[] }>('GET', `/api/packages/${encodeURIComponent(selectedPackage.name)}/versions`);
			availableVersions = versions.data.sort((a, b) => new Date(b.publishedAt || b.createdAt).getTime() - new Date(a.publishedAt || a.createdAt).getTime());
		} catch (error) {
			console.error('Failed to fetch versions:', error);
		} finally {
			loadingVersions = false;
		}
	}

	async function switchToVersion(version: string) {
		if (!packageName || version === selectedPackage?.version || switchingVersion) return;
		
		switchingVersion = true;
		
		try {
			// Update the URL first
			const newUrl = `/packages/${encodeURIComponent(packageName)}?version=${encodeURIComponent(version)}`;
			window.history.pushState({}, '', newUrl);
			
			// Fetch the new version data - this completely replaces the current package state
			await packagesStore.fetchByName(decodeURIComponent(packageName), version);
			
			// Reset all dependent data that needs to reload for the new version
			statsLoaded = false;
			downloadStats = null;
			
			// Reset versions array so it gets refetched (in case the API returns different version lists)
			availableVersions = [];
			loadingVersions = false;
			
			// Close the version selector
			showVersionSelector = false;
		} catch (error) {
			console.error('Failed to switch version:', error);
			// Revert URL on error
			const currentUrl = `/packages/${encodeURIComponent(packageName)}`;
			window.history.replaceState({}, '', selectedPackage ? `${currentUrl}?version=${selectedPackage.version}` : currentUrl);
		} finally {
			switchingVersion = false;
		}
	}

	async function handleDeletePackage() {
		if (!selectedPackage) return;

		deleteError = '';
		try {
			await packagesStore.delete(selectedPackage.name, selectedPackage.version);
			// Navigate back to packages list
			window.location.href = '/packages';
		} catch (error) {
			deleteError = error instanceof Error ? error.message : 'Failed to delete package';
		}
	}

	function handleClickOutside(event: MouseEvent) {
		const target = event.target as Element;
		if (!target.closest('[data-options-menu]')) {
			showOptionsMenu = false;
		}
		if (!target.closest('[data-version-selector]')) {
			showVersionSelector = false;
		}
	}

</script>

<svelte:head>
	<title>{packageName ? `${packageName} - Package` : 'Package'} - Knot Space</title>
	<meta name="description" content="View package details and manage your packages" />
</svelte:head>

{#if loading || switchingVersion}
	<div class="flex items-center justify-center py-16">
		<div class="text-center">
			<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-black mx-auto mb-4"></div>
			<p class="text-muted-foreground">
				{switchingVersion ? 'Switching to version...' : 'Loading package...'}
			</p>
		</div>
	</div>
{:else if !selectedPackage}
	<div class="text-center py-16">
		<div class="text-6xl mb-4">📦</div>
		<h3 class="text-xl font-semibold mb-2">Package not found</h3>
		<p class="text-muted-foreground mb-6">
			The package "{packageName}" could not be found or you don't have permission to view it.
		</p>
		<a 
			href="/packages" 
			class="bg-primary text-primary-foreground hover:bg-primary/90 px-6 py-2 rounded-md font-medium transition-colors inline-block"
		>
			← Back to Packages
		</a>
	</div>
{:else}
	<div class="space-y-6" on:click={handleClickOutside} on:keydown={handleClickOutside} role="button" tabindex="0">
		<!-- Header -->
		<div class="flex items-start justify-between gap-4">
			<div class="flex-1 min-w-0">
				<div class="space-y-2 mb-2">
					<div class="flex items-center justify-between gap-4">
						<h1 class="text-2xl sm:text-3xl font-bold leading-tight" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">{selectedPackage.name}</h1>
						{#if isOwner}
							<div class="flex-shrink-0 relative" data-options-menu>
								<button
									on:click={() => showOptionsMenu = !showOptionsMenu}
									class="border border-input bg-background hover:bg-accent hover:text-accent-foreground p-2 rounded-md text-sm font-medium transition-colors flex items-center gap-2"
									aria-label="Package options"
								>
									<Icon icon="solar:menu-dots-bold" class="w-4 h-4" />
								</button>
								
								{#if showOptionsMenu}
									<div class="absolute right-0 top-full mt-1 bg-background border border-border rounded-md shadow-lg z-10 w-48">
										<button
											on:click={() => {
												showDeleteConfirm = true;
												showOptionsMenu = false;
											}}
											class="w-full text-left px-4 py-2 text-sm hover:bg-muted flex items-center gap-2 text-destructive hover:bg-destructive/10"
										>
											<Icon icon="solar:trash-bin-minimalistic-bold" class="w-4 h-4" />
											Delete Package
										</button>
									</div>
								{/if}
							</div>
						{/if}
					</div>
					<div class="flex items-center gap-3">
						<div class="relative">
							<button
								on:click={() => showVersionSelector = !showVersionSelector}
								class="text-sm sm:text-base text-muted-foreground bg-secondary hover:bg-secondary/80 px-2 sm:px-3 py-1 rounded-full transition-colors flex items-center gap-2"
								class:opacity-50={switchingVersion}
								disabled={loadingVersions || switchingVersion}
							>
								{#if switchingVersion}
									<div class="animate-spin rounded-full h-3 w-3 border-b border-current mr-1"></div>
								{/if}
								v{selectedPackage.version}
								{#if availableVersions.length > 1 && !switchingVersion}
									<Icon icon="solar:alt-arrow-down-bold" class="w-3 h-3" />
								{/if}
							</button>
							
							{#if showVersionSelector && availableVersions.length > 1}
								<div class="absolute top-full left-0 mt-1 bg-background border border-border rounded-lg shadow-lg z-10 min-w-48 max-h-60 overflow-y-auto" data-version-selector>
									<div class="py-1">
										{#each availableVersions as versionInfo}
											<button
												on:click={() => switchToVersion(versionInfo.version)}
												class="w-full text-left px-3 py-2 text-sm hover:bg-muted flex items-center justify-between {versionInfo.version === selectedPackage.version ? 'bg-muted font-medium' : ''}"
											>
												<span>v{versionInfo.version}</span>
												<span class="text-xs text-muted-foreground">
													{new Date(versionInfo.publishedAt || versionInfo.createdAt).toLocaleDateString()}
												</span>
											</button>
										{/each}
									</div>
								</div>
							{/if}
						</div>
						
						{#if loadingVersions}
							<div class="animate-spin rounded-full h-4 w-4 border-b-2 border-primary"></div>
						{/if}
					</div>
				</div>
				
				{#if selectedPackage.description}
					<p class="text-muted-foreground text-lg mb-4">{selectedPackage.description}</p>
				{/if}

				<div class="space-y-3">
					<div class="flex items-center gap-6 text-sm text-muted-foreground">
						<span class="flex items-center gap-2">
							<!-- Profile avatar matching the profile page design -->
							<div class="w-6 h-6 bg-gradient-to-br from-primary to-primary/80 rounded-lg flex items-center justify-center text-primary-foreground text-xs font-bold">
								{selectedPackage.owner.username.charAt(0).toUpperCase()}
							</div>
							by <a href="/users/{encodeURIComponent(selectedPackage.owner.username)}" class="hover:text-primary transition-colors font-medium">{selectedPackage.owner.username}</a>
						</span>
						{#if selectedPackage.team}
							<span class="flex items-center gap-1">
								<Icon icon="solar:users-group-rounded-bold" class="w-4 h-4" />
								{selectedPackage.team.name}
							</span>
						{/if}
					</div>
					
					<!-- Timeline section -->
					<div class="bg-muted/30 rounded-lg p-4 space-y-3">
						<div class="flex items-center gap-2">
							<span class="text-sm font-medium text-foreground">Last updated</span>
							<span class="text-sm text-muted-foreground italic">{formatTimeAgo(selectedPackage.updatedAt || selectedPackage.publishedAt)}</span>
						</div>
						<div class="grid grid-cols-1 sm:grid-cols-2 gap-3 text-xs text-muted-foreground">
							<div class="flex items-center gap-2">
								<Icon icon="solar:calendar-add-bold" class="w-4 h-4 text-green-600" />
								<span>Created {formatDate(selectedPackage.publishedAt)}</span>
							</div>
							<div class="flex items-center gap-2">
								<Icon icon="solar:history-3-bold" class="w-4 h-4 text-blue-600" />
								<span>Updated {formatDate(selectedPackage.updatedAt || selectedPackage.publishedAt)}</span>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>

		<!-- Stats -->
		<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
			<div class="border rounded-lg p-4">
				<div class="flex items-center gap-2 mb-2">
					<Icon icon="solar:download-minimalistic-bold" class="w-5 h-5 text-green-500" />
					<span class="font-medium">Downloads</span>
				</div>
				<div class="text-2xl font-bold">{formatDownloadCount(selectedPackage.totalDownloadsCount || selectedPackage.downloadsCount)}</div>
			</div>
			
			<div class="border rounded-lg p-4">
				<div class="flex items-center gap-2 mb-2">
					<Icon icon="solar:document-bold" class="w-5 h-5 text-blue-500" />
					<span class="font-medium">File Size</span>
				</div>
				<div class="text-2xl font-bold">{formatFileSize(selectedPackage.fileSize)}</div>
			</div>

			<div class="border rounded-lg p-4">
				<div class="flex items-center gap-2 mb-2">
					<Icon icon="solar:tag-bold" class="w-5 h-5 text-purple-500" />
					<span class="font-medium">Version</span>
				</div>
				<div class="text-2xl font-bold">v{selectedPackage.version}</div>
			</div>
		</div>

		<!-- Tags -->
		{#if selectedPackage.tags && selectedPackage.tags.length > 0}
			<div class="space-y-2">
				<h3 class="font-semibold">Tags</h3>
				<div class="flex flex-wrap gap-2">
					{#each selectedPackage.tags as tag}
						<span class="bg-secondary text-secondary-foreground px-2 py-1 rounded text-sm">
							{tag}
						</span>
					{/each}
				</div>
			</div>
		{/if}

		<!-- Installation -->
		<div class="space-y-4">
			<h3 class="text-xl font-semibold">Installation</h3>
			<div class="bg-muted rounded-lg p-4">
				<div class="flex items-center justify-between mb-2">
					<span class="text-sm font-medium">Knot CLI</span>
					<button 
						on:click={async () => {
							await navigator.clipboard.writeText(`knot install @${selectedPackage.name}@${selectedPackage.version}`);
							copySuccess = true;
							setTimeout(() => copySuccess = false, 2000);
						}}
						class="text-xs {copySuccess ? 'text-green-600' : 'text-muted-foreground hover:text-foreground'} transition-colors flex items-center gap-1"
					>
						{#if copySuccess}
							<Icon icon="solar:check-circle-bold" class="w-3 h-3" />
							Copied!
						{:else}
							<Icon icon="solar:copy-bold" class="w-3 h-3" />
							Copy
						{/if}
					</button>
				</div>
				<code class="text-sm">knot install @{selectedPackage.name}@{selectedPackage.version}</code>
			</div>
		</div>

		<!-- File Browser -->
		<div class="space-y-4">
			<h3 class="text-xl font-semibold">Package Files</h3>
			<FileBrowser 
				packageName={selectedPackage.name} 
				packageVersion={selectedPackage.version} 
			/>
		</div>

		<!-- Download Statistics -->
		<div class="space-y-4">
			<h3 class="text-xl font-semibold">Download Statistics</h3>
			{#if loadingStats}
				<div class="flex items-center justify-center py-8">
					<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-current"></div>
				</div>
			{:else if downloadStats && downloadStats.dailyStats.length > 0}
				<div class="bg-card border border-border rounded-lg p-6">
					<Chart
						data={downloadStats.dailyStats}
						title="Downloads per Day (Last 7 Days)"
						color="#000000"
						height={300}
					/>
				</div>
			{:else}
				<div class="bg-card border border-border rounded-lg p-6">
					<div class="flex items-center justify-center py-8 text-muted-foreground">
						<div class="text-center space-y-2">
							<div class="text-2xl">📊</div>
							<p>No download statistics available yet</p>
							<p class="text-sm">Statistics will appear after the first download</p>
						</div>
					</div>
				</div>
			{/if}
		</div>

		<!-- Delete Confirmation Drawer -->
		<Drawer
			bind:open={showDeleteConfirm}
			title="Delete Package"
			description="This action cannot be undone"
			side="right"
			on:close={() => { deleteError = ''; }}
		>
			<div class="space-y-6">
				<div class="flex items-center gap-3 p-4 bg-destructive/10 rounded-lg border border-destructive/20">
					<Icon icon="solar:danger-triangle-bold" class="w-5 h-5 text-destructive flex-shrink-0" />
					<div class="text-sm">
						<p class="font-medium text-destructive mb-1">Permanent Deletion</p>
						<p class="text-muted-foreground">
							You are about to permanently delete <strong>{selectedPackage?.name}</strong> v{selectedPackage?.version}.
						</p>
					</div>
				</div>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">What will happen:</h4>
						<ul class="text-sm text-muted-foreground space-y-1 list-disc list-inside">
							<li>The package will be removed from all registries</li>
							<li>All download statistics will be lost</li>
							<li>Projects using this package may break</li>
							<li>This action cannot be reversed</li>
						</ul>
					</div>

					{#if deleteError}
						<div class="bg-destructive/10 text-destructive border border-destructive/20 rounded-md p-3">
							{deleteError}
						</div>
					{/if}
				</div>

				<div class="flex flex-col gap-3 pt-4 border-t border-border">
					<button
						on:click={handleDeletePackage}
						class="bg-destructive text-destructive-foreground hover:bg-destructive/90 px-4 py-2 rounded-md text-sm font-medium transition-colors flex items-center justify-center gap-2"
					>
						<Icon icon="solar:trash-bin-minimalistic-bold" class="w-4 h-4" />
						Delete Package
					</button>
					<button
						on:click={() => { showDeleteConfirm = false; deleteError = ''; }}
						class="border border-input bg-background hover:bg-accent hover:text-accent-foreground px-4 py-2 rounded-md text-sm font-medium transition-colors"
					>
						Cancel
					</button>
				</div>
			</div>
		</Drawer>
	</div>
{/if}