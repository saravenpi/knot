<script lang="ts">
	import Icon from '@iconify/svelte';

	let showCopied = false;

	async function copyToClipboard(text: string) {
		try {
			await navigator.clipboard.writeText(text);
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
			Configuration Files
		</h1>
		<p class="text-lg text-muted-foreground">
			Comprehensive guide to configuring Knot projects, packages, and applications
		</p>
	</div>

	<!-- Overview -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Configuration Overview</h2>
		<p class="text-muted-foreground leading-relaxed mb-6">
			Knot uses YAML configuration files to define project structure, dependencies, and build processes. 
			Each level of your monorepo has its own configuration file with specific purposes and capabilities.
		</p>
		
		<div class="grid grid-cols-1 md:grid-cols-3 gap-6">
			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-blue-100 rounded-lg flex items-center justify-center mb-4">
					<Icon icon="lucide:folder" class="w-6 h-6 text-blue-600" />
				</div>
				<h3 class="font-semibold mb-2">knot.yml</h3>
				<p class="text-sm text-muted-foreground">
					Project-level configuration defining global settings, scripts, and app configurations.
				</p>
			</div>
			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-green-100 rounded-lg flex items-center justify-center mb-4">
					<Icon icon="lucide:box-bold" class="w-6 h-6 text-green-600" />
				</div>
				<h3 class="font-semibold mb-2">package.yml</h3>
				<p class="text-sm text-muted-foreground">
					Package-level configuration for reusable libraries and components.
				</p>
			</div>
			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-purple-100 rounded-lg flex items-center justify-center mb-4">
					<Icon icon="lucide:programming-bold" class="w-6 h-6 text-purple-600" />
				</div>
				<h3 class="font-semibold mb-2">app.yml</h3>
				<p class="text-sm text-muted-foreground">
					Application-level configuration for individual apps within your monorepo.
				</p>
			</div>
		</div>
	</section>

	<!-- Project Configuration -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Project Configuration (knot.yml)</h2>
		
		<div class="space-y-6">
			<p class="text-muted-foreground">
				The root <code>knot.yml</code> file defines your project's global configuration and metadata.
			</p>

			<div>
				<h3 class="text-lg font-semibold mb-3">Complete Example</h3>
				<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg relative group">
					<pre><code><span class="text-gray-400"># Project metadata</span>
<span class="text-blue-400">name:</span> <span class="text-green-400">my-awesome-project</span>
<span class="text-blue-400">description:</span> <span class="text-green-400">"A modern monorepo for TypeScript applications"</span>
<span class="text-blue-400">version:</span> <span class="text-green-400">1.0.0</span>
<span class="text-blue-400">author:</span> <span class="text-green-400">"Your Name &lt;email@example.com&gt;"</span>
<span class="text-blue-400">license:</span> <span class="text-green-400">MIT</span>

<span class="text-gray-400"># Global scripts available to all apps</span>
<span class="text-blue-400">scripts:</span>
  <span class="text-blue-400">setup:</span> <span class="text-green-400">"npm install && knot link"</span>
  <span class="text-blue-400">test-all:</span> <span class="text-green-400">"knot run test --all"</span>
  <span class="text-blue-400">lint-all:</span> <span class="text-green-400">"knot run lint --all"</span>
  <span class="text-blue-400">build-all:</span> <span class="text-green-400">"knot run build --all"</span>
  <span class="text-blue-400">clean:</span> <span class="text-green-400">"rm -rf */dist */knot_packages"</span>

<span class="text-gray-400"># App-specific configurations</span>
<span class="text-blue-400">apps:</span>
  <span class="text-blue-400">frontend:</span>
    <span class="text-blue-400">tsAlias:</span> <span class="text-green-400">"@"</span>                    <span class="text-gray-400"># TypeScript alias prefix</span>
    <span class="text-blue-400">packages:</span>                        <span class="text-gray-400"># Package dependencies</span>
      - <span class="text-yellow-400">types</span>
      - <span class="text-yellow-400">utils</span>
      - <span class="text-yellow-400">ui-components</span>
      - <span class="text-yellow-400">"@myteam/shared-lib"</span>      <span class="text-gray-400"># Remote package</span>
  <span class="text-blue-400">api:</span>
    <span class="text-blue-400">tsAlias:</span> <span class="text-green-400">"#"</span>
    <span class="text-blue-400">packages:</span>
      - <span class="text-yellow-400">types</span>
      - <span class="text-yellow-400">utils</span>
  <span class="text-blue-400">admin:</span>
    <span class="text-blue-400">tsAlias:</span> <span class="text-green-400">"~"</span>
    <span class="text-blue-400">packages:</span>
      - <span class="text-yellow-400">types</span>
      - <span class="text-yellow-400">ui-components</span></code></pre>
					<button 
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
						on:click={(event) => {
						const codeElement = event.target.closest('.group').querySelector('pre code');
						if (codeElement) copyToClipboard(codeElement.textContent);
					}}
					>
						{#if showCopied}
							<Icon icon="lucide:check-circle" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="lucide:copy" class="w-4 h-4" />
						{/if}
					</button>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Configuration Options</h3>
				<div class="space-y-4">
					<div class="border rounded-lg p-4">
						<h4 class="font-semibold mb-2">Project Metadata</h4>
						<div class="grid grid-cols-1 md:grid-cols-2 gap-4 text-sm">
							<div><code class="bg-muted px-2 py-1 rounded">name</code> - Project name (required)</div>
							<div><code class="bg-muted px-2 py-1 rounded">description</code> - Project description</div>
							<div><code class="bg-muted px-2 py-1 rounded">version</code> - Project version</div>
							<div><code class="bg-muted px-2 py-1 rounded">author</code> - Project author</div>
							<div><code class="bg-muted px-2 py-1 rounded">license</code> - License type</div>
						</div>
					</div>

					<div class="border rounded-lg p-4">
						<h4 class="font-semibold mb-2">Scripts</h4>
						<p class="text-sm text-muted-foreground mb-2">
							Global scripts that can be run from any app directory with <code>knot run &lt;script&gt;</code>
						</p>
						<div class="text-sm space-y-1">
							<div>• Scripts have the lowest priority (overridden by app and package scripts)</div>
							<div>• Support complex commands with <code>&&</code> and <code>||</code> operators</div>
							<div>• Can reference other knot commands</div>
						</div>
					</div>

					<div class="border rounded-lg p-4">
						<h4 class="font-semibold mb-2">App Configurations</h4>
						<p class="text-sm text-muted-foreground mb-2">
							Define how each app should be configured and which packages it depends on.
						</p>
						<div class="text-sm space-y-1">
							<div><code class="bg-muted px-2 py-1 rounded">tsAlias</code> - TypeScript import alias prefix (@, #, ~, etc.)</div>
							<div><code class="bg-muted px-2 py-1 rounded">packages</code> - Array of package dependencies (local and remote)</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Package Configuration -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Package Configuration (package.yml)</h2>
		
		<div class="space-y-6">
			<p class="text-muted-foreground">
				Each package in the <code>packages/</code> directory has its own <code>package.yml</code> file.
			</p>

			<div>
				<h3 class="text-lg font-semibold mb-3">Complete Example</h3>
				<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg relative group">
					<pre><code><span class="text-gray-400"># Package metadata</span>
<span class="text-blue-400">name:</span> <span class="text-green-400">utils</span>
<span class="text-blue-400">description:</span> <span class="text-green-400">"Utility functions and helpers"</span>
<span class="text-blue-400">version:</span> <span class="text-green-400">1.2.0</span>
<span class="text-blue-400">author:</span> <span class="text-green-400">"Your Name &lt;email@example.com&gt;"</span>
<span class="text-blue-400">license:</span> <span class="text-green-400">MIT</span>

<span class="text-gray-400"># Team namespace (for publishing)</span>
<span class="text-blue-400">team:</span> <span class="text-green-400">myteam</span>

<span class="text-gray-400"># Package scripts</span>
<span class="text-blue-400">scripts:</span>
  <span class="text-blue-400">build:</span> <span class="text-green-400">"tsc && rollup -c"</span>
  <span class="text-blue-400">test:</span> <span class="text-green-400">"vitest run"</span>
  <span class="text-blue-400">test:watch:</span> <span class="text-green-400">"vitest"</span>
  <span class="text-blue-400">lint:</span> <span class="text-green-400">"eslint src/ --ext .ts,.tsx"</span>
  <span class="text-blue-400">lint:fix:</span> <span class="text-green-400">"eslint src/ --ext .ts,.tsx --fix"</span>
  <span class="text-blue-400">typecheck:</span> <span class="text-green-400">"tsc --noEmit"</span>

<span class="text-gray-400"># Package tags for discoverability</span>
<span class="text-blue-400">tags:</span>
  - <span class="text-yellow-400">utilities</span>
  - <span class="text-yellow-400">helpers</span>
  - <span class="text-yellow-400">typescript</span>
  - <span class="text-yellow-400">formatting</span>

<span class="text-gray-400"># Package dependencies (other local packages)</span>
<span class="text-blue-400">dependencies:</span>
  - <span class="text-yellow-400">types</span>                     <span class="text-gray-400"># Local package dependency</span></code></pre>
					<button 
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
						on:click={(event) => {
						const codeElement = event.target.closest('.group').querySelector('pre code');
						if (codeElement) copyToClipboard(codeElement.textContent);
					}}
					>
						{#if showCopied}
							<Icon icon="lucide:check-circle" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="lucide:copy" class="w-4 h-4" />
						{/if}
					</button>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Configuration Options</h3>
				<div class="space-y-4">
					<div class="border rounded-lg p-4">
						<h4 class="font-semibold mb-2">Package Metadata</h4>
						<div class="grid grid-cols-1 md:grid-cols-2 gap-4 text-sm">
							<div><code class="bg-muted px-2 py-1 rounded">name</code> - Package name (required, must be unique)</div>
							<div><code class="bg-muted px-2 py-1 rounded">description</code> - Package description</div>
							<div><code class="bg-muted px-2 py-1 rounded">version</code> - Semantic version (required)</div>
							<div><code class="bg-muted px-2 py-1 rounded">author</code> - Package author</div>
							<div><code class="bg-muted px-2 py-1 rounded">license</code> - License type</div>
							<div><code class="bg-muted px-2 py-1 rounded">team</code> - Team namespace for publishing</div>
						</div>
					</div>

					<div class="border rounded-lg p-4">
						<h4 class="font-semibold mb-2">Scripts</h4>
						<p class="text-sm text-muted-foreground mb-2">
							Package-specific scripts with medium priority (override project scripts, overridden by app scripts)
						</p>
						<div class="text-sm space-y-1">
							<div>• <code>build</code> - Build the package for distribution</div>
							<div>• <code>test</code> - Run package tests</div>
							<div>• <code>lint</code> - Lint package code</div>
							<div>• Custom scripts for package-specific tasks</div>
						</div>
					</div>

					<div class="border rounded-lg p-4">
						<h4 class="font-semibold mb-2">Tags & Discovery</h4>
						<p class="text-sm text-muted-foreground mb-2">
							Tags help with package discoverability in the web interface and search
						</p>
						<div class="text-sm space-y-1">
							<div>• Use descriptive keywords</div>
							<div>• Include technology stack (typescript, react, etc.)</div>
							<div>• Add functional categories (utilities, ui, etc.)</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- App Configuration -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">App Configuration (app.yml)</h2>
		
		<div class="space-y-6">
			<p class="text-muted-foreground">
				Each application in the <code>apps/</code> directory has its own <code>app.yml</code> file.
			</p>

			<div>
				<h3 class="text-lg font-semibold mb-3">Complete Example</h3>
				<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg relative group">
					<pre><code><span class="text-gray-400"># App metadata</span>
<span class="text-blue-400">name:</span> <span class="text-green-400">frontend</span>
<span class="text-blue-400">description:</span> <span class="text-green-400">"React frontend application with TypeScript"</span>

<span class="text-gray-400"># Build configuration</span>
<span class="text-blue-400">build:</span> <span class="text-green-400">"npm run build"</span>

<span class="text-gray-400"># App-specific scripts (highest priority)</span>
<span class="text-blue-400">scripts:</span>
  <span class="text-blue-400">dev:</span> <span class="text-green-400">"vite dev --port 3000 --host"</span>
  <span class="text-blue-400">test:</span> <span class="text-green-400">"vitest run"</span>
  <span class="text-blue-400">test:ui:</span> <span class="text-green-400">"vitest --ui"</span>
  <span class="text-blue-400">test:coverage:</span> <span class="text-green-400">"vitest run --coverage"</span>
  <span class="text-blue-400">lint:</span> <span class="text-green-400">"eslint src/ --ext .ts,.tsx"</span>
  <span class="text-blue-400">preview:</span> <span class="text-green-400">"vite preview"</span>
  <span class="text-blue-400">analyze:</span> <span class="text-green-400">"npm run build && npx bundle-analyzer"</span>

<span class="text-gray-400"># Package dependencies</span>
<span class="text-blue-400">packages:</span>
  - <span class="text-yellow-400">types</span>                        <span class="text-gray-400"># Local package</span>
  - <span class="text-yellow-400">utils</span>                       <span class="text-gray-400"># Local package</span>
  - <span class="text-yellow-400">ui-components</span>               <span class="text-gray-400"># Local package</span>
  - <span class="text-yellow-400">"@myteam/shared-lib"</span>         <span class="text-gray-400"># Remote team package</span>
  - <span class="text-yellow-400">"@external/package"</span>          <span class="text-gray-400"># External public package</span>

<span class="text-gray-400"># Environment-specific configurations</span>
<span class="text-blue-400">env:</span>
  <span class="text-blue-400">development:</span>
    <span class="text-blue-400">api_url:</span> <span class="text-green-400">"http://localhost:8080"</span>
  <span class="text-blue-400">production:</span>
    <span class="text-blue-400">api_url:</span> <span class="text-green-400">"https://api.myapp.com"</span></code></pre>
					<button 
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
						on:click={(event) => {
						const codeElement = event.target.closest('.group').querySelector('pre code');
						if (codeElement) copyToClipboard(codeElement.textContent);
					}}
					>
						{#if showCopied}
							<Icon icon="lucide:check-circle" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="lucide:copy" class="w-4 h-4" />
						{/if}
					</button>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Configuration Options</h3>
				<div class="space-y-4">
					<div class="border rounded-lg p-4">
						<h4 class="font-semibold mb-2">App Metadata</h4>
						<div class="grid grid-cols-1 md:grid-cols-2 gap-4 text-sm">
							<div><code class="bg-muted px-2 py-1 rounded">name</code> - App name (required, must match directory)</div>
							<div><code class="bg-muted px-2 py-1 rounded">description</code> - App description</div>
							<div><code class="bg-muted px-2 py-1 rounded">build</code> - Build command for production</div>
						</div>
					</div>

					<div class="border rounded-lg p-4">
						<h4 class="font-semibold mb-2">Scripts</h4>
						<p class="text-sm text-muted-foreground mb-2">
							App-specific scripts with highest priority (override package and project scripts)
						</p>
						<div class="text-sm space-y-1">
							<div>• <code>dev</code> - Development server</div>
							<div>• <code>test</code> - Run app tests</div>
							<div>• <code>lint</code> - Lint app code</div>
							<div>• <code>preview</code> - Preview production build</div>
							<div>• Custom scripts for app-specific tasks</div>
						</div>
					</div>

					<div class="border rounded-lg p-4">
						<h4 class="font-semibold mb-2">Package Dependencies</h4>
						<p class="text-sm text-muted-foreground mb-2">
							Packages that this app depends on and should be linked
						</p>
						<div class="text-sm space-y-1">
							<div>• Local packages: just the package name (e.g., <code>utils</code>)</div>
							<div>• Team packages: <code>@team/package-name</code></div>
							<div>• External packages: <code>@scope/package-name</code></div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Script Priority -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Script Execution Priority</h2>
		
		<div class="space-y-6">
			<p class="text-muted-foreground">
				Knot uses a hierarchical script resolution system when you run <code>knot run &lt;script&gt;</code>.
			</p>

			<div class="bg-blue-50 border border-blue-200 rounded-lg p-6">
				<h3 class="font-semibold mb-4">Priority Order (Highest to Lowest)</h3>
				<div class="space-y-3">
					<div class="flex items-center space-x-3">
						<div class="w-8 h-8 bg-red-100 rounded-full flex items-center justify-center">
							<span class="text-red-600 font-bold text-sm">1</span>
						</div>
						<div>
							<div class="font-medium">App Scripts (app.yml)</div>
							<div class="text-sm text-muted-foreground">Scripts defined in the current app's configuration</div>
						</div>
					</div>
					<div class="flex items-center space-x-3">
						<div class="w-8 h-8 bg-yellow-100 rounded-full flex items-center justify-center">
							<span class="text-yellow-600 font-bold text-sm">2</span>
						</div>
						<div>
							<div class="font-medium">Package Scripts (package.yml)</div>
							<div class="text-sm text-muted-foreground">Scripts in the current package (when run from package directory)</div>
						</div>
					</div>
					<div class="flex items-center space-x-3">
						<div class="w-8 h-8 bg-blue-100 rounded-full flex items-center justify-center">
							<span class="text-blue-600 font-bold text-sm">3</span>
						</div>
						<div>
							<div class="font-medium">Project Scripts (knot.yml)</div>
							<div class="text-sm text-muted-foreground">Global scripts available from any directory</div>
						</div>
					</div>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Example Scenario</h3>
				<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
					<pre><code><span class="text-gray-400"># knot.yml (project)</span>
<span class="text-blue-400">scripts:</span>
  <span class="text-blue-400">test:</span> <span class="text-green-400">"echo 'Running project tests'"</span>

<span class="text-gray-400"># packages/utils/package.yml</span>
<span class="text-blue-400">scripts:</span>
  <span class="text-blue-400">test:</span> <span class="text-green-400">"vitest run"</span>

<span class="text-gray-400"># apps/frontend/app.yml</span>
<span class="text-blue-400">scripts:</span>
  <span class="text-blue-400">test:</span> <span class="text-green-400">"jest --coverage"</span></code></pre>
				</div>
				<div class="mt-3 text-sm text-muted-foreground">
					When you run <code>knot run test</code> from the <code>apps/frontend/</code> directory, 
					it will execute <code>jest --coverage</code> (app script takes priority).
				</div>
			</div>
		</div>
	</section>

	<!-- Variables & Interpolation -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Variables & Interpolation</h2>
		
		<div class="space-y-6">
			<p class="text-muted-foreground">
				Knot provides a powerful variable system that allows you to use dynamic values in your configuration files.
				Variables can be interpolated in any string value across knot.yml, app.yml, and package.yml files.
			</p>

			<div>
				<h3 class="text-lg font-semibold mb-3">Variable Syntax</h3>
				<p class="text-muted-foreground mb-4">
					Knot supports two variable interpolation syntaxes that you can use in your YAML configuration files:
				</p>
				
				<div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
					<div class="border rounded-lg p-4">
						<h4 class="font-semibold mb-2">Standard Syntax</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-3 rounded mb-2">
							<code><span class="text-green-400">name:</span> <span class="text-yellow-400">"{{project_name}}-frontend"</span></code>
						</div>
						<div class="text-sm text-muted-foreground">Use <code class="bg-muted px-1 rounded">{{variable}}</code> for simple variable references</div>
					</div>
					<div class="border rounded-lg p-4">
						<h4 class="font-semibold mb-2">Extended Syntax</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-3 rounded mb-2">
							<code><span class="text-green-400">version:</span> <span class="text-yellow-400">"${project_name}-${version}"</span></code>
						</div>
						<div class="text-sm text-muted-foreground">Use <code class="bg-muted px-1 rounded">${variable}</code> for advanced features and environment fallback</div>
					</div>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Complete Example</h3>
				<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg relative group">
					<pre><code><span class="text-gray-400"># knot.yml - Project configuration with variables</span>
<span class="text-blue-400">name:</span> <span class="text-green-400">"{{project_name}}"</span>
<span class="text-blue-400">description:</span> <span class="text-green-400">"{{project_name}} - Built on ${date}"</span>
<span class="text-blue-400">version:</span> <span class="text-green-400">"1.0.0"</span>

<span class="text-gray-400"># Define custom variables</span>
<span class="text-blue-400">variables:</span>
  <span class="text-blue-400">project_prefix:</span> <span class="text-green-400">"mycompany"</span>
  <span class="text-blue-400">build_target:</span> <span class="text-green-400">"production"</span>

<span class="text-blue-400">scripts:</span>
  <span class="text-blue-400">deploy:</span> <span class="text-green-400">"deploy --name={{project_prefix}}-{{project_name}} --target=${build_target}"</span>

<span class="text-blue-400">apps:</span>
  <span class="text-blue-400">frontend:</span>
    <span class="text-blue-400">variables:</span>
      <span class="text-blue-400">app_title:</span> <span class="text-green-400">"{{project_name}} Dashboard"</span>
    <span class="text-blue-400">packages:</span>
      - <span class="text-yellow-400">types</span></code></pre>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Variable Precedence</h3>
				<div class="bg-blue-50 border border-blue-200 rounded-lg p-6">
					<h4 class="font-semibold mb-4">Resolution Order (Highest to Lowest)</h4>
					<div class="space-y-3">
						<div class="flex items-center space-x-3">
							<div class="w-8 h-8 bg-purple-100 rounded-full flex items-center justify-center">
								<span class="text-purple-600 font-bold text-sm">1</span>
							</div>
							<div>
								<div class="font-medium">Built-in Variables</div>
								<div class="text-sm text-muted-foreground">System-provided variables like project_name, date, timestamp, year</div>
							</div>
						</div>
						<div class="flex items-center space-x-3">
							<div class="w-8 h-8 bg-green-100 rounded-full flex items-center justify-center">
								<span class="text-green-600 font-bold text-sm">2</span>
							</div>
							<div>
								<div class="font-medium">Package Variables</div>
								<div class="text-sm text-muted-foreground">Variables defined in package.yml files</div>
							</div>
						</div>
						<div class="flex items-center space-x-3">
							<div class="w-8 h-8 bg-yellow-100 rounded-full flex items-center justify-center">
								<span class="text-yellow-600 font-bold text-sm">3</span>
							</div>
							<div>
								<div class="font-medium">App Variables</div>
								<div class="text-sm text-muted-foreground">Variables defined in app.yml files</div>
							</div>
						</div>
						<div class="flex items-center space-x-3">
							<div class="w-8 h-8 bg-blue-100 rounded-full flex items-center justify-center">
								<span class="text-blue-600 font-bold text-sm">4</span>
							</div>
							<div>
								<div class="font-medium">Project Variables</div>
								<div class="text-sm text-muted-foreground">Variables defined in knot.yml</div>
							</div>
						</div>
					</div>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Built-in Variables</h3>
				<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
					<div class="border rounded-lg p-4">
						<h4 class="font-semibold mb-3">System Variables</h4>
						<div class="text-sm space-y-2">
							<div class="flex justify-between">
								<code class="bg-muted px-2 py-1 rounded">project_name</code>
								<span class="text-muted-foreground">Name from knot.yml</span>
							</div>
							<div class="flex justify-between">
								<code class="bg-muted px-2 py-1 rounded">project_root</code>
								<span class="text-muted-foreground">Project root path</span>
							</div>
							<div class="flex justify-between">
								<code class="bg-muted px-2 py-1 rounded">date</code>
								<span class="text-muted-foreground">Current date (YYYY-MM-DD)</span>
							</div>
							<div class="flex justify-between">
								<code class="bg-muted px-2 py-1 rounded">timestamp</code>
								<span class="text-muted-foreground">Unix timestamp</span>
							</div>
							<div class="flex justify-between">
								<code class="bg-muted px-2 py-1 rounded">year</code>
								<span class="text-muted-foreground">Current year</span>
							</div>
						</div>
					</div>

					<div class="border rounded-lg p-4">
						<h4 class="font-semibold mb-3">CLI Commands</h4>
						<div class="text-sm space-y-2">
							<div>
								<code class="bg-muted px-2 py-1 rounded">knot vars list</code>
								<div class="text-muted-foreground mt-1">Show all available variables</div>
							</div>
							<div>
								<code class="bg-muted px-2 py-1 rounded">knot vars get &lt;name&gt;</code>
								<div class="text-muted-foreground mt-1">Get a specific variable value</div>
							</div>
						</div>
					</div>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Advanced Features</h3>
				<div class="space-y-4">
					<div class="border rounded-lg p-4">
						<h4 class="font-semibold mb-2">Nested Variable References</h4>
						<p class="text-sm text-muted-foreground mb-3">Variables can reference other variables for complex compositions:</p>
						<div class="bg-black/90 text-white font-mono text-sm p-3 rounded">
							<pre><code><span class="text-blue-400">variables:</span>
  <span class="text-blue-400">env:</span> <span class="text-green-400">"production"</span>
  <span class="text-blue-400">app_name:</span> <span class="text-green-400">"{{project_name}}"</span>
  <span class="text-blue-400">full_name:</span> <span class="text-green-400">"{{app_name}}-{{env}}"</span>         <span class="text-gray-400"># Results in: "myproject-production"</span></code></pre>
						</div>
					</div>

					<div class="border rounded-lg p-4">
						<h4 class="font-semibold mb-2">Environment Variable Fallback</h4>
						<p class="text-sm text-muted-foreground mb-3">Use <code>{'${}'}</code> syntax for environment variable fallback:</p>
						<div class="bg-black/90 text-white font-mono text-sm p-3 rounded">
							<pre><code><span class="text-blue-400">variables:</span>
  <span class="text-blue-400">api_url:</span> <span class="text-green-400">"${API_URL}"</span>           <span class="text-gray-400"># Falls back to $API_URL env var</span>
  <span class="text-blue-400">version:</span> <span class="text-green-400">"${BUILD_VERSION}"</span>     <span class="text-gray-400"># Falls back to $BUILD_VERSION env var</span></code></pre>
						</div>
					</div>

					<div class="border rounded-lg p-4">
						<h4 class="font-semibold mb-2">Error Handling</h4>
						<div class="text-sm space-y-2 text-muted-foreground">
							<div>• <strong>Undefined variables:</strong> Knot will show an error with suggestions of available variables</div>
							<div>• <strong>Circular references:</strong> Automatic detection prevents infinite loops</div>
							<div>• <strong>Type conversion:</strong> All variables are converted to strings when interpolated</div>
							<div>• <strong>Validation:</strong> Variable names must be alphanumeric with underscores</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Environment Variables -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Environment Configuration</h2>
		
		<div class="space-y-6">
			<p class="text-muted-foreground">
				Knot supports environment-specific configurations and can load environment variables.
			</p>

			<div>
				<h3 class="text-lg font-semibold mb-3">Environment Variables</h3>
				<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
					<div class="border rounded-lg p-4">
						<h4 class="font-semibold mb-2">Knot Environment Variables</h4>
						<div class="text-sm space-y-1">
							<div><code class="bg-muted px-2 py-1 rounded">KNOT_TOKEN</code> - Authentication token</div>
							<div><code class="bg-muted px-2 py-1 rounded">KNOT_SPACE_URL</code> - Registry URL</div>
							<div><code class="bg-muted px-2 py-1 rounded">KNOT_ENV</code> - Environment (dev/prod)</div>
						</div>
					</div>

					<div class="border rounded-lg p-4">
						<h4 class="font-semibold mb-2">Custom Variables</h4>
						<div class="text-sm space-y-1">
							<div>Define in your app.yml:</div>
							<div class="bg-muted p-2 rounded font-mono text-xs">
								<pre><code>env:
  API_URL: "https://api.example.com"</code></pre>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Best Practices -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Best Practices</h2>
		
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<div class="space-y-6">
				<div class="border rounded-lg p-6">
					<h3 class="font-semibold mb-3 flex items-center">
						<Icon icon="lucide:check-circle" class="w-5 h-5 mr-2 text-green-600" />
						Configuration Tips
					</h3>
					<ul class="text-sm text-muted-foreground space-y-2">
						<li>• Use semantic versioning for packages</li>
						<li>• Keep script names consistent across configs</li>
						<li>• Use descriptive package and app names</li>
						<li>• Document complex scripts with comments</li>
						<li>• Group related packages with tags</li>
					</ul>
				</div>

				<div class="border rounded-lg p-6">
					<h3 class="font-semibold mb-3 flex items-center">
						<Icon icon="lucide:settings-bold" class="w-5 h-5 mr-2 text-blue-600" />
						Organization
					</h3>
					<ul class="text-sm text-muted-foreground space-y-2">
						<li>• Define common scripts at project level</li>
						<li>• Override with specific implementations in apps</li>
						<li>• Use teams for related packages</li>
						<li>• Keep environment configs separate</li>
					</ul>
				</div>
			</div>

			<div class="space-y-6">
				<div class="border rounded-lg p-6">
					<h3 class="font-semibold mb-3 flex items-center">
						<Icon icon="lucide:shield-warning-bold" class="w-5 h-5 mr-2 text-yellow-600" />
						Common Pitfalls
					</h3>
					<ul class="text-sm text-muted-foreground space-y-2">
						<li>• Don't duplicate package names across teams</li>
						<li>• Avoid circular dependencies between packages</li>
						<li>• Don't hardcode environment-specific values</li>
						<li>• Keep script commands platform-agnostic</li>
					</ul>
				</div>

				<div class="border rounded-lg p-6">
					<h3 class="font-semibold mb-3 flex items-center">
						<Icon icon="lucide:bookmark-bold" class="w-5 h-5 mr-2 text-purple-600" />
						Validation
					</h3>
					<ul class="text-sm text-muted-foreground space-y-2">
						<li>• Knot validates YAML syntax automatically</li>
						<li>• Required fields are enforced</li>
						<li>• Package names must be unique</li>
						<li>• Version numbers must be valid semver</li>
					</ul>
				</div>
			</div>
		</div>
	</section>

	<!-- Next Steps -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Next Steps</h2>
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<a href="/docs/package-linking" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:link" class="w-6 h-6 text-green-600" />
					<h3 class="font-semibold">Package Linking</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Learn how Knot links packages and manages dependencies between apps and packages.
				</p>
			</a>

			<a href="/docs/project-management" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:terminal" class="w-6 h-6 text-blue-600" />
					<h3 class="font-semibold">CLI Commands</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Explore all available Knot CLI commands and their usage patterns.
				</p>
			</a>
		</div>
	</section>
</div>