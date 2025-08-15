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

<div class="max-w-4xl mx-auto py-8 px-6">
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight mb-4" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
			Package Linking
		</h1>
		<p class="text-lg text-muted-foreground">
			Understanding how Knot links packages and manages dependencies in your monorepo
		</p>
	</div>

	<!-- Overview -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">How Package Linking Works</h2>
		<p class="text-muted-foreground leading-relaxed mb-6">
			Knot's package linking system is designed to be simple, reliable, and production-friendly. 
			Unlike traditional symlink-based approaches, Knot uses a copy-first strategy that works 
			seamlessly across different environments and deployment scenarios.
		</p>
		
		<div class="bg-blue-50 border border-blue-200 rounded-lg p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="solar:info-circle-bold" class="w-6 h-6 text-blue-600 mt-1 flex-shrink-0" />
				<div>
					<h3 class="font-semibold text-blue-900 mb-2">Copy-First Approach</h3>
					<p class="text-sm text-blue-700">
						By default, Knot copies package contents to <code>knot_packages/</code> directories in your apps. 
						This ensures compatibility with Docker, CI/CD systems, and deployment environments that don't 
						support symlinks reliably.
					</p>
				</div>
			</div>
		</div>
	</section>

	<!-- Linking Modes -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Linking Modes</h2>
		
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<!-- Copy Mode -->
			<div class="border rounded-lg p-6">
				<div class="flex items-center space-x-3 mb-4">
					<div class="w-12 h-12 bg-green-100 rounded-lg flex items-center justify-center">
						<Icon icon="solar:copy-bold" class="w-6 h-6 text-green-600" />
					</div>
					<div>
						<h3 class="font-semibold">Copy Mode (Default)</h3>
						<p class="text-sm text-muted-foreground">Production-ready package copying</p>
					</div>
				</div>
				
				<div class="space-y-3">
					<div class="bg-black/90 text-green-400 font-mono text-sm p-3 rounded">
						<code>knot link</code>
					</div>
					
					<div class="text-sm space-y-2">
						<div class="flex items-start space-x-2">
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600 mt-0.5 flex-shrink-0" />
							<span class="text-muted-foreground">Works in Docker containers</span>
						</div>
						<div class="flex items-start space-x-2">
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600 mt-0.5 flex-shrink-0" />
							<span class="text-muted-foreground">Compatible with all CI/CD systems</span>
						</div>
						<div class="flex items-start space-x-2">
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600 mt-0.5 flex-shrink-0" />
							<span class="text-muted-foreground">Can be committed to version control</span>
						</div>
						<div class="flex items-start space-x-2">
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600 mt-0.5 flex-shrink-0" />
							<span class="text-muted-foreground">No platform-specific issues</span>
						</div>
					</div>
				</div>
			</div>

			<!-- Symlink Mode -->
			<div class="border rounded-lg p-6">
				<div class="flex items-center space-x-3 mb-4">
					<div class="w-12 h-12 bg-purple-100 rounded-lg flex items-center justify-center">
						<Icon icon="solar:link-bold" class="w-6 h-6 text-purple-600" />
					</div>
					<div>
						<h3 class="font-semibold">Symlink Mode</h3>
						<p class="text-sm text-muted-foreground">Development with live updates</p>
					</div>
				</div>
				
				<div class="space-y-3">
					<div class="bg-black/90 text-purple-400 font-mono text-sm p-3 rounded">
						<code>knot link --symlink</code>
					</div>
					
					<div class="text-sm space-y-2">
						<div class="flex items-start space-x-2">
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600 mt-0.5 flex-shrink-0" />
							<span class="text-muted-foreground">Live updates during development</span>
						</div>
						<div class="flex items-start space-x-2">
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600 mt-0.5 flex-shrink-0" />
							<span class="text-muted-foreground">Instant changes across apps</span>
						</div>
						<div class="flex items-start space-x-2">
							<Icon icon="solar:close-circle-bold" class="w-4 h-4 text-red-600 mt-0.5 flex-shrink-0" />
							<span class="text-muted-foreground">Docker compatibility issues</span>
						</div>
						<div class="flex items-start space-x-2">
							<Icon icon="solar:close-circle-bold" class="w-4 h-4 text-red-600 mt-0.5 flex-shrink-0" />
							<span class="text-muted-foreground">Platform-specific limitations</span>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- How Linking Works -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Step-by-Step Linking Process</h2>
		
		<div class="space-y-8">
			<!-- Step 1 -->
			<div>
				<div class="flex items-center space-x-3 mb-4">
					<div class="w-6 h-6 bg-blue-100 rounded-full flex items-center justify-center">
						<span class="text-blue-600 font-bold text-xs">1</span>
					</div>
					<h3 class="text-lg font-semibold">Configuration Analysis</h3>
				</div>
				<div class="ml-9 space-y-3">
					<p class="text-muted-foreground">
						Knot reads your project configuration and identifies which packages each app depends on.
					</p>
					<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
						<code><span class="text-gray-400"># From knot.yml or app.yml</span>
