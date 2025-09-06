
<script lang="ts">
	import Icon from '@iconify/svelte';
</script>

<svelte:head>
	<title>Package Development - Knot CLI Documentation</title>
	<meta name="description" content="Learn how to develop packages for your Knot projects, from initialization to configuration." />
</svelte:head>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight mb-4" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
			Package Development
		</h1>
		<p class="text-lg text-muted-foreground">
			Create and manage reusable packages within your monorepo.
		</p>
	</div>

	<div class="prose prose-slate max-w-none">
		<h2 class="text-2xl font-bold mb-4">Initializing a Package</h2>
		<p class="mb-6">
			Create a new package in your project's `packages` directory. Knot will generate a `package.yml` file for you.
		</p>
		
		<div class="bg-muted rounded-lg p-4 mb-6">
			<h3 class="font-semibold mb-2">Create a new package</h3>
			<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">knot init:package &lt;package-name&gt;</code></pre>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Package Configuration</h2>
		<p class="mb-6">
			The `package.yml` file contains metadata about your package, including its name, version, and any scripts.
		</p>
		
		<div class="bg-muted rounded-lg p-4 mb-6">
			<h3 class="font-semibold mb-2">package.yml structure</h3>
			<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`# packages/my-package/package.yml
name: my-package
version: 0.1.0
scripts:
  build: tsc`}</code></pre>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Adding Dependencies</h2>
		<p class="mb-6">
			You can add other packages as dependencies to your package. This is done by adding a `packages` key to your `package.yml` file.
		</p>
		
		<div class="bg-muted rounded-lg p-4 mb-6">
			<h3 class="font-semibold mb-2">Package dependencies</h3>
			<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`# packages/my-package/package.yml
name: my-package
version: 0.1.0
packages:
  - other-package
  - @some-team/another-package`}</code></pre>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Package Structure</h2>
		<p class="mb-6">
			A typical package structure includes source files, configuration, and build outputs.
		</p>
		
		<div class="bg-muted rounded-lg p-4 mb-6">
			<h3 class="font-semibold mb-2">Recommended structure</h3>
			<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`packages/my-package/
├── src/
│   └── index.ts
├── package.yml
├── package.json
├── tsconfig.json
└── dist/       # Generated after build`}</code></pre>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Next Steps</h2>
		
		<div class="grid gap-4 mt-6">
			<a href="/docs/dependency-management" class="group border rounded-lg p-4 hover:bg-muted transition-colors">
				<div class="flex items-center justify-between">
					<div>
						<h3 class="font-semibold group-hover:text-primary">Dependency Management</h3>
						<p class="text-sm text-muted-foreground">Advanced dependency configuration with version constraints and resolution strategies</p>
					</div>
					<Icon icon="solar:arrow-right-bold" class="w-5 h-5 text-muted-foreground group-hover:text-primary" />
				</div>
			</a>

			<a href="/docs/version-management" class="group border rounded-lg p-4 hover:bg-muted transition-colors">
				<div class="flex items-center justify-between">
					<div>
						<h3 class="font-semibold group-hover:text-primary">Version Management</h3>
						<p class="text-sm text-muted-foreground">Semantic versioning, package releases, and coordinated version management</p>
					</div>
					<Icon icon="solar:arrow-right-bold" class="w-5 h-5 text-muted-foreground group-hover:text-primary" />
				</div>
			</a>

			<a href="/docs/publishing" class="group border rounded-lg p-4 hover:bg-muted transition-colors">
				<div class="flex items-center justify-between">
					<div>
						<h3 class="font-semibold group-hover:text-primary">Publishing Packages</h3>
						<p class="text-sm text-muted-foreground">Learn how to publish packages to Knot Space</p>
					</div>
					<Icon icon="solar:arrow-right-bold" class="w-5 h-5 text-muted-foreground group-hover:text-primary" />
				</div>
			</a>

			<a href="/docs/typescript" class="group border rounded-lg p-4 hover:bg-muted transition-colors">
				<div class="flex items-center justify-between">
					<div>
						<h3 class="font-semibold group-hover:text-primary">TypeScript Integration</h3>
						<p class="text-sm text-muted-foreground">TypeScript and JavaScript package development</p>
					</div>
					<Icon icon="solar:arrow-right-bold" class="w-5 h-5 text-muted-foreground group-hover:text-primary" />
				</div>
			</a>
		</div>
	</div>
</div>
