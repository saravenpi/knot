<script lang="ts">
	import Icon from '@iconify/svelte';
</script>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight mb-4" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
			Documentation Coming Soon
		</h1>
		<p class="text-lg text-muted-foreground">
			This documentation page is currently being developed and will be available soon.
		</p>
	</div>

	<div class="bg-blue-50 border border-blue-200 rounded-lg p-6">
		<div class="flex items-start space-x-3">
			<Icon icon="solar:construction-bold" class="w-6 h-6 text-blue-600 mt-1 flex-shrink-0" />
			<div>
				<h3 class="font-semibold text-blue-900 mb-2">Under Construction</h3>
				<p class="text-sm text-blue-700 mb-4">
					We're working hard to provide comprehensive documentation for this topic. 
					Please check back soon for detailed guides and examples.
				</p>
				<div class="flex flex-wrap gap-3">
					<a href="/docs" class="inline-flex items-center px-4 py-2 bg-blue-600 text-white text-sm font-medium rounded-md hover:bg-blue-700 transition-colors">
						<Icon icon="solar:home-bold" class="w-4 h-4 mr-2" />
						Back to Documentation
					</a>
					<a href="/docs/quick-start" class="inline-flex items-center px-4 py-2 border border-blue-600 text-blue-600 text-sm font-medium rounded-md hover:bg-blue-50 transition-colors">
						<Icon icon="solar:play-bold" class="w-4 h-4 mr-2" />
						Quick Start Guide
					</a>
				</div>
			</div>
		</div>
	</div>
</div>
