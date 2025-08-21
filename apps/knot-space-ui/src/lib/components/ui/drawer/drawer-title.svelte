<script lang="ts">
	import { Drawer as DrawerPrimitive } from "vaul-svelte";
	import { cn } from "$lib/utils";

	type $$Props = DrawerPrimitive.Title.$$Props;

	let className: $$Props["class"] = undefined;
	export { className as class };
</script>

<DrawerPrimitive.Title
	class={cn("text-lg font-semibold leading-none tracking-tight", className)}
	{...$$restProps}
>
	<slot />
</DrawerPrimitive.Title>