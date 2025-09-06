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

<div class="max-w-5xl mx-auto py-12 px-4 sm:px-6 lg:px-8">
	<div class="mb-16 text-center">
		<div class="inline-flex items-center justify-center w-16 h-16 bg-white border border-gray-200 rounded-2xl shadow-sm mb-6">
			<Icon icon="lucide:wand-2" class="w-8 h-8 text-gray-700" />
		</div>
		<h1 class="text-5xl lg:text-6xl font-bold tracking-tight mb-6 text-black" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
			Introduction to Knot
		</h1>
		<p class="text-xl lg:text-2xl text-gray-600 max-w-3xl mx-auto leading-relaxed">
			A modern monorepo package manager for any programming language
		</p>
	</div>

	<!-- What is Knot -->
	<section class="mb-20">
		<div class="text-center mb-12">
			<h2 class="text-3xl lg:text-4xl font-bold mb-4 text-black">What is Knot?</h2>
			<div class="w-24 h-1 bg-black rounded-full mx-auto"></div>
		</div>
		<div class="bg-white rounded-3xl p-8 lg:p-12 shadow-sm border border-gray-200 mb-12">
			<p class="text-lg lg:text-xl text-slate-700 mb-0 leading-relaxed text-center max-w-4xl mx-auto">
				Knot is a modern package manager designed specifically for TypeScript and JavaScript monorepos.
				It simplifies dependency management, package linking, and build orchestration across multiple
				applications and packages within a single repository.
			</p>
		</div>

		<div class="grid grid-cols-1 md:grid-cols-3 gap-8 mb-8">
			<div class="bg-white rounded-2xl p-8 shadow-sm border border-gray-200 hover:shadow-md transition-all duration-300 h-full">
				<div class="w-14 h-14 bg-blue-100 rounded-2xl flex items-center justify-center mb-6">
					<Icon icon="lucide:zap" class="w-7 h-7 text-blue-600" />
				</div>
				<h3 class="text-xl font-bold mb-3 text-black">Fast & Efficient</h3>
				<p class="text-gray-600 leading-relaxed">
					Smart caching and parallel operations make development faster than traditional package managers.
				</p>
			</div>
			<div class="bg-white rounded-2xl p-8 shadow-sm border border-gray-200 hover:shadow-md transition-all duration-300 h-full">
				<div class="w-14 h-14 bg-green-100 rounded-2xl flex items-center justify-center mb-6">
					<Icon icon="lucide:link" class="w-7 h-7 text-green-600" />
				</div>
				<h3 class="text-xl font-bold mb-3 text-black">Smart Linking</h3>
				<p class="text-gray-600 leading-relaxed">
					Copy-first approach with optional symlinks for better Docker compatibility and deployment.
				</p>
			</div>
			<div class="bg-white rounded-2xl p-8 shadow-sm border border-gray-200 hover:shadow-md transition-all duration-300 h-full">
				<div class="w-14 h-14 bg-purple-100 rounded-2xl flex items-center justify-center mb-6">
					<Icon icon="lucide:code" class="w-7 h-7 text-purple-600" />
				</div>
				<h3 class="text-xl font-bold mb-3 text-black">TypeScript Native</h3>
				<p class="text-gray-600 leading-relaxed">
					Automatic tsconfig.json path mapping and alias configuration for seamless TypeScript development.
				</p>
			</div>
		</div>
	</section>

	<!-- Quick Overview -->
	<section class="mb-20">
		<div class="text-center mb-12">
			<h2 class="text-3xl lg:text-4xl font-bold mb-4 text-black">Quick Overview</h2>
			<div class="w-24 h-1 bg-black rounded-full mx-auto"></div>
		</div>
		<div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
			<div class="group">
				<div class="bg-white rounded-3xl p-8 shadow-sm border border-gray-200 hover:shadow-md transition-all duration-300">
					<div class="flex items-center mb-6">
						<div class="w-12 h-12 bg-blue-100 rounded-xl flex items-center justify-center mr-4">
							<Icon icon="lucide:folder" class="w-6 h-6 text-blue-600" />
						</div>
						<h3 class="text-xl font-bold text-black">Project Structure</h3>
					</div>
					<div class="relative">
						<pre class="text-sm font-mono bg-slate-900 text-white p-6 rounded-2xl overflow-x-auto"><code>MyProject/
