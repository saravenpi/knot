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
	<title>Project Structure - Knot CLI Documentation</title>
	<meta name="description" content="Understanding how Knot organizes monorepo projects, packages, and applications with clear structure and best practices." />
	<meta property="og:title" content="Project Structure - Knot CLI" />
	<meta property="og:description" content="Understanding how Knot organizes monorepo projects, packages, and applications with clear structure and best practices." />
	<meta property="og:url" content="https://knot.klysium.com/docs/project-structure" />
	<meta name="twitter:title" content="Project Structure - Knot CLI" />
	<meta name="twitter:description" content="Understanding how Knot organizes monorepo projects, packages, and applications with clear structure and best practices." />
	<link rel="canonical" href="https://knot.klysium.com/docs/project-structure" />
</svelte:head>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight mb-4" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
			Project Structure
		</h1>
		<p class="text-lg text-muted-foreground">
			Understanding how Knot organizes monorepo projects and their components
		</p>
	</div>

	<!-- Overview -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Overview</h2>
		<p class="text-muted-foreground leading-relaxed mb-6">
			Knot follows a structured approach to organizing monorepos, making it easy to manage multiple packages 
			and applications within a single repository. This structure promotes code reuse, consistency, and scalability.
		</p>
		
		<div class="bg-muted/30 rounded-lg p-6">
			<h3 class="text-lg font-semibold mb-4">Knot CLI Project Structure</h3>
			<pre class="text-sm font-mono bg-black/90 text-white p-4 rounded overflow-x-auto"><code>Knot/
├── <span class="text-blue-400">knot.yml</span>                    # Project configuration
├── <span class="text-yellow-400">packages/</span>                   # Shared packages directory
│   ├── <span class="text-yellow-400">types/</span>                  # TypeScript type definitions
│   │   ├── <span class="text-blue-400">package.yml</span>           # Package configuration
│   │   ├── <span class="text-gray-400">package.json</span>          # NPM package.json
│   │   ├── <span class="text-gray-400">tsconfig.json</span>         # TypeScript config
│   │   ├── <span class="text-green-400">src/</span>                 # Type definitions source
│   │   └── <span class="text-gray-400">dist/</span>                # Compiled output
│   └── <span class="text-yellow-400">utils/</span>                  # Shared utility functions
│       ├── <span class="text-blue-400">package.yml</span>           # Package configuration
│       ├── <span class="text-gray-400">package.json</span>          # NPM package.json
│       ├── <span class="text-gray-400">tsconfig.json</span>         # TypeScript config
│       ├── <span class="text-green-400">src/</span>                 # Utility functions source
│       └── <span class="text-gray-400">dist/</span>                # Compiled output
├── <span class="text-green-400">apps/</span>                       # Applications directory
│   ├── <span class="text-green-400">cli/</span>                    # Knot CLI (Rust)
│   │   ├── <span class="text-orange-400">Cargo.toml</span>           # Rust dependencies
│   │   ├── <span class="text-purple-400">knot_packages/</span>       # Linked packages
│   │   ├── <span class="text-green-400">src/</span>                 # Rust source code
│   │   └── <span class="text-gray-400">target/</span>              # Build artifacts
│   ├── <span class="text-green-400">web/</span>                    # Knot Space UI (SvelteKit)
│   │   ├── <span class="text-blue-400">app.yml</span>              # App configuration
│   │   ├── <span class="text-gray-400">package.json</span>          # NPM dependencies
│   │   ├── <span class="text-gray-400">tsconfig.json</span>         # TypeScript config
│   │   ├── <span class="text-purple-400">knot_packages/</span>       # Linked packages (auto-generated)
│   │   ├── <span class="text-green-400">src/</span>                 # SvelteKit application
│   │   └── <span class="text-gray-400">static/</span>              # Static assets
│   └── <span class="text-green-400">backend/</span>                # Node.js Backend API
│       ├── <span class="text-blue-400">app.yml</span>              # App configuration
│       ├── <span class="text-gray-400">package.json</span>          # NPM dependencies
│       ├── <span class="text-purple-400">knot_packages/</span>       # Linked packages
│       └── <span class="text-green-400">src/</span>                 # Backend source
├── <span class="text-gray-400">README.md</span>                   # Project documentation
└── <span class="text-gray-400">install.sh</span>                  # Installation script</code></pre>
		</div>
	</section>

	<!-- Root Configuration -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Root Configuration (knot.yml)</h2>
		
		<div class="space-y-6">
			<p class="text-muted-foreground">
				The <code>knot.yml</code> file at the project root defines global configuration and project metadata.
			</p>
			
			<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
				<code><span class="text-blue-400">name:</span> <span class="text-green-400">Knot</span>
