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

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight mb-4" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
			CLI Commands Overview
		</h1>
		<p class="text-lg text-muted-foreground">
			Complete reference for all Knot CLI commands with examples and usage patterns
		</p>
	</div>

	<!-- Command Categories -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Command Categories</h2>
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-blue-100 rounded-lg flex items-center justify-center mb-4">
					<Icon icon="solar:folder-bold" class="w-6 h-6 text-blue-600" />
				</div>
				<h3 class="font-semibold mb-2">Project Management</h3>
				<p class="text-sm text-muted-foreground mb-3">
					Initialize, configure, and manage your monorepo projects.
				</p>
				<div class="text-xs space-y-1">
					<div><code>knot init</code></div>
					<div><code>knot status</code></div>
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-green-100 rounded-lg flex items-center justify-center mb-4">
					<Icon icon="solar:box-bold" class="w-6 h-6 text-green-600" />
				</div>
				<h3 class="font-semibold mb-2">Package & App Creation</h3>
				<p class="text-sm text-muted-foreground mb-3">
					Create new packages and applications within your monorepo.
				</p>
				<div class="text-xs space-y-1">
					<div><code>knot init:package</code></div>
					<div><code>knot init:app</code></div>
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-purple-100 rounded-lg flex items-center justify-center mb-4">
					<Icon icon="solar:link-bold" class="w-6 h-6 text-purple-600" />
				</div>
				<h3 class="font-semibold mb-2">Package Linking</h3>
				<p class="text-sm text-muted-foreground mb-3">
					Link packages across your monorepo with TypeScript support.
				</p>
				<div class="text-xs space-y-1">
					<div><code>knot link</code></div>
					<div><code>knot link --symlink</code></div>
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-orange-100 rounded-lg flex items-center justify-center mb-4">
					<Icon icon="solar:sledgehammer-bold-duotone" class="w-6 h-6 text-orange-600" />
				</div>
				<h3 class="font-semibold mb-2">Scripts</h3>
				<p class="text-sm text-muted-foreground mb-3">
					Run scripts across your applications and packages.
				</p>
				<div class="text-xs space-y-1">
					<div><code>knot run</code></div>
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-red-100 rounded-lg flex items-center justify-center mb-4">
					<Icon icon="solar:upload-bold" class="w-6 h-6 text-red-600" />
				</div>
				<h3 class="font-semibold mb-2">Publishing</h3>
				<p class="text-sm text-muted-foreground mb-3">
					Publish packages to Knot Space registry for sharing.
				</p>
				<div class="text-xs space-y-1">
					<div><code>knot publish</code></div>
					<div><code>knot unpublish</code></div>
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-gray-100 rounded-lg flex items-center justify-center mb-4">
					<Icon icon="solar:users-group-two-rounded-bold" class="w-6 h-6 text-gray-600" />
				</div>
				<h3 class="font-semibold mb-2">Authentication & Teams</h3>
				<p class="text-sm text-muted-foreground mb-3">
					Manage authentication and team collaboration.
				</p>
				<div class="text-xs space-y-1">
					<div><code>knot login</code></div>
					<div><code>knot team</code></div>
				</div>
			</div>
		</div>
	</section>

	<!-- Project Management Commands -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Project Management</h2>

		<div class="space-y-8">
			<!-- knot init -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">knot init</h3>
				<p class="text-muted-foreground mb-4">
					Initialize a new Knot monorepo project with default structure and configuration.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Basic Usage</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot init my-project</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot init my-project')}
							>
								{#if showCopied && copyText === 'knot init my-project'}
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
								<code class="bg-muted px-2 py-1 rounded">--description</code>
								<span class="text-muted-foreground">Set project description</span>
							</div>
							<div class="flex items-start space-x-3">
								<code class="bg-muted px-2 py-1 rounded">--author</code>
								<span class="text-muted-foreground">Set project author</span>
							</div>
							<div class="flex items-start space-x-3">
								<code class="bg-muted px-2 py-1 rounded">--license</code>
								<span class="text-muted-foreground">Set project license (default: MIT)</span>
							</div>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Example with Options</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot init my-project --description "A modern web application" --author "Jane Doe"</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot init my-project --description "A modern web application" --author "Jane Doe"')}
							>
								{#if showCopied && copyText.includes('modern web application')}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>
				</div>
			</div>

			<!-- knot status -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">knot status</h3>
				<p class="text-muted-foreground mb-4">
					Display comprehensive information about your project's current state, packages, and dependencies.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Basic Usage</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot status</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
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
						<h4 class="font-medium mb-2">Options</h4>
						<div class="text-sm space-y-2">
							<div class="flex items-start space-x-3">
								<code class="bg-muted px-2 py-1 rounded">--verbose</code>
								<span class="text-muted-foreground">Show detailed package information</span>
							</div>
							<div class="flex items-start space-x-3">
								<code class="bg-muted px-2 py-1 rounded">--json</code>
								<span class="text-muted-foreground">Output status in JSON format</span>
							</div>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Example Output</h4>
						<div class="bg-black/90 text-white font-mono text-xs p-4 rounded-lg">
							<pre><code><span class="text-green-400">Project:</span> my-awesome-project
<span class="text-green-400">Packages:</span> 3 local, 2 remote
<span class="text-green-400">Apps:</span> 2
<span class="text-green-400">Last linked:</span> 2 minutes ago

<span class="text-blue-400">📦 Local Packages:</span>
  • types (v1.0.0)
  • utils (v1.2.1)
  • ui-components (v2.0.0)

<span class="text-purple-400">🏗️ Applications:</span>
  • frontend → @/types, @/utils, @/ui-components
  • api → #/types, #/utils</code></pre>
						</div>
					</div>
				</div>
			</div>

		</div>
	</section>

	<!-- Package & App Creation -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Package & App Creation</h2>

		<div class="space-y-8">
			<!-- knot init:package -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">knot init:package</h3>
				<p class="text-muted-foreground mb-4">
					Create a new package with TypeScript configuration and boilerplate code.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Basic Usage</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot init:package utils</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot init:package utils')}
							>
								{#if showCopied && copyText === 'knot init:package utils'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">With Team Namespace</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot init:package shared-lib --team myteam</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot init:package shared-lib --team myteam')}
							>
								{#if showCopied && copyText.includes('shared-lib')}
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
								<code class="bg-muted px-2 py-1 rounded">--team</code>
								<span class="text-muted-foreground">Set team namespace for the package</span>
							</div>
							<div class="flex items-start space-x-3">
								<code class="bg-muted px-2 py-1 rounded">--description</code>
								<span class="text-muted-foreground">Set package description</span>
							</div>
							<div class="flex items-start space-x-3">
								<code class="bg-muted px-2 py-1 rounded">--template</code>
								<span class="text-muted-foreground">Use specific template (typescript, react, etc.)</span>
							</div>
						</div>
					</div>
				</div>
			</div>

			<!-- knot init:app -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">knot init:app</h3>
				<p class="text-muted-foreground mb-4">
					Create a new application with framework-specific setup and configuration.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Basic Usage</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot init:app frontend</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot init:app frontend')}
							>
								{#if showCopied && copyText === 'knot init:app frontend'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">With Framework Template</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot init:app api --template fastify</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot init:app api --template fastify')}
							>
								{#if showCopied && copyText.includes('fastify')}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Available Templates</h4>
						<div class="grid grid-cols-2 md:grid-cols-3 gap-3 text-sm">
							<div class="bg-muted p-2 rounded"><code>react</code> - React with Vite</div>
							<div class="bg-muted p-2 rounded"><code>nextjs</code> - Next.js application</div>
							<div class="bg-muted p-2 rounded"><code>svelte</code> - SvelteKit app</div>
							<div class="bg-muted p-2 rounded"><code>fastify</code> - Fastify API server</div>
							<div class="bg-muted p-2 rounded"><code>express</code> - Express.js server</div>
							<div class="bg-muted p-2 rounded"><code>vanilla</code> - Plain TypeScript</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Package Linking -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Package Linking</h2>

		<div class="space-y-8">
			<!-- knot link -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">knot link</h3>
				<p class="text-muted-foreground mb-4">
					Link packages to applications and setup TypeScript path mappings. Default behavior is to copy packages.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Basic Usage (Copy Mode)</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot link</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot link')}
							>
								{#if showCopied && copyText === 'knot link'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Symlink Mode (Development)</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot link --symlink</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot link --symlink')}
							>
								{#if showCopied && copyText === 'knot link --symlink'}
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
								<code class="bg-muted px-2 py-1 rounded">--symlink</code>
								<span class="text-muted-foreground">Use symlinks instead of copying (development mode)</span>
							</div>
							<div class="flex items-start space-x-3">
								<code class="bg-muted px-2 py-1 rounded">--force</code>
								<span class="text-muted-foreground">Force re-link even if packages are up to date</span>
							</div>
							<div class="flex items-start space-x-3">
								<code class="bg-muted px-2 py-1 rounded">--app</code>
								<span class="text-muted-foreground">Link packages for specific app only</span>
							</div>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Link Specific App</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot link --app frontend</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot link --app frontend')}
							>
								{#if showCopied && copyText === 'knot link --app frontend'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Scripts -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Scripts</h2>

		<div class="space-y-8">
			<!-- knot run -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">knot run</h3>
				<p class="text-muted-foreground mb-4">
					Execute scripts with context-aware resolution (app → package → project priority).
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Basic Usage</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot run dev</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot run dev')}
							>
								{#if showCopied && copyText === 'knot run dev'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Run Across All Apps</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot run test --all</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot run test --all')}
							>
								{#if showCopied && copyText === 'knot run test --all'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Run in Specific App</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot run lint --app frontend</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot run lint --app frontend')}
							>
								{#if showCopied && copyText === 'knot run lint --app frontend'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Script Resolution Priority</h4>
						<div class="bg-blue-50 border border-blue-200 rounded-lg p-4">
							<div class="space-y-2 text-sm">
								<div class="flex items-center space-x-2">
									<span class="w-6 h-6 bg-red-100 rounded-full flex items-center justify-center text-red-600 text-xs font-bold">1</span>
									<span><strong>App scripts</strong> (app.yml) - Highest priority</span>
								</div>
								<div class="flex items-center space-x-2">
									<span class="w-6 h-6 bg-yellow-100 rounded-full flex items-center justify-center text-yellow-600 text-xs font-bold">2</span>
									<span><strong>Package scripts</strong> (package.yml) - Medium priority</span>
								</div>
								<div class="flex items-center space-x-2">
									<span class="w-6 h-6 bg-blue-100 rounded-full flex items-center justify-center text-blue-600 text-xs font-bold">3</span>
									<span><strong>Project scripts</strong> (knot.yml) - Lowest priority</span>
								</div>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Global Options -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Global Options</h2>

		<div class="space-y-6">
			<p class="text-muted-foreground">
				These options can be used with any Knot command for enhanced functionality.
			</p>

			<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
				<div class="border rounded-lg p-6">
					<h3 class="font-semibold mb-3">Output & Verbosity</h3>
					<div class="text-sm space-y-2">
						<div class="flex items-start space-x-3">
							<code class="bg-muted px-2 py-1 rounded">--verbose</code>
							<span class="text-muted-foreground">Enable verbose output</span>
						</div>
						<div class="flex items-start space-x-3">
							<code class="bg-muted px-2 py-1 rounded">--quiet</code>
							<span class="text-muted-foreground">Suppress non-essential output</span>
						</div>
						<div class="flex items-start space-x-3">
							<code class="bg-muted px-2 py-1 rounded">--json</code>
							<span class="text-muted-foreground">Output in JSON format</span>
						</div>
					</div>
				</div>

				<div class="border rounded-lg p-6">
					<h3 class="font-semibold mb-3">Configuration</h3>
					<div class="text-sm space-y-2">
						<div class="flex items-start space-x-3">
							<code class="bg-muted px-2 py-1 rounded">--config</code>
							<span class="text-muted-foreground">Specify config file path</span>
						</div>
						<div class="flex items-start space-x-3">
							<code class="bg-muted px-2 py-1 rounded">--cwd</code>
							<span class="text-muted-foreground">Set working directory</span>
						</div>
						<div class="flex items-start space-x-3">
							<code class="bg-muted px-2 py-1 rounded">--help</code>
							<span class="text-muted-foreground">Show help for command</span>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Command Examples -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Common Workflows</h2>

		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="solar:rocket-2-bold" class="w-5 h-5 mr-2 text-green-600" />
					New Project Setup
				</h3>
				<div class="space-y-2 text-sm font-mono">
					<div><code>knot init my-project</code></div>
					<div><code>cd my-project</code></div>
					<div><code>knot init:package utils</code></div>
					<div><code>knot init:app frontend</code></div>
					<div><code>knot link</code></div>
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="solar:programming-bold" class="w-5 h-5 mr-2 text-blue-600" />
					Development Workflow
				</h3>
				<div class="space-y-2 text-sm font-mono">
					<div><code>knot link --symlink</code></div>
					<div><code>knot run dev</code></div>
					<div><code>knot run test --all</code></div>
					<div><code>knot run build</code></div>
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="solar:upload-bold" class="w-5 h-5 mr-2 text-purple-600" />
					Package Publishing
				</h3>
				<div class="space-y-2 text-sm font-mono">
					<div><code>knot login</code></div>
					<div><code>cd packages/utils</code></div>
					<div><code>knot run build</code></div>
					<div><code>knot publish</code></div>
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="solar:bug-bold" class="w-5 h-5 mr-2 text-red-600" />
					Debugging & Maintenance
				</h3>
				<div class="space-y-2 text-sm font-mono">
					<div><code>knot status --verbose</code></div>
					<div><code>knot link --force</code></div>
					<div><code>knot run typecheck --all</code></div>
					<div><code>knot --help</code></div>
				</div>
			</div>
		</div>
	</section>

	<!-- Next Steps -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Next Steps</h2>
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<a href="/docs/cli-project" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:folder-bold" class="w-6 h-6 text-blue-600" />
					<h3 class="font-semibold">Project Management</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Deep dive into project initialization, configuration, and management commands.
				</p>
			</a>

			<a href="/docs/cli-packages" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:box-bold" class="w-6 h-6 text-green-600" />
					<h3 class="font-semibold">Package Development</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Learn about package creation, building, testing, and development workflows.
				</p>
			</a>

			<a href="/docs/publishing" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:upload-bold" class="w-6 h-6 text-red-600" />
					<h3 class="font-semibold">Publishing</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Publish and share your packages with the Knot Space registry.
				</p>
			</a>

			<a href="/docs/troubleshooting" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:help-bold" class="w-6 h-6 text-purple-600" />
					<h3 class="font-semibold">Troubleshooting</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Common issues, debugging techniques, and solutions for CLI problems.
				</p>
			</a>
		</div>
	</section>
</div>
