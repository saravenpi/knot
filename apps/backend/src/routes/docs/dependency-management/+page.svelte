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
	<title>Dependency Management - Knot CLI Documentation</title>
	<meta name="description" content="Complete guide to managing dependencies in Knot projects. Learn to add, resolve, check, and sync dependencies across your monorepo with advanced dependency management features." />
	<meta property="og:title" content="Dependency Management - Knot CLI" />
	<meta property="og:description" content="Complete guide to managing dependencies in Knot projects. Learn to add, resolve, check, and sync dependencies across your monorepo with advanced dependency management features." />
	<meta property="og:image" content="/images/og/dependency-management.png" />
	<meta property="og:url" content="https://knot.klysium.com/docs/dependency-management" />
	<meta name="twitter:title" content="Dependency Management - Knot CLI" />
	<meta name="twitter:description" content="Complete guide to managing dependencies in Knot projects. Learn to add, resolve, check, and sync dependencies across your monorepo with advanced dependency management features." />
	<meta name="twitter:image" content="/images/og/dependency-management.png" />
	<link rel="canonical" href="https://knot.klysium.com/docs/dependency-management" />
</svelte:head>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight mb-4" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
			Dependency Management
		</h1>
		<p class="text-lg text-muted-foreground">
			Advanced dependency management for monorepos with intelligent resolution, tree analysis, and cross-app synchronization
		</p>
	</div>

	<!-- Overview -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Overview</h2>
		<div class="bg-blue-50 border border-blue-200 rounded-lg p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="solar:info-circle-bold" class="w-6 h-6 text-blue-600 mt-1 flex-shrink-0" />
				<div>
					<h3 class="font-semibold text-blue-900 mb-2">Smart Dependency Management</h3>
					<p class="text-sm text-blue-700 mb-3">
						Knot provides powerful dependency management tools that understand your monorepo structure.
						All commands work intelligently across apps and packages, with support for version resolution,
						conflict detection, and automated synchronization.
					</p>
					<ul class="text-sm text-blue-700 space-y-1">
						<li>• <strong>Context-aware:</strong> Commands detect your current app/package automatically</li>
						<li>• <strong>Version-smart:</strong> Advanced resolution strategies for complex dependency trees</li>
						<li>• <strong>Cross-app sync:</strong> Keep dependencies consistent across your monorepo</li>
						<li>• <strong>Tree analysis:</strong> Visual dependency tree with configurable depth</li>
					</ul>
				</div>
			</div>
		</div>
	</section>

	<!-- Command Categories -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Dependency Commands</h2>
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-green-100 rounded-lg flex items-center justify-center mb-4">
					<Icon icon="solar:add-circle-bold" class="w-6 h-6 text-green-600" />
				</div>
				<h3 class="font-semibold mb-2">Adding Dependencies</h3>
				<p class="text-sm text-muted-foreground mb-3">
					Add dependencies with support for dev, optional, and version constraints.
				</p>
				<div class="text-xs space-y-1">
					<div><code>knot deps add</code></div>
					<div><code>knot install</code> (alias)</div>
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-blue-100 rounded-lg flex items-center justify-center mb-4">
					<Icon icon="solar:list-bold" class="w-6 h-6 text-blue-600" />
				</div>
				<h3 class="font-semibold mb-2">Listing & Analysis</h3>
				<p class="text-sm text-muted-foreground mb-3">
					View dependency lists, trees, and analyze package relationships.
				</p>
				<div class="text-xs space-y-1">
					<div><code>knot deps list</code></div>
					<div><code>knot deps tree</code></div>
					<div><code>knot deps why</code></div>
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-purple-100 rounded-lg flex items-center justify-center mb-4">
					<Icon icon="solar:settings-bold" class="w-6 h-6 text-purple-600" />
				</div>
				<h3 class="font-semibold mb-2">Resolution & Sync</h3>
				<p class="text-sm text-muted-foreground mb-3">
					Advanced resolution strategies and cross-app synchronization.
				</p>
				<div class="text-xs space-y-1">
					<div><code>knot deps resolve</code></div>
					<div><code>knot deps sync</code></div>
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-orange-100 rounded-lg flex items-center justify-center mb-4">
					<Icon icon="solar:shield-check-bold" class="w-6 h-6 text-orange-600" />
				</div>
				<h3 class="font-semibold mb-2">Health Checks</h3>
				<p class="text-sm text-muted-foreground mb-3">
					Check for issues, outdated packages, and dependency problems.
				</p>
				<div class="text-xs space-y-1">
					<div><code>knot deps check</code></div>
					<div><code>knot deps outdated</code></div>
				</div>
			</div>
		</div>
	</section>

	<!-- Adding Dependencies -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Adding Dependencies</h2>

		<div class="space-y-8">
			<!-- deps add -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">knot deps add</h3>
				<p class="text-muted-foreground mb-4">
					Add dependencies to your app with support for version constraints, dev dependencies, and optional dependencies.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Basic Usage</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot deps add utils</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot deps add utils')}
							>
								{#if showCopied && copyText === 'knot deps add utils'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">With Version Constraints</h4>
						<div class="space-y-2">
							<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
								<code>knot deps add utils@^1.2.0</code>
								<button
									class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
									on:click={() => copyToClipboard('knot deps add utils@^1.2.0')}
								>
									{#if showCopied && copyText === 'knot deps add utils@^1.2.0'}
										<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
									{:else}
										<Icon icon="solar:copy-bold" class="w-4 h-4" />
									{/if}
								</button>
							</div>
							<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
								<code>knot deps add @team/shared-lib@~2.1.0</code>
								<button
									class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
									on:click={() => copyToClipboard('knot deps add @team/shared-lib@~2.1.0')}
								>
									{#if showCopied && copyText === 'knot deps add @team/shared-lib@~2.1.0'}
										<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
									{:else}
										<Icon icon="solar:copy-bold" class="w-4 h-4" />
									{/if}
								</button>
							</div>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Development Dependencies</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot deps add test-utils --dev</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot deps add test-utils --dev')}
							>
								{#if showCopied && copyText === 'knot deps add test-utils --dev'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Optional Dependencies</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot deps add analytics --optional</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot deps add analytics --optional')}
							>
								{#if showCopied && copyText === 'knot deps add analytics --optional'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Target Specific App</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot deps add ui-components --app frontend</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot deps add ui-components --app frontend')}
							>
								{#if showCopied && copyText === 'knot deps add ui-components --app frontend'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Options</h4>
						<div class="text-sm space-y-2">
							<div class="flex items-start space-x-3">
								<code class="bg-muted px-2 py-1 rounded">--dev</code>
								<span class="text-muted-foreground">Add as development dependency</span>
							</div>
							<div class="flex items-start space-x-3">
								<code class="bg-muted px-2 py-1 rounded">--optional</code>
								<span class="text-muted-foreground">Add as optional dependency</span>
							</div>
							<div class="flex items-start space-x-3">
								<code class="bg-muted px-2 py-1 rounded">--app</code>
								<span class="text-muted-foreground">Target specific app (detects from current directory if omitted)</span>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Listing Dependencies -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Listing & Analysis</h2>

		<div class="space-y-8">
			<!-- deps list -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">knot deps list</h3>
				<p class="text-muted-foreground mb-4">
					List dependencies across your monorepo with optional tree view and depth control.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">List All Dependencies</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot deps list</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot deps list')}
							>
								{#if showCopied && copyText === 'knot deps list'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">List for Specific App</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot deps list frontend</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot deps list frontend')}
							>
								{#if showCopied && copyText === 'knot deps list frontend'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Tree View</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot deps list --tree</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot deps list --tree')}
							>
								{#if showCopied && copyText === 'knot deps list --tree'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Limited Depth Tree</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot deps list --tree --depth 2</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot deps list --tree --depth 2')}
							>
								{#if showCopied && copyText === 'knot deps list --tree --depth 2'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Example Output</h4>
						<div class="bg-black/90 text-white font-mono text-xs p-4 rounded-lg">
							<pre><code><span class="text-green-400">📱 frontend</span>
<span class="text-blue-400">  Dependencies:</span>
    • utils (v1.2.3) - local
    • ui-components (v2.0.1) - local
    • lodash (v4.17.21) - npm
<span class="text-yellow-400">  Dev Dependencies:</span>
    • test-utils (v1.0.0) - local
    • @types/node (v18.15.0) - npm

<span class="text-green-400">📱 api</span>
<span class="text-blue-400">  Dependencies:</span>
    • utils (v1.2.3) - local
    • fastify (v4.15.0) - npm</code></pre>
						</div>
					</div>
				</div>
			</div>

			<!-- deps tree -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">knot deps tree</h3>
				<p class="text-muted-foreground mb-4">
					Display detailed dependency trees with visual hierarchy and relationship mapping.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Full Dependency Tree</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot deps tree</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot deps tree')}
							>
								{#if showCopied && copyText === 'knot deps tree'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">App-Specific Tree</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot deps tree frontend --depth 3</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot deps tree frontend --depth 3')}
							>
								{#if showCopied && copyText === 'knot deps tree frontend --depth 3'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Example Tree Output</h4>
						<div class="bg-black/90 text-white font-mono text-xs p-4 rounded-lg">
							<pre><code><span class="text-green-400">frontend</span>
├── <span class="text-blue-400">utils@^1.2.0</span> (1.2.3)
│   ├── <span class="text-gray-400">lodash@^4.17.0</span> (4.17.21)
│   └── <span class="text-gray-400">date-fns@^2.29.0</span> (2.29.3)
├── <span class="text-blue-400">ui-components@^2.0.0</span> (2.0.1)
│   ├── <span class="text-blue-400">utils@^1.2.0</span> (1.2.3) <span class="text-yellow-400">[shared]</span>
│   └── <span class="text-gray-400">react@^18.0.0</span> (18.2.0)
└── <span class="text-purple-400">test-utils@^1.0.0</span> (1.0.0) <span class="text-muted-foreground">[dev]</span>
    └── <span class="text-gray-400">vitest@^0.28.0</span> (0.28.5)</code></pre>
						</div>
					</div>
				</div>
			</div>

			<!-- deps why -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">knot deps why</h3>
				<p class="text-muted-foreground mb-4">
					Explain why a package is included in your dependency tree and trace its dependency path.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Explain Package Inclusion</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot deps why lodash</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot deps why lodash')}
							>
								{#if showCopied && copyText === 'knot deps why lodash'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">For Specific App</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot deps why react --app frontend</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot deps why react --app frontend')}
							>
								{#if showCopied && copyText === 'knot deps why react --app frontend'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Example Explanation</h4>
						<div class="bg-black/90 text-white font-mono text-xs p-4 rounded-lg">
							<pre><code><span class="text-green-400">📦 lodash@4.17.21</span> is included because:

<span class="text-blue-400">Path 1:</span> frontend → utils@^1.2.0 → lodash@^4.17.0
<span class="text-blue-400">Path 2:</span> api → data-utils@^1.0.0 → lodash@^4.17.0

<span class="text-yellow-400">Used by 2 apps:</span> frontend, api
<span class="text-purple-400">Required by 2 packages:</span> utils, data-utils
<span class="text-green-400">Resolved version:</span> 4.17.21 (satisfies all constraints)</code></pre>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Dependency Resolution -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Resolution & Management</h2>

		<div class="space-y-8">
			<!-- deps resolve -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">knot deps resolve</h3>
				<p class="text-muted-foreground mb-4">
					Resolve and install dependencies with configurable resolution strategies for handling version conflicts.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Default Resolution</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot deps resolve</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot deps resolve')}
							>
								{#if showCopied && copyText === 'knot deps resolve'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Resolution Strategies</h4>
						<div class="space-y-2">
							<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
								<code>knot deps resolve --strategy latest</code>
								<button
									class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
									on:click={() => copyToClipboard('knot deps resolve --strategy latest')}
								>
									{#if showCopied && copyText === 'knot deps resolve --strategy latest'}
										<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
									{:else}
										<Icon icon="solar:copy-bold" class="w-4 h-4" />
									{/if}
								</button>
							</div>
							<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
								<code>knot deps resolve --strategy strict</code>
								<button
									class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
									on:click={() => copyToClipboard('knot deps resolve --strategy strict')}
								>
									{#if showCopied && copyText === 'knot deps resolve --strategy strict'}
										<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
									{:else}
										<Icon icon="solar:copy-bold" class="w-4 h-4" />
									{/if}
								</button>
							</div>
							<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
								<code>knot deps resolve --strategy conservative</code>
								<button
									class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
									on:click={() => copyToClipboard('knot deps resolve --strategy conservative')}
								>
									{#if showCopied && copyText === 'knot deps resolve --strategy conservative'}
										<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
									{:else}
										<Icon icon="solar:copy-bold" class="w-4 h-4" />
									{/if}
								</button>
							</div>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Dry Run</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot deps resolve --dry-run</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot deps resolve --dry-run')}
							>
								{#if showCopied && copyText === 'knot deps resolve --dry-run'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">App-Specific Resolution</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot deps resolve --app frontend --strategy latest</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot deps resolve --app frontend --strategy latest')}
							>
								{#if showCopied && copyText === 'knot deps resolve --app frontend --strategy latest'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Resolution Strategies</h4>
						<div class="bg-muted/30 rounded-lg p-4">
							<div class="space-y-3 text-sm">
								<div>
									<strong class="text-green-600">compatible</strong> (default): Balance stability and freshness
								</div>
								<div>
									<strong class="text-blue-600">latest</strong>: Always use the latest available versions
								</div>
								<div>
									<strong class="text-orange-600">strict</strong>: Respect exact version constraints strictly
								</div>
								<div>
									<strong class="text-purple-600">conservative</strong>: Prefer existing installed versions
								</div>
							</div>
						</div>
					</div>
				</div>
			</div>

			<!-- deps sync -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">knot deps sync</h3>
				<p class="text-muted-foreground mb-4">
					Synchronize dependency versions across all apps in your monorepo to maintain consistency.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Sync All Dependencies</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot deps sync</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot deps sync')}
							>
								{#if showCopied && copyText === 'knot deps sync'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Example Sync Output</h4>
						<div class="bg-black/90 text-white font-mono text-xs p-4 rounded-lg">
							<pre><code><span class="text-green-400">🔄 Synchronizing dependencies...</span>

<span class="text-blue-400">Found version conflicts:</span>
  • lodash: frontend@4.17.20, api@4.17.21 → <span class="text-green-400">4.17.21</span>
  • react: frontend@18.2.0, admin@18.1.0 → <span class="text-green-400">18.2.0</span>

<span class="text-green-400">✅ Synchronized 2 packages across 3 apps</span>
<span class="text-yellow-400">💡 Run knot deps resolve to install updated versions</span></code></pre>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Health Checks -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Health Checks & Maintenance</h2>

		<div class="space-y-8">
			<!-- deps check -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">knot deps check</h3>
				<p class="text-muted-foreground mb-4">
					Comprehensive dependency health check to identify conflicts, missing packages, and potential issues.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Run Health Check</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot deps check</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot deps check')}
							>
								{#if showCopied && copyText === 'knot deps check'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Example Check Results</h4>
						<div class="bg-black/90 text-white font-mono text-xs p-4 rounded-lg">
							<pre><code><span class="text-green-400">🔍 Dependency Health Check</span>

<span class="text-green-400">✅ All apps have valid configurations</span>
<span class="text-yellow-400">⚠️  Version conflicts found:</span>
  • lodash: 3 different versions across apps
  • typescript: frontend@4.9.0, api@4.8.0

<span class="text-red-400">❌ Missing dependencies:</span>
  • frontend: missing @types/react for React development

<span class="text-blue-400">💡 Recommendations:</span>
  • Run knot deps sync to resolve version conflicts
  • Run knot deps add @types/react --dev --app frontend</code></pre>
						</div>
					</div>
				</div>
			</div>

			<!-- deps outdated -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">knot deps outdated</h3>
				<p class="text-muted-foreground mb-4">
					Find outdated dependencies across your apps with available updates and security information.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Check All Apps</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot deps outdated</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot deps outdated')}
							>
								{#if showCopied && copyText === 'knot deps outdated'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Check Specific App</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot deps outdated frontend</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot deps outdated frontend')}
							>
								{#if showCopied && copyText === 'knot deps outdated frontend'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Example Outdated Report</h4>
						<div class="bg-black/90 text-white font-mono text-xs p-4 rounded-lg">
							<pre><code><span class="text-green-400">📊 Outdated Dependencies Report</span>

<span class="text-blue-400">frontend:</span>
  • react: 18.1.0 → <span class="text-green-400">18.2.0</span> <span class="text-muted-foreground">(patch)</span>
  • lodash: 4.17.20 → <span class="text-yellow-400">4.17.21</span> <span class="text-red-400">(security fix)</span>
  • typescript: 4.8.0 → <span class="text-blue-400">4.9.4</span> <span class="text-muted-foreground">(minor)</span>

<span class="text-blue-400">api:</span>
  • fastify: 4.14.0 → <span class="text-green-400">4.15.0</span> <span class="text-muted-foreground">(patch)</span>

<span class="text-red-400">🚨 Security updates available for 1 package</span>
<span class="text-green-400">💡 Run knot deps resolve --strategy latest to update</span></code></pre>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Common Workflows -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Common Dependency Workflows</h2>

		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="solar:add-circle-bold" class="w-5 h-5 mr-2 text-green-600" />
					Adding New Dependencies
				</h3>
				<div class="space-y-2 text-sm font-mono">
					<div><code>knot deps add utils</code></div>
					<div><code>knot deps add @types/node --dev</code></div>
					<div><code>knot deps add analytics --optional</code></div>
					<div><code>knot link</code></div>
				</div>
				<p class="text-xs text-muted-foreground mt-2">
					Add dependencies with proper categorization and link to apps
				</p>
			</div>

			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="solar:shield-check-bold" class="w-5 h-5 mr-2 text-blue-600" />
					Dependency Health Check
				</h3>
				<div class="space-y-2 text-sm font-mono">
					<div><code>knot deps check</code></div>
					<div><code>knot deps outdated</code></div>
					<div><code>knot deps sync</code></div>
					<div><code>knot deps resolve</code></div>
				</div>
				<p class="text-xs text-muted-foreground mt-2">
					Regular maintenance to keep dependencies healthy and consistent
				</p>
			</div>

			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="solar:bug-bold" class="w-5 h-5 mr-2 text-orange-600" />
					Debugging Dependencies
				</h3>
				<div class="space-y-2 text-sm font-mono">
					<div><code>knot deps why lodash</code></div>
					<div><code>knot deps tree --depth 2</code></div>
					<div><code>knot deps list --tree</code></div>
					<div><code>knot status --verbose</code></div>
				</div>
				<p class="text-xs text-muted-foreground mt-2">
					Understand dependency relationships and resolve conflicts
				</p>
			</div>

			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="solar:refresh-bold" class="w-5 h-5 mr-2 text-purple-600" />
					Updating Dependencies
				</h3>
				<div class="space-y-2 text-sm font-mono">
					<div><code>knot deps outdated</code></div>
					<div><code>knot deps resolve --strategy latest</code></div>
					<div><code>knot deps sync</code></div>
					<div><code>knot run test --all</code></div>
				</div>
				<p class="text-xs text-muted-foreground mt-2">
					Keep dependencies up-to-date with proper testing
				</p>
			</div>
		</div>
	</section>

	<!-- Troubleshooting -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Troubleshooting</h2>

		<div class="space-y-6">
			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 text-red-600">Version Conflicts</h3>
				<p class="text-sm text-muted-foreground mb-3">
					When different apps require incompatible versions of the same package:
				</p>
				<div class="space-y-2">
					<div class="bg-gray-100 p-3 rounded text-sm">
						<strong>Problem:</strong> frontend needs lodash@^4.17.0, api needs lodash@^3.10.0
					</div>
					<div class="bg-green-50 p-3 rounded text-sm">
						<strong>Solution:</strong>
						<div class="font-mono mt-1">
							<div>1. knot deps check</div>
							<div>2. knot deps resolve --strategy compatible</div>
							<div>3. Update package constraints if needed</div>
						</div>
					</div>
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 text-orange-600">Missing Dependencies</h3>
				<p class="text-sm text-muted-foreground mb-3">
					When packages are imported but not declared in dependencies:
				</p>
				<div class="space-y-2">
					<div class="bg-gray-100 p-3 rounded text-sm">
						<strong>Problem:</strong> Import errors or runtime failures for undeclared dependencies
					</div>
					<div class="bg-green-50 p-3 rounded text-sm">
						<strong>Solution:</strong>
						<div class="font-mono mt-1">
							<div>1. knot deps check</div>
							<div>2. knot deps add missing-package</div>
							<div>3. knot link</div>
						</div>
					</div>
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 text-purple-600">Circular Dependencies</h3>
				<p class="text-sm text-muted-foreground mb-3">
					When packages depend on each other creating circular references:
				</p>
				<div class="space-y-2">
					<div class="bg-gray-100 p-3 rounded text-sm">
						<strong>Problem:</strong> Package A depends on Package B, which depends on Package A
					</div>
					<div class="bg-green-50 p-3 rounded text-sm">
						<strong>Solution:</strong>
						<div class="font-mono mt-1">
							<div>1. knot deps tree --depth 5</div>
							<div>2. Identify circular path</div>
							<div>3. Refactor to shared package or remove circular dependency</div>
						</div>
					</div>
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 text-blue-600">Performance Issues</h3>
				<p class="text-sm text-muted-foreground mb-3">
					When dependency resolution or installation becomes slow:
				</p>
				<div class="space-y-2">
					<div class="bg-gray-100 p-3 rounded text-sm">
						<strong>Problem:</strong> Slow dependency resolution or large dependency trees
					</div>
					<div class="bg-green-50 p-3 rounded text-sm">
						<strong>Solution:</strong>
						<div class="font-mono mt-1">
							<div>1. knot deps tree --depth 3</div>
							<div>2. knot deps outdated</div>
							<div>3. Consider pruning unused dependencies</div>
							<div>4. Use knot deps resolve --strategy conservative</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Best Practices -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Best Practices</h2>
		
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<div class="space-y-4">
				<h3 class="text-lg font-semibold text-green-600">Do</h3>
				<div class="space-y-3 text-sm">
					<div class="flex items-start space-x-2">
						<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600 mt-0.5 flex-shrink-0" />
						<span>Run <code>knot deps check</code> regularly to catch issues early</span>
					</div>
					<div class="flex items-start space-x-2">
						<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600 mt-0.5 flex-shrink-0" />
						<span>Use <code>knot deps sync</code> to maintain version consistency</span>
					</div>
					<div class="flex items-start space-x-2">
						<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600 mt-0.5 flex-shrink-0" />
						<span>Specify appropriate version constraints (^, ~, exact)</span>
					</div>
					<div class="flex items-start space-x-2">
						<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600 mt-0.5 flex-shrink-0" />
						<span>Use <code>--dev</code> flag for development-only dependencies</span>
					</div>
					<div class="flex items-start space-x-2">
						<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600 mt-0.5 flex-shrink-0" />
						<span>Test after dependency updates with <code>knot run test --all</code></span>
					</div>
				</div>
			</div>
			
			<div class="space-y-4">
				<h3 class="text-lg font-semibold text-red-600">Don't</h3>
				<div class="space-y-3 text-sm">
					<div class="flex items-start space-x-2">
						<Icon icon="solar:close-circle-bold" class="w-4 h-4 text-red-600 mt-0.5 flex-shrink-0" />
						<span>Mix different major versions of the same package across apps</span>
					</div>
					<div class="flex items-start space-x-2">
						<Icon icon="solar:close-circle-bold" class="w-4 h-4 text-red-600 mt-0.5 flex-shrink-0" />
						<span>Add dependencies without understanding their purpose</span>
					</div>
					<div class="flex items-start space-x-2">
						<Icon icon="solar:close-circle-bold" class="w-4 h-4 text-red-600 mt-0.5 flex-shrink-0" />
						<span>Ignore version conflicts - resolve them promptly</span>
					</div>
					<div class="flex items-start space-x-2">
						<Icon icon="solar:close-circle-bold" class="w-4 h-4 text-red-600 mt-0.5 flex-shrink-0" />
						<span>Skip testing after dependency changes</span>
					</div>
					<div class="flex items-start space-x-2">
						<Icon icon="solar:close-circle-bold" class="w-4 h-4 text-red-600 mt-0.5 flex-shrink-0" />
						<span>Let dependencies become severely outdated</span>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Next Steps -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Next Steps</h2>
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<a href="/docs/package-linking" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:link-bold" class="w-6 h-6 text-purple-600" />
					<h3 class="font-semibold">Package Linking</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Learn how packages are linked across apps with TypeScript path mapping and symlinks.
				</p>
			</a>

			<a href="/docs/cli-commands" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:terminal-bold" class="w-6 h-6 text-blue-600" />
					<h3 class="font-semibold">CLI Commands</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Complete reference for all Knot CLI commands with examples and usage patterns.
				</p>
			</a>

			<a href="/docs/troubleshooting" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:help-bold" class="w-6 h-6 text-orange-600" />
					<h3 class="font-semibold">Troubleshooting</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Common issues, debugging techniques, and solutions for CLI problems.
				</p>
			</a>

			<a href="/docs/workflows" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:refresh-bold" class="w-6 h-6 text-green-600" />
					<h3 class="font-semibold">Development Workflows</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Best practices and workflows for developing in Knot monorepos.
				</p>
			</a>
		</div>
	</section>
</div>