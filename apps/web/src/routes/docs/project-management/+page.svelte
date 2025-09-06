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
	<meta name="description" content="Learn how to manage your Knot projects, from initialization to configuration and status checks. Complete guide to project lifecycle management." />
	
	<!-- Open Graph Meta Tags -->
	<meta property="og:title" content="Project Management - Knot CLI Documentation" />
	<meta property="og:description" content="Learn how to manage your Knot projects, from initialization to configuration and status checks. Complete guide to project lifecycle management." />
	<meta property="og:image" content="/images/og/project-management.png" />
	<meta property="og:image:width" content="1200" />
	<meta property="og:image:height" content="630" />
	<meta property="og:image:alt" content="Project Management - Knot CLI Documentation" />
	<meta property="og:url" content="https://knot.klysium.com/docs/project-management" />
	<meta property="og:site_name" content="Knot Space" />
	<meta property="og:type" content="article" />
	
	<!-- Twitter Card Meta Tags -->
	<meta name="twitter:card" content="summary_large_image" />
	<meta name="twitter:site" content="@knotspace" />
	<meta name="twitter:creator" content="@knotspace" />
	<meta name="twitter:title" content="Project Management - Knot CLI Documentation" />
	<meta name="twitter:description" content="Learn how to manage your Knot projects, from initialization to configuration and status checks. Complete guide to project lifecycle management." />
	<meta name="twitter:image" content="/images/og/project-management.png" />
	<meta name="twitter:image:alt" content="Project Management - Knot CLI Documentation" />
	
	<!-- Additional SEO -->
	<link rel="canonical" href="https://knot.klysium.com/docs/project-management" />
	<meta name="keywords" content="knot cli, project management, monorepo initialization, project status, knot.yml configuration" />
</svelte:head>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-12">
		<h1 class="text-4xl font-bold tracking-tight mb-4" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
			Project Management
		</h1>
		<p class="text-xl text-muted-foreground">
			Efficiently manage your monorepo with Knot's comprehensive project management tools
		</p>
	</div>

	<!-- Overview -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Project Management Overview</h2>
		<p class="text-muted-foreground leading-relaxed mb-6">
			Knot provides powerful project management capabilities to help you initialize, configure, and maintain monorepos at scale.
			From project creation to status monitoring, Knot streamlines every aspect of your development workflow.
		</p>
		
		<div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-blue-100 rounded-lg flex items-center justify-center mb-4">
					<Icon icon="solar:rocket-2-bold" class="w-6 h-6 text-blue-600" />
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
		<h2 class="text-2xl font-bold mb-6">Initializing a New Project</h2>
		<p class="text-muted-foreground mb-6 leading-relaxed">
			Start a new monorepo with a single command. Knot will set up the basic structure for you, including the root <code>knot.yml</code> configuration file, 
			directory structure, and essential configuration files.
		</p>
		
		<div class="space-y-6">
			<div class="bg-blue-50 border border-blue-200 rounded-lg p-6">
				<div class="flex items-start space-x-3">
					<Icon icon="solar:rocket-2-bold" class="w-6 h-6 text-blue-600 mt-1 flex-shrink-0" />
					<div class="flex-1 min-w-0">
						<h3 class="font-semibold text-blue-900 mb-2">Create a new project</h3>
						<p class="text-sm text-blue-700 mb-4">
							Initialize a new Knot project with the specified name. This creates the project directory and basic configuration.
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
					</div>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Example Project Creation</h3>
				<div class="bg-black/90 text-white font-mono text-sm rounded-lg relative group">
					<div class="overflow-x-auto p-4 pr-12">
						<pre><code><span class="text-gray-400"># Create a new project called "my-app"</span>
<span class="text-green-400">$</span> knot init my-app

<span class="text-blue-400">✓</span> Created project directory: my-app/
<span class="text-blue-400">✓</span> Generated knot.yml configuration
<span class="text-blue-400">✓</span> Created packages/ directory
<span class="text-blue-400">✓</span> Created apps/ directory
<span class="text-blue-400">✓</span> Initialized git repository

