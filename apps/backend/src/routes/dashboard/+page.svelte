<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';

	onMount(() => {
		// Redirect to packages page (dashboard is now packages)
		goto('/packages', { replaceState: true });
	});
</script>

<svelte:head>
	<title>Redirecting - Knot Space</title>
</svelte:head>

<div class="flex items-center justify-center min-h-screen">
	<div class="text-center">
		<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary mx-auto mb-4"></div>
		<p class="text-muted-foreground">Redirecting to packages...</p>
	</div>
</div>