├── <span class="text-blue-400">knot.yml</span>          # Project configuration
├── <span class="text-yellow-400">packages/</span>        # Reusable packages
│   ├── types/
│   └── utils/
└── <span class="text-green-400">apps/</span>             # Applications
    └── frontend/</code></pre>
					</div>
				</div>
			</div>

			<div class="group">
				<div class="bg-white rounded-3xl p-8 shadow-sm border border-gray-200 hover:shadow-md transition-all duration-300">
					<div class="flex items-center mb-6">
						<div class="w-12 h-12 bg-green-100 rounded-xl flex items-center justify-center mr-4">
							<Icon icon="lucide:terminal" class="w-6 h-6 text-green-600" />
						</div>
						<h3 class="text-xl font-bold text-black">Common Workflow</h3>
					</div>
					<div class="relative">
						<div class="bg-slate-900 text-green-400 font-mono text-sm p-6 rounded-2xl overflow-x-auto">
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
			</div>
		</div>
	</section>

	<!-- Key Features -->
	<section class="mb-20">
		<div class="text-center mb-12">
			<h2 class="text-3xl lg:text-4xl font-bold mb-4 text-black">Key Features</h2>
			<div class="w-24 h-1 bg-black rounded-full mx-auto"></div>
		</div>
		<div class="grid grid-cols-1 lg:grid-cols-2 gap-10">
			<div class="bg-white rounded-3xl p-8 shadow-sm border border-gray-200 hover:shadow-md transition-all duration-300 group">
				<div class="flex items-center mb-6">
					<div class="w-12 h-12 bg-orange-100 rounded-xl flex items-center justify-center mr-4">
						<Icon icon="lucide:layers" class="w-6 h-6 text-orange-600" />
					</div>
					<h3 class="text-xl font-bold text-black">Package Management</h3>
				</div>
				<div class="space-y-4">
					<div class="flex items-start group/item">
						<div class="w-6 h-6 bg-green-100 rounded-lg flex items-center justify-center mr-3 mt-0.5">
							<Icon icon="lucide:check-circle" class="w-3.5 h-3.5 text-green-600" />
						</div>
						<p class="text-gray-600 leading-relaxed">Local package linking with copy or symlink modes</p>
					</div>
					<div class="flex items-start group/item">
						<div class="w-6 h-6 bg-green-100 rounded-lg flex items-center justify-center mr-3 mt-0.5">
							<Icon icon="lucide:check-circle" class="w-3.5 h-3.5 text-green-600" />
						</div>
						<p class="text-gray-600 leading-relaxed">Remote package registry with team collaboration</p>
					</div>
					<div class="flex items-start group/item">
						<div class="w-6 h-6 bg-green-100 rounded-lg flex items-center justify-center mr-3 mt-0.5">
							<Icon icon="lucide:check-circle" class="w-3.5 h-3.5 text-green-600" />
						</div>
						<p class="text-gray-600 leading-relaxed">Automatic dependency resolution and updates</p>
					</div>
					<div class="flex items-start group/item">
						<div class="w-6 h-6 bg-green-100 rounded-lg flex items-center justify-center mr-3 mt-0.5">
							<Icon icon="lucide:check-circle" class="w-3.5 h-3.5 text-green-600" />
						</div>
						<p class="text-gray-600 leading-relaxed">Version management and publishing workflows</p>
					</div>
				</div>
			</div>

			<div class="bg-white rounded-3xl p-8 shadow-sm border border-gray-200 hover:shadow-md transition-all duration-300 group">
				<div class="flex items-center mb-6">
					<div class="w-12 h-12 bg-purple-100 rounded-xl flex items-center justify-center mr-4">
						<Icon icon="lucide:code-2" class="w-6 h-6 text-purple-600" />
					</div>
					<h3 class="text-xl font-bold text-black">Developer Experience</h3>
				</div>
				<div class="space-y-4">
					<div class="flex items-start group/item">
						<div class="w-6 h-6 bg-green-100 rounded-lg flex items-center justify-center mr-3 mt-0.5">
							<Icon icon="lucide:check-circle" class="w-3.5 h-3.5 text-green-600" />
						</div>
						<p class="text-gray-600 leading-relaxed">Automatic TypeScript configuration and path mapping</p>
					</div>
					<div class="flex items-start group/item">
						<div class="w-6 h-6 bg-green-100 rounded-lg flex items-center justify-center mr-3 mt-0.5">
							<Icon icon="lucide:check-circle" class="w-3.5 h-3.5 text-green-600" />
						</div>
						<p class="text-gray-600 leading-relaxed">Context-aware script execution with fallbacks</p>
					</div>
					<div class="flex items-start group/item">
						<div class="w-6 h-6 bg-green-100 rounded-lg flex items-center justify-center mr-3 mt-0.5">
							<Icon icon="lucide:check-circle" class="w-3.5 h-3.5 text-green-600" />
						</div>
						<p class="text-gray-600 leading-relaxed">Parallel build execution and caching</p>
					</div>
					<div class="flex items-start group/item">
						<div class="w-6 h-6 bg-green-100 rounded-lg flex items-center justify-center mr-3 mt-0.5">
							<Icon icon="lucide:check-circle" class="w-3.5 h-3.5 text-green-600" />
						</div>
						<p class="text-gray-600 leading-relaxed">Docker-friendly package copying for production</p>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Getting Started -->
	<section class="mb-20">
		<div class="bg-white rounded-3xl p-8 lg:p-12 shadow-sm border border-gray-200">
			<div class="flex items-start space-x-6">
				<div class="flex-shrink-0">
					<div class="w-16 h-16 bg-blue-100 rounded-2xl flex items-center justify-center">
						<Icon icon="lucide:rocket" class="w-8 h-8 text-blue-600" />
					</div>
				</div>
				<div class="flex-1">
					<h3 class="text-2xl font-bold text-black mb-3">Ready to start?</h3>
					<p class="text-lg text-gray-600 mb-8 leading-relaxed">
						Follow our step-by-step guides to get up and running with Knot in minutes.
					</p>
					<div class="flex flex-col sm:flex-row gap-4">
						<a href="/docs/installation" class="group inline-flex items-center justify-center px-8 py-4 bg-black text-white font-semibold rounded-2xl hover:bg-gray-800 transition-all duration-300">
							<Icon icon="lucide:download" class="w-5 h-5 mr-3" />
							Installation Guide
						</a>
						<a href="/docs/quick-start" class="group inline-flex items-center justify-center px-8 py-4 border-2 border-gray-300 text-gray-700 font-semibold rounded-2xl hover:bg-gray-50 hover:border-gray-400 transition-all duration-300">
							<Icon icon="lucide:play" class="w-5 h-5 mr-3" />
							Quick Start
						</a>
					</div>
				</div>
			</div>
		</div>
	</section>
</div>