<span class="text-blue-400">apps:</span>
  <span class="text-blue-400">frontend:</span>
    <span class="text-blue-400">packages:</span> [<span class="text-yellow-400">types</span>, <span class="text-yellow-400">utils</span>, <span class="text-yellow-400">"@team/shared"</span>]</code>
					</div>
				</div>
			</div>

			<!-- Step 2 -->
			<div>
				<div class="flex items-center space-x-3 mb-4">
					<div class="w-6 h-6 bg-green-100 rounded-full flex items-center justify-center">
						<span class="text-green-600 font-bold text-xs">2</span>
					</div>
					<h3 class="text-lg font-semibold">Package Resolution</h3>
				</div>
				<div class="ml-9 space-y-3">
					<p class="text-muted-foreground">
						Local packages are found in the <code>packages/</code> directory. Remote packages are downloaded from registries.
					</p>
					<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
						<div class="border rounded p-3">
							<h4 class="font-medium mb-2">Local Packages</h4>
							<div class="text-sm text-muted-foreground">
								Found in: <code>packages/utils/</code><br>
								Source: <code>src/</code> directory
							</div>
						</div>
						<div class="border rounded p-3">
							<h4 class="font-medium mb-2">Remote Packages</h4>
							<div class="text-sm text-muted-foreground">
								Downloaded from: Knot registry<br>
								Cached locally for performance
							</div>
						</div>
					</div>
				</div>
			</div>

			<!-- Step 3 -->
			<div>
				<div class="flex items-center space-x-3 mb-4">
					<div class="w-6 h-6 bg-purple-100 rounded-full flex items-center justify-center">
						<span class="text-purple-600 font-bold text-xs">3</span>
					</div>
					<h3 class="text-lg font-semibold">Directory Creation</h3>
				</div>
				<div class="ml-9 space-y-3">
					<p class="text-muted-foreground">
						The <code>knot_packages/</code> directory is created in each app that has package dependencies.
					</p>
					<div class="bg-muted/30 rounded-lg p-4">
						<div class="text-sm font-mono">
							apps/frontend/<br>
							├── knot_packages/  <span class="text-green-600"># Auto-generated</span><br>
							│   ├── types/<br>
							│   ├── utils/<br>
							│   └── @team_shared/  <span class="text-blue-600"># Team packages</span><br>
							└── src/
						</div>
					</div>
				</div>
			</div>

			<!-- Step 4 -->
			<div>
				<div class="flex items-center space-x-3 mb-4">
					<div class="w-6 h-6 bg-orange-100 rounded-full flex items-center justify-center">
						<span class="text-orange-600 font-bold text-xs">4</span>
					</div>
					<h3 class="text-lg font-semibold">TypeScript Configuration</h3>
				</div>
				<div class="ml-9 space-y-3">
					<p class="text-muted-foreground">
						Your app's <code>tsconfig.json</code> is automatically updated with path mappings for clean imports.
					</p>
					<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
						<pre><code>{@html `<span class="text-gray-400">// tsconfig.json</span>
{
  <span class="text-blue-400">"compilerOptions"</span>: {
    <span class="text-blue-400">"baseUrl"</span>: <span class="text-green-400">"."</span>,
    <span class="text-blue-400">"paths"</span>: {
      <span class="text-green-400">"@/types"</span>: [<span class="text-green-400">"./knot_packages/types"</span>],
      <span class="text-green-400">"@/utils"</span>: [<span class="text-green-400">"./knot_packages/utils"</span>],
      <span class="text-green-400">"@/shared"</span>: [<span class="text-green-400">"./knot_packages/@team_shared"</span>]
    }
  }
}`}</code></pre>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- TypeScript Aliases -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">TypeScript Aliases</h2>
		
		<div class="space-y-6">
			<p class="text-muted-foreground">
				Knot automatically configures TypeScript path mappings based on your chosen alias prefix, 
				making package imports clean and consistent across your applications.
			</p>

			<div>
				<h3 class="text-lg font-semibold mb-3">Alias Configuration</h3>
				<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
					<code><span class="text-gray-400"># In knot.yml or app.yml</span>
