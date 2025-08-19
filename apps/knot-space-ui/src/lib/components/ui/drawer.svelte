<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { fade, fly } from 'svelte/transition';
	import Icon from '@iconify/svelte';

	export let open = false;
	export let title = '';
	export let description = '';
	export let side: 'left' | 'right' | 'bottom' = 'right';

	const dispatch = createEventDispatcher();

	function close() {
		open = false;
		dispatch('close');
	}

	function handleBackdropClick(event: MouseEvent) {
		if (event.target === event.currentTarget) {
			close();
		}
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Escape') {
			close();
		}
	}

	$: if (open) {
		document.body.style.overflow = 'hidden';
	} else {
		document.body.style.overflow = '';
	}
</script>

<svelte:window on:keydown={handleKeydown} />

{#if open}
	<div
		class="fixed inset-0 z-50 flex"
		transition:fade={{ duration: 200 }}
		on:click={handleBackdropClick}
		role="dialog"
		aria-modal="true"
	>
		<!-- Backdrop -->
		<div class="fixed inset-0 bg-black/50"></div>
		
		<!-- Drawer content -->
		<div
			class="relative z-10 {side === 'right' ? 'ml-auto' : side === 'left' ? 'mr-auto' : 'mt-auto'} 
			       {side === 'bottom' ? 'w-full max-h-[80vh]' : 'h-full w-full max-w-md'} 
			       bg-background border-l border-border shadow-lg flex flex-col"
			transition:fly={{
				x: side === 'right' ? 320 : side === 'left' ? -320 : 0,
				y: side === 'bottom' ? 320 : 0,
				duration: 300
			}}
		>
			<!-- Header -->
			<div class="flex items-center justify-between p-6 border-b border-border flex-shrink-0">
				<div class="space-y-1">
					{#if title}
						<h2 class="text-lg font-semibold leading-none tracking-tight">{title}</h2>
					{/if}
					{#if description}
						<p class="text-sm text-muted-foreground">{description}</p>
					{/if}
				</div>
				<button
					on:click={close}
					class="rounded-sm opacity-70 ring-offset-background transition-opacity hover:opacity-100 focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:pointer-events-none"
				>
					<Icon icon="solar:close-circle-bold" class="h-4 w-4" />
					<span class="sr-only">Close</span>
				</button>
			</div>

			<!-- Content with scroll -->
			<div class="p-6 overflow-y-auto flex-1">
				<slot />
			</div>
		</div>
	</div>
{/if}