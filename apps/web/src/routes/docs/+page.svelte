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
		<div class="inline-flex items-center justify-center w-16 h-16 bg-gradient-to-br from-blue-500 to-purple-600 rounded-2xl shadow-lg mb-6">
			<Icon icon="lucide:wand-2" class="w-8 h-8 text-white" />
		</div>
		<h1 class="text-5xl lg:text-6xl font-bold tracking-tight mb-6 bg-gradient-to-r from-slate-900 via-blue-900 to-purple-900 bg-clip-text text-transparent" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
			Introduction to Knot
		</h1>
		<p class="text-xl lg:text-2xl text-slate-600 max-w-3xl mx-auto leading-relaxed">
			A modern monorepo package manager for any programming language
		</p>
	</div>

	<!-- What is Knot -->
	<section class="mb-20">
		<div class="text-center mb-12">
			<h2 class="text-3xl lg:text-4xl font-bold mb-4 text-slate-900">What is Knot?</h2>
			<div class="w-24 h-1 bg-gradient-to-r from-blue-500 to-purple-600 rounded-full mx-auto"></div>
		</div>
		<div class="bg-white/60 backdrop-blur-sm rounded-3xl p-8 lg:p-12 shadow-xl border border-white/20 mb-12">
			<p class="text-lg lg:text-xl text-slate-700 mb-0 leading-relaxed text-center max-w-4xl mx-auto">
				Knot is a modern package manager designed specifically for TypeScript and JavaScript monorepos.
				It simplifies dependency management, package linking, and build orchestration across multiple
				applications and packages within a single repository.
			</p>
		</div>

		<div class="grid grid-cols-1 md:grid-cols-3 gap-8 mb-8">
			<div class="group relative">
				<div class="absolute -inset-0.5 bg-gradient-to-r from-blue-500 to-purple-600 rounded-2xl opacity-0 group-hover:opacity-100 transition-opacity duration-300 blur"></div>
				<div class="relative bg-white rounded-2xl p-8 shadow-lg border border-slate-100 hover:shadow-2xl transition-all duration-300 h-full">
					<div class="w-14 h-14 bg-gradient-to-br from-blue-500 to-blue-600 rounded-2xl flex items-center justify-center mb-6 shadow-lg">
						<Icon icon="lucide:zap" class="w-7 h-7 text-white" />
					</div>
					<h3 class="text-xl font-bold mb-3 text-slate-900">Fast & Efficient</h3>
					<p class="text-slate-600 leading-relaxed">
						Smart caching and parallel operations make development faster than traditional package managers.
					</p>
				</div>
			</div>
			<div class="group relative">
				<div class="absolute -inset-0.5 bg-gradient-to-r from-green-500 to-emerald-600 rounded-2xl opacity-0 group-hover:opacity-100 transition-opacity duration-300 blur"></div>
				<div class="relative bg-white rounded-2xl p-8 shadow-lg border border-slate-100 hover:shadow-2xl transition-all duration-300 h-full">
					<div class="w-14 h-14 bg-gradient-to-br from-green-500 to-emerald-600 rounded-2xl flex items-center justify-center mb-6 shadow-lg">
						<Icon icon="lucide:link" class="w-7 h-7 text-white" />
					</div>
					<h3 class="text-xl font-bold mb-3 text-slate-900">Smart Linking</h3>
					<p class="text-slate-600 leading-relaxed">
						Copy-first approach with optional symlinks for better Docker compatibility and deployment.
					</p>
				</div>
			</div>
			<div class="group relative">
				<div class="absolute -inset-0.5 bg-gradient-to-r from-purple-500 to-indigo-600 rounded-2xl opacity-0 group-hover:opacity-100 transition-opacity duration-300 blur"></div>
				<div class="relative bg-white rounded-2xl p-8 shadow-lg border border-slate-100 hover:shadow-2xl transition-all duration-300 h-full">
					<div class="w-14 h-14 bg-gradient-to-br from-purple-500 to-indigo-600 rounded-2xl flex items-center justify-center mb-6 shadow-lg">
						<Icon icon="lucide:code" class="w-7 h-7 text-white" />
					</div>
					<h3 class="text-xl font-bold mb-3 text-slate-900">TypeScript Native</h3>
					<p class="text-slate-600 leading-relaxed">
						Automatic tsconfig.json path mapping and alias configuration for seamless TypeScript development.
					</p>
				</div>
			</div>
		</div>
	</section>

	<!-- Quick Overview -->
	<section class="mb-20">
		<div class="text-center mb-12">
			<h2 class="text-3xl lg:text-4xl font-bold mb-4 text-slate-900">Quick Overview</h2>
			<div class="w-24 h-1 bg-gradient-to-r from-blue-500 to-purple-600 rounded-full mx-auto"></div>
		</div>
		<div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
			<div class="group">
				<div class="bg-white/70 backdrop-blur-sm rounded-3xl p-8 shadow-xl border border-white/20 hover:shadow-2xl transition-all duration-300">
					<div class="flex items-center mb-6">
						<div class="w-12 h-12 bg-gradient-to-br from-blue-500 to-blue-600 rounded-xl flex items-center justify-center shadow-lg mr-4">
							<Icon icon="lucide:folder" class="w-6 h-6 text-white" />
						</div>
						<h3 class="text-xl font-bold text-slate-900">Project Structure</h3>
					</div>
					<div class="relative">
						<div class="absolute inset-0 bg-gradient-to-br from-slate-900/10 to-slate-800/10 rounded-2xl"></div>
						<pre class="relative text-sm font-mono bg-slate-900 text-white p-6 rounded-2xl overflow-x-auto shadow-inner"><code>MyProject/
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
				<div class="bg-white/70 backdrop-blur-sm rounded-3xl p-8 shadow-xl border border-white/20 hover:shadow-2xl transition-all duration-300">
					<div class="flex items-center mb-6">
						<div class="w-12 h-12 bg-gradient-to-br from-emerald-500 to-green-600 rounded-xl flex items-center justify-center shadow-lg mr-4">
							<Icon icon="lucide:terminal" class="w-6 h-6 text-white" />
						</div>
						<h3 class="text-xl font-bold text-slate-900">Common Workflow</h3>
					</div>
					<div class="relative">
						<div class="absolute inset-0 bg-gradient-to-br from-slate-900/10 to-slate-800/10 rounded-2xl"></div>
						<div class="relative bg-slate-900 text-green-400 font-mono text-sm p-6 rounded-2xl overflow-x-auto shadow-inner">
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
			<h2 class="text-3xl lg:text-4xl font-bold mb-4 text-slate-900">Key Features</h2>
			<div class="w-24 h-1 bg-gradient-to-r from-blue-500 to-purple-600 rounded-full mx-auto"></div>
		</div>
		<div class="grid grid-cols-1 lg:grid-cols-2 gap-10">
			<div class="bg-white/70 backdrop-blur-sm rounded-3xl p-8 shadow-xl border border-white/20 hover:shadow-2xl transition-all duration-300 group">
				<div class="flex items-center mb-6">
					<div class="w-12 h-12 bg-gradient-to-br from-orange-500 to-red-500 rounded-xl flex items-center justify-center shadow-lg mr-4 group-hover:scale-110 transition-transform duration-300">
						<Icon icon="lucide:layers" class="w-6 h-6 text-white" />
					</div>
					<h3 class="text-xl font-bold text-slate-900">Package Management</h3>
				</div>
				<div class="space-y-4">
					<div class="flex items-start group/item">
						<div class="w-6 h-6 bg-gradient-to-br from-green-500 to-emerald-600 rounded-lg flex items-center justify-center mr-3 mt-0.5 shadow-sm group-hover/item:scale-110 transition-transform duration-200">
							<Icon icon="lucide:check-circle" class="w-3.5 h-3.5 text-white" />
						</div>
						<p class="text-slate-700 leading-relaxed">Local package linking with copy or symlink modes</p>
					</div>
					<div class="flex items-start group/item">
						<div class="w-6 h-6 bg-gradient-to-br from-green-500 to-emerald-600 rounded-lg flex items-center justify-center mr-3 mt-0.5 shadow-sm group-hover/item:scale-110 transition-transform duration-200">
							<Icon icon="lucide:check-circle" class="w-3.5 h-3.5 text-white" />
						</div>
						<p class="text-slate-700 leading-relaxed">Remote package registry with team collaboration</p>
					</div>
					<div class="flex items-start group/item">
						<div class="w-6 h-6 bg-gradient-to-br from-green-500 to-emerald-600 rounded-lg flex items-center justify-center mr-3 mt-0.5 shadow-sm group-hover/item:scale-110 transition-transform duration-200">
							<Icon icon="lucide:check-circle" class="w-3.5 h-3.5 text-white" />
						</div>
						<p class="text-slate-700 leading-relaxed">Automatic dependency resolution and updates</p>
					</div>
					<div class="flex items-start group/item">
						<div class="w-6 h-6 bg-gradient-to-br from-green-500 to-emerald-600 rounded-lg flex items-center justify-center mr-3 mt-0.5 shadow-sm group-hover/item:scale-110 transition-transform duration-200">
							<Icon icon="lucide:check-circle" class="w-3.5 h-3.5 text-white" />
						</div>
						<p class="text-slate-700 leading-relaxed">Version management and publishing workflows</p>
					</div>
				</div>
			</div>

			<div class="bg-white/70 backdrop-blur-sm rounded-3xl p-8 shadow-xl border border-white/20 hover:shadow-2xl transition-all duration-300 group">
				<div class="flex items-center mb-6">
					<div class="w-12 h-12 bg-gradient-to-br from-purple-500 to-indigo-600 rounded-xl flex items-center justify-center shadow-lg mr-4 group-hover:scale-110 transition-transform duration-300">
						<Icon icon="lucide:code-2" class="w-6 h-6 text-white" />
					</div>
					<h3 class="text-xl font-bold text-slate-900">Developer Experience</h3>
				</div>
				<div class="space-y-4">
					<div class="flex items-start group/item">
						<div class="w-6 h-6 bg-gradient-to-br from-green-500 to-emerald-600 rounded-lg flex items-center justify-center mr-3 mt-0.5 shadow-sm group-hover/item:scale-110 transition-transform duration-200">
							<Icon icon="lucide:check-circle" class="w-3.5 h-3.5 text-white" />
						</div>
						<p class="text-slate-700 leading-relaxed">Automatic TypeScript configuration and path mapping</p>
					</div>
					<div class="flex items-start group/item">
						<div class="w-6 h-6 bg-gradient-to-br from-green-500 to-emerald-600 rounded-lg flex items-center justify-center mr-3 mt-0.5 shadow-sm group-hover/item:scale-110 transition-transform duration-200">
							<Icon icon="lucide:check-circle" class="w-3.5 h-3.5 text-white" />
						</div>
						<p class="text-slate-700 leading-relaxed">Context-aware script execution with fallbacks</p>
					</div>
					<div class="flex items-start group/item">
						<div class="w-6 h-6 bg-gradient-to-br from-green-500 to-emerald-600 rounded-lg flex items-center justify-center mr-3 mt-0.5 shadow-sm group-hover/item:scale-110 transition-transform duration-200">
							<Icon icon="lucide:check-circle" class="w-3.5 h-3.5 text-white" />
						</div>
						<p class="text-slate-700 leading-relaxed">Parallel build execution and caching</p>
					</div>
					<div class="flex items-start group/item">
						<div class="w-6 h-6 bg-gradient-to-br from-green-500 to-emerald-600 rounded-lg flex items-center justify-center mr-3 mt-0.5 shadow-sm group-hover/item:scale-110 transition-transform duration-200">
							<Icon icon="lucide:check-circle" class="w-3.5 h-3.5 text-white" />
						</div>
						<p class="text-slate-700 leading-relaxed">Docker-friendly package copying for production</p>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Getting Started -->
	<section class="mb-20">
		<div class="relative">
			<div class="absolute inset-0 bg-gradient-to-r from-blue-600/10 via-purple-600/10 to-indigo-600/10 rounded-3xl blur-3xl"></div>
			<div class="relative bg-gradient-to-br from-blue-50 to-purple-50 border border-blue-200/50 rounded-3xl p-8 lg:p-12 shadow-xl">
				<div class="flex items-start space-x-6">
					<div class="flex-shrink-0">
						<div class="w-16 h-16 bg-gradient-to-br from-blue-500 to-purple-600 rounded-2xl flex items-center justify-center shadow-xl">
							<Icon icon="lucide:rocket" class="w-8 h-8 text-white" />
						</div>
					</div>
					<div class="flex-1">
						<h3 class="text-2xl font-bold text-slate-900 mb-3">Ready to start?</h3>
						<p class="text-lg text-slate-700 mb-8 leading-relaxed">
							Follow our step-by-step guides to get up and running with Knot in minutes.
						</p>
						<div class="flex flex-col sm:flex-row gap-4">
							<a href="/docs/installation" class="group inline-flex items-center justify-center px-8 py-4 bg-black text-white font-semibold rounded-2xl hover:bg-gray-800 transition-all duration-300 shadow-lg hover:shadow-xl hover:scale-105">
								<Icon icon="lucide:download" class="w-5 h-5 mr-3 group-hover:rotate-12 transition-transform duration-300" />
								Installation Guide
							</a>
							<a href="/docs/quick-start" class="group inline-flex items-center justify-center px-8 py-4 border-2 border-blue-600/30 text-slate-700 font-semibold rounded-2xl hover:bg-white/80 hover:border-blue-600/50 transition-all duration-300 shadow-lg hover:shadow-xl hover:scale-105 backdrop-blur-sm">
								<Icon icon="lucide:play" class="w-5 h-5 mr-3 group-hover:scale-110 transition-transform duration-300" />
								Quick Start
							</a>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>
</div>

