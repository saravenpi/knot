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
	<title>Project Management - Knot CLI Documentation</title>
	<meta name="description" content="Learn how to manage your Knot projects, from initialization to configuration, status checks, and project operations." />
	<meta property="og:title" content="Project Management - Knot CLI" />
	<meta property="og:description" content="Learn how to manage your Knot projects, from initialization to configuration, status checks, and project operations." />
	<meta property="og:image" content="/images/og/project-management.png" />
	<meta property="og:url" content="https://knot.klysium.com/docs/project-management" />
	<meta name="twitter:title" content="Project Management - Knot CLI" />
	<meta name="twitter:description" content="Learn how to manage your Knot projects, from initialization to configuration, status checks, and project operations." />
	<meta name="twitter:image" content="/images/og/project-management.png" />
	<link rel="canonical" href="https://knot.klysium.com/docs/project-management" />
</svelte:head>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight mb-4" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
			Project Management
		</h1>
		<p class="text-lg text-muted-foreground">
			Efficiently manage your monorepo with Knot's comprehensive project management commands
		</p>
	</div>

	<!-- Initializing a Project -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<Icon icon="solar:rocket-bold" class="w-8 h-8 text-green-600" />
			<h2 class="text-2xl font-bold">Initializing a Project</h2>
		</div>
		<p class="text-muted-foreground mb-6 leading-relaxed">
			Start a new monorepo with a single command. Knot will set up the basic structure for you, including the root <code class="bg-muted px-1.5 py-0.5 rounded text-sm">knot.yml</code> configuration file and organize your workspace.
		</p>
		
		<div class="space-y-6">
			<div>
				<h3 class="text-lg font-semibold mb-3">Create a new project</h3>
				<div class="bg-black/90 text-green-400 font-mono text-sm rounded-lg relative group">
					<div class="overflow-x-auto p-4 pr-12">
						<code class="whitespace-nowrap block">knot init &lt;project-name&gt;</code>
					</div>
					<button 
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100 z-10"
						on:click={() => copyToClipboard('knot init my-project')}
					>
						{#if showCopied && copyText === 'knot init my-project'}
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="solar:copy-bold" class="w-4 h-4" />
						{/if}
					</button>
				</div>
				<p class="text-sm text-muted-foreground mt-2">
					This creates a new directory with your project name and sets up the initial project structure.
				</p>
			</div>

			<div class="bg-muted/30 rounded-lg p-6">
				<h4 class="font-semibold mb-3 flex items-center space-x-2">
					<Icon icon="solar:folder-bold" class="w-5 h-5 text-blue-600" />
					<span>Generated Project Structure</span>
				</h4>
				<pre class="text-sm font-mono"><code>my-project/
├── <span class="text-blue-400">knot.yml</span>          # Project configuration
├── <span class="text-yellow-400">packages/</span>        # Shared packages go here
├── <span class="text-green-400">apps/</span>             # Applications go here
├── <span class="text-gray-400">node_modules/</span>     # Dependencies
├── <span class="text-purple-400">tsconfig.json</span>    # TypeScript configuration
└── <span class="text-gray-400">README.md</span>         # Project documentation</code></pre>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Initialize in existing directory</h3>
				<div class="bg-black/90 text-green-400 font-mono text-sm rounded-lg relative group">
					<div class="overflow-x-auto p-4 pr-12">
						<code class="whitespace-nowrap block">knot init .</code>
					</div>
					<button 
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100 z-10"
						on:click={() => copyToClipboard('knot init .')}
					>
						{#if showCopied && copyText === 'knot init .'}
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="solar:copy-bold" class="w-4 h-4" />
						{/if}
					</button>
				</div>
				<p class="text-sm text-muted-foreground mt-2">
					Initialize Knot in your current directory without creating a new folder.
				</p>
			</div>
		</div>
	</section>

	<!-- Project Configuration -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<Icon icon="solar:settings-bold" class="w-8 h-8 text-blue-600" />
			<h2 class="text-2xl font-bold">Project Configuration</h2>
		</div>
		<p class="text-muted-foreground mb-6 leading-relaxed">
			The <code class="bg-muted px-1.5 py-0.5 rounded text-sm">knot.yml</code> file is the heart of your Knot project. It defines the project's metadata, scripts, and global configurations that apply to your entire monorepo.
		</p>
		
		<div class="space-y-6">
			<div>
				<h3 class="text-lg font-semibold mb-3">Basic knot.yml structure</h3>
				<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
					<pre><code>{@html `<span class="text-gray-400"># knot.yml - Project configuration</span>
<span class="text-blue-400">name:</span> <span class="text-green-400">"my-awesome-project"</span>
<span class="text-blue-400">description:</span> <span class="text-green-400">"A modern TypeScript monorepo"</span>
<span class="text-blue-400">version:</span> <span class="text-green-400">"1.0.0"</span>
<span class="text-blue-400">author:</span> <span class="text-green-400">"Your Name"</span>

<span class="text-blue-400">scripts:</span>
  <span class="text-blue-400">test:</span> <span class="text-green-400">"echo 'Running tests for all packages'"</span>
  <span class="text-blue-400">lint:</span> <span class="text-green-400">"eslint . --ext .ts,.tsx,.js,.jsx"</span>
  <span class="text-blue-400">build:</span> <span class="text-green-400">"knot run build --all"</span>
  <span class="text-blue-400">dev:</span> <span class="text-green-400">"knot run dev --parallel"</span>`}</code></pre>
				</div>
				<p class="text-sm text-muted-foreground mt-2">
					Define project metadata and global scripts that can be run from anywhere in your monorepo.
				</p>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Advanced configuration options</h3>
				<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
					<pre><code>{@html `<span class="text-gray-400"># Advanced knot.yml configuration</span>
<span class="text-blue-400">workspaces:</span>
  <span class="text-blue-400">packages:</span> <span class="text-green-400">["packages/*", "tools/*"]</span>
  <span class="text-blue-400">apps:</span> <span class="text-green-400">["apps/*", "examples/*"]</span>

<span class="text-blue-400">typescript:</span>
  <span class="text-blue-400">aliases:</span>
    <span class="text-blue-400">"@utils/*":</span> <span class="text-green-400">"packages/utils/src/*"</span>
    <span class="text-blue-400">"@components/*":</span> <span class="text-green-400">"packages/ui/src/components/*"</span>

<span class="text-blue-400">build:</span>
  <span class="text-blue-400">parallel:</span> <span class="text-yellow-400">true</span>
  <span class="text-blue-400">watch:</span> <span class="text-yellow-400">true</span>
  <span class="text-blue-400">target:</span> <span class="text-green-400">"es2020"</span>`}</code></pre>
				</div>
			</div>
		</div>
	</section>

	<!-- Project Status -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<Icon icon="solar:chart-bold" class="w-8 h-8 text-purple-600" />
			<h2 class="text-2xl font-bold">Checking Project Status</h2>
		</div>
		<p class="text-muted-foreground mb-6 leading-relaxed">
			Get comprehensive insights into your project's health, dependencies, and workspace organization with Knot's status commands.
		</p>
		
		<div class="space-y-6">
			<div>
				<h3 class="text-lg font-semibold mb-3">Project overview</h3>
				<div class="bg-black/90 text-green-400 font-mono text-sm rounded-lg relative group">
					<div class="overflow-x-auto p-4 pr-12">
						<code class="whitespace-nowrap block">knot status</code>
					</div>
					<button 
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100 z-10"
						on:click={() => copyToClipboard('knot status')}
					>
						{#if showCopied && copyText === 'knot status'}
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="solar:copy-bold" class="w-4 h-4" />
						{/if}
					</button>
				</div>
				<p class="text-sm text-muted-foreground mt-2">
					Shows a comprehensive overview of all packages, apps, dependencies, and their status.
				</p>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Detailed workspace information</h3>
				<div class="bg-black/90 text-green-400 font-mono text-sm rounded-lg relative group">
					<div class="overflow-x-auto p-4 pr-12">
						<code class="whitespace-nowrap block">knot status --detailed</code>
					</div>
					<button 
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100 z-10"
						on:click={() => copyToClipboard('knot status --detailed')}
					>
						{#if showCopied && copyText === 'knot status --detailed'}
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="solar:copy-bold" class="w-4 h-4" />
						{/if}
					</button>
				</div>
				<p class="text-sm text-muted-foreground mt-2">
					Includes version information, build status, and dependency analysis for each workspace.
				</p>
			</div>

			<div class="bg-blue-50 border border-blue-200 rounded-lg p-6">
				<div class="flex items-start space-x-3">
					<Icon icon="solar:chart-2-bold" class="w-6 h-6 text-blue-600 mt-1 flex-shrink-0" />
					<div>
						<h4 class="font-semibold text-blue-900 mb-2">Status Information Includes</h4>
						<ul class="text-sm text-blue-700 space-y-1">
							<li>• <strong>Workspace overview</strong>: All packages and apps in your project</li>
							<li>• <strong>Dependency graph</strong>: Internal dependencies between workspaces</li>
							<li>• <strong>Build status</strong>: Which packages are built and up-to-date</li>
							<li>• <strong>Version information</strong>: Current versions of all workspaces</li>
							<li>• <strong>Link status</strong>: Which packages are properly linked</li>
						</ul>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Project Scripts -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<Icon icon="solar:play-bold" class="w-8 h-8 text-orange-600" />
			<h2 class="text-2xl font-bold">Running Project Scripts</h2>
		</div>
		<p class="text-muted-foreground mb-6 leading-relaxed">
			Execute scripts across your entire monorepo or target specific workspaces with Knot's powerful script execution system.
		</p>
		
		<div class="space-y-6">
			<div>
				<h3 class="text-lg font-semibold mb-3">Run project-wide scripts</h3>
				<div class="bg-black/90 text-green-400 font-mono text-sm rounded-lg relative group">
					<div class="overflow-x-auto p-4 pr-12">
						<code class="whitespace-nowrap block">knot run &lt;script-name&gt;</code>
					</div>
					<button 
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100 z-10"
						on:click={() => copyToClipboard('knot run build')}
					>
						{#if showCopied && copyText === 'knot run build'}
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="solar:copy-bold" class="w-4 h-4" />
						{/if}
					</button>
				</div>
				<p class="text-sm text-muted-foreground mt-2">
					Executes the specified script defined in your <code>knot.yml</code> file.
				</p>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Run scripts in parallel</h3>
				<div class="bg-black/90 text-green-400 font-mono text-sm rounded-lg relative group">
					<div class="overflow-x-auto p-4 pr-12">
						<code class="whitespace-nowrap block">knot run build --parallel</code>
					</div>
					<button 
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100 z-10"
						on:click={() => copyToClipboard('knot run build --parallel')}
					>
						{#if showCopied && copyText === 'knot run build --parallel'}
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="solar:copy-bold" class="w-4 h-4" />
						{/if}
					</button>
				</div>
				<p class="text-sm text-muted-foreground mt-2">
					Execute scripts across multiple workspaces simultaneously for faster builds.
				</p>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Target specific workspaces</h3>
				<div class="bg-black/90 text-green-400 font-mono text-sm rounded-lg relative group">
					<div class="overflow-x-auto p-4 pr-12">
						<code class="whitespace-nowrap block">knot run test --filter="packages/*"</code>
					</div>
					<button 
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100 z-10"
						on:click={() => copyToClipboard('knot run test --filter="packages/*"')}
					>
						{#if showCopied && copyText === 'knot run test --filter="packages/*"'}
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="solar:copy-bold" class="w-4 h-4" />
						{/if}
					</button>
				</div>
				<p class="text-sm text-muted-foreground mt-2">
					Run scripts only in workspaces that match the specified pattern.
				</p>
			</div>
		</div>
	</section>

	<!-- Project Maintenance -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<Icon icon="solar:settings-minimalistic-bold" class="w-8 h-8 text-red-600" />
			<h2 class="text-2xl font-bold">Project Maintenance</h2>
		</div>
		<p class="text-muted-foreground mb-6 leading-relaxed">
			Keep your monorepo healthy and optimized with maintenance commands for cleaning, updating, and reorganizing your project.
		</p>
		
		<div class="space-y-6">
			<div>
				<h3 class="text-lg font-semibold mb-3">Clean build artifacts</h3>
				<div class="bg-black/90 text-green-400 font-mono text-sm rounded-lg relative group">
					<div class="overflow-x-auto p-4 pr-12">
						<code class="whitespace-nowrap block">knot clean</code>
					</div>
					<button 
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100 z-10"
						on:click={() => copyToClipboard('knot clean')}
					>
						{#if showCopied && copyText === 'knot clean'}
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="solar:copy-bold" class="w-4 h-4" />
						{/if}
					</button>
				</div>
				<p class="text-sm text-muted-foreground mt-2">
					Removes build outputs, temporary files, and cached data from all workspaces.
				</p>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Update dependencies</h3>
				<div class="bg-black/90 text-green-400 font-mono text-sm rounded-lg relative group">
					<div class="overflow-x-auto p-4 pr-12">
						<code class="whitespace-nowrap block">knot update</code>
					</div>
					<button 
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100 z-10"
						on:click={() => copyToClipboard('knot update')}
					>
						{#if showCopied && copyText === 'knot update'}
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="solar:copy-bold" class="w-4 h-4" />
						{/if}
					</button>
				</div>
				<p class="text-sm text-muted-foreground mt-2">
					Updates dependencies across all workspaces and refreshes package links.
				</p>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Validate project structure</h3>
				<div class="bg-black/90 text-green-400 font-mono text-sm rounded-lg relative group">
					<div class="overflow-x-auto p-4 pr-12">
						<code class="whitespace-nowrap block">knot validate</code>
					</div>
					<button 
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100 z-10"
						on:click={() => copyToClipboard('knot validate')}
					>
						{#if showCopied && copyText === 'knot validate'}
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="solar:copy-bold" class="w-4 h-4" />
						{/if}
					</button>
				</div>
				<p class="text-sm text-muted-foreground mt-2">
					Checks for common issues in project structure, dependencies, and configuration.
				</p>
			</div>
		</div>
	</section>

	<!-- Best Practices -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<Icon icon="solar:star-bold" class="w-8 h-8 text-yellow-500" />
			<h2 class="text-2xl font-bold">Best Practices</h2>
		</div>
		
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<div class="bg-green-50 border border-green-200 rounded-lg p-6">
				<div class="flex items-start space-x-3">
					<Icon icon="solar:check-circle-bold" class="w-6 h-6 text-green-600 mt-1 flex-shrink-0" />
					<div>
						<h3 class="font-semibold text-green-900 mb-2">Project Organization</h3>
						<ul class="text-sm text-green-700 space-y-1">
							<li>• Keep packages focused and single-purpose</li>
							<li>• Use consistent naming conventions</li>
							<li>• Document your project structure in README</li>
							<li>• Regularly run <code>knot status</code> to monitor health</li>
						</ul>
					</div>
				</div>
			</div>

			<div class="bg-blue-50 border border-blue-200 rounded-lg p-6">
				<div class="flex items-start space-x-3">
					<Icon icon="solar:settings-bold" class="w-6 h-6 text-blue-600 mt-1 flex-shrink-0" />
					<div>
						<h3 class="font-semibold text-blue-900 mb-2">Configuration Tips</h3>
						<ul class="text-sm text-blue-700 space-y-1">
							<li>• Version your <code>knot.yml</code> configuration</li>
							<li>• Use environment-specific scripts</li>
							<li>• Leverage TypeScript aliases for clean imports</li>
							<li>• Set up parallel builds for performance</li>
						</ul>
					</div>
				</div>
			</div>

			<div class="bg-orange-50 border border-orange-200 rounded-lg p-6">
				<div class="flex items-start space-x-3">
					<Icon icon="solar:shield-check-bold" class="w-6 h-6 text-orange-600 mt-1 flex-shrink-0" />
					<div>
						<h3 class="font-semibold text-orange-900 mb-2">Maintenance</h3>
						<ul class="text-sm text-orange-700 space-y-1">
							<li>• Run <code>knot clean</code> before major builds</li>
							<li>• Validate project structure regularly</li>
							<li>• Keep dependencies up to date</li>
							<li>• Monitor build performance metrics</li>
						</ul>
					</div>
				</div>
			</div>

			<div class="bg-purple-50 border border-purple-200 rounded-lg p-6">
				<div class="flex items-start space-x-3">
					<Icon icon="solar:users-group-two-rounded-bold" class="w-6 h-6 text-purple-600 mt-1 flex-shrink-0" />
					<div>
						<h3 class="font-semibold text-purple-900 mb-2">Team Collaboration</h3>
						<ul class="text-sm text-purple-700 space-y-1">
							<li>• Share consistent development scripts</li>
							<li>• Document workspace dependencies</li>
							<li>• Use project-level linting and formatting</li>
							<li>• Establish clear contribution guidelines</li>
						</ul>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Next Steps -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Next Steps</h2>
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
			<a href="/docs/package-development" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:box-bold" class="w-6 h-6 text-blue-600" />
					<h3 class="font-semibold">Package Development</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Learn how to create, build, and manage packages within your monorepo.
				</p>
			</a>

			<a href="/docs/configuration" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:terminal-bold" class="w-6 h-6 text-green-600" />
					<h3 class="font-semibold">CLI Commands</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Complete reference for all Knot CLI commands and their options.
				</p>
			</a>

			<a href="/docs/workflows" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:cpu-bolt-bold" class="w-6 h-6 text-purple-600" />
					<h3 class="font-semibold">Workflows</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Set up CI/CD workflows and automation for your Knot projects.
				</p>
			</a>

			<a href="/docs/teams" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:users-group-two-rounded-bold" class="w-6 h-6 text-orange-600" />
					<h3 class="font-semibold">Team Management</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Collaborate with your team and manage permissions effectively.
				</p>
			</a>

			<a href="/docs/typescript" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:code-bold" class="w-6 h-6 text-red-600" />
					<h3 class="font-semibold">TypeScript Setup</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Configure TypeScript for optimal development experience.
				</p>
			</a>

			<a href="/docs/troubleshooting" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:bug-bold" class="w-6 h-6 text-yellow-600" />
					<h3 class="font-semibold">Troubleshooting</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Common issues and solutions for Knot project management.
				</p>
			</a>
		</div>
	</section>
</div>