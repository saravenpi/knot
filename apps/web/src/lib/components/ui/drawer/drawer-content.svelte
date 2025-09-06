<script lang="ts">
	import { Drawer as DrawerPrimitive } from "vaul-svelte";
	import { cn } from "$lib/utils";

	type $$Props = DrawerPrimitive.Content.$$Props;

	let className: $$Props["class"] = undefined;
	export { className as class };

	export let side: "top" | "bottom" | "left" | "right" = "bottom";
</script>

<DrawerPrimitive.Portal>
	<DrawerPrimitive.Overlay
		class="fixed inset-0 z-50 bg-black/80"
	/>
	<DrawerPrimitive.Content
		class={cn(
			"fixed z-50 gap-4 bg-background p-6 shadow-lg transition ease-in-out data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:duration-300 data-[state=open]:duration-500",
			{
				"inset-x-0 bottom-0 border-t data-[state=closed]:slide-out-to-bottom data-[state=open]:slide-in-from-bottom": side === "bottom",
				"inset-x-0 top-0 border-b data-[state=closed]:slide-out-to-top data-[state=open]:slide-in-from-top": side === "top",
				"inset-y-0 left-0 h-full w-3/4 border-r data-[state=closed]:slide-out-to-left data-[state=open]:slide-in-from-left sm:max-w-sm": side === "left",
				"inset-y-0 right-0 h-full w-3/4 border-l data-[state=closed]:slide-out-to-right data-[state=open]:slide-in-from-right sm:max-w-sm": side === "right"
			},
			className
		)}
		{...$$restProps}
	>
		{#if side === "bottom"}
			<div class="mx-auto mt-4 h-2 w-[100px] rounded-full bg-muted"></div>
		{/if}
		<slot />
	</DrawerPrimitive.Content>
</DrawerPrimitive.Portal>