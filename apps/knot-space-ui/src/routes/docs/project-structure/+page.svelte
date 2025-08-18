<script lang="ts">
	import Icon from '@iconify/svelte';
</script>

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
			<h3 class="text-lg font-semibold mb-4">Basic Project Structure</h3>
			<pre class="text-sm font-mono bg-black/90 text-white p-4 rounded overflow-x-auto"><code>my-project/
├── <span class="text-blue-400">knot.yml</span>                    # Project configuration
├── <span class="text-yellow-400">packages/</span>                   # Shared packages directory
│   ├── <span class="text-yellow-400">types/</span>                  # Type definitions package
│   │   ├── <span class="text-blue-400">package.yml</span>           # Package configuration
│   │   ├── <span class="text-gray-400">package.json</span>          # NPM package.json
│   │   ├── <span class="text-gray-400">tsconfig.json</span>         # TypeScript config
│   │   └── <span class="text-green-400">src/</span>                 # Source code
│   └── <span class="text-yellow-400">utils/</span>                  # Utility functions package
│       ├── <span class="text-blue-400">package.yml</span>           # Package configuration
│       ├── <span class="text-gray-400">package.json</span>          # NPM package.json
│       └── <span class="text-green-400">src/</span>                 # Source code
├── <span class="text-green-400">apps/</span>                       # Applications directory
│   ├── <span class="text-green-400">frontend/</span>               # React/Vue/etc frontend app
│   │   ├── <span class="text-blue-400">app.yml</span>              # App configuration
│   │   ├── <span class="text-gray-400">package.json</span>          # NPM dependencies
│   │   ├── <span class="text-gray-400">tsconfig.json</span>         # TypeScript config
│   │   ├── <span class="text-purple-400">knot_packages/</span>       # Linked packages (auto-generated)
│   │   └── <span class="text-green-400">src/</span>                 # Application source
│   └── <span class="text-green-400">api/</span>                    # Backend API app
│       ├── <span class="text-blue-400">app.yml</span>              # App configuration
│       ├── <span class="text-gray-400">package.json</span>          # NPM dependencies
│       └── <span class="text-green-400">src/</span>                 # Application source
└── <span class="text-gray-400">README.md</span>                   # Project documentation</code></pre>
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
				<code><span class="text-blue-400">name:</span> <span class="text-green-400">my-awesome-project</span>
<span class="text-blue-400">description:</span> <span class="text-green-400">"A modern monorepo with shared packages"</span>
<span class="text-blue-400">version:</span> <span class="text-green-400">1.0.0</span>

<span class="text-gray-400"># Global scripts available to all apps</span>
<span class="text-blue-400">scripts:</span>
  <span class="text-blue-400">setup:</span> <span class="text-green-400">"npm install"</span>
  <span class="text-blue-400">test-all:</span> <span class="text-green-400">"knot run test --all"</span>
  <span class="text-blue-400">lint-all:</span> <span class="text-green-400">"knot run lint --all"</span>

<span class="text-gray-400"># App-specific configurations</span>
<span class="text-blue-400">apps:</span>
  <span class="text-blue-400">frontend:</span>
    <span class="text-blue-400">tsAlias:</span> <span class="text-green-400">"@"</span>                  <span class="text-gray-400"># TypeScript alias prefix</span>
    <span class="text-blue-400">packages:</span> [<span class="text-yellow-400">types</span>, <span class="text-yellow-400">utils</span>]        <span class="text-gray-400"># Local packages to link</span>
  <span class="text-blue-400">api:</span>
    <span class="text-blue-400">tsAlias:</span> <span class="text-green-400">"#"</span>
    <span class="text-blue-400">packages:</span> [<span class="text-yellow-400">types</span>, <span class="text-yellow-400">utils</span>]</code>
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
							<Icon icon="solar:box-bold" class="w-6 h-6 text-blue-600" />
							<h4 class="font-semibold">Library Package</h4>
						</div>
						<p class="text-sm text-muted-foreground mb-3">
							Contains reusable functions, components, or utilities.
						</p>
						<div class="text-xs font-mono bg-muted p-2 rounded">
							packages/utils/<br>
							packages/ui-components/
						</div>
					</div>

					<div class="border rounded-lg p-6">
						<div class="flex items-center space-x-3 mb-3">
							<Icon icon="solar:code-bold" class="w-6 h-6 text-purple-600" />
							<h4 class="font-semibold">Type Definitions</h4>
						</div>
						<p class="text-sm text-muted-foreground mb-3">
							Shared TypeScript type definitions and interfaces.
						</p>
						<div class="text-xs font-mono bg-muted p-2 rounded">
							packages/types/<br>
							packages/api-types/
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
				The <code>apps/</code> directory contains your applications that consume packages from the packages directory.
			</p>

			<div>
				<h3 class="text-lg font-semibold mb-3">App Configuration (app.yml)</h3>
				<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
					<code><span class="text-blue-400">name:</span> <span class="text-green-400">frontend</span>
<span class="text-blue-400">description:</span> <span class="text-green-400">"React frontend application"</span>

<span class="text-gray-400"># Build configuration</span>
<span class="text-blue-400">build:</span> <span class="text-green-400">"npm run build"</span>

<span class="text-gray-400"># App-specific scripts</span>
<span class="text-blue-400">scripts:</span>
  <span class="text-blue-400">dev:</span> <span class="text-green-400">"vite dev --port 3000"</span>
  <span class="text-blue-400">test:</span> <span class="text-green-400">"vitest run"</span>
  <span class="text-blue-400">preview:</span> <span class="text-green-400">"vite preview"</span>

<span class="text-gray-400"># Package dependencies</span>
<span class="text-blue-400">packages:</span>
  - <span class="text-yellow-400">types</span>                        <span class="text-gray-400"># Local package</span>
  - <span class="text-yellow-400">utils</span>                       <span class="text-gray-400"># Local package</span>
  - <span class="text-yellow-400">"@myteam/shared-ui"</span>          <span class="text-gray-400"># Remote team package</span></code>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Linked Packages (knot_packages/)</h3>
				<div class="bg-blue-50 border border-blue-200 rounded-lg p-6">
					<div class="flex items-start space-x-3">
						<Icon icon="solar:info-circle-bold" class="w-6 h-6 text-blue-600 mt-1 flex-shrink-0" />
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
						<Icon icon="solar:folder-bold" class="w-5 h-5 mr-2 text-green-600" />
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
						<Icon icon="solar:programming-bold" class="w-5 h-5 mr-2 text-blue-600" />
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
						<Icon icon="solar:check-circle-bold" class="w-5 h-5 mr-2 text-green-600" />
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
						<Icon icon="solar:danger-triangle-bold" class="w-5 h-5 mr-2 text-red-600" />
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
					<code><span class="text-gray-400">// Instead of relative paths:</span>
<span class="text-blue-400">import</span> { formatCurrency } <span class="text-blue-400">from</span> <span class="text-green-400">'../../knot_packages/utils'</span>;

<span class="text-gray-400">// Use clean aliases:</span>
<span class="text-blue-400">import</span> { formatCurrency } <span class="text-blue-400">from</span> <span class="text-green-400">'@/utils'</span>;</code>
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
					<Icon icon="solar:settings-bold" class="w-6 h-6 text-blue-600" />
					<h3 class="font-semibold">Configuration Files</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Learn about all configuration options for projects, packages, and apps.
				</p>
			</a>

			<a href="/docs/package-linking" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:link-bold" class="w-6 h-6 text-green-600" />
					<h3 class="font-semibold">Package Linking</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Understand how Knot links packages and the different linking modes available.
				</p>
			</a>
		</div>
	</section>
</div>