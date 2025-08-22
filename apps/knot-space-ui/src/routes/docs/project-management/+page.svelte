
<script lang="ts">
	import Icon from '@iconify/svelte';

	let copyText = '';
	let showCopied = false;

	async function copyToClipboard(text: string) {
		try {
			await navigator.clipboard.writeText(text);
			copyText = text;
			showCopied = true;
			setTimeout(() => {
				showCopied = false;
			}, 2000);
		} catch (err) {
			console.error('Failed to copy text: ', err);
		}
	}
</script>

<svelte:head>
	<title>Project Management - Knot CLI Documentation</title>
	<meta name="description" content="Learn how to manage your Knot projects." />
	<meta property="og:title" content="Project Management - Knot CLI" />
	<meta property="og:description" content="Learn how to manage your Knot projects." />
	<meta property="og:image" content="/images/og/project-management.png" />
	<meta property="og:url" content="https://knot-space.com/docs/project-management" />
	<meta name="twitter:title" content="Project Management - Knot CLI" />
	<meta name="twitter:description" content="Learn how to manage your Knot projects." />
	<meta name="twitter:image" content="/images/og/project-management.png" />
	<link rel="canonical" href="https://knot-space.com/docs/project-management" />
</svelte:head>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight mb-4" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
			Project Management
		</h1>
		<p class="text-lg text-muted-foreground">
			Manage your projects with ease
		</p>
	</div>

	<!-- Initializing a Project -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Initializing a Project</h2>
		<div class="bg-blue-50 border border-blue-200 rounded-lg p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="solar:folder-with-files-add-bold" class="w-6 h-6 text-blue-600 mt-1 flex-shrink-0" />
				<div class="flex-1 min-w-0">
					<h3 class="font-semibold text-blue-900 mb-2">Initialize a new project</h3>
					<p class="text-sm text-blue-700 mb-4">
						The `knot init` command will create a new project.
					</p>
					<div class="bg-black/90 text-green-400 font-mono text-sm rounded-lg relative group w-full max-w-full">
						<div class="overflow-x-auto overflow-y-hidden p-4 pr-12 max-w-full">
							<pre class="whitespace-nowrap m-0 min-w-max"><code>knot init my-project</code></pre>
						</div>
						<button
							class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100 z-10"
							on:click={() => copyToClipboard('knot init my-project')}
							title="Copy to clipboard"
						>
							{#if showCopied && copyText === 'knot init my-project'}
								<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
							{:else}
								<Icon icon="solar:copy-bold" class="w-4 h-4" />
							{/if}
						</button>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Project Structure -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Project Structure</h2>
		<div class="bg-purple-50 border border-purple-200 rounded-lg p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="solar:folder-with-files-bold" class="w-6 h-6 text-purple-600 mt-1 flex-shrink-0" />
				<div class="flex-1 min-w-0">
					<h3 class="font-semibold text-purple-900 mb-2">Understanding the project structure</h3>
					<p class="text-sm text-purple-700 mb-4">
						Knot projects have a simple and intuitive directory structure.
					</p>
					<pre class="text-sm font-mono"><code>my-project/
├── knot.yml
├── apps/
└── packages/</code></pre>
				</div>
			</div>
		</div>
	</section>

	<!-- Next Steps -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Next Steps</h2>
		<div class="bg-green-50 border border-green-200 rounded-lg p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="solar:check-circle-bold" class="w-6 h-6 text-green-600 mt-1 flex-shrink-0" />
				<div>
					<h3 class="font-semibold text-green-900 mb-2">Ready to start your next project?</h3>
					<p class="text-sm text-green-700 mb-4">
						Now that you know how to manage your projects, let's explore more advanced topics.
					</p>
					<div class="flex flex-wrap gap-3">
						<a href="/docs/package-development" class="inline-flex items-center px-4 py-2 bg-green-600 text-white text-sm font-medium rounded-md hover:bg-green-700 transition-colors">
							<Icon icon="solar:box-bold" class="w-4 h-4 mr-2" />
							Package Development
						</a>
						<a href="/docs/workflows" class="inline-flex items-center px-4 py-2 border border-green-600 text-green-600 text-sm font-medium rounded-md hover:bg-green-50 transition-colors">
							<Icon icon="solar:settings-bold" class="w-4 h-4 mr-2" />
							Workflow Best Practices
						</a>
					</div>
				</div>
			</div>
		</div>
	</section>
</div>