<span class="text-blue-400">description:</span> <span class="text-green-400">A modern monorepo package manager for TypeScript/JavaScript projects</span>

<span class="text-gray-400"># Global scripts available to all apps</span>
<span class="text-blue-400">scripts:</span>
  <span class="text-blue-400">setup:</span> <span class="text-green-400">"echo 'Setting up Knot development environment...'"</span>
  <span class="text-blue-400">build-all:</span> <span class="text-green-400">"echo 'Building all apps...' && cd apps/cli && cargo build --release"</span>
  <span class="text-blue-400">dev-ui:</span> <span class="text-green-400">"cd apps/backend && npm run dev"</span>
  <span class="text-blue-400">install-cli:</span> <span class="text-green-400">"cd apps/cli && cargo build --release && sudo cp target/release/knot /usr/local/bin/"</span>

<span class="text-gray-400"># App-specific configurations</span>
<span class="text-blue-400">apps:</span>
  <span class="text-blue-400">cli:</span>
    <span class="text-blue-400">description:</span> <span class="text-green-400">"The Knot CLI tool for monorepo package management"</span>
  <span class="text-blue-400">web:</span>
    <span class="text-blue-400">description:</span> <span class="text-green-400">"Backend API server for Knot packages"</span>
    <span class="text-blue-400">tsAlias:</span> <span class="text-green-400">"#"</span>                  <span class="text-gray-400"># TypeScript alias prefix</span>
    <span class="text-blue-400">packages:</span> [<span class="text-yellow-400">types</span>]              <span class="text-gray-400"># Local packages to link</span>
  <span class="text-blue-400">backend:</span>
    <span class="text-blue-400">description:</span> <span class="text-green-400">"Frontend UI for Knot Space package registry"</span>
    <span class="text-blue-400">tsAlias:</span> <span class="text-green-400">true</span>
    <span class="text-blue-400">packages:</span> [<span class="text-yellow-400">types</span>, <span class="text-yellow-400">utils</span>]

<span class="text-gray-400"># Package definitions</span>
<span class="text-blue-400">packages:</span>
  <span class="text-blue-400">types:</span>
    <span class="text-blue-400">description:</span> <span class="text-green-400">"Shared TypeScript types for all Knot applications"</span>
  <span class="text-blue-400">utils:</span>
    <span class="text-blue-400">description:</span> <span class="text-green-400">"Shared utility functions for Knot applications"</span></code>
			</div>
		</div>
	</section>

	<!-- Packages Directory -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Packages Directory</h2>
		
		<div class="space-y-6">
			<p class="text-muted-foreground">
				The <code>packages/</code> directory contains reusable code libraries that can be shared across 
				multiple applications within your monorepo.
			</p>

			<div>
				<h3 class="text-lg font-semibold mb-3">Package Configuration (package.yml)</h3>
				<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
					<code><span class="text-blue-400">name:</span> <span class="text-green-400">utils</span>
<span class="text-blue-400">description:</span> <span class="text-green-400">"Utility functions and helpers"</span>
<span class="text-blue-400">version:</span> <span class="text-green-400">1.0.0</span>
<span class="text-blue-400">team:</span> <span class="text-green-400">myteam</span>                     <span class="text-gray-400"># Optional team namespace</span>

<span class="text-gray-400"># Package-specific scripts</span>
<span class="text-blue-400">scripts:</span>
  <span class="text-blue-400">build:</span> <span class="text-green-400">"tsc && rollup -c"</span>
  <span class="text-blue-400">test:</span> <span class="text-green-400">"vitest run"</span>
  <span class="text-blue-400">lint:</span> <span class="text-green-400">"eslint src/"</span>

