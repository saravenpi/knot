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
  - <span class="text-yellow-400">"@team/shared@^1.0.0"</span>     <span class="text-gray-400"># Remote dependency</span>

<span class="text-gray-400"># Development dependencies</span>
<span class="text-blue-400">devDependencies:</span>
  - <span class="text-yellow-400">"@types/node@^20.0.0"</span>     <span class="text-gray-400"># npm types</span>

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

	<!-- Package Aliases in Development -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<Icon icon="lucide:link" class="w-8 h-8 text-green-600" />
			<h2 class="text-2xl font-bold">Using Aliases in Package Development</h2>
		</div>
		
		<div class="space-y-8">
			<div class="bg-blue-50 border border-blue-200 rounded-lg p-6">
				<div class="flex items-start space-x-3">
					<Icon icon="lucide:lightbulb" class="w-6 h-6 text-blue-600 mt-1 flex-shrink-0" />
					<div>
						<h3 class="font-semibold text-blue-900 mb-2">Why Use Aliases in Package Development?</h3>
						<p class="text-sm text-blue-700">
							Package aliases provide clean, consistent import paths that make your packages easier to use and maintain. 
							They abstract away complex package structures and enable seamless refactoring.
						</p>
					</div>
				</div>
			</div>

			<!-- Setting Up Aliases for New Packages -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-4">Setting Up Aliases for New Packages</h3>
				<p class="text-muted-foreground mb-4">
					When creating packages, consider how they will be imported and choose meaningful names that work well with aliases.
				</p>
				
				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Package Creation with Alias-Friendly Names</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm rounded-lg relative group">
							<div class="overflow-x-auto p-4 pr-12">
								<code class="whitespace-nowrap block">knot init:package user-management</code>
							</div>
							<button 
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100 z-10"
								on:click={() => copyToClipboard('knot init:package user-management')}
							>
								{#if showCopied && copyText === 'knot init:package user-management'}
									<Icon icon="lucide:check-circle" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="lucide:copy" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>
					
					<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
						<div class="border rounded p-4">
							<h4 class="font-semibold mb-2 text-green-700">Good Package Names</h4>
							<ul class="text-sm text-muted-foreground space-y-1">
								<li>• <code>user-management</code> → <code>@/user-management</code></li>
								<li>• <code>payment-processing</code> → <code>@/payment-processing</code></li>
								<li>• <code>data-validation</code> → <code>@/data-validation</code></li>
								<li>• <code>auth-middleware</code> → <code>@/auth-middleware</code></li>
							</ul>
						</div>
						
						<div class="border rounded p-4">
							<h4 class="font-semibold mb-2 text-red-700">Avoid These Names</h4>
							<ul class="text-sm text-muted-foreground space-y-1">
								<li>• <code>utils</code> (too generic)</li>
								<li>• <code>helpers</code> (unclear purpose)</li>
								<li>• <code>common</code> (not descriptive)</li>
								<li>• <code>shared</code> (overly broad)</li>
							</ul>
						</div>
					</div>
				</div>
			</div>

			<!-- Package Structure with Aliases -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-4">Package Structure for Alias Usage</h3>
				<p class="text-muted-foreground mb-4">
					Structure your packages to work optimally with the alias system and provide clear entry points.
				</p>
				
				<div class="space-y-4">
					<div class="bg-muted/30 rounded-lg p-4">
						<h4 class="font-semibold mb-2">Example: Well-Structured Package</h4>
						<div class="text-sm font-mono">
							packages/user-management/<br>
							├── src/<br>
							│   ├── <span class="text-blue-400">index.ts</span>          <span class="text-green-600"># Main exports</span><br>
							│   ├── <span class="text-yellow-400">types.ts</span>         <span class="text-green-600"># Type definitions</span><br>
							│   ├── <span class="text-purple-400">services/</span><br>
							│   │   ├── <span class="text-gray-400">UserService.ts</span><br>
							│   │   └── <span class="text-gray-400">AuthService.ts</span><br>
							│   └── <span class="text-purple-400">utils/</span><br>
							│       ├── <span class="text-gray-400">validation.ts</span><br>
							│       └── <span class="text-gray-400">formatting.ts</span><br>
							├── <span class="text-blue-400">package.yml</span><br>
							└── <span class="text-gray-400">README.md</span>
						</div>
					</div>
					
					<div>
						<h4 class="font-medium mb-2">Clear Export Structure</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-gray-400">// packages/user-management/src/index.ts</span>
<span class="text-purple-400">export</span> {"{ "}<span class="text-yellow-400">UserService, AuthService</span> {"} "}<span class="text-purple-400">from</span> <span class="text-green-400">'./services'</span>;
<span class="text-purple-400">export</span> {"{ "}<span class="text-yellow-400">validateUser, formatUserName</span> {"} "}<span class="text-purple-400">from</span> <span class="text-green-400">'./utils'</span>;
<span class="text-purple-400">export</span> <span class="text-purple-400">type</span> {"{ "}<span class="text-yellow-400">User, AuthToken, UserRole</span> {"} "}<span class="text-purple-400">from</span> <span class="text-green-400">'./types'</span>;

<span class="text-gray-400">// Clean imports from consuming apps:</span>
<span class="text-gray-400">// import { UserService, User } from '@/user-management';</span></code></pre>
						</div>
					</div>
				</div>
			</div>

			<!-- Development Workflow with Aliases -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-4">Development Workflow</h3>
				<p class="text-muted-foreground mb-4">
					Optimize your package development workflow by leveraging aliases and linking modes.
				</p>
				
				<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
					<div class="space-y-4">
						<h4 class="font-semibold">Development Phase</h4>
						<div class="border rounded p-4 bg-purple-50">
							<div class="flex items-center space-x-2 mb-2">
								<Icon icon="lucide:code" class="w-4 h-4 text-purple-600" />
								<span class="font-medium text-purple-900">Use Symlink Mode</span>
							</div>
							<div class="bg-black/90 text-purple-400 font-mono text-sm p-3 rounded">
								<code>knot link --symlink</code>
							</div>
							<ul class="text-xs text-muted-foreground mt-2 space-y-1">
								<li>• Live updates while coding</li>
								<li>• Instant feedback in consuming apps</li>
								<li>• Perfect for debugging issues</li>
							</ul>
						</div>
						
						<div class="border rounded p-4 bg-blue-50">
							<div class="flex items-center space-x-2 mb-2">
								<Icon icon="lucide:eye" class="w-4 h-4 text-blue-600" />
								<span class="font-medium text-blue-900">Watch Mode Development</span>
							</div>
							<div class="bg-black/90 text-blue-400 font-mono text-sm p-3 rounded">
								<code>knot run build --watch</code>
							</div>
							<ul class="text-xs text-muted-foreground mt-2 space-y-1">
								<li>• Automatic rebuilds on changes</li>
								<li>• Works with symlinked packages</li>
								<li>• Maintains type safety</li>
							</ul>
						</div>
					</div>
					
					<div class="space-y-4">
						<h4 class="font-semibold">Testing & Production</h4>
						<div class="border rounded p-4 bg-green-50">
							<div class="flex items-center space-x-2 mb-2">
								<Icon icon="lucide:copy" class="w-4 h-4 text-green-600" />
								<span class="font-medium text-green-900">Use Copy Mode</span>
							</div>
							<div class="bg-black/90 text-green-400 font-mono text-sm p-3 rounded">
								<code>knot link</code>
							</div>
							<ul class="text-xs text-muted-foreground mt-2 space-y-1">
								<li>• Stable production environment</li>
								<li>• Docker and CI/CD compatible</li>
								<li>• Exact version control</li>
							</ul>
						</div>
						
						<div class="border rounded p-4 bg-yellow-50">
							<div class="flex items-center space-x-2 mb-2">
								<Icon icon="lucide:test-tube" class="w-4 h-4 text-yellow-600" />
								<span class="font-medium text-yellow-900">Pre-commit Testing</span>
							</div>
							<div class="bg-black/90 text-yellow-400 font-mono text-sm p-3 rounded">
								<code>knot run test</code>
							</div>
							<ul class="text-xs text-muted-foreground mt-2 space-y-1">
								<li>• Automatic package updates</li>
								<li>• Ensures consistency</li>
								<li>• Validates alias imports</li>
							</ul>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Package Naming Strategies -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Package Naming Strategies for Aliases</h2>
		
		<div class="space-y-6">
			<p class="text-muted-foreground">
				Choose package names that work well with your alias system and provide clear, intuitive imports for your team.
			</p>
			
			<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
				<div class="border rounded-lg p-6">
					<h3 class="font-semibold mb-3 flex items-center">
						<Icon icon="lucide:layers" class="w-5 h-5 mr-2 text-blue-600" />
						Domain-Driven Naming
					</h3>
					<div class="space-y-3">
						<p class="text-sm text-muted-foreground">
							Organize packages around business domains rather than technical layers.
						</p>
						<div class="bg-black/90 text-white font-mono text-sm p-3 rounded">
							<code><span class="text-gray-400"># Business domain packages</span>
<span class="text-yellow-400">user-account</span>     <span class="text-gray-400"># @/user-account</span>
<span class="text-yellow-400">order-management</span> <span class="text-gray-400"># @/order-management</span>
<span class="text-yellow-400">product-catalog</span>  <span class="text-gray-400"># @/product-catalog</span>
<span class="text-yellow-400">payment-gateway</span>  <span class="text-gray-400"># @/payment-gateway</span></code>
						</div>
					</div>
				</div>
				
				<div class="border rounded-lg p-6">
					<h3 class="font-semibold mb-3 flex items-center">
						<Icon icon="lucide:component" class="w-5 h-5 mr-2 text-purple-600" />
						Feature-Based Naming
					</h3>
					<div class="space-y-3">
						<p class="text-sm text-muted-foreground">
							Name packages after specific features or capabilities they provide.
						</p>
						<div class="bg-black/90 text-white font-mono text-sm p-3 rounded">
							<code><span class="text-gray-400"># Feature-based packages</span>
<span class="text-yellow-400">email-templates</span>  <span class="text-gray-400"># @/email-templates</span>
<span class="text-yellow-400">data-export</span>      <span class="text-gray-400"># @/data-export</span>
<span class="text-yellow-400">search-filters</span>   <span class="text-gray-400"># @/search-filters</span>
<span class="text-yellow-400">audit-logging</span>    <span class="text-gray-400"># @/audit-logging</span></code>
						</div>
					</div>
				</div>
			</div>
			
			<div class="bg-yellow-50 border border-yellow-200 rounded-lg p-6">
				<div class="flex items-start space-x-3">
					<Icon icon="lucide:lightbulb" class="w-6 h-6 text-yellow-600 mt-1 flex-shrink-0" />
					<div>
						<h3 class="font-semibold text-yellow-900 mb-2">Naming Best Practices</h3>
						<ul class="text-sm text-yellow-700 space-y-1">
							<li>• Use kebab-case for consistency across systems</li>
							<li>• Include the primary noun (user, order, payment)</li>
							<li>• Add descriptive action or purpose (management, processing, validation)</li>
							<li>• Keep names between 2-3 words for readability</li>
							<li>• Test how the alias import looks: <code>import ... from '@/package-name'</code></li>
						</ul>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Package Dependencies with Aliases -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Managing Package Dependencies</h2>
		
		<div class="space-y-8">
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">Inter-Package Dependencies</h3>
				<p class="text-muted-foreground mb-4">
					When packages depend on other packages, aliases provide clean import paths that remain stable during refactoring.
				</p>
				
				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Package Configuration</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-gray-400"># packages/user-management/package.yml</span>
<span class="text-blue-400">name:</span> user-management
<span class="text-blue-400">dependencies:</span>
  - <span class="text-yellow-400">shared-types</span>       <span class="text-gray-400"># Local package</span>
  - <span class="text-yellow-400">data-validation</span>   <span class="text-gray-400"># Local package</span>
  - <span class="text-yellow-400">auth-middleware</span>   <span class="text-gray-400"># Local package</span></code></pre>
						</div>
					</div>
					
					<div>
						<h4 class="font-medium mb-2">Using Dependencies with Aliases</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-gray-400">// packages/user-management/src/UserService.ts</span>
<span class="text-purple-400">import</span> {"{ "}<span class="text-yellow-400">User, UserRole</span> {"} "}<span class="text-purple-400">from</span> <span class="text-green-400">'@/shared-types'</span>;
<span class="text-purple-400">import</span> {"{ "}<span class="text-yellow-400">validateEmail</span> {"} "}<span class="text-purple-400">from</span> <span class="text-green-400">'@/data-validation'</span>;
<span class="text-purple-400">import</span> {"{ "}<span class="text-yellow-400">requireAuth</span> {"} "}<span class="text-purple-400">from</span> <span class="text-green-400">'@/auth-middleware'</span>;

<span class="text-purple-400">export</span> <span class="text-purple-400">class</span> <span class="text-yellow-400">UserService</span> {"{"}
  <span class="text-purple-400">async</span> <span class="text-yellow-400">createUser</span>(userData: <span class="text-blue-400">Partial</span>&lt;<span class="text-blue-400">User</span>&gt;): <span class="text-blue-400">Promise</span>&lt;<span class="text-blue-400">User</span>&gt; {"{"}
    <span class="text-purple-400">if</span> (!<span class="text-yellow-400">validateEmail</span>(userData.email)) {"{"}
      <span class="text-purple-400">throw</span> <span class="text-purple-400">new</span> <span class="text-blue-400">Error</span>(<span class="text-green-400">'Invalid email'</span>);
    {"}"}
    <span class="text-gray-400">// Implementation...</span>
  {"}"}
{"}"}</code></pre>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Testing Packages with Aliases -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Testing Packages with Aliases</h2>
		
		<div class="space-y-8">
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">Test Configuration</h3>
				<p class="text-muted-foreground mb-4">
					Ensure your test environment properly resolves aliases for comprehensive package testing.
				</p>
				
				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Jest Configuration</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-gray-400">// packages/user-management/jest.config.js</span>
<span class="text-purple-400">module.exports</span> = {"{"}
  <span class="text-blue-400">moduleNameMapping:</span> {"{"}
    <span class="text-green-400">'^@/(.*)$'</span>: <span class="text-green-400">'&lt;rootDir&gt;/../$1/src'</span>
  {"}"},
  <span class="text-blue-400">testEnvironment:</span> <span class="text-green-400">'node'</span>,
  <span class="text-blue-400">roots:</span> [<span class="text-green-400">'&lt;rootDir&gt;/src'</span>, <span class="text-green-400">'&lt;rootDir&gt;/tests'</span>]
{"}"};</code></pre>
						</div>
					</div>
					
					<div>
						<h4 class="font-medium mb-2">Test Files with Aliases</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-gray-400">// packages/user-management/tests/UserService.test.ts</span>
<span class="text-purple-400">import</span> {"{ "}<span class="text-yellow-400">UserService</span> {"} "}<span class="text-purple-400">from</span> <span class="text-green-400">'../src/UserService'</span>;
<span class="text-purple-400">import</span> {"{ "}<span class="text-yellow-400">User</span> {"} "}<span class="text-purple-400">from</span> <span class="text-green-400">'@/shared-types'</span>; <span class="text-gray-400">// Alias works in tests</span>

<span class="text-yellow-400">describe</span>(<span class="text-green-400">'UserService'</span>, () => {"{"}
  <span class="text-yellow-400">test</span>(<span class="text-green-400">'should create user with valid data'</span>, <span class="text-purple-400">async</span> () => {"{"}
    <span class="text-purple-400">const</span> userData: <span class="text-blue-400">Partial</span>&lt;<span class="text-blue-400">User</span>&gt; = {"{"}
      email: <span class="text-green-400">'test@example.com'</span>,
      name: <span class="text-green-400">'John Doe'</span>
    {"}"};
    
    <span class="text-purple-400">const</span> userService = <span class="text-purple-400">new</span> <span class="text-yellow-400">UserService</span>();
    <span class="text-purple-400">const</span> result = <span class="text-purple-400">await</span> userService.<span class="text-yellow-400">createUser</span>(userData);
    
    <span class="text-yellow-400">expect</span>(result.email).<span class="text-yellow-400">toBe</span>(<span class="text-green-400">'test@example.com'</span>);
  {"}"});
{"}"});</code></pre>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Next Steps -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Next Steps</h2>
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
			<a href="/docs/aliases" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:link" class="w-6 h-6 text-blue-600" />
					<h3 class="font-semibold">Package Aliases</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Comprehensive guide to configuring and using package aliases effectively.
				</p>
			</a>

			<a href="/docs/publishing" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:upload" class="w-6 h-6 text-red-600" />
					<h3 class="font-semibold">Publishing Guide</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Detailed guide to publishing packages to Knot Space and other registries.
				</p>
			</a>

			<a href="/docs/inter-package-dependencies" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:layers" class="w-6 h-6 text-purple-600" />
					<h3 class="font-semibold">Inter-Package Dependencies</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Master dependencies between packages with comprehensive examples.
				</p>
			</a>

			<a href="/docs/package-linking" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:link" class="w-6 h-6 text-blue-600" />
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
		</div>
	</section>
</div>