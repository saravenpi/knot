<script lang="ts">
	import { cn } from "$lib/utils";

	type $$Props = {
		class?: string;
	};

	let className: $$Props["class"] = undefined;
	export { className as class };
</script>

<div class={cn("grid gap-1.5 p-4 text-center sm:text-left", className)} {...$$restProps}>
	<slot />
</div>