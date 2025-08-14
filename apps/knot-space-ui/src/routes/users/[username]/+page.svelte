<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { usersApi, type User, type Package } from '../../../lib/api';
	import Icon from '@iconify/svelte';

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

	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric'
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
	<title>{username ? `${username} - Profile` : 'User Profile'} - Knot Space</title>
	<meta name="description" content="View user profile and published packages" />
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
	<div class="space-y-6">
		<!-- Header -->
		<div class="flex items-start justify-between">
			<div class="flex-1">
				<div class="flex items-center gap-4 mb-4">
					<div class="w-16 h-16 bg-primary rounded-full flex items-center justify-center text-primary-foreground text-2xl font-bold">
						{userProfile.username.charAt(0).toUpperCase()}
					</div>
					<div>
						<h1 class="text-3xl font-bold">{userProfile.username}</h1>
						<p class="text-muted-foreground">
							Member since {formatDate(userProfile.createdAt)}
						</p>
					</div>
				</div>
			</div>
		</div>

		<!-- Stats -->
		<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
			<div class="border rounded-lg p-4">
				<div class="flex items-center gap-2 mb-2">
					<Icon icon="solar:box-minimalistic-bold" class="w-5 h-5 text-muted-foreground" />
					<span class="font-medium">Packages</span>
				</div>
				<div class="text-2xl font-bold">{userStats.totalPackages.toLocaleString()}</div>
			</div>
			
			<div class="border rounded-lg p-4">
				<div class="flex items-center gap-2 mb-2">
					<Icon icon="solar:download-minimalistic-bold" class="w-5 h-5 text-muted-foreground" />
					<span class="font-medium">Total Downloads</span>
				</div>
				<div class="text-2xl font-bold">{userStats.totalDownloads.toLocaleString()}</div>
			</div>

			<div class="border rounded-lg p-4">
				<div class="flex items-center gap-2 mb-2">
					<Icon icon="solar:users-group-rounded-bold" class="w-5 h-5 text-muted-foreground" />
					<span class="font-medium">Teams</span>
				</div>
				<div class="text-2xl font-bold">{userStats.totalTeams}</div>
			</div>
		</div>

		<!-- Packages Section -->
		<div class="space-y-4">
			<div class="flex items-center justify-between">
				<h2 class="text-2xl font-semibold">Published Packages</h2>
			</div>

			{#if userPackages.length === 0 && !packagesLoading}
				<div class="text-center py-12">
					<div class="text-6xl mb-4">📦</div>
					<h3 class="text-xl font-semibold mb-2">No packages published</h3>
					<p class="text-muted-foreground">
						{userProfile.username} hasn't published any packages yet.
					</p>
				</div>
			{:else}
				<div class="space-y-4">
					{#each userPackages as pkg}
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
										{#if pkg.team}
											<span class="flex items-center gap-1">
												<span class="w-2 h-2 bg-primary rounded-full"></span>
												{pkg.team.name}
											</span>
										{/if}
										<span>{formatDate(pkg.publishedAt || pkg.createdAt)}</span>
									</div>
								</div>
								
								<div class="flex flex-col items-end gap-2 text-sm text-muted-foreground ml-4">
									<div class="flex items-center gap-1">
										<Icon icon="solar:download-minimalistic-bold" class="h-4 w-4" />
										{(parseInt(pkg.downloadsCount?.toString() || '0')).toLocaleString()}
									</div>
									<div class="text-xs">
										{formatFileSize(pkg.fileSize)}
									</div>
								</div>
							</div>
						</div>
					{/each}

					{#if packagesLoading}
						<div class="flex items-center justify-center py-8">
							<div class="animate-spin rounded-full h-6 w-6 border-b-2 border-current"></div>
						</div>
					{/if}

					{#if hasMore && !packagesLoading}
						<div class="text-center pt-4">
							<button
								on:click={() => loadUserPackages(false)}
								class="bg-secondary text-secondary-foreground hover:bg-secondary/80 px-6 py-2 rounded-md font-medium transition-colors"
							>
								Load More Packages
							</button>
						</div>
					{/if}
				</div>
			{/if}
		</div>
	</div>
{/if}