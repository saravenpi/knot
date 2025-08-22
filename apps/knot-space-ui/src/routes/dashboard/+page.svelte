<script lang="ts">
	import { onMount } from 'svelte';
	import { packagesStore, authStore } from '$lib/stores';
	import { packagesApi } from '../../lib/api';
	import { formatLargeNumber, formatTimeAgo } from '../../lib/utils/format';
	import PackageCard from '$lib/components/PackageCard.svelte';

	let stats = {
		totalPackages: 0,
		totalDownloads: 0,
		totalUsers: 0
	};
	let statsLoading = true;
	let packagesLoading = true;
	let recentPackages: any[] = [];

	onMount(async () => {
		// Fetch stats
		try {
			const timeout = new Promise((_, reject) => {
				setTimeout(() => reject(new Error('Request timeout')), 3000);
			});

			const statsResult = await Promise.race([packagesApi.getGlobalStats(), timeout]);

			stats = statsResult as any;
			statsLoading = false;
		} catch (error) {
			console.error('Failed to fetch stats:', error);
			// Set default values when backend is not available
			stats = {
				totalPackages: 0,
				totalDownloads: 0,
				totalUsers: 0
			};
			statsLoading = false;
		}

		// Fetch recent packages
		try {
			await packagesStore.fetchAll();
			// Get the 5 most recent packages
			recentPackages = $packagesStore.packages.slice(0, 5);
			packagesLoading = false;
		} catch (error) {
			console.error('Failed to fetch packages:', error);
			packagesLoading = false;
		}
	});

	$: packages = $packagesStore.packages;
</script>

<svelte:head>
	<title>Dashboard - Knot Space</title>
</svelte:head>

<div class="space-y-6">
	<h1 class="text-3xl font-bold">Dashboard</h1>
	
	<div class="grid grid-cols-1 md:grid-cols-3 gap-6">
		<div class="border rounded-lg p-6">
			<h3 class="text-lg font-medium text-muted-foreground mb-2">Total Packages</h3>
			{#if statsLoading}
				<div class="animate-pulse bg-muted rounded w-16 h-8"></div>
			{:else}
				<div class="text-3xl font-bold">{formatLargeNumber(stats.totalPackages)}</div>
			{/if}
			</div>
		<div class="border rounded-lg p-6">
			<h3 class="text-lg font-medium text-muted-foreground mb-2">Total Downloads</h3>
			{#if statsLoading}
				<div class="animate-pulse bg-muted rounded w-16 h-8"></div>
			{:else}
				<div class="text-3xl font-bold">{formatLargeNumber(stats.totalDownloads)}</div>
			{/if}
		</div>
		<div class="border rounded-lg p-6">
			<h3 class="text-lg font-medium text-muted-foreground mb-2">Total Users</h3>
			{#if statsLoading}
				<div class="animate-pulse bg-muted rounded w-16 h-8"></div>
			{:else}
				<div class="text-3xl font-bold">{formatLargeNumber(stats.totalUsers)}</div>
			{/if}
		</div>
	</div>

	<!-- Recent Packages Section -->
	<div class="mt-8">
		<div class="flex items-center justify-between mb-4">
			<h2 class="text-2xl font-bold">Recent Packages</h2>
			<a href="/packages" class="text-primary hover:underline text-sm">View all →</a>
		</div>
		
		{#if packagesLoading}
			<div class="space-y-4">
				{#each Array(3) as _}
					<div class="border rounded-lg p-6 animate-pulse">
						<div class="flex items-start justify-between mb-4">
							<div class="flex-1">
								<div class="h-5 bg-muted rounded w-1/3 mb-2"></div>
								<div class="h-4 bg-muted rounded w-1/4 mb-2"></div>
								<div class="h-4 bg-muted rounded w-2/3"></div>
							</div>
							<div class="h-6 bg-muted rounded w-16"></div>
						</div>
					</div>
				{/each}
			</div>
		{:else if recentPackages.length === 0}
			<div class="border rounded-lg p-8 text-center">
				<div class="text-4xl mb-2">📦</div>
				<p class="text-muted-foreground">No packages published yet</p>
				<p class="text-sm text-muted-foreground mt-2">Be the first to publish a package!</p>
			</div>
		{:else}
			<div class="space-y-4">
				{#each recentPackages as pkg}
					<PackageCard {pkg} />
				{/each}
			</div>
		{/if}
	</div>
</div>