<span class="text-blue-400">apps:</span>
  <span class="text-blue-400">frontend:</span>
    <span class="text-blue-400">tsAlias:</span> <span class="text-green-400">"@"</span>        <span class="text-gray-400"># Results in @/package-name</span>
  <span class="text-blue-400">api:</span>
    <span class="text-blue-400">tsAlias:</span> <span class="text-green-400">"#"</span>        <span class="text-gray-400"># Results in #/package-name</span>
  <span class="text-blue-400">admin:</span>
    <span class="text-blue-400">tsAlias:</span> <span class="text-green-400">"~"</span>        <span class="text-gray-400"># Results in ~/package-name</span></code>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Import Examples</h3>
				<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
					<div>
						<h4 class="font-medium mb-2">Before Linking</h4>
						<div class="bg-red-50 border border-red-200 rounded p-3 text-sm font-mono">
							<span class="text-red-700">// Ugly relative paths</span><br>
							import * from '../../../packages/utils'
						</div>
					</div>
					<div>
						<h4 class="font-medium mb-2">After Linking</h4>
						<div class="bg-green-50 border border-green-200 rounded p-3 text-sm font-mono">
							<span class="text-green-700">// Clean alias imports</span><br>
							import * from '@/utils'
						</div>
					</div>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Common Alias Patterns</h3>
				<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
					<div class="border rounded p-4">
						<h4 class="font-semibold mb-2">@ Prefix</h4>
						<div class="text-sm text-muted-foreground space-y-1">
							<div><code>@/utils</code></div>
							<div><code>@/types</code></div>
							<div><code>@/components</code></div>
						</div>
						<div class="text-xs text-muted-foreground mt-2">Most common choice</div>
					</div>
					<div class="border rounded p-4">
						<h4 class="font-semibold mb-2"># Prefix</h4>
						<div class="text-sm text-muted-foreground space-y-1">
							<div><code>#/utils</code></div>
							<div><code>#/types</code></div>
							<div><code>#/components</code></div>
						</div>
						<div class="text-xs text-muted-foreground mt-2">Good for backend apps</div>
					</div>
					<div class="border rounded p-4">
						<h4 class="font-semibold mb-2">~ Prefix</h4>
						<div class="text-sm text-muted-foreground space-y-1">
							<div><code>~/utils</code></div>
							<div><code>~/types</code></div>
							<div><code>~/components</code></div>
						</div>
						<div class="text-xs text-muted-foreground mt-2">Alternative option</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Package Types -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Package Types & Handling</h2>
		
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<!-- Local Packages -->
			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-4 flex items-center">
					<Icon icon="solar:home-bold" class="w-5 h-5 mr-2 text-blue-600" />
					Local Packages
				</h3>
				<div class="space-y-3">
					<p class="text-sm text-muted-foreground">
						Packages stored in your <code>packages/</code> directory
					</p>
					<div class="bg-black/90 text-white font-mono text-xs p-3 rounded">
						<code><span class="text-blue-400">packages:</span>
  - <span class="text-yellow-400">utils</span>
  - <span class="text-yellow-400">types</span>
  - <span class="text-yellow-400">ui-components</span></code>
					</div>
					<ul class="text-sm text-muted-foreground space-y-1">
						<li>• Source copied from <code>packages/&lt;name&gt;/src/</code></li>
						<li>• Always uses latest local version</li>
						<li>• No version constraints</li>
						<li>• Instant availability</li>
					</ul>
				</div>
			</div>

			<!-- Remote Packages -->
			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-4 flex items-center">
					<Icon icon="solar:cloud-bold" class="w-5 h-5 mr-2 text-green-600" />
					Remote Packages
				</h3>
				<div class="space-y-3">
					<p class="text-sm text-muted-foreground">
						Packages downloaded from Knot registry
					</p>
					<div class="bg-black/90 text-white font-mono text-xs p-3 rounded">
						<code><span class="text-blue-400">packages:</span>
  - <span class="text-yellow-400">"@team/shared-lib"</span>
  - <span class="text-yellow-400">"@company/design-system"</span>
  - <span class="text-yellow-400">"@external/utility"</span></code>
					</div>
					<ul class="text-sm text-muted-foreground space-y-1">
						<li>• Downloaded and cached locally</li>
						<li>• Version management support</li>
						<li>• Team namespace support</li>
						<li>• Authentication required for private packages</li>
					</ul>
				</div>
			</div>
		</div>
	</section>

	<!-- Workflow -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Development Workflow</h2>
		
		<div class="space-y-6">
			<p class="text-muted-foreground">
				Here's the typical workflow for developing with linked packages in a Knot monorepo.
			</p>

			<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
				<!-- Development Mode -->
				<div class="border rounded-lg p-6">
					<h3 class="font-semibold mb-3 flex items-center">
						<Icon icon="solar:code-bold" class="w-5 h-5 mr-2 text-purple-600" />
						Development Mode
					</h3>
					<div class="space-y-3">
						<div class="bg-black/90 text-purple-400 font-mono text-sm p-3 rounded relative group">
							<code>knot link --symlink</code>
							<button 
								class="absolute top-1 right-1 p-1.5 rounded bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot link --symlink')}
							>
								{#if showCopied && copyText === 'knot link --symlink'}
									<Icon icon="solar:check-circle-bold" class="w-3 h-3 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-3 h-3" />
								{/if}
							</button>
						</div>
						<ul class="text-sm text-muted-foreground space-y-1">
							<li>• Use for active development</li>
							<li>• Changes immediately reflect in apps</li>
							<li>• Hot reload works across packages</li>
							<li>• Perfect for debugging</li>
						</ul>
					</div>
				</div>

				<!-- Production Mode -->
				<div class="border rounded-lg p-6">
					<h3 class="font-semibold mb-3 flex items-center">
						<Icon icon="solar:rocket-bold" class="w-5 h-5 mr-2 text-green-600" />
						Production Mode
					</h3>
					<div class="space-y-3">
						<div class="bg-black/90 text-green-400 font-mono text-sm p-3 rounded relative group">
							<code>knot link</code>
							<button 
								class="absolute top-1 right-1 p-1.5 rounded bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot link')}
							>
								{#if showCopied && copyText === 'knot link'}
									<Icon icon="solar:check-circle-bold" class="w-3 h-3 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-3 h-3" />
								{/if}
							</button>
						</div>
						<ul class="text-sm text-muted-foreground space-y-1">
							<li>• Use for builds and deployment</li>
							<li>• Works in any environment</li>
							<li>• Docker compatible</li>
							<li>• Stable and reliable</li>
						</ul>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Pre-commit Hook -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Automatic Updates</h2>
		
		<div class="space-y-6">
			<div class="bg-blue-50 border border-blue-200 rounded-lg p-6">
				<div class="flex items-start space-x-3">
					<Icon icon="solar:refresh-bold" class="w-6 h-6 text-blue-600 mt-1 flex-shrink-0" />
					<div>
						<h3 class="font-semibold text-blue-900 mb-2">Pre-commit Hook</h3>
						<p class="text-sm text-blue-700 mb-3">
							Knot automatically includes a pre-commit hook that updates copied packages 
							before each commit, ensuring your linked packages are always current.
						</p>
						<div class="bg-black/90 text-green-400 font-mono text-xs p-3 rounded">
							<code># Automatically runs before each commit
knot link --update-copied</code>
						</div>
					</div>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Manual Updates</h3>
				<p class="text-muted-foreground mb-3">
					You can also manually update copied packages when needed:
				</p>
				<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
					<code>knot link --force</code>
					<button 
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
						on:click={() => copyToClipboard('knot link --force')}
					>
						{#if showCopied && copyText === 'knot link --force'}
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="solar:copy-bold" class="w-4 h-4" />
						{/if}
					</button>
				</div>
				<p class="text-sm text-muted-foreground mt-2">
					Forces re-linking of all packages, useful when switching between modes or after package updates.
				</p>
			</div>
		</div>
	</section>

	<!-- Troubleshooting -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Troubleshooting</h2>
		
		<div class="space-y-6">
			<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
				<div class="border rounded-lg p-6">
					<h3 class="font-semibold mb-3 flex items-center">
						<Icon icon="solar:danger-triangle-bold" class="w-5 h-5 mr-2 text-red-600" />
						Common Issues
					</h3>
					<div class="space-y-3 text-sm">
						<div>
							<div class="font-medium">Package not found</div>
							<div class="text-muted-foreground">Check package name in configuration</div>
						</div>
						<div>
							<div class="font-medium">TypeScript errors</div>
							<div class="text-muted-foreground">Run <code>knot link --force</code> to refresh</div>
						</div>
						<div>
							<div class="font-medium">Import paths not working</div>
							<div class="text-muted-foreground">Verify tsAlias configuration</div>
						</div>
					</div>
				</div>

				<div class="border rounded-lg p-6">
					<h3 class="font-semibold mb-3 flex items-center">
						<Icon icon="solar:tools-bold" class="w-5 h-5 mr-2 text-blue-600" />
						Debug Commands
					</h3>
					<div class="space-y-3 text-sm">
						<div>
							<div class="font-medium"><code>knot status</code></div>
							<div class="text-muted-foreground">Show project and linking status</div>
						</div>
						<div>
							<div class="font-medium"><code>knot link --verbose</code></div>
							<div class="text-muted-foreground">Detailed linking output</div>
						</div>
						<div>
							<div class="font-medium"><code>knot link --dry-run</code></div>
							<div class="text-muted-foreground">Preview changes without applying</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Next Steps -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Next Steps</h2>
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<a href="/docs/typescript" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:code-2-bold" class="w-6 h-6 text-blue-600" />
					<h3 class="font-semibold">TypeScript Integration</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Deep dive into TypeScript configuration and advanced features.
				</p>
			</a>

			<a href="/docs/cli-commands" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:terminal-bold" class="w-6 h-6 text-green-600" />
					<h3 class="font-semibold">CLI Commands</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Explore all linking commands and their options in detail.
				</p>
			</a>
		</div>
	</section>
</div>