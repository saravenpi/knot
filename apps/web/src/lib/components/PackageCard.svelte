<script lang="ts">
	import { goto } from '$app/navigation';
	import { formatDownloadCount, formatFileSize, formatDate, formatTimeAgo } from '../utils/format';
	import type { Package } from '../api';

	export let pkg: Package;
	export let showOwner: boolean = true;
	export let showUpdatedTime: boolean = true;
</script>

<a 
	href="/packages/{encodeURIComponent(pkg.name)}"
	class="block border rounded-lg p-4 sm:p-6 hover:shadow-md transition-all duration-200 hover:border-primary/50 cursor-pointer"
>
	<!-- Header with title and stats -->
	<div class="flex items-start justify-between gap-4 mb-4">
		<div class="flex-1 min-w-0">
			<div class="flex flex-col sm:flex-row sm:items-center gap-2 mb-2">
				<h3 class="font-semibold text-lg truncate hover:text-primary transition-colors">
					{pkg.name}
				</h3>
				<span class="text-sm text-muted-foreground bg-secondary px-2 py-1 rounded flex-shrink-0 w-fit">
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
						<span class="px-2 py-1 text-xs bg-secondary text-secondary-foreground rounded-full border">
							{tag}
						</span>
					{/each}
				</div>
			{/if}
		</div>
		
		<!-- Right side stats - visible on desktop -->
		<div class="hidden sm:flex flex-col items-end gap-2 text-sm text-muted-foreground flex-shrink-0">
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

	<!-- Bottom metadata section - improved responsive layout -->
	<div class="border-t border-border pt-3 mt-3">
		<!-- Desktop layout -->
		<div class="hidden sm:flex items-center justify-between text-sm text-muted-foreground">
			<div class="flex items-center gap-4">
				{#if showOwner && pkg.owner}
					<span>by 
						<button 
							class="hover:text-primary transition-colors font-medium"
							on:click={(e) => {
								e.stopPropagation();
								e.preventDefault();
								goto(`/users/${encodeURIComponent(pkg.owner.username)}`);
							}}
						>{pkg.owner.username}</button>
					</span>
				{/if}
				{#if pkg.team}
					<span class="flex items-center gap-1">
						<span class="w-2 h-2 bg-primary rounded-full"></span>
						{pkg.team.name}
					</span>
				{/if}
				<span>{formatDate(pkg.publishedAt || pkg.createdAt)}</span>
			</div>
			
			<div class="flex items-center gap-3 text-xs">
				{#if showUpdatedTime && pkg.updatedAt && pkg.updatedAt !== pkg.createdAt}
					<span class="flex items-center gap-1">
						<svg class="h-3 w-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
						</svg>
						Updated {formatTimeAgo(pkg.updatedAt)}
					</span>
				{/if}
			</div>
		</div>

		<!-- Mobile layout -->
		<div class="sm:hidden space-y-2">
			<div class="flex flex-wrap items-center gap-x-3 gap-y-1 text-sm text-muted-foreground">
				{#if showOwner && pkg.owner}
					<span>by 
						<button 
							class="hover:text-primary transition-colors font-medium"
							on:click={(e) => {
								e.stopPropagation();
								e.preventDefault();
								goto(`/users/${encodeURIComponent(pkg.owner.username)}`);
							}}
						>{pkg.owner.username}</button>
					</span>
				{/if}
				{#if pkg.team}
					<span class="flex items-center gap-1">
						<span class="w-2 h-2 bg-primary rounded-full"></span>
						{pkg.team.name}
					</span>
				{/if}
				<span>{formatDate(pkg.publishedAt || pkg.createdAt)}</span>
			</div>
			
			<div class="flex items-center justify-between text-xs text-muted-foreground">
				{#if showUpdatedTime && pkg.updatedAt && pkg.updatedAt !== pkg.createdAt}
					<span class="flex items-center gap-1">
						<svg class="h-3 w-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
						</svg>
						Updated {formatTimeAgo(pkg.updatedAt)}
					</span>
				{:else}
					<div></div>
				{/if}
				<div class="flex items-center gap-3">
					<div class="flex items-center gap-1">
						<svg class="h-3 w-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M9 19l3 3m0 0l3-3m-3 3V10" />
						</svg>
						{formatDownloadCount(pkg.totalDownloadsCount || pkg.downloadsCount)}
					</div>
					<span>{formatFileSize(pkg.fileSize)}</span>
				</div>
			</div>
		</div>
	</div>
</a>