<script lang="ts">
	import { 
		Drawer, 
		DrawerContent, 
		DrawerHeader, 
		DrawerTitle, 
		DrawerDescription,
		DrawerClose
	} from "./drawer";
	import Icon from '@iconify/svelte';

	export let open = false;
	export let title = '';
	export let description = '';
	export let side: 'left' | 'right' | 'bottom' | 'top' = 'right';

	// Handle close event for backward compatibility
	function handleClose() {
		open = false;
	}
</script>

<Drawer bind:open>
	<DrawerContent {side} class="outline-none">
		<DrawerHeader>
			<div class="flex items-center justify-between">
				<div class="space-y-1">
					{#if title}
						<DrawerTitle>{title}</DrawerTitle>
					{/if}
					{#if description}
						<DrawerDescription>{description}</DrawerDescription>
					{/if}
				</div>
				<DrawerClose 
					class="rounded-sm opacity-70 ring-offset-background transition-opacity hover:opacity-100 focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:pointer-events-none"
					on:click={handleClose}
				>
					<Icon icon="solar:close-circle-bold" class="h-4 w-4" />
					<span class="sr-only">Close</span>
				</DrawerClose>
			</div>
		</DrawerHeader>

		<div class="px-4 pb-4 overflow-y-auto flex-1">
			<slot />
		</div>
	</DrawerContent>
</Drawer>