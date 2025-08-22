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
	<title>Package Development - Knot CLI Documentation</title>
	<meta name="description" content="Learn how to develop packages for your Knot projects." />
	<meta property="og:title" content="Package Development - Knot CLI" />
	<meta property="og:description" content="Learn how to develop packages for your Knot projects." />
	<meta property="og:image" content="/images/og/package-development.png" />
	<meta property="og:url" content="https://knot-space.com/docs/package-development" />
	<meta name="twitter:title" content="Package Development - Knot CLI" />
	<meta name="twitter:description" content="Learn how to develop packages for your Knot projects." />
	<meta name="twitter:image" content="/images/og/package-development.png" />
	<link rel="canonical" href="https://knot-space.com/docs/package-development" />
</svelte:head>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight mb-4" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
			Package Development
		</h1>
		<p class="text-lg text-muted-foreground">
			Develop and manage your packages with ease
		</p>
	</div>

	<!-- Creating a Package -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Creating a Package</h2>
		<div class="bg-blue-50 border border-blue-200 rounded-lg p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="solar:box-add-bold" class="w-6 h-6 text-blue-600 mt-1 flex-shrink-0" />
				<div class="flex-1 min-w-0">
					<h3 class="font-semibold text-blue-900 mb-2">Initialize a new package</h3>
					<p class="text-sm text-blue-700 mb-4">
						The `knot init:package` command will create a new package in your project.
					</p>
					<div class="bg-black/90 text-green-400 font-mono text-sm rounded-lg relative group w-full max-w-full">
						<div class="overflow-x-auto overflow-y-hidden p-4 pr-12 max-w-full">
							<pre class="whitespace-nowrap m-0 min-w-max"><code>knot init:package my-package --team my-team</code></pre>
						</div>
						<button
							class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100 z-10"
							on:click={() => copyToClipboard('knot init:package my-package --team my-team')}
							title="Copy to clipboard"
						>
							{#if showCopied && copyText === 'knot init:package my-package --team my-team'}
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

	<!-- Package Configuration -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Package Configuration</h2>
		<div class="bg-purple-50 border border-purple-200 rounded-lg p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="solar:settings-bold" class="w-6 h-6 text-purple-600 mt-1 flex-shrink-0" />
				<div class="flex-1 min-w-0">
					<h3 class="font-semibold text-purple-900 mb-2">The `package.yml` file</h3>
					<p class="text-sm text-purple-700 mb-4">
						The `package.yml` file contains the configuration for your package.
					</p>
					<pre class="text-sm font-mono bg-black/90 text-white p-4 rounded-lg"><code>name: my-package
team: my-team
version: 1.0.0

scripts:
  build: "tsc"
  test: "jest"</code></pre>
				</div>
			</div>
		</div>
	</section>


	<!-- Publishing a Package -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Publishing a Package</h2>
		<div class="bg-green-50 border border-green-200 rounded-lg p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="solar:upload-bold" class="w-6 h-6 text-green-600 mt-1 flex-shrink-0" />
				<div class="flex-1 min-w-0">
					<h3 class="font-semibold text-green-900 mb-2">Publish your package to Knot Space</h3>
					<p class="text-sm text-green-700 mb-4">
						The `knot publish` command will publish your package to the Knot Space registry.
					</p>
					<div class="bg-black/90 text-green-400 font-mono text-sm rounded-lg relative group w-full max-w-full">
						<div class="overflow-x-auto overflow-y-hidden p-4 pr-12 max-w-full">
							<pre class="whitespace-nowrap m-0 min-w-max"><code>knot publish my-package</code></pre>
						</div>
						<button
							class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100 z-10"
							on:click={() => copyToClipboard('knot publish my-package')}
							title="Copy to clipboard"
						>
							{#if showCopied && copyText === 'knot publish my-package'}
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

	<!-- Next Steps -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Next Steps</h2>
		<div class="bg-gray-50 border border-gray-200 rounded-lg p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="solar:check-circle-bold" class="w-6 h-6 text-gray-600 mt-1 flex-shrink-0" />
				<div>
					<h3 class="font-semibold text-gray-900 mb-2">Ready to build amazing packages?</h3>
					<p class="text-sm text-gray-700 mb-4">
						Now that you know how to develop and publish packages, let's explore more advanced topics.
					</p>
					<div class="flex flex-wrap gap-3">
						<a href="/docs/publishing" class="inline-flex items-center px-4 py-2 bg-gray-600 text-white text-sm font-medium rounded-md hover:bg-gray-700 transition-colors">
							<Icon icon="solar:upload-bold" class="w-4 h-4 mr-2" />
							Publishing Guide
						</a>
						<a href="/docs/workflows" class="inline-flex items-center px-4 py-2 border border-gray-600 text-gray-600 text-sm font-medium rounded-md hover:bg-gray-50 transition-colors">
							<Icon icon="solar:settings-bold" class="w-4 h-4 mr-2" />
							Workflow Best Practices
						</a>
					</div>
				</div>
			</div>
		</div>
	</section>
</div>