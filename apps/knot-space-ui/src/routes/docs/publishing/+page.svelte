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
	<title>Publishing Packages - Knot CLI Documentation</title>
	<meta name="description" content="Learn how to publish packages to Knot registry, including version management, authentication, CI/CD workflows, and automated GitHub Actions." />
	<meta property="og:title" content="Publishing Packages - Knot CLI" />
	<meta property="og:description" content="Learn how to publish packages to Knot registry, including version management, authentication, CI/CD workflows, and automated GitHub Actions." />
	<meta property="og:image" content="/images/og/publishing.png" />
	<meta property="og:url" content="https://knot-space.com/docs/publishing" />
	<meta name="twitter:title" content="Publishing Packages - Knot CLI" />
	<meta name="twitter:description" content="Learn how to publish packages to Knot registry, including version management, authentication, CI/CD workflows, and automated GitHub Actions." />
	<meta name="twitter:image" content="/images/og/publishing.png" />
	<link rel="canonical" href="https://knot-space.com/docs/publishing" />
</svelte:head>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight mb-4" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
			Publishing Packages
		</h1>
		<p class="text-lg text-muted-foreground">
			Share your packages with the world through Knot Space registry and team collaboration
		</p>
	</div>

	<!-- Overview -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Overview</h2>
		<p class="text-muted-foreground leading-relaxed mb-6">
			Knot Space registry allows you to publish and share packages with your team or the public. 
			Whether you're building internal company libraries or contributing to open source, 
			Knot provides a seamless publishing experience with version management and access controls.
		</p>
		
		<div class="grid grid-cols-1 md:grid-cols-3 gap-6">
			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-green-100 rounded-lg flex items-center justify-center mb-4">
					<Icon icon="solar:upload-bold" class="w-6 h-6 text-green-600" />
				</div>
				<h3 class="font-semibold mb-2">Easy Publishing</h3>
				<p class="text-sm text-muted-foreground">
					One-command publishing with automatic validation and version management.
				</p>
			</div>
			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-blue-100 rounded-lg flex items-center justify-center mb-4">
					<Icon icon="solar:users-group-two-rounded-bold" class="w-6 h-6 text-blue-600" />
				</div>
				<h3 class="font-semibold mb-2">Team Collaboration</h3>
				<p class="text-sm text-muted-foreground">
					Organize packages by teams with granular access controls and permissions.
				</p>
			</div>
			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-purple-100 rounded-lg flex items-center justify-center mb-4">
					<Icon icon="solar:shield-check-bold" class="w-6 h-6 text-purple-600" />
				</div>
				<h3 class="font-semibold mb-2">Version Management</h3>
				<p class="text-sm text-muted-foreground">
					Automatic semantic versioning with changelog generation and rollback support.
				</p>
			</div>
		</div>
	</section>

	<!-- Authentication -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Authentication</h2>
		
		<div class="space-y-8">
			<div>
				<h3 class="text-lg font-semibold mb-4">Getting Started</h3>
				<p class="text-muted-foreground mb-4">
					Before publishing packages, you need to authenticate with Knot Space registry.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Login to Knot Space</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot login</code>
							<button 
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot login')}
							>
								{#if showCopied && copyText === 'knot login'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
						<p class="text-sm text-muted-foreground mt-2">
							Opens browser for OAuth authentication or prompts for credentials.
						</p>
					</div>

					<div>
						<h4 class="font-medium mb-2">Login with Token</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot login --token your-auth-token</code>
							<button 
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot login --token your-auth-token')}
							>
								{#if showCopied && copyText.includes('--token')}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
						<p class="text-sm text-muted-foreground mt-2">
							Use for CI/CD environments or programmatic access.
						</p>
					</div>

					<div>
						<h4 class="font-medium mb-2">Verify Authentication</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot whoami</code>
							<button 
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot whoami')}
							>
								{#if showCopied && copyText === 'knot whoami'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
						<p class="text-sm text-muted-foreground mt-2">
							Shows your current authenticated user and associated teams.
						</p>
					</div>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-4">Environment Configuration</h3>
				<p class="text-muted-foreground mb-4">
					Configure your registry URL and authentication settings for different environments.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Set Registry URL</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot config set registry https://knot.mycompany.com</code>
							<button 
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot config set registry https://knot.mycompany.com')}
							>
								{#if showCopied && copyText.includes('mycompany')}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Environment Variables</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-gray-400"># Set in your environment or CI/CD</span>
<span class="text-blue-400">export</span> <span class="text-yellow-400">KNOT_TOKEN</span>=<span class="text-green-400">"your-auth-token"</span>
<span class="text-blue-400">export</span> <span class="text-yellow-400">KNOT_SPACE_URL</span>=<span class="text-green-400">"https://knot.mycompany.com"</span></code></pre>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Publishing Process -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Publishing Process</h2>
		
		<div class="space-y-8">
			<div>
				<h3 class="text-lg font-semibold mb-4">Basic Publishing</h3>
				<p class="text-muted-foreground mb-4">
					Publish packages from their directory with automatic validation and build verification.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Publish Current Package</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>cd packages/utils && knot publish</code>
							<button 
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('cd packages/utils && knot publish')}
							>
								{#if showCopied && copyText.includes('packages/utils')}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
						<p class="text-sm text-muted-foreground mt-2">
							Publishes the package in the current directory to Knot Space registry.
						</p>
					</div>

					<div>
						<h4 class="font-medium mb-2">Pre-publication Checklist</h4>
						<div class="bg-blue-50 border border-blue-200 rounded-lg p-6">
							<h5 class="font-semibold text-blue-900 mb-3">Knot automatically verifies:</h5>
							<ul class="text-sm text-blue-700 space-y-1">
								<li>✓ Package configuration is valid</li>
								<li>✓ Version follows semantic versioning</li>
								<li>✓ Build completes successfully</li>
								<li>✓ Tests pass (if test script exists)</li>
								<li>✓ No uncommitted changes (git)</li>
								<li>✓ Authentication and permissions</li>
							</ul>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Example Publishing Flow</h4>
						<div class="bg-black/90 text-white font-mono text-xs p-4 rounded-lg">
							<pre><code><span class="text-green-400">$ cd packages/utils && knot publish</span>

<span class="text-blue-400">🔍 Validating package configuration...</span>
<span class="text-green-400">✓ package.yml is valid</span>
<span class="text-green-400">✓ Version 1.2.0 follows semver</span>

<span class="text-blue-400">🏗️ Building package...</span>
<span class="text-green-400">✓ TypeScript compilation successful</span>
<span class="text-green-400">✓ All tests pass</span>

<span class="text-blue-400">📦 Publishing to Knot Space...</span>
<span class="text-green-400">✓ Package uploaded successfully</span>
<span class="text-green-400">✓ Registry updated</span>

<span class="text-yellow-400">🎉 Successfully published utils@1.2.0</span>
<span class="text-gray-400">   View at: https://knot.space/packages/utils</span></code></pre>
						</div>
					</div>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-4">Publishing Options</h3>
				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Dry Run</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot publish --dry-run</code>
							<button 
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot publish --dry-run')}
							>
								{#if showCopied && copyText === 'knot publish --dry-run'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
						<p class="text-sm text-muted-foreground mt-2">
							Validate and build without actually publishing to the registry.
						</p>
					</div>

					<div>
						<h4 class="font-medium mb-2">Force Publish</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot publish --force</code>
							<button 
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot publish --force')}
							>
								{#if showCopied && copyText === 'knot publish --force'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
						<p class="text-sm text-muted-foreground mt-2">
							Skip some validation checks (use with caution in emergency situations).
						</p>
					</div>

					<div>
						<h4 class="font-medium mb-2">Tag Releases</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot publish --tag beta</code>
							<button 
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot publish --tag beta')}
							>
								{#if showCopied && copyText.includes('--tag beta')}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
						<p class="text-sm text-muted-foreground mt-2">
							Publish with a distribution tag (latest, beta, alpha, next).
						</p>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Team Publishing -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Team Publishing</h2>
		
		<div class="space-y-8">
			<div>
				<h3 class="text-lg font-semibold mb-4">Team-scoped Packages</h3>
				<p class="text-muted-foreground mb-4">
					Organize packages under team namespaces for better organization and access control.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Configure Team Namespace</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg relative group">
							<pre><code><span class="text-gray-400"># packages/shared-lib/package.yml</span>
<span class="text-blue-400">name:</span> <span class="text-green-400">shared-lib</span>
<span class="text-blue-400">team:</span> <span class="text-green-400">myteam</span>           <span class="text-gray-400"># Published as @myteam/shared-lib</span>
<span class="text-blue-400">version:</span> <span class="text-green-400">1.0.0</span></code></pre>
							<button 
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard(`# packages/shared-lib/package.yml
name: shared-lib
team: myteam           # Published as @myteam/shared-lib
version: 1.0.0`)}
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
						<h4 class="font-medium mb-2">Publishing Team Package</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>cd packages/shared-lib && knot publish</code>
							<button 
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('cd packages/shared-lib && knot publish')}
							>
								{#if showCopied && copyText.includes('shared-lib')}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
						<p class="text-sm text-muted-foreground mt-2">
							Publishes to registry as <code>@myteam/shared-lib</code> with team access controls.
						</p>
					</div>

					<div>
						<h4 class="font-medium mb-2">Using Team Packages</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-gray-400"># In your app configuration</span>
<span class="text-blue-400">packages:</span>
  - <span class="text-yellow-400">types</span>                    <span class="text-gray-400"># Local package</span>
  - <span class="text-yellow-400">"@myteam/shared-lib"</span>     <span class="text-gray-400"># Team package from registry</span></code></pre>
						</div>
					</div>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-4">Access Control</h3>
				<p class="text-muted-foreground mb-4">
					Team packages inherit access permissions from team membership and roles.
				</p>

				<div class="grid grid-cols-1 md:grid-cols-3 gap-6">
					<div class="border rounded-lg p-6">
						<h4 class="font-semibold mb-3 text-green-600">Team Members</h4>
						<ul class="text-sm text-muted-foreground space-y-1">
							<li>• View team packages</li>
							<li>• Install team packages</li>
							<li>• View package metadata</li>
							<li>• Access documentation</li>
						</ul>
					</div>

					<div class="border rounded-lg p-6">
						<h4 class="font-semibold mb-3 text-blue-600">Team Admins</h4>
						<ul class="text-sm text-muted-foreground space-y-1">
							<li>• All member permissions</li>
							<li>• Publish new versions</li>
							<li>• Manage package settings</li>
							<li>• View download analytics</li>
						</ul>
					</div>

					<div class="border rounded-lg p-6">
						<h4 class="font-semibold mb-3 text-purple-600">Team Owners</h4>
						<ul class="text-sm text-muted-foreground space-y-1">
							<li>• All admin permissions</li>
							<li>• Delete packages</li>
							<li>• Transfer ownership</li>
							<li>• Manage team members</li>
						</ul>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Version Management -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Version Management</h2>
		
		<div class="space-y-8">
			<div>
				<h3 class="text-lg font-semibold mb-4">Semantic Versioning</h3>
				<p class="text-muted-foreground mb-4">
					Knot enforces semantic versioning to ensure predictable updates and compatibility.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Version Bumping</h4>
						<div class="grid grid-cols-1 md:grid-cols-3 gap-3">
							<div class="border rounded-lg p-4">
								<h5 class="font-medium mb-2">Patch Release</h5>
								<div class="bg-black/90 text-green-400 font-mono text-sm p-3 rounded mb-2">
									<code>knot version patch</code>
								</div>
								<p class="text-xs text-muted-foreground">Bug fixes, no breaking changes</p>
							</div>
							<div class="border rounded-lg p-4">
								<h5 class="font-medium mb-2">Minor Release</h5>
								<div class="bg-black/90 text-green-400 font-mono text-sm p-3 rounded mb-2">
									<code>knot version minor</code>
								</div>
								<p class="text-xs text-muted-foreground">New features, backward compatible</p>
							</div>
							<div class="border rounded-lg p-4">
								<h5 class="font-medium mb-2">Major Release</h5>
								<div class="bg-black/90 text-green-400 font-mono text-sm p-3 rounded mb-2">
									<code>knot version major</code>
								</div>
								<p class="text-xs text-muted-foreground">Breaking changes</p>
							</div>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Pre-release Versions</h4>
						<div class="space-y-3">
							<div class="bg-black/90 text-green-400 font-mono text-sm p-3 rounded">
								<code>knot version 2.0.0-beta.1</code>
							</div>
							<div class="bg-black/90 text-green-400 font-mono text-sm p-3 rounded">
								<code>knot version prerelease --preid alpha</code>
							</div>
						</div>
					</div>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-4">Release Management</h3>
				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Complete Release Workflow</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<div class="space-y-1">
								<div><code># 1. Update version</code></div>
								<div><code>knot version minor</code></div>
								<div><code># 2. Build and test</code></div>
								<div><code>knot run build && knot run test</code></div>
								<div><code># 3. Publish to registry</code></div>
								<div><code>knot publish</code></div>
								<div><code># 4. Create git tag</code></div>
								<div><code>git tag v1.3.0 && git push --tags</code></div>
							</div>
							<button 
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard(`# 1. Update version
knot version minor
# 2. Build and test
knot run build && knot run test
# 3. Publish to registry
knot publish
# 4. Create git tag
git tag v1.3.0 && git push --tags`)}
							>
								{#if showCopied && copyText.includes('Update version')}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Automated Releases</h4>
						<p class="text-sm text-muted-foreground mb-2">
							Set up automated releases with CI/CD:
						</p>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code>{@html `<span class="text-gray-400"># .github/workflows/release.yml</span>
<span class="text-blue-400">name:</span> <span class="text-green-400">Release</span>
<span class="text-blue-400">on:</span>
  <span class="text-blue-400">push:</span>
    <span class="text-blue-400">tags:</span> [<span class="text-green-400">'v*'</span>]

<span class="text-blue-400">jobs:</span>
  <span class="text-blue-400">publish:</span>
    <span class="text-blue-400">runs-on:</span> <span class="text-green-400">ubuntu-latest</span>
    <span class="text-blue-400">steps:</span>
      - <span class="text-blue-400">run:</span> <span class="text-green-400">knot login --token \${{ secrets.KNOT_TOKEN }}</span>
      - <span class="text-blue-400">run:</span> <span class="text-green-400">knot publish</span>`}</code></pre>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Package Discovery -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Package Discovery</h2>
		
		<div class="space-y-8">
			<div>
				<h3 class="text-lg font-semibold mb-4">Package Metadata</h3>
				<p class="text-muted-foreground mb-4">
					Optimize your package for discoverability with rich metadata and documentation.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Package Tags</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-gray-400"># package.yml</span>
<span class="text-blue-400">tags:</span>
  - <span class="text-yellow-400">utilities</span>
  - <span class="text-yellow-400">typescript</span>
  - <span class="text-yellow-400">react</span>
  - <span class="text-yellow-400">ui-components</span>
  - <span class="text-yellow-400">design-system</span></code></pre>
						</div>
						<p class="text-sm text-muted-foreground mt-2">
							Tags help users discover your package in the Knot Space web interface.
						</p>
					</div>

					<div>
						<h4 class="font-medium mb-2">Rich Description</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-blue-400">description:</span> <span class="text-green-400">"React UI components following Material Design principles. Includes buttons, forms, navigation, and data display components with full TypeScript support and customizable themes."</span></code></pre>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Documentation</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-gray-400"># package.yml</span>
<span class="text-blue-400">documentation:</span>
  <span class="text-blue-400">homepage:</span> <span class="text-green-400">"https://ui.mycompany.com"</span>
  <span class="text-blue-400">repository:</span> <span class="text-green-400">"https://github.com/mycompany/ui-components"</span>
  <span class="text-blue-400">issues:</span> <span class="text-green-400">"https://github.com/mycompany/ui-components/issues"</span></code></pre>
						</div>
					</div>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-4">README Best Practices</h3>
				<div class="space-y-4">
					<div class="border rounded-lg p-6">
						<h4 class="font-semibold mb-3">Essential Sections</h4>
						<ul class="text-sm text-muted-foreground space-y-1">
							<li>• Clear package description and purpose</li>
							<li>• Installation instructions</li>
							<li>• Basic usage examples</li>
							<li>• API documentation or links</li>
							<li>• Contributing guidelines</li>
							<li>• License information</li>
						</ul>
					</div>

					<div class="border rounded-lg p-6">
						<h4 class="font-semibold mb-3">Code Examples</h4>
						<ul class="text-sm text-muted-foreground space-y-1">
							<li>• Include practical, copy-paste examples</li>
							<li>• Show import statements</li>
							<li>• Demonstrate common use cases</li>
							<li>• Include TypeScript types</li>
							<li>• Show configuration options</li>
						</ul>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Unpublishing -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Package Management</h2>
		
		<div class="space-y-8">
			<div>
				<h3 class="text-lg font-semibold mb-4">Unpublishing Packages</h3>
				<p class="text-muted-foreground mb-4">
					Remove packages or specific versions from the registry when necessary.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Unpublish Specific Version</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot unpublish utils@1.2.0</code>
							<button 
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot unpublish utils@1.2.0')}
							>
								{#if showCopied && copyText === 'knot unpublish utils@1.2.0'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Unpublish Entire Package</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot unpublish utils --force</code>
							<button 
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot unpublish utils --force')}
							>
								{#if showCopied && copyText === 'knot unpublish utils --force'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div class="bg-red-50 border border-red-200 rounded-lg p-6">
						<div class="flex items-start space-x-3">
							<Icon icon="solar:danger-triangle-bold" class="w-6 h-6 text-red-600 mt-1 flex-shrink-0" />
							<div>
								<h4 class="font-semibold text-red-900 mb-2">Unpublishing Warning</h4>
								<p class="text-sm text-red-700">
									Unpublishing packages can break projects that depend on them. Only unpublish in emergency 
									situations or for packages with no external dependencies. Consider deprecation instead.
								</p>
							</div>
						</div>
					</div>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-4">Package Deprecation</h3>
				<p class="text-muted-foreground mb-4">
					Mark packages as deprecated to discourage new usage while preserving existing installations.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Deprecate Package</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot deprecate utils "This package is deprecated. Use @myteam/new-utils instead."</code>
							<button 
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot deprecate utils "This package is deprecated. Use @myteam/new-utils instead."')}
							>
								{#if showCopied && copyText.includes('deprecated')}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Remove Deprecation</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot undeprecate utils</code>
							<button 
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot undeprecate utils')}
							>
								{#if showCopied && copyText === 'knot undeprecate utils'}
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

	<!-- Next Steps -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Next Steps</h2>
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<a href="/docs/teams" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:users-group-two-rounded-bold" class="w-6 h-6 text-blue-600" />
					<h3 class="font-semibold">Team Management</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Learn how to create and manage teams for collaborative package development.
				</p>
			</a>

			<a href="/docs/permissions" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:shield-check-bold" class="w-6 h-6 text-green-600" />
					<h3 class="font-semibold">Permissions</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Understand access controls, roles, and security for package publishing.
				</p>
			</a>

			<a href="/docs/cli-packages" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:box-bold" class="w-6 h-6 text-purple-600" />
					<h3 class="font-semibold">Package Development</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Go deeper into package development workflows and best practices.
				</p>
			</a>

			<a href="/docs/self-hosting" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:server-bold" class="w-6 h-6 text-red-600" />
					<h3 class="font-semibold">Self-Hosting</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Set up your own private Knot Space registry for internal package management.
				</p>
			</a>
		</div>
	</section>
</div>