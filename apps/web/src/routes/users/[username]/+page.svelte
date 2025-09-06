<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { usersApi, type User, type Package } from '../../../lib/api';
	import { formatDownloadCount, formatFileSize, formatDate, formatLargeNumber } from '../../../lib/utils/format';
	import Icon from '@iconify/svelte';
	import PackageCard from '../../../lib/components/PackageCard.svelte';

	$: username = $page.params.username;
	
	let userProfile: User | null = null;
	let userPackages: Package[] = [];
	let userStats = { totalPackages: 0, totalDownloads: 0, totalTeams: 0 };
	let loading = true;
	let error = '';
	let packagesLoading = false;
	let offset = 0;
	let hasMore = true;

	onMount(async () => {
		if (username) {
			await loadUserProfile();
		}
	});

	async function loadUserProfile() {
		loading = true;
		error = '';
		
		try {
			// Load user profile and stats in parallel
			const [profile, stats] = await Promise.all([
				usersApi.getUserProfile(decodeURIComponent(username)),
				usersApi.getUserStats(decodeURIComponent(username))
			]);
			
			userProfile = profile;
			userStats = stats;
			
			// Load initial packages
			await loadUserPackages();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load user profile';
			console.error('Failed to load user profile:', err);
		} finally {
			loading = false;
		}
	}

	async function loadUserPackages(reset = true) {
		if (reset) {
			offset = 0;
			userPackages = [];
		}
		
		packagesLoading = true;
		
		try {
			const result = await usersApi.getUserPackages(decodeURIComponent(username), offset, 10);
			
			if (reset) {
				userPackages = result.packages;
			} else {
				userPackages = [...userPackages, ...result.packages];
			}
			
			hasMore = result.pagination.hasMore;
			offset += result.packages.length;
		} catch (err) {
			console.error('Failed to load user packages:', err);
		} finally {
			packagesLoading = false;
		}
	}

	async function loadMorePackages() {
		if (!packagesLoading && hasMore) {
			await loadUserPackages(false);
		}
	}
</script>

<svelte:head>
	<title>{userProfile ? `${userProfile.username} - Knot Space` : 'User Profile - Knot Space'}</title>
	<meta name="description" content={userProfile ? `${userProfile.username}'s profile on Knot Space` : 'User profile on Knot Space'} />
</svelte:head>

