<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { packagesStore, authStore } from '../../../../lib/stores';
	import Icon from '@iconify/svelte';
	import Chart from '../../../../lib/components/ui/chart.svelte';
	import { requestApi } from '../../../../lib/api';

	$: packageName = $page.params.name;
	$: selectedPackage = $packagesStore.selectedPackage;
	$: loading = $packagesStore.loading;
	$: currentUser = $authStore.user;

	let showDeleteConfirm = false;
	let deleteError = '';
	let downloadStats: { dailyStats: Array<{ date: string; downloads: number }> } | null = null;
	let loadingStats = false;

	$: isOwner = currentUser && selectedPackage && (
		selectedPackage.owner.id === currentUser.id ||
		(selectedPackage.team && selectedPackage.team.members.some(member => 
			member.user.id === currentUser.id && (member.role === 'owner' || member.role === 'admin')
		))
	);

	onMount(async () => {
		if (packageName) {
			try {
				await packagesStore.fetchByName(decodeURIComponent(packageName));
			} catch (error) {
				console.error('Failed to fetch package:', error);
			}
		}
	});

	// Fetch download statistics when package is loaded
	$: if (selectedPackage && selectedPackage.version && !loadingStats) {
		fetchDownloadStats();
	}

	async function fetchDownloadStats() {
		if (!selectedPackage) return;
		
		loadingStats = true;
		try {
			const response = await requestApi(
				'GET',
				`/api/packages/${encodeURIComponent(selectedPackage.name)}/${encodeURIComponent(selectedPackage.version)}/stats`
			);
			downloadStats = response;
		} catch (error) {
			console.error('Failed to fetch download stats:', error);
			downloadStats = null;
		} finally {
			loadingStats = false;
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

	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function formatFileSize(bytes: number | string | undefined): string {
		const numBytes = parseInt(bytes?.toString() || '0');
		if (numBytes < 1024) return numBytes + ' B';
		if (numBytes < 1024 * 1024) return (numBytes / 1024).toFixed(1) + ' KB';
		return (numBytes / (1024 * 1024)).toFixed(1) + ' MB';
	}
</script>

<svelte:head>
	<title>{packageName ? `${packageName} - Package` : 'Package'} - Knot Space</title>
	<meta name="description" content="View package details and manage your packages" />
</svelte:head>

{#if loading}
	<div class="flex items-center justify-center py-16">
		<div class="text-center">
			<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-black mx-auto mb-4"></div>
			<p class="text-muted-foreground">Loading package...</p>
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
	<div class="space-y-6">
		<!-- Header -->
		<div class="flex items-start justify-between">
			<div class="flex-1">
				<div class="flex items-center gap-2 mb-2">
					<a href="/packages" class="text-muted-foreground hover:text-foreground transition-colors">
						<Icon icon="solar:arrow-left-bold" class="w-5 h-5" />
					</a>
					<h1 class="text-3xl font-bold">{selectedPackage.name}</h1>
					<span class="text-lg text-muted-foreground bg-secondary px-3 py-1 rounded">
						v{selectedPackage.version}
					</span>
				</div>
				
				{#if selectedPackage.description}
					<p class="text-muted-foreground text-lg mb-4">{selectedPackage.description}</p>
				{/if}

				<div class="flex items-center gap-6 text-sm text-muted-foreground">
					<span class="flex items-center gap-1">
						<Icon icon="solar:user-circle-bold" class="w-4 h-4" />
						by <a href="/users/{encodeURIComponent(selectedPackage.owner.username)}" class="hover:text-primary transition-colors font-medium">{selectedPackage.owner.username}</a>
					</span>
					{#if selectedPackage.team}
						<span class="flex items-center gap-1">
							<Icon icon="solar:users-group-rounded-bold" class="w-4 h-4" />
							{selectedPackage.team.name}
						</span>
					{/if}
					<span class="flex items-center gap-1">
						<Icon icon="solar:calendar-bold" class="w-4 h-4" />
						{formatDate(selectedPackage.publishedAt)}
					</span>
				</div>
			</div>

			{#if isOwner}
				<div class="flex items-center gap-2">
					<button
						on:click={() => showDeleteConfirm = true}
						class="border border-destructive text-destructive hover:bg-destructive hover:text-destructive-foreground px-4 py-2 rounded-md text-sm font-medium transition-colors flex items-center gap-2"
					>
						<Icon icon="solar:trash-bin-minimalistic-bold" class="w-4 h-4" />
						Delete Package
					</button>
				</div>
			{/if}
		</div>

		<!-- Stats -->
		<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
			<div class="border rounded-lg p-4">
				<div class="flex items-center gap-2 mb-2">
					<Icon icon="solar:download-minimalistic-bold" class="w-5 h-5 text-muted-foreground" />
					<span class="font-medium">Downloads</span>
				</div>
				<div class="text-2xl font-bold">{(parseInt(selectedPackage.downloadsCount?.toString() || '0')).toLocaleString()}</div>
			</div>
			
			<div class="border rounded-lg p-4">
				<div class="flex items-center gap-2 mb-2">
					<Icon icon="solar:document-bold" class="w-5 h-5 text-muted-foreground" />
					<span class="font-medium">File Size</span>
				</div>
				<div class="text-2xl font-bold">{formatFileSize(selectedPackage.fileSize)}</div>
			</div>

			<div class="border rounded-lg p-4">
				<div class="flex items-center gap-2 mb-2">
					<Icon icon="solar:tag-bold" class="w-5 h-5 text-muted-foreground" />
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
						color="#8b5cf6"
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

		<!-- Installation -->
		<div class="space-y-4">
			<h3 class="text-xl font-semibold">Installation</h3>
			<div class="bg-muted rounded-lg p-4">
				<div class="flex items-center justify-between mb-2">
					<span class="text-sm font-medium">Knot CLI</span>
					<button 
						on:click={() => navigator.clipboard.writeText(`knot add ${selectedPackage.name}`)}
						class="text-xs text-muted-foreground hover:text-foreground transition-colors"
					>
						Copy
					</button>
				</div>
				<code class="text-sm">knot add {selectedPackage.name}</code>
			</div>
		</div>

		<!-- Delete Confirmation Modal -->
		{#if showDeleteConfirm}
			<div class="fixed inset-0 bg-black/50 flex items-center justify-center p-4 z-50">
				<div class="bg-background border border-border rounded-lg p-6 max-w-md w-full">
					<div class="flex items-center gap-3 mb-4">
						<Icon icon="solar:danger-triangle-bold" class="w-6 h-6 text-destructive" />
						<h3 class="text-lg font-semibold">Delete Package</h3>
					</div>
					
					<p class="text-muted-foreground mb-4">
						Are you sure you want to delete <strong>{selectedPackage.name}</strong> v{selectedPackage.version}?
						This action cannot be undone.
					</p>

					{#if deleteError}
						<div class="bg-destructive/10 text-destructive border border-destructive/20 rounded-md p-3 mb-4">
							{deleteError}
						</div>
					{/if}

					<div class="flex justify-end gap-3">
						<button
							on:click={() => { showDeleteConfirm = false; deleteError = ''; }}
							class="border border-input bg-background hover:bg-accent hover:text-accent-foreground px-4 py-2 rounded-md text-sm font-medium transition-colors"
						>
							Cancel
						</button>
						<button
							on:click={handleDeletePackage}
							class="bg-destructive text-destructive-foreground hover:bg-destructive/90 px-4 py-2 rounded-md text-sm font-medium transition-colors"
						>
							Delete Package
						</button>
					</div>
				</div>
			</div>
		{/if}
	</div>
{/if}