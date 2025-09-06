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
	<meta name="description" content="Learn how to create, manage, and publish packages with Knot CLI. Covers package development workflows, dependency management, and publishing strategies." />
	<meta property="og:title" content="Package Development Guide - Knot CLI" />
	<meta property="og:description" content="Learn how to create, manage, and publish packages with Knot CLI. Covers package development workflows, dependency management, and publishing strategies." />
	<meta property="og:image" content="/images/og/package-development.png" />
	<meta property="og:url" content="https://knot.klysium.com/docs/package-development" />
	<meta name="twitter:title" content="Package Development Guide - Knot CLI" />
	<meta name="twitter:description" content="Learn how to create, manage, and publish packages with Knot CLI. Covers package development workflows, dependency management, and publishing strategies." />
	<meta name="twitter:image" content="/images/og/package-development.png" />
	<link rel="canonical" href="https://knot.klysium.com/docs/package-development" />
</svelte:head>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight mb-4" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
			Package Development
		</h1>
		<p class="text-lg text-muted-foreground">
			Comprehensive guide to creating, developing, and managing packages with Knot CLI
		</p>
	</div>

	<!-- Package Overview -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Package Development Workflow</h2>
		<p class="text-muted-foreground leading-relaxed mb-6">
			Knot CLI provides powerful tools for package development, from creation to publishing. 
			Packages are reusable libraries that can be shared across your monorepo or published to registries 
			for broader use.
		</p>
		
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-green-100 rounded-lg flex items-center justify-center mb-4">
					<Icon icon="lucide:plus-circle" class="w-6 h-6 text-green-600" />
				</div>
				<h3 class="font-semibold mb-2">Creation</h3>
				<p class="text-sm text-muted-foreground">
					Generate new packages with proper structure, configuration, and TypeScript setup.
				</p>
			</div>
			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-blue-100 rounded-lg flex items-center justify-center mb-4">
					<Icon icon="lucide:code" class="w-6 h-6 text-blue-600" />
				</div>
				<h3 class="font-semibold mb-2">Development</h3>
				<p class="text-sm text-muted-foreground">
					Build, test, and iterate on packages with watch mode and hot reloading.
				</p>
			</div>
			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-purple-100 rounded-lg flex items-center justify-center mb-4">
					<Icon icon="lucide:upload" class="w-6 h-6 text-purple-600" />
				</div>
				<h3 class="font-semibold mb-2">Publishing</h3>
				<p class="text-sm text-muted-foreground">
					Publish packages to registries with versioning and dependency management.
				</p>
			</div>
		</div>
	</section>

	<!-- Creating Packages -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<Icon icon="lucide:add-circle-bold" class="w-8 h-8 text-green-600" />
			<h2 class="text-2xl font-bold">Creating Packages</h2>
		</div>

		<div class="space-y-8">
			<div>
				<h3 class="text-lg font-semibold mb-3">Generate a new package</h3>
				<div class="bg-black/90 text-green-400 font-mono text-sm rounded-lg relative group">
					<div class="overflow-x-auto p-4 pr-12">
						<code class="whitespace-nowrap block">knot init:package &lt;package-name&gt;</code>
					</div>
					<button 
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100 z-10"
						on:click={() => copyToClipboard('knot init:package utils')}
					>
						{#if showCopied && copyText === 'knot init:package utils'}
							<Icon icon="lucide:check-circle" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="lucide:copy" class="w-4 h-4" />
						{/if}
					</button>
				</div>
				<p class="text-sm text-muted-foreground mt-2">
					Creates a new package in the <code>packages/</code> directory with complete structure and configuration.
				</p>
			</div>

			<div class="bg-muted/30 rounded-lg p-6">
				<h4 class="font-semibold mb-3 flex items-center space-x-2">
					<Icon icon="lucide:folder" class="w-5 h-5 text-blue-600" />
					<span>Generated Package Structure</span>
				</h4>
				<pre class="text-sm font-mono"><code>packages/utils/
├── <span class="text-blue-400">src/</span>
│   ├── <span class="text-green-400">index.ts</span>          # Main entry point
│   └── <span class="text-yellow-400">types.ts</span>         # Type definitions
├── <span class="text-purple-400">tests/</span>               # Test files
│   └── <span class="text-gray-400">index.test.ts</span>
├── <span class="text-blue-400">package.yml</span>           # Package configuration
├── <span class="text-yellow-400">package.json</span>        # npm package file
├── <span class="text-green-400">tsconfig.json</span>       # TypeScript config
├── <span class="text-red-400">rollup.config.js</span>     # Build configuration
└── <span class="text-gray-400">README.md</span>            # Package documentation</code></pre>
			</div>
		</div>
	</section>

	<!-- Package Development -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<Icon icon="lucide:code" class="w-8 h-8 text-blue-600" />
			<h2 class="text-2xl font-bold">Package Development</h2>
		</div>

		<div class="space-y-8">
			<div>
				<h3 class="text-lg font-semibold mb-3">Development workflow commands</h3>
				<p class="text-muted-foreground mb-4">
					Navigate to your package directory to run package-specific commands:
				</p>
				
				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Build the package</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm rounded-lg relative group">
							<div class="overflow-x-auto p-4 pr-12">
								<code class="whitespace-nowrap block">cd packages/utils && knot run build</code>
							</div>
							<button 
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100 z-10"
								on:click={() => copyToClipboard('cd packages/utils && knot run build')}
							>
								{#if showCopied && copyText === 'cd packages/utils && knot run build'}
									<Icon icon="lucide:check-circle" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="lucide:copy" class="w-4 h-4" />
								{/if}
							</button>
						</div>
						<p class="text-sm text-muted-foreground mt-2">
							Compiles TypeScript and bundles the package for distribution.
						</p>
					</div>

					<div>
						<h4 class="font-medium mb-2">Watch mode for development</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm rounded-lg relative group">
							<div class="overflow-x-auto p-4 pr-12">
								<code class="whitespace-nowrap block">knot run build --watch</code>
							</div>
							<button 
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100 z-10"
								on:click={() => copyToClipboard('knot run build --watch')}
							>
								{#if showCopied && copyText === 'knot run build --watch'}
									<Icon icon="lucide:check-circle" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="lucide:copy" class="w-4 h-4" />
								{/if}
							</button>
						</div>
						<p class="text-sm text-muted-foreground mt-2">
							Automatically rebuilds when source files change.
						</p>
					</div>

					<div>
						<h4 class="font-medium mb-2">Run tests</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm rounded-lg relative group">
							<div class="overflow-x-auto p-4 pr-12">
								<code class="whitespace-nowrap block">knot run test</code>
							</div>
							<button 
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100 z-10"
								on:click={() => copyToClipboard('knot run test')}
							>
								{#if showCopied && copyText === 'knot run test'}
									<Icon icon="lucide:check-circle" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="lucide:copy" class="w-4 h-4" />
								{/if}
							</button>
						</div>
						<p class="text-sm text-muted-foreground mt-2">
							Runs the test suite with coverage reporting.
						</p>
					</div>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Package configuration</h3>
				<p class="text-muted-foreground mb-4">
					The <code>package.yml</code> file defines your package metadata and build configuration:
				</p>
				
				<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
					<pre><code>{@html `<span class="text-gray-400"># Package metadata</span>
<span class="text-blue-400">name:</span> <span class="text-green-400">utils</span>
<span class="text-blue-400">description:</span> <span class="text-green-400">"Shared utility functions and helpers"</span>
<span class="text-blue-400">version:</span> <span class="text-green-400">1.0.0</span>
<span class="text-blue-400">author:</span> <span class="text-green-400">"Your Team"</span>

<span class="text-gray-400"># Build scripts</span>
<span class="text-blue-400">scripts:</span>
  <span class="text-blue-400">build:</span> <span class="text-green-400">"rollup -c"</span>
  <span class="text-blue-400">test:</span> <span class="text-green-400">"vitest run"</span>
  <span class="text-blue-400">test:watch:</span> <span class="text-green-400">"vitest"</span>
  <span class="text-blue-400">lint:</span> <span class="text-green-400">"eslint src/ --ext .ts"</span>

<span class="text-gray-400"># Package dependencies</span>
<span class="text-blue-400">dependencies:</span>
  - <span class="text-yellow-400">types</span>                     <span class="text-gray-400"># Local dependency</span>

<span class="text-gray-400"># Tags for discoverability</span>
<span class="text-blue-400">tags:</span>
  - <span class="text-yellow-400">utilities</span>
  - <span class="text-yellow-400">helpers</span>
  - <span class="text-yellow-400">typescript</span>`}</code></pre>
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
						<Icon icon="lucide:check-circle" class="w-5 h-5 mr-2 text-green-600" />
						Package Design
					</h3>
					<ul class="text-sm text-muted-foreground space-y-2">
						<li>• Keep packages focused and single-purpose</li>
						<li>• Use descriptive names and clear documentation</li>
						<li>• Follow semantic versioning strictly</li>
						<li>• Provide comprehensive TypeScript types</li>
						<li>• Include usage examples in README</li>
					</ul>
				</div>

				<div class="border rounded-lg p-6">
					<h3 class="font-semibold mb-3 flex items-center">
						<Icon icon="lucide:code" class="w-5 h-5 mr-2 text-blue-600" />
						Development
					</h3>
					<ul class="text-sm text-muted-foreground space-y-2">
						<li>• Use watch mode during development</li>
						<li>• Write tests before implementing features</li>
						<li>• Maintain high test coverage (>90%)</li>
						<li>• Use linting and formatting tools</li>
						<li>• Document public APIs with JSDoc</li>
					</ul>
				</div>
			</div>

			<div class="space-y-6">
				<div class="border rounded-lg p-6">
					<h3 class="font-semibold mb-3 flex items-center">
						<Icon icon="lucide:upload" class="w-5 h-5 mr-2 text-purple-600" />
						Publishing
					</h3>
					<ul class="text-sm text-muted-foreground space-y-2">
						<li>• Always run tests before publishing</li>
						<li>• Use <code>--dry-run</code> to preview changes</li>
						<li>• Update CHANGELOG for each release</li>
						<li>• Tag releases in version control</li>
						<li>• Consider backward compatibility</li>
					</ul>
				</div>

				<div class="border rounded-lg p-6">
					<h3 class="font-semibold mb-3 flex items-center">
						<Icon icon="lucide:shield-alert" class="w-5 h-5 mr-2 text-yellow-600" />
						Common Pitfalls
					</h3>
					<ul class="text-sm text-muted-foreground space-y-2">
						<li>• Don't publish without testing</li>
						<li>• Avoid circular dependencies</li>
						<li>• Don't include development files in builds</li>
						<li>• Keep bundle sizes reasonable</li>
						<li>• Version dependencies appropriately</li>
					</ul>
				</div>
			</div>
		</div>
	</section>

	<!-- Next Steps -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Next Steps</h2>
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
			<a href="/docs/publishing" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:upload" class="w-6 h-6 text-red-600" />
					<h3 class="font-semibold">Publishing Guide</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Detailed guide to publishing packages to Knot Space and other registries.
				</p>
			</a>

			<a href="/docs/package-linking" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:link" class="w-6 h-6 text-purple-600" />
					<h3 class="font-semibold">Package Linking</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Understand how Knot links packages and manages dependencies.
				</p>
			</a>

			<a href="/docs/typescript" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:code" class="w-6 h-6 text-orange-600" />
					<h3 class="font-semibold">TypeScript Setup</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Configure TypeScript for optimal development experience.
				</p>
			</a>

			<a href="/docs/project-management" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:settings" class="w-6 h-6 text-blue-600" />
					<h3 class="font-semibold">Project Management</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Project-level operations and monorepo management commands.
				</p>
			</a>

			<a href="/docs/workflows" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:cpu" class="w-6 h-6 text-green-600" />
					<h3 class="font-semibold">Workflow Best Practices</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Optimize your development workflow with Knot CLI.
				</p>
			</a>

			<a href="/docs/troubleshooting" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:shield-alert" class="w-6 h-6 text-red-600" />
					<h3 class="font-semibold">Troubleshooting</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Solve common issues and debugging tips.
				</p>
			</a>
		</div>
	</section>
</div>