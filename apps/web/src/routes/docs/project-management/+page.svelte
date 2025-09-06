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
	<meta property="og:url" content="https://knot.klysium.com/docs/project-management" />
	<meta name="twitter:title" content="Project Management - Knot CLI" />
	<meta name="twitter:description" content="Learn how to manage your Knot projects, from initialization to configuration and status checks." />
	<link rel="canonical" href="https://knot.klysium.com/docs/project-management" />
</svelte:head>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
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
		
		<div class="space-y-6">
			<p class="text-muted-foreground">
				Start a new monorepo with a single command. Knot will set up the basic structure for you, including the root <code>knot.yml</code> configuration file.
			</p>

			<div>
				<h3 class="text-lg font-semibold mb-3">Create a new project</h3>
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
				<div class="mt-2 text-sm text-muted-foreground">
					This creates the project directory, basic configuration, and initializes git.
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Example Output</h3>
				<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
					<pre><code><span class="text-green-400">$</span> knot init my-app

<span class="text-blue-400">✓</span> Created project directory: my-app/
<span class="text-blue-400">✓</span> Generated knot.yml configuration
<span class="text-blue-400">✓</span> Created packages/ directory
<span class="text-blue-400">✓</span> Created apps/ directory
<span class="text-blue-400">✓</span> Initialized git repository</code></pre>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Generated Structure</h3>
				<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
					<pre><code>my-app/
├── <span class="text-blue-400">knot.yml</span>          # Project configuration
├── <span class="text-yellow-400">packages/</span>        # Reusable packages
├── <span class="text-green-400">apps/</span>             # Applications
├── <span class="text-gray-400">node_modules/</span>     # Dependencies
├── <span class="text-purple-400">.gitignore</span>       # Git ignore rules
└── <span class="text-orange-400">README.md</span>        # Project documentation</code></pre>
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

			<div>
				<h3 class="text-lg font-semibold mb-3">Configuration Options</h3>
				<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
					<div class="border rounded-lg p-4">
						<h4 class="font-semibold mb-2">Project Metadata</h4>
						<div class="text-sm space-y-1">
							<div><code class="bg-muted px-2 py-1 rounded">name</code> - Project name (required)</div>
							<div><code class="bg-muted px-2 py-1 rounded">description</code> - Project description</div>
							<div><code class="bg-muted px-2 py-1 rounded">version</code> - Project version</div>
						</div>
					</div>

					<div class="border rounded-lg p-4">
						<h4 class="font-semibold mb-2">Global Scripts</h4>
						<div class="text-sm space-y-1">
							<div><code class="bg-muted px-2 py-1 rounded">setup</code> - Initialize project</div>
							<div><code class="bg-muted px-2 py-1 rounded">test-all</code> - Run all tests</div>
							<div><code class="bg-muted px-2 py-1 rounded">build-all</code> - Build all apps</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Checking Project Status -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Checking Project Status</h2>
		
		<div class="space-y-6">
			<p class="text-muted-foreground">
				Get a quick overview of your project's status, including a list of all apps and packages and their dependencies.
			</p>

			<div>
				<h3 class="text-lg font-semibold mb-3">Project status command</h3>
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

			<div>
				<h3 class="text-lg font-semibold mb-3">Example Status Output</h3>
				<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
					<pre><code><span class="text-blue-400">📦 Project:</span> <span class="text-white">my-awesome-project</span> <span class="text-gray-400">(v1.0.0)</span>
<span class="text-green-400">✓</span> <span class="text-gray-400">Configuration valid</span>
<span class="text-green-400">✓</span> <span class="text-gray-400">Git repository initialized</span>

<span class="text-yellow-400">📱 Apps (2):</span>
  <span class="text-blue-400">frontend</span> <span class="text-gray-400">- React application</span>
    <span class="text-green-400">✓</span> <span class="text-gray-400">Linked: types, utils</span>
  
  <span class="text-blue-400">api</span> <span class="text-gray-400">- Node.js API server</span>
    <span class="text-green-400">✓</span> <span class="text-gray-400">Linked: types, utils</span>

<span class="text-purple-400">📚 Packages (2):</span>
  <span class="text-blue-400">types</span> <span class="text-gray-400">(v1.0.0) - Shared TypeScript types</span>
  <span class="text-blue-400">utils</span> <span class="text-gray-400">(v1.2.0) - Utility functions</span></code></pre>
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
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<a href="/docs/quick-start" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:play-bold" class="w-6 h-6 text-green-600" />
					<h3 class="font-semibold">Quick Start Guide</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Get started with your first Knot project and learn the basic workflow.
				</p>
			</a>

			<a href="/docs/configuration" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:settings-bold" class="w-6 h-6 text-blue-600" />
					<h3 class="font-semibold">Configuration Files</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Learn about all configuration options for projects, packages, and apps.
				</p>
			</a>

			<a href="/docs/package-linking" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:link-bold" class="w-6 h-6 text-purple-600" />
					<h3 class="font-semibold">Package Linking</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Understand how Knot links packages and the different linking modes available.
				</p>
			</a>

			<a href="/docs/cli-commands" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:terminal-bold" class="w-6 h-6 text-orange-600" />
					<h3 class="font-semibold">CLI Commands Reference</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Complete command reference for all Knot CLI operations.
				</p>
			</a>
		</div>
	</section>
</div>