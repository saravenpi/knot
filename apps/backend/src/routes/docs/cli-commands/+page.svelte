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
					<div><code>knot delete</code></div>
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
					<div><code>knot auth</code></div>
					<div><code>knot team</code></div>
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-yellow-100 rounded-lg flex items-center justify-center mb-4">
					<Icon icon="solar:settings-bold" class="w-6 h-6 text-yellow-600" />
				</div>
				<h3 class="font-semibold mb-2">Dependency Management</h3>
				<p class="text-sm text-muted-foreground mb-3">
					Complete dependency management system with resolution and analysis.
				</p>
				<div class="text-xs space-y-1">
					<div><code>knot deps</code></div>
					<div><code>knot install</code></div>
				</div>
				<a href="/docs/dependency-management" class="text-xs text-blue-600 hover:underline mt-2 block">
					→ Full Dependency Management Guide
				</a>
			</div>

			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-indigo-100 rounded-lg flex items-center justify-center mb-4">
					<Icon icon="solar:tag-bold" class="w-6 h-6 text-indigo-600" />
				</div>
				<h3 class="font-semibold mb-2">Version Management</h3>
				<p class="text-sm text-muted-foreground mb-3">
					Manage package versions and releases with semantic versioning.
				</p>
				<div class="text-xs space-y-1">
					<div><code>knot version</code></div>
					<div><code>knot delete</code></div>
					<div><code>knot upgrade</code></div>
				</div>
				<a href="/docs/version-management" class="text-xs text-blue-600 hover:underline mt-2 block">
					→ Full Version Management Guide
				</a>
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
						<h4 class="font-medium mb-2">Alias</h4>
						<div class="text-sm text-muted-foreground">
							<code class="bg-muted px-2 py-1 rounded">knot s</code> - Short alias for status command
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
								<span class="text-muted-foreground">Use a published package as template (e.g., @svelte-starter, @my-team/template)</span>
							</div>
							<div class="flex items-start space-x-3">
								<code class="bg-muted px-2 py-1 rounded">--here</code>
								<span class="text-muted-foreground">Create package in current directory instead of creating new subfolder</span>
							</div>
							<div class="flex items-start space-x-3">
								<code class="bg-muted px-2 py-1 rounded">--version</code>
								<span class="text-muted-foreground">Set initial version (default: 1.0.0)</span>
							</div>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">With Template from Knot Space</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot init:package my-lib --template @typescript-lib</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot init:package my-lib --template @typescript-lib')}
							>
								{#if showCopied && copyText.includes('typescript-lib')}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
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
						<h4 class="font-medium mb-2">With Template from Knot Space</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot init:app api --template @fastify-starter</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot init:app api --template @fastify-starter')}
							>
								{#if showCopied && copyText.includes('fastify-starter')}
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
								<code class="bg-muted px-2 py-1 rounded">--template</code>
								<span class="text-muted-foreground">Use a published package as template (e.g., @svelte-app, @nextjs-starter)</span>
							</div>
							<div class="flex items-start space-x-3">
								<code class="bg-muted px-2 py-1 rounded">--description</code>
								<span class="text-muted-foreground">Set app description</span>
							</div>
							<div class="flex items-start space-x-3">
								<code class="bg-muted px-2 py-1 rounded">--here</code>
								<span class="text-muted-foreground">Create app in current directory instead of creating new subfolder</span>
							</div>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Template Usage</h4>
						<div class="text-sm text-muted-foreground">
							<p class="mb-2">Templates are now fetched from Knot Space. Any published package can be used as a template by prefixing it with <code class="bg-muted px-1 rounded">@</code>.</p>
							<p>When prompted interactively, you can choose whether to use a template or create a basic project structure.</p>
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
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Alias</h4>
						<div class="text-sm text-muted-foreground">
							<code class="bg-muted px-2 py-1 rounded">knot l</code> - Short alias for link command
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
						<h4 class="font-medium mb-2">Run Specific Script</h4>
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
						<h4 class="font-medium mb-2">Interactive Script Selection</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot run</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot run')}
							>
								{#if showCopied && copyText === 'knot run'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
						<div class="text-sm text-muted-foreground mt-2">
							When no script name is provided, shows an interactive menu to select available scripts.
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Direct Script Execution</h4>
						<div class="text-sm text-muted-foreground mb-3">
							You can also run any script directly as if it were a built-in command:
						</div>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-3 rounded-lg">
							<code>knot dev</code>
						</div>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-3 rounded-lg mt-2">
							<code>knot build</code>
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

					<div>
						<h4 class="font-medium mb-2">Alias</h4>
						<div class="text-sm text-muted-foreground">
							<code class="bg-muted px-2 py-1 rounded">knot r</code> - Short alias for run command
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Authentication -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Authentication</h2>

		<div class="space-y-8">
			<!-- knot auth -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">knot auth</h3>
				<p class="text-muted-foreground mb-4">
					Check authentication status and view current user information.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Basic Usage</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot auth</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot auth')}
							>
								{#if showCopied && copyText === 'knot auth'}
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
							<pre><code><span class="text-green-400">✅ Authenticated as:</span> john_doe
<span class="text-green-400">📧 Email:</span> john@example.com</code></pre>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Authentication Setup</h4>
						<div class="text-sm text-muted-foreground">
							<p class="mb-2">Authentication is managed via environment variable. Set your token with:</p>
							<div class="bg-muted px-3 py-2 rounded font-mono text-xs">
								export KNOT_TOKEN=your_api_token_here
							</div>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Alias</h4>
						<div class="text-sm text-muted-foreground">
							<code class="bg-muted px-2 py-1 rounded">knot a</code> - Short alias for auth command
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Dependency Management -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Dependency Management</h2>

		<div class="space-y-8">
			<!-- knot deps -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">knot deps</h3>
				<p class="text-muted-foreground mb-4">
					Complete dependency management system for analyzing, resolving, and maintaining dependencies.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Subcommands</h4>
						<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
							<div class="border rounded p-3">
								<div class="font-mono text-sm mb-1"><code>knot deps add</code></div>
								<div class="text-xs text-muted-foreground">Add a dependency to an app</div>
							</div>
							<div class="border rounded p-3">
								<div class="font-mono text-sm mb-1"><code>knot deps list</code></div>
								<div class="text-xs text-muted-foreground">List dependencies for apps</div>
							</div>
							<div class="border rounded p-3">
								<div class="font-mono text-sm mb-1"><code>knot deps resolve</code></div>
								<div class="text-xs text-muted-foreground">Resolve and install dependencies</div>
							</div>
							<div class="border rounded p-3">
								<div class="font-mono text-sm mb-1"><code>knot deps check</code></div>
								<div class="text-xs text-muted-foreground">Check for dependency issues</div>
							</div>
							<div class="border rounded p-3">
								<div class="font-mono text-sm mb-1"><code>knot deps tree</code></div>
								<div class="text-xs text-muted-foreground">Show dependency tree</div>
							</div>
							<div class="border rounded p-3">
								<div class="font-mono text-sm mb-1"><code>knot deps outdated</code></div>
								<div class="text-xs text-muted-foreground">Show outdated dependencies</div>
							</div>
							<div class="border rounded p-3">
								<div class="font-mono text-sm mb-1"><code>knot deps why</code></div>
								<div class="text-xs text-muted-foreground">Explain why package is included</div>
							</div>
							<div class="border rounded p-3">
								<div class="font-mono text-sm mb-1"><code>knot deps sync</code></div>
								<div class="text-xs text-muted-foreground">Synchronize dependency versions</div>
							</div>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Common Usage Examples</h4>
						<div class="space-y-3">
							<div>
								<div class="text-sm font-medium mb-1">Add a dependency</div>
								<div class="bg-black/90 text-green-400 font-mono text-sm p-3 rounded-lg">
									<code>knot deps add utils@^1.0.0 --app frontend</code>
								</div>
							</div>
							<div>
								<div class="text-sm font-medium mb-1">Show dependency tree</div>
								<div class="bg-black/90 text-green-400 font-mono text-sm p-3 rounded-lg">
									<code>knot deps tree --depth 3</code>
								</div>
							</div>
							<div>
								<div class="text-sm font-medium mb-1">Check for issues</div>
								<div class="bg-black/90 text-green-400 font-mono text-sm p-3 rounded-lg">
									<code>knot deps check</code>
								</div>
							</div>
						</div>
					</div>
				</div>
			</div>

			<!-- knot install -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">knot install</h3>
				<p class="text-muted-foreground mb-4">
					Install a package dependency to the current app and automatically link it.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Basic Usage</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot install utils</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot install utils')}
							>
								{#if showCopied && copyText === 'knot install utils'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">With Version Specification</h4>
						<div class="space-y-2">
							<div class="bg-black/90 text-green-400 font-mono text-sm p-3 rounded-lg">
								<code>knot install utils@1.2.3</code>
							</div>
							<div class="bg-black/90 text-green-400 font-mono text-sm p-3 rounded-lg">
								<code>knot install utils@^1.0.0</code>
							</div>
							<div class="bg-black/90 text-green-400 font-mono text-sm p-3 rounded-lg">
								<code>knot install @team/package@latest</code>
							</div>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Options</h4>
						<div class="text-sm space-y-2">
							<div class="flex items-start space-x-3">
								<code class="bg-muted px-2 py-1 rounded">--no-link</code>
								<span class="text-muted-foreground">Skip automatic linking after installing</span>
							</div>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Aliases</h4>
						<div class="text-sm text-muted-foreground space-y-1">
							<div><code class="bg-muted px-2 py-1 rounded">knot i</code> - Short alias</div>
							<div><code class="bg-muted px-2 py-1 rounded">knot add</code> - Alternative alias</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Version Management -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Version Management</h2>

		<div class="space-y-8">
			<!-- knot version -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">knot version</h3>
				<p class="text-muted-foreground mb-4">
					Manage package versions with semantic versioning support.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Subcommands</h4>
						<div class="space-y-3">
							<div class="border rounded p-3">
								<div class="font-mono text-sm mb-1"><code>knot version patch</code></div>
								<div class="text-xs text-muted-foreground">Bump patch version (bug fixes) - e.g., 1.0.0 → 1.0.1</div>
							</div>
							<div class="border rounded p-3">
								<div class="font-mono text-sm mb-1"><code>knot version minor</code></div>
								<div class="text-xs text-muted-foreground">Bump minor version (new features) - e.g., 1.0.0 → 1.1.0</div>
							</div>
							<div class="border rounded p-3">
								<div class="font-mono text-sm mb-1"><code>knot version major</code></div>
								<div class="text-xs text-muted-foreground">Bump major version (breaking changes) - e.g., 1.0.0 → 2.0.0</div>
							</div>
							<div class="border rounded p-3">
								<div class="font-mono text-sm mb-1"><code>knot version prerelease</code></div>
								<div class="text-xs text-muted-foreground">Create pre-release version - e.g., 1.0.0 → 1.0.1-alpha.0</div>
							</div>
							<div class="border rounded p-3">
								<div class="font-mono text-sm mb-1"><code>knot version set</code></div>
								<div class="text-xs text-muted-foreground">Set specific version - e.g., knot version set 2.0.0</div>
							</div>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Usage Examples</h4>
						<div class="space-y-3">
							<div>
								<div class="text-sm font-medium mb-1">Bug fix release</div>
								<div class="bg-black/90 text-green-400 font-mono text-sm p-3 rounded-lg">
									<code>knot version patch</code>
								</div>
							</div>
							<div>
								<div class="text-sm font-medium mb-1">Pre-release with identifier</div>
								<div class="bg-black/90 text-green-400 font-mono text-sm p-3 rounded-lg">
									<code>knot version prerelease --preid beta</code>
								</div>
							</div>
							<div>
								<div class="text-sm font-medium mb-1">Set specific version</div>
								<div class="bg-black/90 text-green-400 font-mono text-sm p-3 rounded-lg">
									<code>knot version set 3.1.4</code>
								</div>
							</div>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Alias</h4>
						<div class="text-sm text-muted-foreground">
							<code class="bg-muted px-2 py-1 rounded">knot v</code> - Short alias for version command
						</div>
					</div>
				</div>
			</div>

			<!-- knot delete -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">knot delete</h3>
				<p class="text-muted-foreground mb-4">
					Delete a specific version of a package from Knot Space registry.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Basic Usage</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot delete my-package 1.2.3</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot delete my-package 1.2.3')}
							>
								{#if showCopied && copyText === 'knot delete my-package 1.2.3'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Syntax</h4>
						<div class="text-sm text-muted-foreground">
							<p>Both package name and version are required parameters:</p>
							<div class="mt-2 font-mono text-xs bg-muted px-3 py-2 rounded">
								knot delete &lt;package-name&gt; &lt;version&gt;
							</div>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Alias</h4>
						<div class="text-sm text-muted-foreground">
							<code class="bg-muted px-2 py-1 rounded">knot d</code> - Short alias for delete command
						</div>
					</div>

					<div class="bg-amber-50 border border-amber-200 rounded-lg p-4">
						<div class="flex items-start space-x-2">
							<Icon icon="solar:danger-triangle-bold" class="w-5 h-5 text-amber-600 mt-0.5" />
							<div class="text-sm text-amber-800">
								<strong>Warning:</strong> Package deletion is permanent and cannot be undone. Use with caution.
							</div>
						</div>
					</div>
				</div>
			</div>

			<!-- knot upgrade -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">knot upgrade</h3>
				<p class="text-muted-foreground mb-4">
					Update Knot CLI to the latest version.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Basic Usage</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot upgrade</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot upgrade')}
							>
								{#if showCopied && copyText === 'knot upgrade'}
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
								<code class="bg-muted px-2 py-1 rounded">--force</code>
								<span class="text-muted-foreground">Force upgrade even if already on latest version</span>
							</div>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Force Upgrade</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg">
							<code>knot upgrade --force</code>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Alias</h4>
						<div class="text-sm text-muted-foreground">
							<code class="bg-muted px-2 py-1 rounded">knot u</code> - Short alias for upgrade command
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Team Management -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Team Management</h2>

		<div class="space-y-8">
			<!-- knot team -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">knot team</h3>
				<p class="text-muted-foreground mb-4">
					Manage teams for collaborative package development and publishing.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Subcommands</h4>
						<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
							<div class="border rounded p-3">
								<div class="font-mono text-sm mb-1"><code>knot team create</code></div>
								<div class="text-xs text-muted-foreground">Create a new team</div>
							</div>
							<div class="border rounded p-3">
								<div class="font-mono text-sm mb-1"><code>knot team list</code></div>
								<div class="text-xs text-muted-foreground">List all teams</div>
							</div>
							<div class="border rounded p-3">
								<div class="font-mono text-sm mb-1"><code>knot team info</code></div>
								<div class="text-xs text-muted-foreground">Show team information</div>
							</div>
							<div class="border rounded p-3">
								<div class="font-mono text-sm mb-1"><code>knot team add-member</code></div>
								<div class="text-xs text-muted-foreground">Add member to team</div>
							</div>
							<div class="border rounded p-3">
								<div class="font-mono text-sm mb-1"><code>knot team remove-member</code></div>
								<div class="text-xs text-muted-foreground">Remove member from team</div>
							</div>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Usage Examples</h4>
						<div class="space-y-3">
							<div>
								<div class="text-sm font-medium mb-1">Create a team</div>
								<div class="bg-black/90 text-green-400 font-mono text-sm p-3 rounded-lg">
									<code>knot team create my-team --description "Development team"</code>
								</div>
							</div>
							<div>
								<div class="text-sm font-medium mb-1">Add team member</div>
								<div class="bg-black/90 text-green-400 font-mono text-sm p-3 rounded-lg">
									<code>knot team add-member my-team john_doe --role admin</code>
								</div>
							</div>
							<div>
								<div class="text-sm font-medium mb-1">Show team info</div>
								<div class="bg-black/90 text-green-400 font-mono text-sm p-3 rounded-lg">
									<code>knot team info my-team</code>
								</div>
							</div>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Member Roles</h4>
						<div class="text-sm space-y-2">
							<div class="flex items-start space-x-3">
								<code class="bg-muted px-2 py-1 rounded">admin</code>
								<span class="text-muted-foreground">Full team management permissions</span>
							</div>
							<div class="flex items-start space-x-3">
								<code class="bg-muted px-2 py-1 rounded">member</code>
								<span class="text-muted-foreground">Can publish packages under team namespace (default)</span>
							</div>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Alias</h4>
						<div class="text-sm text-muted-foreground">
							<code class="bg-muted px-2 py-1 rounded">knot t</code> - Short alias for team command
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Command Aliases -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Command Aliases</h2>

		<div class="space-y-6">
			<p class="text-muted-foreground">
				Knot CLI provides short aliases for frequently used commands to improve your workflow efficiency.
			</p>

			<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
				<div class="border rounded-lg p-4">
					<h3 class="font-semibold mb-3">Core Commands</h3>
					<div class="text-sm space-y-2">
						<div class="flex justify-between">
							<code class="bg-muted px-2 py-1 rounded">knot r</code>
							<span class="text-muted-foreground">run</span>
						</div>
						<div class="flex justify-between">
							<code class="bg-muted px-2 py-1 rounded">knot s</code>
							<span class="text-muted-foreground">status</span>
						</div>
						<div class="flex justify-between">
							<code class="bg-muted px-2 py-1 rounded">knot l</code>
							<span class="text-muted-foreground">link</span>
						</div>
					</div>
				</div>

				<div class="border rounded-lg p-4">
					<h3 class="font-semibold mb-3">Package Management</h3>
					<div class="text-sm space-y-2">
						<div class="flex justify-between">
							<code class="bg-muted px-2 py-1 rounded">knot i</code>
							<span class="text-muted-foreground">install</span>
						</div>
						<div class="flex justify-between">
							<code class="bg-muted px-2 py-1 rounded">knot add</code>
							<span class="text-muted-foreground">install</span>
						</div>
						<div class="flex justify-between">
							<code class="bg-muted px-2 py-1 rounded">knot p</code>
							<span class="text-muted-foreground">publish</span>
						</div>
						<div class="flex justify-between">
							<code class="bg-muted px-2 py-1 rounded">knot d</code>
							<span class="text-muted-foreground">delete</span>
						</div>
					</div>
				</div>

				<div class="border rounded-lg p-4">
					<h3 class="font-semibold mb-3">System & Teams</h3>
					<div class="text-sm space-y-2">
						<div class="flex justify-between">
							<code class="bg-muted px-2 py-1 rounded">knot a</code>
							<span class="text-muted-foreground">auth</span>
						</div>
						<div class="flex justify-between">
							<code class="bg-muted px-2 py-1 rounded">knot t</code>
							<span class="text-muted-foreground">team</span>
						</div>
						<div class="flex justify-between">
							<code class="bg-muted px-2 py-1 rounded">knot v</code>
							<span class="text-muted-foreground">version</span>
						</div>
						<div class="flex justify-between">
							<code class="bg-muted px-2 py-1 rounded">knot u</code>
							<span class="text-muted-foreground">upgrade</span>
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
					<div><code>knot run test</code></div>
					<div><code>knot run build</code></div>
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="solar:upload-bold" class="w-5 h-5 mr-2 text-purple-600" />
					Package Publishing
				</h3>
				<div class="space-y-2 text-sm font-mono">
					<div><code>knot auth</code></div>
					<div><code>cd packages/utils</code></div>
					<div><code>knot run build</code></div>
					<div><code>knot publish</code></div>
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="solar:settings-bold" class="w-5 h-5 mr-2 text-yellow-600" />
					Dependency Management
				</h3>
				<div class="space-y-2 text-sm font-mono">
					<div><code>knot deps check</code></div>
					<div><code>knot deps outdated</code></div>
					<div><code>knot deps sync</code></div>
					<div><code>knot deps tree</code></div>
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="solar:bug-bold" class="w-5 h-5 mr-2 text-red-600" />
					Debugging & Maintenance
				</h3>
				<div class="space-y-2 text-sm font-mono">
					<div><code>knot status</code></div>
					<div><code>knot link</code></div>
					<div><code>knot run typecheck</code></div>
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

			<a href="/docs/dependency-management" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:settings-bold" class="w-6 h-6 text-yellow-600" />
					<h3 class="font-semibold">Dependency Management</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Advanced dependency management with resolution strategies and analysis tools.
				</p>
			</a>

			<a href="/docs/version-management" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:tag-bold" class="w-6 h-6 text-indigo-600" />
					<h3 class="font-semibold">Version Management</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Semantic versioning, package releases, and coordinated version management.
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
