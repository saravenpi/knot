<script lang="ts">
	import Icon from '@iconify/svelte';
</script>

<svelte:head>
	<title>Introduction to Knot - Modern Monorepo Package Manager</title>
	<meta name="description" content="Knot is a modern package manager designed for TypeScript and JavaScript monorepos. Learn about smart package linking, TypeScript integration, and team collaboration." />
	<meta property="og:title" content="Introduction to Knot - Modern Monorepo Package Manager" />
	<meta property="og:description" content="Knot is a modern package manager designed for TypeScript and JavaScript monorepos. Learn about smart package linking, TypeScript integration, and team collaboration." />
	<meta property="og:url" content="https://knot.klysium.com/docs" />
	<meta name="twitter:title" content="Introduction to Knot - Modern Monorepo Package Manager" />
	<meta name="twitter:description" content="Knot is a modern package manager designed for TypeScript and JavaScript monorepos. Learn about smart package linking, TypeScript integration, and team collaboration." />
	<link rel="canonical" href="https://knot.klysium.com/docs" />
</svelte:head>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-12">
		<h1 class="text-4xl font-bold tracking-tight mb-4" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
			Introduction to Knot
		</h1>
		<p class="text-xl text-muted-foreground">
			A modern monorepo package manager for TypeScript/JavaScript projects
		</p>
	</div>

	<!-- What is Knot -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">What is Knot?</h2>
		<p class="text-muted-foreground mb-6 leading-relaxed">
			Knot is a modern package manager designed specifically for TypeScript and JavaScript monorepos.
			It simplifies dependency management, package linking, and build orchestration across multiple
			applications and packages within a single repository.
		</p>

		<div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-blue-100 rounded-lg flex items-center justify-center mb-4">
					<Icon icon="solar:bolt-bold-duotone" class="w-6 h-6 text-blue-600" />
				</div>
				<h3 class="font-semibold mb-2">Fast & Efficient</h3>
				<p class="text-sm text-muted-foreground">
					Smart caching and parallel operations make development faster than traditional package managers.
				</p>
			</div>
			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-green-100 rounded-lg flex items-center justify-center mb-4">
					<Icon icon="solar:link-bold" class="w-6 h-6 text-green-600" />
				</div>
				<h3 class="font-semibold mb-2">Smart Linking</h3>
				<p class="text-sm text-muted-foreground">
					Copy-first approach with optional symlinks for better Docker compatibility and deployment.
				</p>
			</div>
			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-purple-100 rounded-lg flex items-center justify-center mb-4">
					<Icon icon="solar:code-bold" class="w-6 h-6 text-purple-600" />
				</div>
				<h3 class="font-semibold mb-2">TypeScript Native</h3>
				<p class="text-sm text-muted-foreground">
					Automatic tsconfig.json path mapping and alias configuration for seamless TypeScript development.
				</p>
			</div>
		</div>
	</section>

	<!-- Quick Overview -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Quick Overview</h2>
		<div class="space-y-6">
			<div class="bg-muted/30 rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3 flex items-center">
					<Icon icon="solar:folder-bold" class="w-5 h-5 mr-2 text-blue-600" />
					Project Structure
				</h3>
				<pre class="text-sm font-mono bg-black/90 text-white p-4 rounded overflow-x-auto"><code>MyProject/
├── <span class="text-blue-400">knot.yml</span>          # Project configuration
├── <span class="text-yellow-400">packages/</span>        # Reusable packages
│   ├── types/
│   └── utils/
└── <span class="text-green-400">apps/</span>             # Applications
    └── frontend/</code></pre>
			</div>

			<div class="bg-muted/30 rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3 flex items-center">
					<Icon icon="solar:terminal-bold" class="w-5 h-5 mr-2 text-gray-600" />
					Common Workflow
				</h3>
				<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded overflow-x-auto">
					<pre><code># Initialize a new project
knot init MyProject

# Create packages and apps
knot init:package utils
knot init:app frontend