{#if loading}
	<div class="flex items-center justify-center py-16">
		<div class="text-center">
			<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-black mx-auto mb-4"></div>
			<p class="text-muted-foreground">Loading profile...</p>
		</div>
	</div>
{:else if error}
	<div class="text-center py-16">
		<div class="text-6xl mb-4">👤</div>
		<h3 class="text-xl font-semibold mb-2">User not found</h3>
		<p class="text-muted-foreground mb-6">
			{error}
		</p>
		<a 
			href="/packages" 
			class="bg-primary text-primary-foreground hover:bg-primary/90 px-6 py-2 rounded-md font-medium transition-colors inline-block"
		>
			← Browse Packages
		</a>
	</div>
{:else if userProfile}
	<!-- Profile content now uses the authenticated layout -->
	<div class="space-y-8">
		
		<!-- Profile Header -->
		<div class="bg-card rounded-xl border shadow-sm p-6 sm:p-8">
			<div class="flex flex-col sm:flex-row items-start sm:items-center gap-6">
				<!-- Avatar -->
				<div class="w-20 h-20 sm:w-24 sm:h-24 bg-gradient-to-br from-primary to-primary/80 rounded-2xl flex items-center justify-center text-primary-foreground text-3xl sm:text-4xl font-bold shadow-lg">
					{userProfile.username.charAt(0).toUpperCase()}
				</div>
				
				<!-- User Info -->
				<div class="flex-1 min-w-0">
					<h1 class="text-3xl sm:text-4xl font-bold text-foreground mb-2 truncate" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
						{userProfile.username}
					</h1>
					<div class="flex flex-wrap gap-4 text-muted-foreground mb-4">
						<span class="flex items-center gap-1">
							<Icon icon="solar:calendar-bold" class="w-4 h-4" />
							Joined {formatDate(userProfile.createdAt)}
						</span>
					</div>
				</div>
			</div>
		</div>

		<!-- Stats Cards -->
		<div class="grid grid-cols-1 sm:grid-cols-3 gap-4 sm:gap-6">
			<div class="bg-card rounded-lg border p-6">
				<div class="flex items-center gap-4">
					<div class="w-12 h-12 bg-blue-500/10 rounded-xl flex items-center justify-center p-3">
						<Icon icon="solar:box-bold" class="w-6 h-6 text-blue-500" />
					</div>
					<div class="flex-1">
						<div class="text-2xl sm:text-3xl font-bold text-foreground">
							{formatLargeNumber(userStats.totalPackages)}
						</div>
						<div class="text-sm text-muted-foreground font-medium">
							Package{userStats.totalPackages === 1 ? '' : 's'}
						</div>
					</div>
				</div>
			</div>
			
			<div class="bg-card rounded-lg border p-6">
				<div class="flex items-center gap-4">
					<div class="w-12 h-12 bg-green-500/10 rounded-xl flex items-center justify-center p-3">
						<Icon icon="solar:download-bold" class="w-6 h-6 text-green-500" />
					</div>
					<div class="flex-1">
						<div class="text-2xl sm:text-3xl font-bold text-foreground">
							{formatLargeNumber(userStats.totalDownloads)}
						</div>
						<div class="text-sm text-muted-foreground font-medium">
							Total Downloads
						</div>
					</div>
				</div>
			</div>
			
			<div class="bg-card rounded-lg border p-6">
				<div class="flex items-center gap-4">
					<div class="w-12 h-12 bg-purple-500/10 rounded-xl flex items-center justify-center p-3">
						<Icon icon="solar:users-group-rounded-bold" class="w-6 h-6 text-purple-500" />
					</div>
					<div class="flex-1">
						<div class="text-2xl sm:text-3xl font-bold text-foreground">
							{formatLargeNumber(userStats.totalTeams)}
						</div>
						<div class="text-sm text-muted-foreground font-medium">
							Team{userStats.totalTeams === 1 ? '' : 's'}
						</div>
					</div>
				</div>
			</div>
		</div>

		<!-- Packages Section -->
		<div class="bg-card rounded-xl border shadow-sm">
			<div class="p-6 sm:p-8 border-b">
				<div class="flex items-center justify-between">
					<h2 class="text-2xl font-bold">Packages</h2>
					<span class="text-sm text-muted-foreground">
						{userStats.totalPackages} package{userStats.totalPackages === 1 ? '' : 's'}
					</span>
				</div>
			</div>
			
			<div class="p-6 sm:p-8">
				{#if userPackages.length === 0 && !packagesLoading}
					<div class="text-center py-12">
						<div class="text-6xl mb-4">📦</div>
						<h3 class="text-xl font-semibold mb-2">No packages yet</h3>
						<p class="text-muted-foreground">
							{userProfile.username} hasn't published any packages yet.
						</p>
					</div>
				{:else}
					<div class="space-y-4">
						{#each userPackages as pkg}
							<PackageCard {pkg} showOwner={false} showUpdatedTime={false} />
						{/each}
						
						{#if hasMore}
							<div class="text-center pt-6">
								<button
									on:click={loadMorePackages}
									disabled={packagesLoading}
									class="bg-primary text-primary-foreground hover:bg-primary/90 disabled:opacity-50 disabled:cursor-not-allowed px-6 py-2 rounded-md font-medium transition-colors"
								>
									{#if packagesLoading}
										<span class="flex items-center">
											<div class="animate-spin rounded-full h-4 w-4 border-b-2 border-current mr-2"></div>
											Loading...
										</span>
									{:else}
										Load More Packages
									{/if}
								</button>
							</div>
						{/if}
					</div>
				{/if}
			</div>
		</div>
	</div>
{/if}