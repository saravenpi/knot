<script lang="ts">
	import { cn } from "$lib/utils";

	type $$Props = {
		class?: string;
	};

	let className: $$Props["class"] = undefined;
	export { className as class };
</script>

<div class={cn("mt-auto flex flex-col gap-2 p-4", className)} {...$$restProps}>
	<slot />
</div>