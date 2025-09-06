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
	<title>Version Management - Knot CLI Documentation</title>
	<meta name="description" content="Complete guide to version management in Knot CLI with semantic versioning, package releases, and version control strategies." />
	<meta property="og:title" content="Version Management - Knot CLI" />
	<meta property="og:description" content="Complete guide to version management in Knot CLI with semantic versioning, package releases, and version control strategies." />
	<meta property="og:image" content="/images/og/version-management.png" />
	<meta property="og:url" content="https://knot.klysium.com/docs/version-management" />
	<meta name="twitter:title" content="Version Management - Knot CLI" />
	<meta name="twitter:description" content="Complete guide to version management in Knot CLI with semantic versioning, package releases, and version control strategies." />
	<meta name="twitter:image" content="/images/og/version-management.png" />
	<link rel="canonical" href="https://knot.klysium.com/docs/version-management" />
</svelte:head>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight mb-4" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
			Version Management
		</h1>
		<p class="text-lg text-muted-foreground">
			Comprehensive guide to managing package versions, releases, and semantic versioning in Knot
		</p>
	</div>

	<!-- Overview -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Overview</h2>
		<div class="prose prose-gray max-w-none">
			<p class="text-muted-foreground">
				Knot provides powerful version management capabilities that help you maintain consistent versioning across your monorepo packages. From semantic versioning to coordinated releases, Knot ensures your packages are properly versioned and dependencies are managed effectively.
			</p>
		</div>
		
		<div class="mt-6 grid grid-cols-1 md:grid-cols-3 gap-6">
			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-blue-100 rounded-lg flex items-center justify-center mb-4">
					<Icon icon="solar:tag-bold" class="w-6 h-6 text-blue-600" />
				</div>
				<h3 class="font-semibold mb-2">Semantic Versioning</h3>
				<p class="text-sm text-muted-foreground">
					Automatic semantic version calculation based on changes and conventional commits.
				</p>
			</div>

			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-green-100 rounded-lg flex items-center justify-center mb-4">
					<Icon icon="solar:refresh-bold" class="w-6 h-6 text-green-600" />
				</div>
				<h3 class="font-semibold mb-2">Coordinated Releases</h3>
				<p class="text-sm text-muted-foreground">
					Release multiple packages together with dependency graph awareness.
				</p>
			</div>

			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-purple-100 rounded-lg flex items-center justify-center mb-4">
					<Icon icon="solar:shield-check-bold" class="w-6 h-6 text-purple-600" />
				</div>
				<h3 class="font-semibold mb-2">Version Constraints</h3>
				<p class="text-sm text-muted-foreground">
					Flexible constraint system with range specifications and compatibility checks.
				</p>
			</div>
		</div>
	</section>

	<!-- Version Commands -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Version Management Commands</h2>
		
		<div class="space-y-8">
			<!-- knot version -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">knot version</h3>
				<p class="text-muted-foreground mb-4">
					Display current version information and manage package versions with semantic versioning support.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Display Version Information</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot version</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot version')}
							>
								{#if showCopied && copyText === 'knot version'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Bump Version (Auto-detect)</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot version bump</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot version bump')}
							>
								{#if showCopied && copyText === 'knot version bump'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
						<p class="text-sm text-muted-foreground mt-2">
							Automatically determines version bump type based on conventional commits.
						</p>
					</div>

					<div>
						<h4 class="font-medium mb-2">Specific Version Bumps</h4>
						<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
							<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
								<code>knot version major</code>
								<button
									class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
									on:click={() => copyToClipboard('knot version major')}
								>
									{#if showCopied && copyText === 'knot version major'}
										<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
									{:else}
										<Icon icon="solar:copy-bold" class="w-4 h-4" />
									{/if}
								</button>
							</div>
							<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
								<code>knot version minor</code>
								<button
									class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
									on:click={() => copyToClipboard('knot version minor')}
								>
									{#if showCopied && copyText === 'knot version minor'}
										<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
									{:else}
										<Icon icon="solar:copy-bold" class="w-4 h-4" />
									{/if}
								</button>
							</div>
							<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
								<code>knot version patch</code>
								<button
									class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
									on:click={() => copyToClipboard('knot version patch')}
								>
									{#if showCopied && copyText === 'knot version patch'}
										<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
									{:else}
										<Icon icon="solar:copy-bold" class="w-4 h-4" />
									{/if}
								</button>
							</div>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Prerelease Versions</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot version prerelease --tag alpha</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot version prerelease --tag alpha')}
							>
								{#if showCopied && copyText.includes('prerelease')}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>
				</div>
			</div>

			<!-- knot upgrade -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">knot upgrade</h3>
				<p class="text-muted-foreground mb-4">
					Upgrade dependencies across your workspace with intelligent conflict resolution.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Upgrade All Dependencies</h4>
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
						<h4 class="font-medium mb-2">Upgrade Specific Package</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot upgrade lodash --target "^4.17.21"</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot upgrade lodash --target "^4.17.21"')}
							>
								{#if showCopied && copyText.includes('lodash')}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Interactive Upgrade</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot upgrade --interactive</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot upgrade --interactive')}
							>
								{#if showCopied && copyText.includes('interactive')}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>
				</div>
			</div>

			<!-- knot delete -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">knot delete</h3>
				<p class="text-muted-foreground mb-4">
					Delete specific package versions from the registry with safety checks.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Delete Specific Version</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot delete @myteam/utils@1.0.0</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot delete @myteam/utils@1.0.0')}
							>
								{#if showCopied && copyText.includes('@myteam/utils')}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div class="bg-yellow-50 border border-yellow-200 rounded-lg p-4">
						<div class="flex items-start space-x-3">
							<Icon icon="solar:warning-bold" class="w-5 h-5 text-yellow-600 mt-0.5 flex-shrink-0" />
							<div>
								<h4 class="font-semibold text-yellow-900 mb-1">Caution</h4>
								<p class="text-sm text-yellow-700">
									Package deletion is permanent and may break dependent packages. Use with extreme caution and only for packages you own.
								</p>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Semantic Versioning -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Semantic Versioning</h2>
		
		<div class="space-y-6">
			<div class="prose prose-gray max-w-none">
				<p class="text-muted-foreground">
					Knot follows the Semantic Versioning (SemVer) specification, automatically determining appropriate version bumps based on your changes and conventional commit messages.
				</p>
			</div>

			<div class="grid grid-cols-1 md:grid-cols-3 gap-6">
				<div class="border rounded-lg p-6">
					<div class="w-12 h-12 bg-red-100 rounded-lg flex items-center justify-center mb-4">
						<span class="text-red-600 font-bold text-lg">X</span>
					</div>
					<h3 class="font-semibold mb-2 text-red-600">Major (X.y.z)</h3>
					<p class="text-sm text-muted-foreground mb-3">Breaking changes that affect API compatibility</p>
					<div class="text-xs space-y-1">
						<div><code class="bg-muted px-1 rounded">1.0.0 → 2.0.0</code></div>
						<div class="text-muted-foreground">• API changes</div>
						<div class="text-muted-foreground">• Breaking changes</div>
					</div>
				</div>

				<div class="border rounded-lg p-6">
					<div class="w-12 h-12 bg-yellow-100 rounded-lg flex items-center justify-center mb-4">
						<span class="text-yellow-600 font-bold text-lg">Y</span>
					</div>
					<h3 class="font-semibold mb-2 text-yellow-600">Minor (x.Y.z)</h3>
					<p class="text-sm text-muted-foreground mb-3">New features that maintain backward compatibility</p>
					<div class="text-xs space-y-1">
						<div><code class="bg-muted px-1 rounded">1.0.0 → 1.1.0</code></div>
						<div class="text-muted-foreground">• New features</div>
						<div class="text-muted-foreground">• Enhancements</div>
					</div>
				</div>

				<div class="border rounded-lg p-6">
					<div class="w-12 h-12 bg-green-100 rounded-lg flex items-center justify-center mb-4">
						<span class="text-green-600 font-bold text-lg">Z</span>
					</div>
					<h3 class="font-semibold mb-2 text-green-600">Patch (x.y.Z)</h3>
					<p class="text-sm text-muted-foreground mb-3">Bug fixes and small improvements</p>
					<div class="text-xs space-y-1">
						<div><code class="bg-muted px-1 rounded">1.0.0 → 1.0.1</code></div>
						<div class="text-muted-foreground">• Bug fixes</div>
						<div class="text-muted-foreground">• Patches</div>
					</div>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-4">Conventional Commits Integration</h3>
				<div class="bg-blue-50 border border-blue-200 rounded-lg p-6">
					<div class="space-y-4">
						<div>
							<h4 class="font-medium mb-2">Automatic Version Detection</h4>
							<p class="text-sm text-muted-foreground mb-3">
								Knot analyzes your commit messages to determine the appropriate version bump:
							</p>
							<div class="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm">
								<div>
									<code class="bg-white px-2 py-1 rounded text-xs">feat: add login</code>
									<div class="text-blue-700 mt-1">→ Minor bump</div>
								</div>
								<div>
									<code class="bg-white px-2 py-1 rounded text-xs">fix: auth bug</code>
									<div class="text-blue-700 mt-1">→ Patch bump</div>
								</div>
								<div>
									<code class="bg-white px-2 py-1 rounded text-xs">feat!: new API</code>
									<div class="text-blue-700 mt-1">→ Major bump</div>
								</div>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Prerelease Versions -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Prerelease Versions</h2>
		
		<div class="space-y-6">
			<div class="prose prose-gray max-w-none">
				<p class="text-muted-foreground">
					Knot supports prerelease versions for testing and staged releases with common prerelease tags.
				</p>
			</div>

			<div class="grid grid-cols-1 md:grid-cols-4 gap-4">
				<div class="border rounded-lg p-4 text-center">
					<h4 class="font-semibold text-blue-600 mb-2">alpha</h4>
					<div class="text-xs text-muted-foreground mb-2">1.0.0-alpha.1</div>
					<p class="text-xs text-muted-foreground">Early development</p>
				</div>
				<div class="border rounded-lg p-4 text-center">
					<h4 class="font-semibold text-green-600 mb-2">beta</h4>
					<div class="text-xs text-muted-foreground mb-2">1.0.0-beta.1</div>
					<p class="text-xs text-muted-foreground">Feature complete</p>
				</div>
				<div class="border rounded-lg p-4 text-center">
					<h4 class="font-semibold text-yellow-600 mb-2">rc</h4>
					<div class="text-xs text-muted-foreground mb-2">1.0.0-rc.1</div>
					<p class="text-xs text-muted-foreground">Release candidate</p>
				</div>
				<div class="border rounded-lg p-4 text-center">
					<h4 class="font-semibold text-purple-600 mb-2">next</h4>
					<div class="text-xs text-muted-foreground mb-2">1.0.0-next.1</div>
					<p class="text-xs text-muted-foreground">Experimental</p>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-4">Working with Prereleases</h3>
				<div class="space-y-4">
					<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
						<pre><code># Create alpha prerelease
knot version prerelease --tag alpha

# Increment existing prerelease
knot version prerelease

# Publish prerelease
knot publish --tag alpha</code></pre>
						<button
							class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
							on:click={() => copyToClipboard('# Create alpha prerelease\nknot version prerelease --tag alpha\n\n# Increment existing prerelease\nknot version prerelease\n\n# Publish prerelease\nknot publish --tag alpha')}
						>
							{#if showCopied && copyText.includes('alpha')}
								<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
							{:else}
								<Icon icon="solar:copy-bold" class="w-4 h-4" />
							{/if}
						</button>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Version Constraints -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Version Constraints & Ranges</h2>
		
		<div class="space-y-6">
			<div class="prose prose-gray max-w-none">
				<p class="text-muted-foreground">
					Knot provides flexible version constraint syntax for specifying dependency requirements with precise control over compatibility.
				</p>
			</div>

			<div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
				<div>
					<h3 class="text-lg font-semibold mb-4">Basic Operators</h3>
					<div class="space-y-3">
						<div class="flex items-center justify-between p-3 border rounded">
							<code class="text-sm">^1.2.3</code>
							<span class="text-sm text-muted-foreground">≥1.2.3 &lt;2.0.0</span>
						</div>
						<div class="flex items-center justify-between p-3 border rounded">
							<code class="text-sm">~1.2.3</code>
							<span class="text-sm text-muted-foreground">≥1.2.3 &lt;1.3.0</span>
						</div>
						<div class="flex items-center justify-between p-3 border rounded">
							<code class="text-sm">>=1.2.3</code>
							<span class="text-sm text-muted-foreground">Greater than or equal</span>
						</div>
						<div class="flex items-center justify-between p-3 border rounded">
							<code class="text-sm">&lt;1.5.0</code>
							<span class="text-sm text-muted-foreground">Less than</span>
						</div>
					</div>
				</div>

				<div>
					<h3 class="text-lg font-semibold mb-4">Advanced Ranges</h3>
					<div class="space-y-3">
						<div class="p-3 border rounded">
							<code class="text-sm block mb-1">1.2.3 - 2.3.4</code>
							<span class="text-xs text-muted-foreground">Range between versions</span>
						</div>
						<div class="p-3 border rounded">
							<code class="text-sm block mb-1">1.2.3 || 2.0.0</code>
							<span class="text-xs text-muted-foreground">Either version (OR)</span>
						</div>
						<div class="p-3 border rounded">
							<code class="text-sm block mb-1">^1.0.0 || ^2.0.0</code>
							<span class="text-xs text-muted-foreground">Multiple compatible ranges</span>
						</div>
						<div class="p-3 border rounded">
							<code class="text-sm block mb-1">1.0.0-alpha.1</code>
							<span class="text-xs text-muted-foreground">Specific prerelease</span>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Coordinated Releases -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Coordinated Releases</h2>
		
		<div class="space-y-6">
			<div class="prose prose-gray max-w-none">
				<p class="text-muted-foreground">
					Knot can manage coordinated releases across multiple packages, ensuring that dependent packages are updated together with proper version constraints.
				</p>
			</div>

			<div class="space-y-6">
				<div>
					<h3 class="text-lg font-semibold mb-4">Release Strategies</h3>
					<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
						<div class="border rounded-lg p-6">
							<h4 class="font-semibold text-blue-600 mb-3">Independent Releases</h4>
							<p class="text-sm text-muted-foreground mb-3">
								Each package maintains its own version independently.
							</p>
							<div class="bg-black/90 text-green-400 font-mono text-xs p-3 rounded">
								<code>knot version --independent</code>
							</div>
						</div>

						<div class="border rounded-lg p-6">
							<h4 class="font-semibold text-green-600 mb-3">Fixed/Locked Releases</h4>
							<p class="text-sm text-muted-foreground mb-3">
								All packages share the same version number.
							</p>
							<div class="bg-black/90 text-green-400 font-mono text-xs p-3 rounded">
								<code>knot version --fixed</code>
							</div>
						</div>
					</div>
				</div>

				<div>
					<h3 class="text-lg font-semibold mb-4">Release Workflow</h3>
					<div class="space-y-4">
						<div class="bg-blue-50 border border-blue-200 rounded-lg p-6">
							<h4 class="font-semibold text-blue-900 mb-3">Typical Release Process</h4>
							<div class="space-y-2 text-sm text-blue-700 font-mono">
								<div>1. <code>knot version bump</code> - Calculate version bumps</div>
								<div>2. <code>knot run build --all</code> - Build all packages</div>
								<div>3. <code>knot run test --all</code> - Run all tests</div>
								<div>4. <code>knot publish --all</code> - Publish packages</div>
								<div>5. <code>git tag v1.2.3</code> - Tag the release</div>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Best Practices -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Version Management Best Practices</h2>
		
		<div class="grid grid-cols-1 md:grid-cols-2 gap-8">
			<div class="space-y-6">
				<div>
					<h3 class="text-lg font-semibold mb-4">Versioning Strategy</h3>
					<ul class="space-y-3 text-sm text-muted-foreground">
						<li class="flex items-start space-x-2">
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600 mt-0.5 flex-shrink-0" />
							<span>Use conventional commits for automatic version detection</span>
						</li>
						<li class="flex items-start space-x-2">
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600 mt-0.5 flex-shrink-0" />
							<span>Follow semantic versioning principles strictly</span>
						</li>
						<li class="flex items-start space-x-2">
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600 mt-0.5 flex-shrink-0" />
							<span>Use prerelease versions for testing</span>
						</li>
						<li class="flex items-start space-x-2">
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600 mt-0.5 flex-shrink-0" />
							<span>Maintain clear changelog documentation</span>
						</li>
					</ul>
				</div>

				<div>
					<h3 class="text-lg font-semibold mb-4">Dependency Management</h3>
					<ul class="space-y-3 text-sm text-muted-foreground">
						<li class="flex items-start space-x-2">
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600 mt-0.5 flex-shrink-0" />
							<span>Use appropriate version ranges for flexibility</span>
						</li>
						<li class="flex items-start space-x-2">
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600 mt-0.5 flex-shrink-0" />
							<span>Pin critical dependencies to exact versions</span>
						</li>
						<li class="flex items-start space-x-2">
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600 mt-0.5 flex-shrink-0" />
							<span>Regularly audit and update dependencies</span>
						</li>
						<li class="flex items-start space-x-2">
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600 mt-0.5 flex-shrink-0" />
							<span>Test dependency upgrades in isolation</span>
						</li>
					</ul>
				</div>
			</div>

			<div class="space-y-6">
				<div>
					<h3 class="text-lg font-semibold mb-4">Release Management</h3>
					<ul class="space-y-3 text-sm text-muted-foreground">
						<li class="flex items-start space-x-2">
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600 mt-0.5 flex-shrink-0" />
							<span>Coordinate releases across dependent packages</span>
						</li>
						<li class="flex items-start space-x-2">
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600 mt-0.5 flex-shrink-0" />
							<span>Test thoroughly before publishing</span>
						</li>
						<li class="flex items-start space-x-2">
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600 mt-0.5 flex-shrink-0" />
							<span>Use CI/CD for consistent releases</span>
						</li>
						<li class="flex items-start space-x-2">
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600 mt-0.5 flex-shrink-0" />
							<span>Tag releases in version control</span>
						</li>
					</ul>
				</div>

				<div>
					<h3 class="text-lg font-semibold mb-4">Monitoring</h3>
					<ul class="space-y-3 text-sm text-muted-foreground">
						<li class="flex items-start space-x-2">
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600 mt-0.5 flex-shrink-0" />
							<span>Monitor package usage and downloads</span>
						</li>
						<li class="flex items-start space-x-2">
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600 mt-0.5 flex-shrink-0" />
							<span>Track breaking changes and their impact</span>
						</li>
						<li class="flex items-start space-x-2">
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600 mt-0.5 flex-shrink-0" />
							<span>Set up alerts for failed releases</span>
						</li>
						<li class="flex items-start space-x-2">
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600 mt-0.5 flex-shrink-0" />
							<span>Review version compatibility regularly</span>
						</li>
					</ul>
				</div>
			</div>
		</div>
	</section>

	<!-- Next Steps -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Next Steps</h2>
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<a href="/docs/dependency-management" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:settings-bold" class="w-6 h-6 text-yellow-600" />
					<h3 class="font-semibold">Dependency Management</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Learn about advanced dependency configuration and resolution strategies.
				</p>
			</a>

			<a href="/docs/publishing" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:upload-bold" class="w-6 h-6 text-red-600" />
					<h3 class="font-semibold">Publishing</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Publish and share your versioned packages with the Knot Space registry.
				</p>
			</a>

			<a href="/docs/cli-commands" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:terminal-bold" class="w-6 h-6 text-blue-600" />
					<h3 class="font-semibold">CLI Commands</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Complete reference for all version management CLI commands.
				</p>
			</a>

			<a href="/docs/workflows" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:code-circle-bold" class="w-6 h-6 text-green-600" />
					<h3 class="font-semibold">Workflow Best Practices</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Learn effective workflows for version management in team environments.
				</p>
			</a>
		</div>
	</section>
</div>