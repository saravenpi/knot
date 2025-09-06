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
	<meta name="description" content="Learn how to manage your Knot projects, from initialization to configuration and status checks." />
	<meta property="og:title" content="Project Management - Knot CLI" />
	<meta property="og:description" content="Learn how to manage your Knot projects, from initialization to configuration and status checks." />
	<meta property="og:image" content="/images/og/project-management.png" />
	<meta property="og:url" content="https://knot.klysium.com/docs/project-management" />
	<meta name="twitter:title" content="Project Management - Knot CLI" />
	<meta name="twitter:description" content="Learn how to manage your Knot projects, from initialization to configuration and status checks." />
	<meta name="twitter:image" content="/images/og/project-management.png" />
	<link rel="canonical" href="https://knot.klysium.com/docs/project-management" />
</svelte:head>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6 overflow-x-hidden">
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight mb-4" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
			Project Management
		</h1>
		<p class="text-lg text-muted-foreground">
			Efficiently manage your monorepo with Knot's project management commands
		</p>
	</div>

	<!-- Overview -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Overview</h2>
		<p class="text-muted-foreground leading-relaxed mb-6">
			Knot provides powerful project management capabilities to help you initialize, configure, and maintain monorepos at scale.
			From project creation to status monitoring, Knot streamlines every aspect of your development workflow.
		</p>
		
		<div class="grid grid-cols-1 md:grid-cols-3 gap-6">
			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-blue-100 rounded-lg flex items-center justify-center mb-4">
					<Icon icon="solar:folder-bold" class="w-6 h-6 text-blue-600" />
				</div>
				<h3 class="font-semibold mb-2">Quick Initialization</h3>
				<p class="text-sm text-muted-foreground">
					Set up new projects instantly with intelligent defaults and best practices built-in.
				</p>
			</div>
			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-green-100 rounded-lg flex items-center justify-center mb-4">
					<Icon icon="solar:settings-bold" class="w-6 h-6 text-green-600" />
				</div>
				<h3 class="font-semibold mb-2">Flexible Configuration</h3>
				<p class="text-sm text-muted-foreground">
					Customize project structure, scripts, and dependencies through simple YAML configuration.
				</p>
			</div>
			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-purple-100 rounded-lg flex items-center justify-center mb-4">
					<Icon icon="solar:chart-bold" class="w-6 h-6 text-purple-600" />
				</div>
				<h3 class="font-semibold mb-2">Real-time Status</h3>
				<p class="text-sm text-muted-foreground">
					Monitor project health, dependencies, and package statuses with comprehensive reporting.
				</p>
			</div>
		</div>
	</section>

	<!-- Initializing a Project -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Initializing a Project</h2>
		<div class="bg-blue-50 border border-blue-200 rounded-lg p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="solar:rocket-2-bold" class="w-6 h-6 text-blue-600 mt-1 flex-shrink-0" />
				<div class="flex-1 min-w-0">
					<h3 class="font-semibold text-blue-900 mb-2">Create a new project</h3>
					<p class="text-sm text-blue-700 mb-4">
						Start a new monorepo with a single command. Knot will set up the basic structure for you, including the root knot.yml configuration file.
					</p>
					<div class="bg-black/90 text-green-400 font-mono text-sm rounded-lg relative group w-full max-w-full">
						<div class="overflow-x-auto overflow-y-hidden p-4 pr-12 max-w-full">
							<pre class="whitespace-nowrap m-0 min-w-max"><code>knot init &lt;project-name&gt;</code></pre>
						</div>
						<button
							class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100 z-10"
							on:click={() => copyToClipboard('knot init <project-name>')}
							title="Copy to clipboard"
						>
							{#if showCopied && copyText === 'knot init <project-name>'}
								<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
							{:else}
								<Icon icon="solar:copy-bold" class="w-4 h-4" />
							{/if}
						</button>
					</div>
					<div class="mt-2 text-sm text-blue-700">
						This creates the project directory, basic configuration, and initializes git.
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Project Configuration -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Project Configuration</h2>
		
		<div class="space-y-6">
			<p class="text-muted-foreground">
				The <code>knot.yml</code> file is the heart of your Knot project. It defines the project's name, description, and any scripts that can be run from the root.
			</p>

			<div>
				<h3 class="text-lg font-semibold mb-3">knot.yml Example</h3>
				<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg relative group">
					<pre><code><span class="text-gray-400"># knot.yml</span>
<span class="text-blue-400">name:</span> <span class="text-green-400">my-awesome-project</span>
<span class="text-blue-400">description:</span> <span class="text-green-400">"A modern monorepo for TypeScript applications"</span>
<span class="text-blue-400">version:</span> <span class="text-green-400">1.0.0</span>

<span class="text-gray-400"># Global scripts</span>
<span class="text-blue-400">scripts:</span>
  <span class="text-blue-400">setup:</span> <span class="text-green-400">"npm install && knot link"</span>
  <span class="text-blue-400">test-all:</span> <span class="text-green-400">"knot run test --all"</span>
  <span class="text-blue-400">build-all:</span> <span class="text-green-400">"knot run build --all"</span>
  <span class="text-blue-400">lint-all:</span> <span class="text-green-400">"knot run lint --all"</span>

<span class="text-gray-400"># App configurations</span>
<span class="text-blue-400">apps:</span>
  <span class="text-blue-400">frontend:</span>
    <span class="text-blue-400">tsAlias:</span> <span class="text-green-400">"@"</span>
    <span class="text-blue-400">packages:</span>
      - <span class="text-yellow-400">types</span>
      - <span class="text-yellow-400">utils</span>
  <span class="text-blue-400">api:</span>
    <span class="text-blue-400">tsAlias:</span> <span class="text-green-400">"#"</span>
    <span class="text-blue-400">packages:</span>
      - <span class="text-yellow-400">types</span>
      - <span class="text-yellow-400">utils</span></code></pre>
					<button 
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
						on:click={() => copyToClipboard(document.querySelector('section:nth-of-type(3) pre code').textContent)}
					>
						{#if showCopied}
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="solar:copy-bold" class="w-4 h-4" />
						{/if}
					</button>
				</div>
			</div>
		</div>
	</section>

	<!-- Checking Project Status -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Checking Project Status</h2>
		<div class="bg-purple-50 border border-purple-200 rounded-lg p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="solar:chart-bold" class="w-6 h-6 text-purple-600 mt-1 flex-shrink-0" />
				<div class="flex-1 min-w-0">
					<h3 class="font-semibold text-purple-900 mb-2">Project Status Overview</h3>
					<p class="text-sm text-purple-700 mb-4">
						Get a comprehensive overview of your project's status, including all apps, packages, and their dependencies.
					</p>
					<div class="bg-black/90 text-green-400 font-mono text-sm rounded-lg relative group w-full max-w-full">
						<div class="overflow-x-auto overflow-y-hidden p-4 pr-12 max-w-full">
							<pre class="whitespace-nowrap m-0 min-w-max"><code>knot status</code></pre>
						</div>
						<button
							class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100 z-10"
							on:click={() => copyToClipboard('knot status')}
						>
							{#if showCopied && copyText === 'knot status'}
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

	<!-- Advanced Commands -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Advanced Project Management</h2>
		
		<div class="space-y-6">
			<p class="text-muted-foreground">
				Beyond basic initialization and status checking, Knot provides advanced features for managing complex monorepos.
			</p>

			<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
				<div class="border rounded-lg p-6">
					<div class="flex items-center space-x-3 mb-3">
						<Icon icon="solar:link-bold" class="w-6 h-6 text-green-600" />
						<h3 class="font-semibold">Package Linking</h3>
					</div>
					<p class="text-sm text-muted-foreground mb-3">
						Link packages to apps for development workflow.
					</p>
					<div class="bg-black/90 text-green-400 font-mono text-sm p-2 rounded">
						<code>knot link</code>
					</div>
				</div>

				<div class="border rounded-lg p-6">
					<div class="flex items-center space-x-3 mb-3">
						<Icon icon="solar:play-bold" class="w-6 h-6 text-blue-600" />
						<h3 class="font-semibold">Run Scripts</h3>
					</div>
					<p class="text-sm text-muted-foreground mb-3">
						Execute scripts across all packages and apps.
					</p>
					<div class="bg-black/90 text-green-400 font-mono text-sm p-2 rounded">
						<code>knot run build --all</code>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Best Practices -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Best Practices</h2>
		
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="solar:check-circle-bold" class="w-5 h-5 mr-2 text-green-600" />
					Project Organization
				</h3>
				<ul class="text-sm text-muted-foreground space-y-2">
					<li>• Use descriptive project and package names</li>
					<li>• Follow semantic versioning for packages</li>
					<li>• Keep related packages grouped logically</li>
					<li>• Document complex project configurations</li>
					<li>• Regularly audit and clean unused dependencies</li>
				</ul>
			</div>

			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="solar:shield-warning-bold" class="w-5 h-5 mr-2 text-yellow-600" />
					Common Pitfalls
				</h3>
				<ul class="text-sm text-muted-foreground space-y-2">
					<li>• Avoid circular dependencies between packages</li>
					<li>• Don't skip running <code>knot link</code> after configuration changes</li>
					<li>• Keep package versions synchronized in apps</li>
					<li>• Don't hardcode environment-specific values in knot.yml</li>
				</ul>
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
					<h3 class="font-semibold text-green-900 mb-2">Ready to manage your first project?</h3>
					<p class="text-sm text-green-700 mb-4">
						Now that you understand project management basics, explore related topics to master Knot's capabilities.
					</p>
					<div class="flex flex-wrap gap-3">
						<a href="/docs/quick-start" class="inline-flex items-center px-4 py-2 bg-green-600 text-white text-sm font-medium rounded-md hover:bg-green-700 transition-colors">
							<Icon icon="solar:play-bold" class="w-4 h-4 mr-2" />
							Quick Start Guide
						</a>
						<a href="/docs/configuration" class="inline-flex items-center px-4 py-2 border border-green-600 text-green-600 text-sm font-medium rounded-md hover:bg-green-50 transition-colors">
							<Icon icon="solar:settings-bold" class="w-4 h-4 mr-2" />
							Configuration Guide
						</a>
					</div>
				</div>
			</div>
		</div>

		<div class="grid grid-cols-1 md:grid-cols-2 gap-6 mt-6">
			<a href="/docs/cli-commands" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:terminal-bold" class="w-6 h-6 text-blue-600" />
					<h3 class="font-semibold">CLI Commands Reference</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Complete command reference for all Knot CLI operations and advanced usage patterns.
				</p>
			</a>

			<a href="/docs/package-linking" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:link-bold" class="w-6 h-6 text-purple-600" />
					<h3 class="font-semibold">Package Linking</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Learn how Knot links packages and manages dependencies between apps and packages.
				</p>
			</a>
		</div>
	</section>
</div>