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
	<title>Quick Start Guide - Knot CLI Documentation</title>
	<meta name="description" content="Get started with Knot CLI in under 5 minutes. Learn to initialize projects, create packages, and link dependencies for TypeScript monorepos." />
	<meta property="og:title" content="Quick Start Guide - Knot CLI" />
	<meta property="og:description" content="Get started with Knot CLI in under 5 minutes. Learn to initialize projects, create packages, and link dependencies for TypeScript monorepos." />
	<meta property="og:image" content="/images/og/quick-start.png" />
	<meta property="og:url" content="https://knot.klysium.com/docs/quick-start" />
	<meta name="twitter:title" content="Quick Start Guide - Knot CLI" />
	<meta name="twitter:description" content="Get started with Knot CLI in under 5 minutes. Learn to initialize projects, create packages, and link dependencies for TypeScript monorepos." />
	<meta name="twitter:image" content="/images/og/quick-start.png" />
	<link rel="canonical" href="https://knot.klysium.com/docs/quick-start" />
</svelte:head>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight mb-4" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
			Quick Start Guide
		</h1>
		<p class="text-lg text-muted-foreground">
			Get your first Knot project up and running in under 5 minutes
		</p>
	</div>

	<!-- Prerequisites -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Prerequisites</h2>
		<div class="bg-blue-50 border border-blue-200 rounded-lg p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="solar:info-circle-bold" class="w-6 h-6 text-blue-600 mt-1 flex-shrink-0" />
				<div>
					<h3 class="font-semibold text-blue-900 mb-2">Before we start</h3>
					<ul class="text-sm text-blue-700 space-y-1">
						<li>• Node.js 18+ installed on your system</li>
						<li>• Knot CLI installed (<a href="/docs/installation" class="underline hover:no-underline">installation guide</a>)</li>
						<li>• A text editor or IDE (VS Code recommended)</li>
						<li>• Basic knowledge of command-line tools</li>
					</ul>
				</div>
			</div>
		</div>
	</section>

	<!-- Step 1 -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<div class="w-8 h-8 bg-green-100 rounded-full flex items-center justify-center">
				<span class="text-green-600 font-bold text-sm">1</span>
			</div>
			<h2 class="text-2xl font-bold">Create Your First Project</h2>
		</div>

		<div class="space-y-6">
			<div>
				<h3 class="text-lg font-semibold mb-3">Initialize a new project</h3>
				<div class="bg-black/90 text-green-400 font-mono text-sm rounded-lg relative group">
					<div class="overflow-x-auto p-4 pr-12">
						<code class="whitespace-nowrap block">knot init my-awesome-project</code>
					</div>
					<button 
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100 z-10"
						on:click={() => copyToClipboard('knot init my-awesome-project')}
					>
						{#if showCopied && copyText === 'knot init my-awesome-project'}
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="solar:copy-bold" class="w-4 h-4" />
						{/if}
					</button>
				</div>
				<p class="text-sm text-muted-foreground mt-2">
					This creates a new directory with your project structure and configuration files.
				</p>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Navigate to your project</h3>
				<div class="bg-black/90 text-green-400 font-mono text-sm rounded-lg relative group">
					<div class="overflow-x-auto p-4 pr-12">
						<code class="whitespace-nowrap block">cd my-awesome-project</code>
					</div>
					<button 
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100 z-10"
						on:click={() => copyToClipboard('cd my-awesome-project')}
					>
						{#if showCopied && copyText === 'cd my-awesome-project'}
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="solar:copy-bold" class="w-4 h-4" />
						{/if}
					</button>
				</div>
			</div>

			<div class="bg-muted/30 rounded-lg p-6">
				<h4 class="font-semibold mb-3">Project structure created</h4>
				<pre class="text-sm font-mono"><code>my-awesome-project/
├── <span class="text-blue-400">knot.yml</span>          # Project configuration
├── <span class="text-yellow-400">packages/</span>        # Your packages will go here
├── <span class="text-green-400">apps/</span>             # Your applications will go here
└── <span class="text-gray-400">README.md</span>         # Getting started guide</code></pre>
			</div>
		</div>
	</section>

	<!-- Step 2 -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<div class="w-8 h-8 bg-blue-100 rounded-full flex items-center justify-center">
				<span class="text-blue-600 font-bold text-sm">2</span>
			</div>
			<h2 class="text-2xl font-bold">Create Your First Package</h2>
		</div>

		<div class="space-y-6">
			<div>
				<h3 class="text-lg font-semibold mb-3">Generate a utility package</h3>
				<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
					<code>knot init:package utils --template @typescript-lib</code>
					<button 
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
						on:click={() => copyToClipboard('knot init:package utils --template @typescript-lib')}
					>
						{#if showCopied && copyText === 'knot init:package utils --template @typescript-lib'}
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="solar:copy-bold" class="w-4 h-4" />
						{/if}
					</button>
				</div>
				<p class="text-sm text-muted-foreground mt-2">
					This creates a new package in the <code>packages/utils/</code> directory using a template from Knot Space.
					You can also run the command without <code>--template</code> to create a basic structure.
				</p>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Add some utility functions</h3>
				<p class="text-muted-foreground mb-3">
					Edit <code>packages/utils/src/index.ts</code> and add your first utility:
				</p>
				<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
					<pre><code>{@html `<span class="text-blue-400">export</span> <span class="text-purple-400">function</span> <span class="text-yellow-400">formatCurrency</span>(<span class="text-green-400">amount</span>: <span class="text-blue-400">number</span>): <span class="text-blue-400">string</span> {
  <span class="text-purple-400">return</span> <span class="text-green-400">new</span> <span class="text-yellow-400">Intl</span>.<span class="text-yellow-400">NumberFormat</span>(<span class="text-green-400">'en-US'</span>, {
    <span class="text-red-400">style</span>: <span class="text-green-400">'currency'</span>,
    <span class="text-red-400">currency</span>: <span class="text-green-400">'USD'</span>
  }).<span class="text-yellow-400">format</span>(<span class="text-green-400">amount</span>);
}

<span class="text-blue-400">export</span> <span class="text-purple-400">function</span> <span class="text-yellow-400">slugify</span>(<span class="text-green-400">text</span>: <span class="text-blue-400">string</span>): <span class="text-blue-400">string</span> {
  <span class="text-purple-400">return</span> <span class="text-green-400">text</span>
    .<span class="text-yellow-400">toLowerCase</span>()
    .<span class="text-yellow-400">replace</span>(<span class="text-green-400">/[^a-z0-9]+/g</span>, <span class="text-green-400">'-'</span>)
    .<span class="text-yellow-400">replace</span>(<span class="text-green-400">/^-+|-+$/g</span>, <span class="text-green-400">''</span>);
}`}</code></pre>
				</div>
			</div>
		</div>
	</section>

	<!-- Step 3 -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<div class="w-8 h-8 bg-purple-100 rounded-full flex items-center justify-center">
				<span class="text-purple-600 font-bold text-sm">3</span>
			</div>
			<h2 class="text-2xl font-bold">Create Your First App</h2>
		</div>

		<div class="space-y-6">
			<div>
				<h3 class="text-lg font-semibold mb-3">Generate an app (optionally with template)</h3>
				<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
					<code>knot init:app frontend --template @react-starter</code>
					<button 
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
						on:click={() => copyToClipboard('knot init:app frontend --template @react-starter')}
					>
						{#if showCopied && copyText === 'knot init:app frontend --template @react-starter'}
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="solar:copy-bold" class="w-4 h-4" />
						{/if}
					</button>
				</div>
				<p class="text-sm text-muted-foreground mt-2">
					This creates a new app in the <code>apps/frontend/</code> directory.
				</p>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Add the package as a dependency</h3>
				<p class="text-muted-foreground mb-3">
					Add the utils package as a dependency to your frontend app:
				</p>
				<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
					<code>knot deps add utils --app frontend</code>
					<button 
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
						on:click={() => copyToClipboard('knot deps add utils --app frontend')}
					>
						{#if showCopied && copyText === 'knot deps add utils --app frontend'}
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="solar:copy-bold" class="w-4 h-4" />
						{/if}
					</button>
				</div>
				<p class="text-sm text-muted-foreground mt-2">
					This adds the utils package to your app's dependencies in the <code>app.yml</code> file and automatically resolves and links it.
				</p>
			</div>
		</div>
	</section>

	<!-- Step 4 -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<div class="w-8 h-8 bg-orange-100 rounded-full flex items-center justify-center">
				<span class="text-orange-600 font-bold text-sm">4</span>
			</div>
			<h2 class="text-2xl font-bold">Use Your Package in the App</h2>
		</div>

		<div class="space-y-6">
			<div>
				<h3 class="text-lg font-semibold mb-3">Import and use your utilities</h3>
				<p class="text-muted-foreground mb-3">
					Your package is now linked and ready to use! Edit <code>apps/frontend/src/App.tsx</code> to use your utilities:
				</p>
				<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
					<pre><code>{@html `<span class="text-blue-400">import</span> { <span class="text-yellow-400">formatCurrency</span>, <span class="text-yellow-400">slugify</span> } <span class="text-blue-400">from</span> <span class="text-green-400">'#/utils'</span>;

<span class="text-blue-400">function</span> <span class="text-yellow-400">App</span>() {
  <span class="text-purple-400">const</span> <span class="text-yellow-400">price</span> = <span class="text-green-400">29.99</span>;
  <span class="text-purple-400">const</span> <span class="text-yellow-400">title</span> = <span class="text-green-400">"My Awesome Product"</span>;

  <span class="text-purple-400">return</span> (
    &lt;<span class="text-red-400">div</span>&gt;
      &lt;<span class="text-red-400">h1</span>&gt;{<span class="text-yellow-400">title</span>}&lt;/<span class="text-red-400">h1</span>&gt;
      &lt;<span class="text-red-400">p</span>&gt;Price: {<span class="text-yellow-400">formatCurrency</span>(<span class="text-yellow-400">price</span>)}&lt;/<span class="text-red-400">p</span>&gt;
      &lt;<span class="text-red-400">p</span>&gt;Slug: {<span class="text-yellow-400">slugify</span>(<span class="text-yellow-400">title</span>)}&lt;/<span class="text-red-400">p</span>&gt;
    &lt;/<span class="text-red-400">div</span>&gt;
  );
}`}</code></pre>
				</div>
				<p class="text-sm text-muted-foreground mt-2">
					The <code>#/utils</code> import alias was automatically configured when you added the dependency. TypeScript will provide full intellisense and type checking.
				</p>
			</div>

			<div class="bg-blue-50 border border-blue-200 rounded-lg p-4">
				<div class="flex items-start space-x-3">
					<Icon icon="solar:info-circle-bold" class="w-5 h-5 text-blue-600 mt-1 flex-shrink-0" />
					<div>
						<h4 class="font-semibold text-blue-900 mb-1">Automatic Linking</h4>
						<p class="text-sm text-blue-700">
							When you use <code>knot deps add</code>, packages are automatically resolved, linked, and TypeScript aliases are set up. No manual linking required!
						</p>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Step 5 -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<div class="w-8 h-8 bg-red-100 rounded-full flex items-center justify-center">
				<span class="text-red-600 font-bold text-sm">5</span>
			</div>
			<h2 class="text-2xl font-bold">Build and Run</h2>
		</div>

		<div class="space-y-6">
			<div>
				<h3 class="text-lg font-semibold mb-3">Install external dependencies</h3>
				<p class="text-muted-foreground mb-3">
					If your app needs external NPM packages, install them in the app directory:
				</p>
				<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
					<code>cd apps/frontend && npm install</code>
					<button 
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
						on:click={() => copyToClipboard('cd apps/frontend && npm install')}
					>
						{#if showCopied && copyText === 'cd apps/frontend && npm install'}
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="solar:copy-bold" class="w-4 h-4" />
						{/if}
					</button>
				</div>
				<p class="text-sm text-muted-foreground mt-2">
					Your local packages are already linked - this step is only for external dependencies from NPM.
				</p>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Start development server</h3>
				<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
					<code>cd apps/frontend && knot run dev</code>
					<button 
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
						on:click={() => copyToClipboard('cd apps/frontend && knot run dev')}
					>
						{#if showCopied && copyText === 'cd apps/frontend && knot run dev'}
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="solar:copy-bold" class="w-4 h-4" />
						{/if}
					</button>
				</div>
				<p class="text-sm text-muted-foreground mt-2">
					This runs the dev script from your frontend app's <code>app.yml</code> configuration. Your app should be available at <code>http://localhost:3000</code> (or the port specified in your app).
				</p>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Build for production</h3>
				<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
					<code>cd apps/frontend && knot run build</code>
					<button 
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
						on:click={() => copyToClipboard('cd apps/frontend && knot run build')}
					>
						{#if showCopied && copyText === 'cd apps/frontend && knot run build'}
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="solar:copy-bold" class="w-4 h-4" />
						{/if}
					</button>
				</div>
				<p class="text-sm text-muted-foreground mt-2">
					This builds your frontend app for production deployment. Run from the app directory to use app-specific scripts.
				</p>
			</div>
		</div>
	</section>

	<!-- Quick Workflow Summary -->
	<section class="mb-12">
		<div class="bg-blue-50 border border-blue-200 rounded-lg p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="solar:info-circle-bold" class="w-6 h-6 text-blue-600 mt-1 flex-shrink-0" />
				<div>
					<h3 class="font-semibold text-blue-900 mb-2">Key Workflow Changes</h3>
					<p class="text-sm text-blue-700 mb-3">
						The Knot CLI has been updated with improved dependency management:
					</p>
					<ul class="text-sm text-blue-700 space-y-1 mb-4">
						<li>• Use <code class="bg-blue-100 px-1 rounded">knot deps add</code> to add local packages as dependencies</li>
						<li>• Dependencies are automatically resolved and linked - no manual linking required</li>
						<li>• TypeScript aliases are set up automatically when dependencies are added</li>
						<li>• Run scripts from app directories using <code class="bg-blue-100 px-1 rounded">cd apps/app-name && knot run script</code></li>
					</ul>
				</div>
			</div>
		</div>
	</section>

	<!-- Success -->
	<section class="mb-12">
		<div class="bg-green-50 border border-green-200 rounded-lg p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="solar:check-circle-bold" class="w-6 h-6 text-green-600 mt-1 flex-shrink-0" />
				<div>
					<h3 class="font-semibold text-green-900 mb-2">Congratulations! 🎉</h3>
					<p class="text-sm text-green-700 mb-4">
						You've successfully created your first Knot monorepo with a shared package and a React app that uses it.
						Your utilities are automatically typed and available with clean imports.
					</p>
					<div class="flex flex-wrap gap-3">
						<a href="/docs/package-linking" class="inline-flex items-center px-4 py-2 bg-green-600 text-white text-sm font-medium rounded-md hover:bg-green-700 transition-colors">
							<Icon icon="solar:link-bold" class="w-4 h-4 mr-2" />
							Learn Package Linking
						</a>
						<a href="/docs/publishing" class="inline-flex items-center px-4 py-2 border border-green-600 text-green-600 text-sm font-medium rounded-md hover:bg-green-50 transition-colors">
							<Icon icon="solar:upload-bold" class="w-4 h-4 mr-2" />
							Publish Your Package
						</a>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Next Steps -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Next Steps</h2>
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<div class="border rounded-lg p-6">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:settings-bold" class="w-6 h-6 text-yellow-600" />
					<h3 class="font-semibold">Dependency Management</h3>
				</div>
				<p class="text-sm text-muted-foreground mb-4">
					Learn advanced dependency configuration with version constraints, conditional dependencies, and resolution strategies.
				</p>
				<a href="/docs/dependency-management" class="text-sm text-primary hover:underline">
					Dependency Guide →
				</a>
			</div>

			<div class="border rounded-lg p-6">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:tag-bold" class="w-6 h-6 text-indigo-600" />
					<h3 class="font-semibold">Version Management</h3>
				</div>
				<p class="text-sm text-muted-foreground mb-4">
					Master semantic versioning, package releases, and coordinated version management across your monorepo.
				</p>
				<a href="/docs/version-management" class="text-sm text-primary hover:underline">
					Version Guide →
				</a>
			</div>

			<div class="border rounded-lg p-6">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:code-circle-bold" class="w-6 h-6 text-blue-600" />
					<h3 class="font-semibold">Advanced Configuration</h3>
				</div>
				<p class="text-sm text-muted-foreground mb-4">
					Learn about advanced configuration options, custom TypeScript aliases, and build optimization.
				</p>
				<a href="/docs/configuration" class="text-sm text-primary hover:underline">
					Configuration Guide →
				</a>
			</div>

			<div class="border rounded-lg p-6">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:users-group-two-rounded-bold" class="w-6 h-6 text-purple-600" />
					<h3 class="font-semibold">Team Collaboration</h3>
				</div>
				<p class="text-sm text-muted-foreground mb-4">
					Set up teams, manage permissions, and collaborate on packages with your team members.
				</p>
				<a href="/docs/teams" class="text-sm text-primary hover:underline">
					Team Management →
				</a>
			</div>

			<div class="border rounded-lg p-6">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:upload-bold" class="w-6 h-6 text-green-600" />
					<h3 class="font-semibold">Package Publishing</h3>
				</div>
				<p class="text-sm text-muted-foreground mb-4">
					Share your packages with the world by publishing them to Knot Space registry.
				</p>
				<a href="/docs/publishing" class="text-sm text-primary hover:underline">
					Publishing Guide →
				</a>
			</div>

			<div class="border rounded-lg p-6">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:server-bold" class="w-6 h-6 text-red-600" />
					<h3 class="font-semibold">Self-Hosting</h3>
				</div>
				<p class="text-sm text-muted-foreground mb-4">
					Host your own private Knot Space instance for complete control over your package registry.
				</p>
				<a href="/docs/self-hosting" class="text-sm text-primary hover:underline">
					Self-Hosting Guide →
				</a>
			</div>
		</div>
	</section>
</div>