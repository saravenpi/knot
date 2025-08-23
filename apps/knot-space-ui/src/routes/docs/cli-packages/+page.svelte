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
			<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`# Publish current package
knot publish

# Publish with specific tag
knot publish --tag beta

# Publish with access level
knot publish --access public

# Dry run (simulate publish)
knot publish --dry-run`}</code></pre>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Installing Packages</h2>
		
		<div class="space-y-4">
			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">knot install</h3>
				<p class="text-sm text-muted-foreground mb-3">Install packages and dependencies</p>
				<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`# Install all dependencies
knot install

# Install specific package
knot install @org/package-name

# Install as dev dependency
knot install --dev typescript

# Install exact version
knot install @org/package@1.2.3`}</code></pre>
			</div>

			<div class="bg-yellow-50 border border-yellow-200 rounded-lg p-4">
				<div class="flex items-start space-x-3">
					<Icon icon="solar:info-circle-bold" class="w-5 h-5 text-yellow-600 mt-0.5 flex-shrink-0" />
					<div class="text-sm text-yellow-800">
						<p class="font-semibold mb-1">Note: 'add' is an alias</p>
						<p>The <code class="bg-yellow-100 px-1 rounded">knot add</code> command is an alias for <code class="bg-yellow-100 px-1 rounded">knot install</code> for compatibility.</p>
					</div>
				</div>
			</div>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Managing Dependencies</h2>
		
		<div class="space-y-4">
			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">knot update</h3>
				<p class="text-sm text-muted-foreground mb-3">Update packages to latest versions</p>
				<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`# Update all packages
knot update

# Update specific package
knot update @org/package

# Update to latest major version
knot update @org/package --latest

# Interactive update
knot update --interactive`}</code></pre>
			</div>

			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">knot remove</h3>
				<p class="text-sm text-muted-foreground mb-3">Remove packages from your project</p>
				<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`# Remove package
knot remove @org/package

# Remove multiple packages
knot remove package1 package2

# Remove from specific workspace
knot remove @org/utils --workspace @org/app`}</code></pre>
			</div>

			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">knot list</h3>
				<p class="text-sm text-muted-foreground mb-3">List installed packages</p>
				<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`# List all packages
knot list

# List with details
knot list --long

# List only direct dependencies
knot list --depth 0

# List outdated packages
knot list --outdated`}</code></pre>
			</div>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Package Information</h2>
		
		<div class="space-y-4">
			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">knot show</h3>
				<p class="text-sm text-muted-foreground mb-3">Display package information</p>
				<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`# Show package info
knot show @org/package

# Show specific version
knot show @org/package@1.2.3

# Show README
knot show @org/package --readme

# Show dependencies
knot show @org/package --deps`}</code></pre>
			</div>

			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">knot search</h3>
				<p class="text-sm text-muted-foreground mb-3">Search for packages in the registry</p>
				<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`# Search for packages
knot search react

# Search with filters
knot search --tag typescript

# Search by author
knot search --author username

# Limit results
knot search react --limit 10`}</code></pre>
			</div>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Version Management</h2>
		
		<div class="space-y-4">
			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">knot version</h3>
				<p class="text-sm text-muted-foreground mb-3">Manage package versions</p>
				<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`# Bump version
knot version patch
knot version minor
knot version major

# Set specific version
knot version 2.0.0

# Prerelease versions
knot version prerelease --preid beta

# Tag a version
knot version --tag latest`}</code></pre>
			</div>

			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">knot tag</h3>
				<p class="text-sm text-muted-foreground mb-3">Manage package tags</p>
				<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`# Add tag to version
knot tag @org/package@1.2.3 latest

# Remove tag
knot tag --remove @org/package beta

# List tags
knot tag --list @org/package`}</code></pre>
			</div>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Package Scripts</h2>
		
		<div class="bg-muted rounded-lg p-4 mb-6">
			<h3 class="font-semibold mb-2">Lifecycle Scripts</h3>
			<p class="text-sm text-muted-foreground mb-3">Scripts that run automatically during package operations</p>
			<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`{
  "scripts": {
    "prepublish": "knot test && knot build",
    "postpublish": "echo 'Published successfully!'",
    "preinstall": "echo 'Installing dependencies...'",
    "postinstall": "knot build",
    "preuninstall": "echo 'Removing package...'",
    "postuninstall": "echo 'Cleanup complete'"
  }
}`}</code></pre>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Authentication</h2>
		
		<div class="space-y-4">
			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">knot login</h3>
				<p class="text-sm text-muted-foreground mb-3">Authenticate with the registry</p>
				<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`# Login interactively
knot login

# Login with token
knot login --token YOUR_TOKEN

# Login to specific registry
knot login --registry https://custom.registry.com`}</code></pre>
			</div>

			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">knot logout</h3>
				<p class="text-sm text-muted-foreground mb-3">Logout from the registry</p>
				<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`# Logout
knot logout

# Logout from specific registry
knot logout --registry https://custom.registry.com`}</code></pre>
			</div>

			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">knot whoami</h3>
				<p class="text-sm text-muted-foreground mb-3">Display current authenticated user</p>
				<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`# Show current user
knot whoami

# Check specific registry
knot whoami --registry https://custom.registry.com`}</code></pre>
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