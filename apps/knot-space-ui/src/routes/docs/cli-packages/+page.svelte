<script lang="ts">
	import Icon from '@iconify/svelte';
</script>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight mb-4" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
			Package Commands
		</h1>
		<p class="text-lg text-muted-foreground">
			Manage packages with Knot's powerful CLI commands.
		</p>
	</div>

	<div class="prose prose-slate max-w-none">
		<h2 class="text-2xl font-bold mb-4">Publishing Packages</h2>
		
		<div class="bg-muted rounded-lg p-4 mb-6">
			<h3 class="font-semibold mb-2">knot publish</h3>
			<p class="text-sm text-muted-foreground mb-3">Publish a package to the registry</p>
			<div class="bg-black/90 text-white font-mono text-sm rounded-lg">
				<pre class="overflow-x-auto p-4"><code>{@html `<span class="text-gray-400"># Publish current package</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">publish</span>

<span class="text-gray-400"># Publish with specific tag</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">publish</span> <span class="text-yellow-400">--tag</span> <span class="text-purple-400">beta</span>

<span class="text-gray-400"># Publish with access level</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">publish</span> <span class="text-yellow-400">--access</span> <span class="text-purple-400">public</span>

<span class="text-gray-400"># Dry run (simulate publish)</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">publish</span> <span class="text-yellow-400">--dry-run</span>`}</code></pre>
			</div>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Installing Packages</h2>
		
		<div class="space-y-4">
			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">knot install</h3>
				<p class="text-sm text-muted-foreground mb-3">Install packages and dependencies</p>
				<div class="bg-black/90 text-white font-mono text-sm rounded-lg">
					<pre class="overflow-x-auto p-4"><code>{@html `<span class="text-gray-400"># Install package (latest version by default)</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">install</span> <span class="text-purple-400">utils</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">install</span> <span class="text-purple-400">@org/package-name</span>

<span class="text-gray-400"># Install specific version (exact)</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">install</span> <span class="text-purple-400">@org/package-name</span><span class="text-yellow-400">@1.2.3</span>

<span class="text-gray-400"># Install with version ranges (npm-style)</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">install</span> <span class="text-purple-400">@org/package-name</span><span class="text-yellow-400">@^1.0.0</span>    <span class="text-gray-400"># Compatible with 1.x.x</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">install</span> <span class="text-purple-400">@org/package-name</span><span class="text-yellow-400">@~1.2.0</span>    <span class="text-gray-400"># Compatible with 1.2.x</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">install</span> <span class="text-purple-400">@org/package-name</span><span class="text-yellow-400">@latest</span>    <span class="text-gray-400"># Latest version</span>

<span class="text-gray-400"># Install team package with version</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">install</span> <span class="text-purple-400">@team/package</span><span class="text-yellow-400">@2.1.0</span>`}</code></pre>
				</div>
			</div>

			<div class="bg-blue-50 border border-blue-200 rounded-lg p-4">
				<div class="flex items-start space-x-3">
					<Icon icon="solar:info-circle-bold" class="w-5 h-5 text-blue-600 mt-0.5 flex-shrink-0" />
					<div class="text-sm text-blue-800">
						<p class="font-semibold mb-1">Version Storage</p>
						<p>Packages are saved with version specifications in <code class="bg-blue-100 px-1 rounded">app.yml</code>:</p>
						<div class="bg-blue-900/20 border border-blue-300/30 rounded-lg mt-2">
							<pre class="overflow-x-auto p-3 text-xs"><code class="text-blue-800">{@html `<span class="text-blue-600">packages</span><span class="text-blue-500">:</span>
  <span class="text-blue-500">-</span> <span class="text-blue-700">utils</span><span class="text-blue-500">@</span><span class="text-blue-600">latest</span>
  <span class="text-blue-500">-</span> <span class="text-blue-700">@org/ui</span><span class="text-blue-500">@</span><span class="text-blue-600">^2.0.0</span>
  <span class="text-blue-500">-</span> <span class="text-blue-700">@team/lib</span><span class="text-blue-500">@</span><span class="text-blue-600">1.5.2</span>`}</code></pre>
						</div>
					</div>
				</div>
			</div>

			<div class="bg-yellow-50 border border-yellow-200 rounded-lg p-4">
				<div class="flex items-start space-x-3">
					<Icon icon="solar:info-circle-bold" class="w-5 h-5 text-yellow-600 mt-0.5 flex-shrink-0" />
					<div class="text-sm text-yellow-800">
						<p class="font-semibold mb-1">Backward Compatibility</p>
						<p>The <code class="bg-yellow-100 px-1 rounded">knot add</code> command is an alias for <code class="bg-yellow-100 px-1 rounded">knot install</code>.</p>
					</div>
				</div>
			</div>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Managing Dependencies</h2>
		
		<div class="space-y-4">
			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">knot update</h3>
				<p class="text-sm text-muted-foreground mb-3">Update packages to latest versions</p>
				<div class="bg-black/90 text-white font-mono text-sm rounded-lg">
					<pre class="overflow-x-auto p-4"><code>{@html `<span class="text-gray-400"># Update all packages</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">update</span>

<span class="text-gray-400"># Update specific package</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">update</span> <span class="text-purple-400">@org/package</span>

<span class="text-gray-400"># Update to latest major version</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">update</span> <span class="text-purple-400">@org/package</span> <span class="text-yellow-400">--latest</span>

<span class="text-gray-400"># Interactive update</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">update</span> <span class="text-yellow-400">--interactive</span>`}</code></pre>
				</div>
			</div>

			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">knot remove</h3>
				<p class="text-sm text-muted-foreground mb-3">Remove packages from your project</p>
				<div class="bg-black/90 text-white font-mono text-sm rounded-lg">
					<pre class="overflow-x-auto p-4"><code>{@html `<span class="text-gray-400"># Remove package</span>
<span class="text-green-400">knot</span> <span class="text-red-400">remove</span> <span class="text-purple-400">@org/package</span>

<span class="text-gray-400"># Remove multiple packages</span>
<span class="text-green-400">knot</span> <span class="text-red-400">remove</span> <span class="text-purple-400">package1</span> <span class="text-purple-400">package2</span>

<span class="text-gray-400"># Remove from specific workspace</span>
<span class="text-green-400">knot</span> <span class="text-red-400">remove</span> <span class="text-purple-400">@org/utils</span> <span class="text-yellow-400">--workspace</span> <span class="text-purple-400">@org/app</span>`}</code></pre>
				</div>
			</div>

			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">knot list</h3>
				<p class="text-sm text-muted-foreground mb-3">List installed packages</p>
				<div class="bg-black/90 text-white font-mono text-sm rounded-lg">
					<pre class="overflow-x-auto p-4"><code>{@html `<span class="text-gray-400"># List all packages</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">list</span>

<span class="text-gray-400"># List with details</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">list</span> <span class="text-yellow-400">--long</span>

<span class="text-gray-400"># List only direct dependencies</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">list</span> <span class="text-yellow-400">--depth</span> <span class="text-purple-400">0</span>

<span class="text-gray-400"># List outdated packages</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">list</span> <span class="text-yellow-400">--outdated</span>`}</code></pre>
				</div>
			</div>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Package Information</h2>
		
		<div class="space-y-4">
			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">knot show</h3>
				<p class="text-sm text-muted-foreground mb-3">Display package information</p>
				<div class="bg-black/90 text-white font-mono text-sm rounded-lg">
					<pre class="overflow-x-auto p-4"><code>{@html `<span class="text-gray-400"># Show package info</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">show</span> <span class="text-purple-400">@org/package</span>

<span class="text-gray-400"># Show specific version</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">show</span> <span class="text-purple-400">@org/package</span><span class="text-yellow-400">@1.2.3</span>

<span class="text-gray-400"># Show README</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">show</span> <span class="text-purple-400">@org/package</span> <span class="text-yellow-400">--readme</span>

<span class="text-gray-400"># Show dependencies</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">show</span> <span class="text-purple-400">@org/package</span> <span class="text-yellow-400">--deps</span>`}</code></pre>
				</div>
			</div>

			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">knot search</h3>
				<p class="text-sm text-muted-foreground mb-3">Search for packages in the registry</p>
				<div class="bg-black/90 text-white font-mono text-sm rounded-lg">
					<pre class="overflow-x-auto p-4"><code>{@html `<span class="text-gray-400"># Search for packages</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">search</span> <span class="text-purple-400">react</span>

<span class="text-gray-400"># Search with filters</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">search</span> <span class="text-yellow-400">--tag</span> <span class="text-purple-400">typescript</span>

<span class="text-gray-400"># Search by author</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">search</span> <span class="text-yellow-400">--author</span> <span class="text-purple-400">username</span>

<span class="text-gray-400"># Limit results</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">search</span> <span class="text-purple-400">react</span> <span class="text-yellow-400">--limit</span> <span class="text-purple-400">10</span>`}</code></pre>
				</div>
			</div>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Version Management</h2>
		
		<div class="space-y-4">
			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">knot version</h3>
				<p class="text-sm text-muted-foreground mb-3">Manage package versions</p>
				<div class="bg-black/90 text-white font-mono text-sm rounded-lg">
					<pre class="overflow-x-auto p-4"><code>{@html `<span class="text-gray-400"># Bump version</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">version</span> <span class="text-purple-400">patch</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">version</span> <span class="text-purple-400">minor</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">version</span> <span class="text-purple-400">major</span>

<span class="text-gray-400"># Set specific version</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">version</span> <span class="text-purple-400">2.0.0</span>

<span class="text-gray-400"># Prerelease versions</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">version</span> <span class="text-purple-400">prerelease</span> <span class="text-yellow-400">--preid</span> <span class="text-purple-400">beta</span>

<span class="text-gray-400"># Tag a version</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">version</span> <span class="text-yellow-400">--tag</span> <span class="text-purple-400">latest</span>`}</code></pre>
				</div>
			</div>

			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">knot tag</h3>
				<p class="text-sm text-muted-foreground mb-3">Manage package tags</p>
				<div class="bg-black/90 text-white font-mono text-sm rounded-lg">
					<pre class="overflow-x-auto p-4"><code>{@html `<span class="text-gray-400"># Add tag to version</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">tag</span> <span class="text-purple-400">@org/package</span><span class="text-yellow-400">@1.2.3</span> <span class="text-purple-400">latest</span>

<span class="text-gray-400"># Remove tag</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">tag</span> <span class="text-yellow-400">--remove</span> <span class="text-purple-400">@org/package</span> <span class="text-purple-400">beta</span>

<span class="text-gray-400"># List tags</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">tag</span> <span class="text-yellow-400">--list</span> <span class="text-purple-400">@org/package</span>`}</code></pre>
				</div>
			</div>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Package Scripts</h2>
		
		<div class="bg-muted rounded-lg p-4 mb-6">
			<h3 class="font-semibold mb-2">Lifecycle Scripts</h3>
			<p class="text-sm text-muted-foreground mb-3">Scripts that run automatically during package operations</p>
			<div class="bg-black/90 text-white font-mono text-sm rounded-lg">
				<pre class="overflow-x-auto p-4"><code>{@html `<span class="text-white">{</span>
  <span class="text-blue-400">"scripts"</span><span class="text-white">:</span> <span class="text-white">{</span>
    <span class="text-blue-400">"prepublish"</span><span class="text-white">:</span> <span class="text-green-400">"knot test && knot build"</span><span class="text-white">,</span>
    <span class="text-blue-400">"postpublish"</span><span class="text-white">:</span> <span class="text-green-400">"echo 'Published successfully!'"</span><span class="text-white">,</span>
    <span class="text-blue-400">"preinstall"</span><span class="text-white">:</span> <span class="text-green-400">"echo 'Installing dependencies...'"</span><span class="text-white">,</span>
    <span class="text-blue-400">"postinstall"</span><span class="text-white">:</span> <span class="text-green-400">"knot build"</span><span class="text-white">,</span>
    <span class="text-blue-400">"preuninstall"</span><span class="text-white">:</span> <span class="text-green-400">"echo 'Removing package...'"</span><span class="text-white">,</span>
    <span class="text-blue-400">"postuninstall"</span><span class="text-white">:</span> <span class="text-green-400">"echo 'Cleanup complete'"</span>
  <span class="text-white">}</span>
<span class="text-white">}</span>`}</code></pre>
			</div>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Authentication</h2>
		
		<div class="space-y-4">
			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">knot login</h3>
				<p class="text-sm text-muted-foreground mb-3">Authenticate with the registry</p>
				<div class="bg-black/90 text-white font-mono text-sm rounded-lg">
					<pre class="overflow-x-auto p-4"><code>{@html `<span class="text-gray-400"># Login interactively</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">login</span>

<span class="text-gray-400"># Login with token</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">login</span> <span class="text-yellow-400">--token</span> <span class="text-purple-400">YOUR_TOKEN</span>

<span class="text-gray-400"># Login to specific registry</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">login</span> <span class="text-yellow-400">--registry</span> <span class="text-purple-400">https://custom.registry.com</span>`}</code></pre>
				</div>
			</div>

			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">knot logout</h3>
				<p class="text-sm text-muted-foreground mb-3">Logout from the registry</p>
				<div class="bg-black/90 text-white font-mono text-sm rounded-lg">
					<pre class="overflow-x-auto p-4"><code>{@html `<span class="text-gray-400"># Logout</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">logout</span>

<span class="text-gray-400"># Logout from specific registry</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">logout</span> <span class="text-yellow-400">--registry</span> <span class="text-purple-400">https://custom.registry.com</span>`}</code></pre>
				</div>
			</div>

			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">knot whoami</h3>
				<p class="text-sm text-muted-foreground mb-3">Display current authenticated user</p>
				<div class="bg-black/90 text-white font-mono text-sm rounded-lg">
					<pre class="overflow-x-auto p-4"><code>{@html `<span class="text-gray-400"># Show current user</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">whoami</span>

<span class="text-gray-400"># Check specific registry</span>
<span class="text-green-400">knot</span> <span class="text-blue-400">whoami</span> <span class="text-yellow-400">--registry</span> <span class="text-purple-400">https://custom.registry.com</span>`}</code></pre>
				</div>
			</div>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Next Steps</h2>
		
		<div class="grid gap-4 mt-6">
			<a href="/docs/publishing" class="group border rounded-lg p-4 hover:bg-muted transition-colors">
				<div class="flex items-center justify-between">
					<div>
						<h3 class="font-semibold group-hover:text-primary">Publishing Guide</h3>
						<p class="text-sm text-muted-foreground">Learn best practices for publishing packages</p>
					</div>
					<Icon icon="solar:arrow-right-bold" class="w-5 h-5 text-muted-foreground group-hover:text-primary" />
				</div>
			</a>

			<a href="/docs/cli-project" class="group border rounded-lg p-4 hover:bg-muted transition-colors">
				<div class="flex items-center justify-between">
					<div>
						<h3 class="font-semibold group-hover:text-primary">Project Commands</h3>
						<p class="text-sm text-muted-foreground">Explore project management commands</p>
					</div>
					<Icon icon="solar:arrow-right-bold" class="w-5 h-5 text-muted-foreground group-hover:text-primary" />
				</div>
			</a>
		</div>
	</div>
</div>