<span class="text-yellow-400">Next steps:</span>
  <span class="text-green-400">cd my-app</span>
  <span class="text-green-400">knot init:app frontend</span>    <span class="text-gray-400"># Create your first app</span>
  <span class="text-green-400">knot init:package utils</span>   <span class="text-gray-400"># Create your first package</span></code></pre>
					</div>
					<button
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
						on:click={() => copyToClipboard('knot init my-app')}
					>
						{#if showCopied && copyText === 'knot init my-app'}
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="solar:copy-bold" class="w-4 h-4" />
						{/if}
					</button>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Generated Project Structure</h3>
				<div class="bg-muted/30 rounded-lg p-6">
					<pre class="text-sm font-mono bg-black/90 text-white p-4 rounded overflow-x-auto"><code>my-app/
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
		<p class="text-muted-foreground mb-6 leading-relaxed">
			The <code>knot.yml</code> file is the heart of your Knot project. It defines the project's metadata, global scripts, 
			and app configurations that control how packages are linked and built.
		</p>
		
		<div class="space-y-6">
			<div>
				<h3 class="text-lg font-semibold mb-3">Complete knot.yml Example</h3>
				<div class="bg-black/90 text-white font-mono text-sm rounded-lg relative group">
					<div class="overflow-x-auto p-4 pr-12">
						<pre><code><span class="text-gray-400"># knot.yml - Project configuration</span>
<span class="text-blue-400">name:</span> <span class="text-green-400">my-awesome-project</span>
<span class="text-blue-400">description:</span> <span class="text-green-400">"A modern monorepo for TypeScript applications"</span>
<span class="text-blue-400">version:</span> <span class="text-green-400">1.0.0</span>
<span class="text-blue-400">author:</span> <span class="text-green-400">"Your Name &lt;email@example.com&gt;"</span>
<span class="text-blue-400">license:</span> <span class="text-green-400">MIT</span>

<span class="text-gray-400"># Global scripts available to all apps</span>
<span class="text-blue-400">scripts:</span>
  <span class="text-blue-400">setup:</span> <span class="text-green-400">"npm install && knot link"</span>
  <span class="text-blue-400">test-all:</span> <span class="text-green-400">"knot run test --all"</span>
  <span class="text-blue-400">build-all:</span> <span class="text-green-400">"knot run build --all"</span>
  <span class="text-blue-400">lint-all:</span> <span class="text-green-400">"knot run lint --all"</span>
  <span class="text-blue-400">clean:</span> <span class="text-green-400">"rm -rf */dist */knot_packages"</span>

<span class="text-gray-400"># App-specific configurations</span>
<span class="text-blue-400">apps:</span>
  <span class="text-blue-400">frontend:</span>
    <span class="text-blue-400">tsAlias:</span> <span class="text-green-400">"@"</span>                    <span class="text-gray-400"># TypeScript alias prefix</span>
    <span class="text-blue-400">packages:</span>                        <span class="text-gray-400"># Package dependencies</span>
      - <span class="text-yellow-400">types</span>
      - <span class="text-yellow-400">utils</span>
      - <span class="text-yellow-400">ui-components</span>
  <span class="text-blue-400">api:</span>
    <span class="text-blue-400">tsAlias:</span> <span class="text-green-400">"#"</span>
    <span class="text-blue-400">packages:</span>
      - <span class="text-yellow-400">types</span>
      - <span class="text-yellow-400">utils</span></code></pre>
					</div>
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
				<div class="space-y-4">
					<div class="border rounded-lg p-4">
						<h4 class="font-semibold mb-2">Project Metadata</h4>
						<div class="grid grid-cols-1 md:grid-cols-2 gap-4 text-sm">
							<div><code class="bg-muted px-2 py-1 rounded">name</code> - Project name (required)</div>
							<div><code class="bg-muted px-2 py-1 rounded">description</code> - Project description</div>
							<div><code class="bg-muted px-2 py-1 rounded">version</code> - Project version</div>
							<div><code class="bg-muted px-2 py-1 rounded">author</code> - Project author</div>
							<div><code class="bg-muted px-2 py-1 rounded">license</code> - License type</div>
						</div>
					</div>

					<div class="border rounded-lg p-4">
						<h4 class="font-semibold mb-2">Global Scripts</h4>
						<p class="text-sm text-muted-foreground mb-2">
							Project-level scripts that can be run from any directory with <code>knot run &lt;script&gt;</code>
						</p>
						<div class="text-sm space-y-1">
							<div>• <code>setup</code> - Initialize project dependencies and links</div>
							<div>• <code>test-all</code> - Run tests across all packages and apps</div>
							<div>• <code>build-all</code> - Build all packages and applications</div>
							<div>• <code>clean</code> - Remove build artifacts and linked packages</div>
						</div>
					</div>

					<div class="border rounded-lg p-4">
						<h4 class="font-semibold mb-2">App Configurations</h4>
						<p class="text-sm text-muted-foreground mb-2">
							Define how each app should be configured and which packages it depends on
						</p>
						<div class="text-sm space-y-1">
							<div><code class="bg-muted px-2 py-1 rounded">tsAlias</code> - TypeScript import alias prefix (@, #, ~, etc.)</div>
							<div><code class="bg-muted px-2 py-1 rounded">packages</code> - Array of package dependencies (local and remote)</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Checking Project Status -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Project Status & Health</h2>
		<p class="text-muted-foreground mb-6 leading-relaxed">
			Monitor your project's health with comprehensive status reporting. Get insights into package dependencies, 
			linking status, and potential issues across your entire monorepo.
		</p>
		
		<div class="space-y-6">
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

			<div>
				<h3 class="text-lg font-semibold mb-3">Example Status Output</h3>
				<div class="bg-black/90 text-white font-mono text-sm rounded-lg">
					<div class="p-4">
						<pre><code><span class="text-blue-400">📦 Project:</span> <span class="text-white">my-awesome-project</span> <span class="text-gray-400">(v1.0.0)</span>
<span class="text-green-400">✓</span> <span class="text-gray-400">Configuration valid</span>
<span class="text-green-400">✓</span> <span class="text-gray-400">Git repository initialized</span>

<span class="text-yellow-400">📱 Apps (2):</span>
  <span class="text-blue-400">frontend</span> <span class="text-gray-400">- React application</span>
    <span class="text-green-400">✓</span> <span class="text-gray-400">Linked: types, utils, ui-components</span>
    <span class="text-green-400">✓</span> <span class="text-gray-400">TypeScript alias: @</span>
  
  <span class="text-blue-400">api</span> <span class="text-gray-400">- Node.js API server</span>
    <span class="text-green-400">✓</span> <span class="text-gray-400">Linked: types, utils</span>
    <span class="text-green-400">✓</span> <span class="text-gray-400">TypeScript alias: #</span>

<span class="text-purple-400">📚 Packages (3):</span>
  <span class="text-blue-400">types</span> <span class="text-gray-400">(v1.0.0) - Shared TypeScript types</span>
    <span class="text-green-400">✓</span> <span class="text-gray-400">Build: ready</span>
    <span class="text-green-400">✓</span> <span class="text-gray-400">Used by: frontend, api</span>
  
  <span class="text-blue-400">utils</span> <span class="text-gray-400">(v1.2.0) - Utility functions</span>
    <span class="text-green-400">✓</span> <span class="text-gray-400">Build: ready</span>
    <span class="text-green-400">✓</span> <span class="text-gray-400">Used by: frontend, api</span>
  
  <span class="text-blue-400">ui-components</span> <span class="text-gray-400">(v2.1.0) - React UI components</span>
    <span class="text-green-400">✓</span> <span class="text-gray-400">Build: ready</span>
    <span class="text-green-400">✓</span> <span class="text-gray-400">Used by: frontend</span>

<span class="text-green-400">🔗 Linking Status:</span>
  <span class="text-green-400">✓</span> <span class="text-gray-400">All packages linked successfully</span>
  <span class="text-green-400">✓</span> <span class="text-gray-400">TypeScript configurations updated</span>
  <span class="text-green-400">✓</span> <span class="text-gray-400">No circular dependencies detected</span></code></pre>
					</div>
				</div>
			</div>

			<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
				<div class="border rounded-lg p-4">
					<h4 class="font-semibold mb-2 flex items-center">
						<Icon icon="solar:check-circle-bold" class="w-4 h-4 mr-2 text-green-600" />
						Status Indicators
					</h4>
					<div class="text-sm space-y-1">
						<div><span class="text-green-600">✓</span> Green checkmark - Everything working correctly</div>
						<div><span class="text-yellow-600">⚠</span> Yellow warning - Attention needed</div>
						<div><span class="text-red-600">✗</span> Red error - Critical issue requiring action</div>
						<div><span class="text-blue-600">ℹ</span> Blue info - Additional information available</div>
					</div>
				</div>

				<div class="border rounded-lg p-4">
					<h4 class="font-semibold mb-2 flex items-center">
						<Icon icon="solar:info-circle-bold" class="w-4 h-4 mr-2 text-blue-600" />
						Status Categories
					</h4>
					<div class="text-sm space-y-1">
						<div>• Project metadata and configuration</div>
						<div>• Application status and dependencies</div>
						<div>• Package versions and build status</div>
						<div>• Linking status and TypeScript configuration</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Advanced Project Management -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Advanced Project Management</h2>
		
		<div class="space-y-6">
			<p class="text-muted-foreground">
				Beyond basic initialization and status checking, Knot provides advanced features for managing complex monorepos.
			</p>

			<div class="grid grid-cols-1 md:grid-cols-2 gap-8">
				<div>
					<h3 class="text-lg font-semibold mb-4 flex items-center">
						<Icon icon="solar:folder-bold" class="w-5 h-5 mr-2 text-orange-600" />
						Project Structure Management
					</h3>
					<ul class="space-y-2 text-sm text-muted-foreground">
						<li class="flex items-start">
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 mr-2 text-green-600 mt-0.5 flex-shrink-0" />
							Automated directory structure validation
						</li>
						<li class="flex items-start">
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 mr-2 text-green-600 mt-0.5 flex-shrink-0" />
							Consistent naming convention enforcement
						</li>
						<li class="flex items-start">
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 mr-2 text-green-600 mt-0.5 flex-shrink-0" />
							Template-based app and package creation
						</li>
						<li class="flex items-start">
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 mr-2 text-green-600 mt-0.5 flex-shrink-0" />
							Bulk operations on multiple components
						</li>
					</ul>
				</div>

				<div>
					<h3 class="text-lg font-semibold mb-4 flex items-center">
						<Icon icon="solar:settings-bold" class="w-5 h-5 mr-2 text-purple-600" />
						Configuration Management
					</h3>
					<ul class="space-y-2 text-sm text-muted-foreground">
						<li class="flex items-start">
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 mr-2 text-green-600 mt-0.5 flex-shrink-0" />
							YAML schema validation and auto-completion
						</li>
						<li class="flex items-start">
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 mr-2 text-green-600 mt-0.5 flex-shrink-0" />
							Configuration inheritance and overrides
						</li>
						<li class="flex items-start">
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 mr-2 text-green-600 mt-0.5 flex-shrink-0" />
							Environment-specific configuration profiles
						</li>
						<li class="flex items-start">
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 mr-2 text-green-600 mt-0.5 flex-shrink-0" />
							Migration tools for configuration updates
						</li>
					</ul>
				</div>
			</div>
		</div>
	</section>

	<!-- Best Practices -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Best Practices</h2>
		
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<div class="space-y-6">
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
						<Icon icon="solar:workflow-bold" class="w-5 h-5 mr-2 text-blue-600" />
						Development Workflow
					</h3>
					<ul class="text-sm text-muted-foreground space-y-2">
						<li>• Run <code>knot status</code> regularly to monitor health</li>
						<li>• Use global scripts for common tasks</li>
						<li>• Maintain consistent script naming across projects</li>
						<li>• Set up continuous integration for status checks</li>
					</ul>
				</div>
			</div>

			<div class="space-y-6">
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

				<div class="border rounded-lg p-6">
					<h3 class="font-semibold mb-3 flex items-center">
						<Icon icon="solar:bug-bold" class="w-5 h-5 mr-2 text-red-600" />
						Troubleshooting
					</h3>
					<ul class="text-sm text-muted-foreground space-y-2">
						<li>• Check YAML syntax if commands fail</li>
						<li>• Verify package names match directory names</li>
						<li>• Run <code>knot status</code> to identify issues</li>
						<li>• Clear node_modules and re-link if problems persist</li>
					</ul>
				</div>
			</div>
		</div>
	</section>

	<!-- Next Steps -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Next Steps</h2>
		<div class="bg-green-50 border border-green-200 rounded-lg p-6 mb-8">
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

		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
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

			<a href="/docs/typescript" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:code-bold" class="w-6 h-6 text-orange-600" />
					<h3 class="font-semibold">TypeScript Integration</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Discover how Knot automatically configures TypeScript for seamless development.
				</p>
			</a>

			<a href="/docs/troubleshooting" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:bug-bold" class="w-6 h-6 text-red-600" />
					<h3 class="font-semibold">Troubleshooting</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Common issues and solutions for project management problems.
				</p>
			</a>
		</div>
	</section>
</div>