<script lang="ts">
	import { Drawer as DrawerPrimitive } from "vaul-svelte";
	import { cn } from "$lib/utils";

	type $$Props = DrawerPrimitive.Description.$$Props;

	let className: $$Props["class"] = undefined;
	export { className as class };
</script>

<DrawerPrimitive.Description
	class={cn("text-sm text-muted-foreground", className)}
	{...$$restProps}
>
	<slot />
</DrawerPrimitive.Description>