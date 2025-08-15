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
	<!-- Main container with proper margins and max width -->
	<div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-8 space-y-8">
		
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
					<div class="flex items-center gap-2 text-muted-foreground mb-4">
						<Icon icon="solar:calendar-bold-duotone" class="w-4 h-4" />
						<span class="text-sm sm:text-base">
							Member since {formatDate(userProfile.createdAt)}
						</span>
					</div>
					
					<!-- Additional user info could go here -->
					<div class="flex flex-wrap gap-2">
						<span class="inline-flex items-center gap-1 px-3 py-1 bg-primary/10 text-primary rounded-full text-sm font-medium">
							<Icon icon="solar:user-check-rounded-bold" class="w-3 h-3" />
							Verified Developer
						</span>
					</div>
				</div>
			</div>
		</div>

		<!-- Stats Grid -->
		<div class="grid grid-cols-1 sm:grid-cols-3 gap-4 sm:gap-6">
			<div class="bg-card rounded-lg border p-6 hover:shadow-md transition-all duration-200">
				<div class="flex items-center justify-between mb-4">
					<div class="p-2 bg-blue-100 dark:bg-blue-900/20 rounded-lg">
						<Icon icon="solar:box-minimalistic-bold-duotone" class="w-6 h-6 text-blue-600 dark:text-blue-400" />
					</div>
					<div class="text-right">
						<div class="text-2xl sm:text-3xl font-bold text-foreground">
							{userStats.totalPackages.toLocaleString()}
						</div>
						<div class="text-sm text-muted-foreground">Packages</div>
					</div>
				</div>
				<div class="text-sm text-muted-foreground">
					Total published packages
				</div>
			</div>
			
			<div class="bg-card rounded-lg border p-6 hover:shadow-md transition-all duration-200">
				<div class="flex items-center justify-between mb-4">
					<div class="p-2 bg-green-100 dark:bg-green-900/20 rounded-lg">
						<Icon icon="solar:download-minimalistic-bold-duotone" class="w-6 h-6 text-green-600 dark:text-green-400" />
					</div>
					<div class="text-right">
						<div class="text-2xl sm:text-3xl font-bold text-foreground">
							{userStats.totalDownloads.toLocaleString()}
						</div>
						<div class="text-sm text-muted-foreground">Downloads</div>
					</div>
				</div>
				<div class="text-sm text-muted-foreground">
					Across all packages
				</div>
			</div>

			<div class="bg-card rounded-lg border p-6 hover:shadow-md transition-all duration-200">
				<div class="flex items-center justify-between mb-4">
					<div class="p-2 bg-purple-100 dark:bg-purple-900/20 rounded-lg">
						<Icon icon="solar:users-group-rounded-bold-duotone" class="w-6 h-6 text-purple-600 dark:text-purple-400" />
					</div>
					<div class="text-right">
						<div class="text-2xl sm:text-3xl font-bold text-foreground">
							{userStats.totalTeams}
						</div>
						<div class="text-sm text-muted-foreground">Teams</div>
					</div>
				</div>
				<div class="text-sm text-muted-foreground">
					Team memberships
				</div>
			</div>
		</div>

		<!-- Packages Section -->
		<div class="space-y-6">
			<div class="flex items-center justify-between">
				<div>
					<h2 class="text-2xl sm:text-3xl font-bold text-foreground">Published Packages</h2>
					<p class="text-muted-foreground mt-1">
						Packages published by {userProfile.username}
					</p>
				</div>
			</div>

			{#if userPackages.length === 0 && !packagesLoading}
				<div class="bg-card rounded-xl border text-center py-12 sm:py-16">
					<div class="w-16 h-16 mx-auto mb-4 p-3 bg-muted/50 rounded-full">
						<Icon icon="solar:box-minimalistic-bold-duotone" class="w-full h-full text-muted-foreground" />
					</div>
					<h3 class="text-xl sm:text-2xl font-semibold mb-2 text-foreground">No packages published</h3>
					<p class="text-muted-foreground max-w-md mx-auto">
						{userProfile.username} hasn't published any packages yet. Check back later for updates!
					</p>
				</div>
			{:else}
				<div class="space-y-4">
					{#each userPackages as pkg}
						<div class="bg-card rounded-lg border p-6 hover:shadow-lg hover:border-primary/20 transition-all duration-200">
							<div class="flex flex-col sm:flex-row sm:items-start sm:justify-between gap-4">
								<div class="flex-1 min-w-0">
									<!-- Package header -->
									<div class="flex flex-col sm:flex-row sm:items-center gap-2 mb-3">
										<h3 class="font-semibold text-lg sm:text-xl text-foreground truncate">
											<a 
												href="/packages/{encodeURIComponent(pkg.name)}" 
												class="hover:text-primary transition-colors"
											>
												{pkg.name}
											</a>
										</h3>
										<span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-secondary text-secondary-foreground self-start">
											v{pkg.version}
										</span>
									</div>
									
									<!-- Description -->
									{#if pkg.description}
										<p class="text-muted-foreground mb-4 line-clamp-2 text-sm sm:text-base">
											{pkg.description}
										</p>
									{/if}
									
									<!-- Metadata -->
									<div class="flex flex-wrap items-center gap-4 text-sm text-muted-foreground">
										{#if pkg.team}
											<span class="flex items-center gap-1.5">
												<div class="w-2 h-2 bg-primary rounded-full"></div>
												<span class="font-medium">{pkg.team.name}</span>
											</span>
										{/if}
										<span class="flex items-center gap-1.5">
											<Icon icon="solar:calendar-bold" class="w-3 h-3" />
											{formatDate(pkg.publishedAt || pkg.createdAt)}
										</span>
									</div>
								</div>
								
								<!-- Package stats -->
								<div class="flex sm:flex-col items-center sm:items-end gap-4 sm:gap-2 text-sm">
									<div class="flex items-center gap-1.5 text-muted-foreground">
										<Icon icon="solar:download-minimalistic-bold" class="w-4 h-4" />
										<span class="font-medium">
											{(parseInt(pkg.downloadsCount?.toString() || '0')).toLocaleString()}
										</span>
									</div>
									<div class="text-xs text-muted-foreground">
										{formatFileSize(pkg.fileSize)}
									</div>
								</div>
							</div>
						</div>
					{/each}

					{#if packagesLoading}
						<div class="flex items-center justify-center py-12">
							<div class="flex items-center gap-3">
								<div class="animate-spin rounded-full h-6 w-6 border-2 border-primary border-t-transparent"></div>
								<span class="text-muted-foreground">Loading packages...</span>
							</div>
						</div>
					{/if}

					{#if hasMore && !packagesLoading}
						<div class="text-center pt-6">
							<button
								on:click={() => loadUserPackages(false)}
								class="bg-primary text-primary-foreground hover:bg-primary/90 px-6 py-3 rounded-lg font-medium transition-colors shadow-sm"
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