<span class="text-gray-400"># Package tags for discoverability</span>
<span class="text-blue-400">tags:</span>
  - <span class="text-yellow-400">utilities</span>
  - <span class="text-yellow-400">helpers</span>
  - <span class="text-yellow-400">typescript</span></code>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Package Types</h3>
				<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
					<div class="border rounded-lg p-6">
						<div class="flex items-center space-x-3 mb-3">
							<Icon icon="lucide:code" class="w-6 h-6 text-purple-600" />
							<h4 class="font-semibold">Type Definitions</h4>
						</div>
						<p class="text-sm text-muted-foreground mb-3">
							Shared TypeScript type definitions for API interfaces, data models, and common structures.
						</p>
						<div class="text-xs font-mono bg-muted p-2 rounded">
							packages/types/
						</div>
					</div>

					<div class="border rounded-lg p-6">
						<div class="flex items-center space-x-3 mb-3">
							<Icon icon="lucide:box-bold" class="w-6 h-6 text-blue-600" />
							<h4 class="font-semibold">Utility Functions</h4>
						</div>
						<p class="text-sm text-muted-foreground mb-3">
							Common utility functions and helpers used across Knot applications.
						</p>
						<div class="text-xs font-mono bg-muted p-2 rounded">
							packages/utils/
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Apps Directory -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Apps Directory</h2>
		
		<div class="space-y-6">
			<p class="text-muted-foreground">
				The <code>apps/</code> directory contains three main applications that make up the Knot ecosystem:
			</p>

			<div>
				<h3 class="text-lg font-semibold mb-3">Application Types</h3>
				<div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-6">
					<div class="border rounded-lg p-6">
						<div class="flex items-center space-x-3 mb-3">
							<Icon icon="lucide:terminal" class="w-6 h-6 text-orange-600" />
							<h4 class="font-semibold">CLI (Rust)</h4>
						</div>
						<p class="text-sm text-muted-foreground mb-3">
							The core Knot CLI tool built with Rust for high performance monorepo management.
						</p>
						<div class="text-xs font-mono bg-muted p-2 rounded">
							apps/cli/
						</div>
					</div>

					<div class="border rounded-lg p-6">
						<div class="flex items-center space-x-3 mb-3">
							<Icon icon="lucide:browser-bold" class="w-6 h-6 text-green-600" />
							<h4 class="font-semibold">Web UI (SvelteKit)</h4>
						</div>
						<p class="text-sm text-muted-foreground mb-3">
							Modern web interface for Knot Space package registry built with SvelteKit.
						</p>
						<div class="text-xs font-mono bg-muted p-2 rounded">
							apps/web/
						</div>
					</div>

					<div class="border rounded-lg p-6">
						<div class="flex items-center space-x-3 mb-3">
							<Icon icon="lucide:server-bold" class="w-6 h-6 text-blue-600" />
							<h4 class="font-semibold">Backend API (Node.js)</h4>
						</div>
						<p class="text-sm text-muted-foreground mb-3">
							REST API server handling package registry operations and user management.
						</p>
						<div class="text-xs font-mono bg-muted p-2 rounded">
							apps/backend/
						</div>
					</div>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">App Configuration (app.yml)</h3>
				<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
					<code><span class="text-blue-400">name:</span> <span class="text-green-400">knot-space-ui</span>
<span class="text-blue-400">description:</span> <span class="text-green-400">Frontend UI for Knot Space package registry</span>

<span class="text-gray-400"># Build configuration</span>
<span class="text-blue-400">build:</span> <span class="text-green-400">"npm run build"</span>