# Link packages and set up TypeScript
knot link

# Build everything
knot run build

# Publish packages
knot publish</code></pre>
				</div>
			</div>
		</div>
	</section>

	<!-- Key Features -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Key Features</h2>
		<div class="grid grid-cols-1 md:grid-cols-2 gap-8">
			<div>
				<h3 class="text-lg font-semibold mb-4 flex items-center">
					<Icon icon="solar:layers-bold" class="w-5 h-5 mr-2 text-orange-600" />
					Package Management
				</h3>
				<ul class="space-y-2 text-sm text-muted-foreground">
					<li class="flex items-start">
						<Icon icon="solar:check-circle-bold" class="w-4 h-4 mr-2 text-green-600 mt-0.5 flex-shrink-0" />
						Local package linking with copy or symlink modes
					</li>
					<li class="flex items-start">
						<Icon icon="solar:check-circle-bold" class="w-4 h-4 mr-2 text-green-600 mt-0.5 flex-shrink-0" />
						Remote package registry with team collaboration
					</li>
					<li class="flex items-start">
						<Icon icon="solar:check-circle-bold" class="w-4 h-4 mr-2 text-green-600 mt-0.5 flex-shrink-0" />
						Automatic dependency resolution and updates
					</li>
					<li class="flex items-start">
						<Icon icon="solar:check-circle-bold" class="w-4 h-4 mr-2 text-green-600 mt-0.5 flex-shrink-0" />
						Version management and publishing workflows
					</li>
				</ul>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-4 flex items-center">
					<Icon icon="solar:code-2-bold" class="w-5 h-5 mr-2 text-purple-600" />
					Developer Experience
				</h3>
				<ul class="space-y-2 text-sm text-muted-foreground">
					<li class="flex items-start">
						<Icon icon="solar:check-circle-bold" class="w-4 h-4 mr-2 text-green-600 mt-0.5 flex-shrink-0" />
						Automatic TypeScript configuration and path mapping
					</li>
					<li class="flex items-start">
						<Icon icon="solar:check-circle-bold" class="w-4 h-4 mr-2 text-green-600 mt-0.5 flex-shrink-0" />
						Context-aware script execution with fallbacks
					</li>
					<li class="flex items-start">
						<Icon icon="solar:check-circle-bold" class="w-4 h-4 mr-2 text-green-600 mt-0.5 flex-shrink-0" />
						Parallel build execution and caching
					</li>
					<li class="flex items-start">
						<Icon icon="solar:check-circle-bold" class="w-4 h-4 mr-2 text-green-600 mt-0.5 flex-shrink-0" />
						Docker-friendly package copying for production
					</li>
				</ul>
			</div>
		</div>
	</section>

	<!-- Getting Started -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Getting Started</h2>
		<div class="bg-blue-50 border border-blue-200 rounded-lg p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="solar:rocket-2-bold" class="w-6 h-6 text-blue-600 mt-1 flex-shrink-0" />
				<div>
					<h3 class="font-semibold text-blue-900 mb-2">Ready to start?</h3>
					<p class="text-sm text-blue-700 mb-4">
						Follow our step-by-step guides to get up and running with Knot in minutes.
					</p>
					<div class="flex flex-wrap gap-3">
						<a href="/docs/installation" class="inline-flex items-center px-4 py-2 bg-blue-600 text-white text-sm font-medium rounded-md hover:bg-blue-700 transition-colors">
							<Icon icon="solar:download-bold" class="w-4 h-4 mr-2" />
							Installation Guide
						</a>
						<a href="/docs/quick-start" class="inline-flex items-center px-4 py-2 border border-blue-600 text-blue-600 text-sm font-medium rounded-md hover:bg-blue-50 transition-colors">
							<Icon icon="solar:play-bold" class="w-4 h-4 mr-2" />
							Quick Start
						</a>
					</div>
				</div>
			</div>
		</div>
	</section>
</div>

