<script lang="ts">
	import { onMount } from 'svelte';
	import { packagesStore, authStore } from '../../lib/stores';
	import { packagesApi } from '../../lib/api';
	import { formatLargeNumber } from '../../lib/utils/format';

	let stats = {
		totalPackages: 0,
		totalDownloads: 0,
		totalUsers: 0
	};
	let statsLoading = true;

	onMount(async () => {
		try {
			const timeout = new Promise((_, reject) => {
				setTimeout(() => reject(new Error('Request timeout')), 3000);
			});

			const statsResult = await Promise.race([packagesApi.getGlobalStats(), timeout]);

			stats = statsResult as any;
			statsLoading = false;
		} catch (error) {
			console.error('Failed to fetch data:', error);
			// Set default values when backend is not available
			stats = {
				totalPackages: 0,
				totalDownloads: 0,
				totalUsers: 0
			};
			statsLoading = false;
		}
	});
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
</div>