<span class="text-gray-400"># Frontend-specific scripts</span>
<span class="text-blue-400">scripts:</span>
  <span class="text-blue-400">dev:</span> <span class="text-green-400">"npm run dev"</span>
  <span class="text-blue-400">build:</span> <span class="text-green-400">"npm run build"</span>
  <span class="text-blue-400">preview:</span> <span class="text-green-400">"npm run preview"</span>
  <span class="text-blue-400">check:</span> <span class="text-green-400">"npm run check"</span>
  <span class="text-blue-400">lint:</span> <span class="text-green-400">"npm run lint"</span>
  <span class="text-blue-400">format:</span> <span class="text-green-400">"npm run format"</span>
  <span class="text-blue-400">clean:</span> <span class="text-green-400">"rm -rf .svelte-kit build node_modules"</span></code>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Linked Packages (knot_packages/)</h3>
				<div class="bg-blue-50 border border-blue-200 rounded-lg p-6">
					<div class="flex items-start space-x-3">
						<Icon icon="lucide:info" class="w-6 h-6 text-blue-600 mt-1 flex-shrink-0" />
						<div>
							<h4 class="font-semibold text-blue-900 mb-2">Auto-generated Directory</h4>
							<p class="text-sm text-blue-700">
								The <code>knot_packages/</code> directory is automatically created when you run <code>knot link</code>. 
								It contains copies (or symlinks) of your local packages, making them available to your applications.
							</p>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- File Organization -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">File Organization Best Practices</h2>
		
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<div class="space-y-6">
				<div class="border rounded-lg p-6">
					<h3 class="font-semibold mb-3 flex items-center">
						<Icon icon="lucide:folder" class="w-5 h-5 mr-2 text-green-600" />
						Package Structure
					</h3>
					<div class="text-sm font-mono bg-muted p-3 rounded">
						packages/utils/<br>
						├── package.yml<br>
						├── package.json<br>
						├── tsconfig.json<br>
						├── src/<br>
						│   ├── index.ts<br>
						│   ├── helpers/<br>
						│   └── types/<br>
						├── tests/<br>
						└── dist/
					</div>
				</div>

				<div class="border rounded-lg p-6">
					<h3 class="font-semibold mb-3 flex items-center">
						<Icon icon="lucide:programming-bold" class="w-5 h-5 mr-2 text-blue-600" />
						App Structure
					</h3>
					<div class="text-sm font-mono bg-muted p-3 rounded">
						apps/frontend/<br>
						├── app.yml<br>
						├── package.json<br>
						├── tsconfig.json<br>
						├── knot_packages/  # Auto-generated<br>
						├── src/<br>
						├── public/<br>
						└── dist/
					</div>
				</div>
			</div>

			<div class="space-y-6">
				<div class="border rounded-lg p-6">
					<h3 class="font-semibold mb-3 flex items-center">
						<Icon icon="lucide:check-circle" class="w-5 h-5 mr-2 text-green-600" />
						Best Practices
					</h3>
					<ul class="text-sm text-muted-foreground space-y-2">
						<li>• Keep packages focused on a single responsibility</li>
						<li>• Use descriptive package names</li>
						<li>• Include proper TypeScript definitions</li>
						<li>• Write comprehensive tests for packages</li>
						<li>• Document public APIs clearly</li>
					</ul>
				</div>

				<div class="border rounded-lg p-6">
					<h3 class="font-semibold mb-3 flex items-center">
						<Icon icon="lucide:danger-triangle-bold" class="w-5 h-5 mr-2 text-red-600" />
						Common Pitfalls
					</h3>
					<ul class="text-sm text-muted-foreground space-y-2">
						<li>• Don't modify knot_packages/ manually</li>
						<li>• Avoid circular dependencies between packages</li>
						<li>• Don't commit knot_packages/ to version control</li>
						<li>• Keep package versions synchronized</li>
					</ul>
				</div>
			</div>
		</div>
	</section>

	<!-- TypeScript Integration -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">TypeScript Integration</h2>
		
		<div class="space-y-6">
			<p class="text-muted-foreground">
				Knot automatically configures TypeScript path mapping to make package imports clean and intuitive.
			</p>

			<div>
				<h3 class="text-lg font-semibold mb-3">Automatic tsconfig.json Updates</h3>
				<p class="text-muted-foreground mb-3">
					When you run <code>knot link</code>, your app's tsconfig.json is automatically updated with path mappings:
				</p>
				<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
					<pre><code>{@html `{
  <span class="text-blue-400">"compilerOptions"</span>: {
    <span class="text-blue-400">"baseUrl"</span>: <span class="text-green-400">"."</span>,
    <span class="text-blue-400">"paths"</span>: {
      <span class="text-green-400">"@/types"</span>: [<span class="text-green-400">"./knot_packages/types"</span>],
      <span class="text-green-400">"@/utils"</span>: [<span class="text-green-400">"./knot_packages/utils"</span>]
    }
  }
}`}</code></pre>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Clean Import Syntax</h3>
				<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
					<pre><code>{@html `<span class="text-gray-400">// Instead of relative paths:</span>
<span class="text-blue-400">import</span> type { User } <span class="text-blue-400">from</span> <span class="text-green-400">'../../knot_packages/types'</span>;

<span class="text-gray-400">// Use clean aliases:</span>
<span class="text-blue-400">import</span> type { User } <span class="text-blue-400">from</span> <span class="text-green-400">'@/types'</span>;`}</code></pre>
				</div>
			</div>
		</div>
	</section>

	<!-- Next Steps -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Next Steps</h2>
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<a href="/docs/configuration" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:settings-bold" class="w-6 h-6 text-blue-600" />
					<h3 class="font-semibold">Configuration Files</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Learn about all configuration options for projects, packages, and apps.
				</p>
			</a>

			<a href="/docs/package-linking" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:link" class="w-6 h-6 text-green-600" />
					<h3 class="font-semibold">Package Linking</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Understand how Knot links packages and the different linking modes available.
				</p>
			</a>
		</div>
	</section>
</div>