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
	<meta name="description" content="Learn how to manage your Knot projects, from initialization to configuration and status checks." />
	<meta property="og:title" content="Project Management - Knot CLI" />
	<meta property="og:description" content="Learn how to manage your Knot projects, from initialization to configuration and status checks." />
	<meta property="og:url" content="https://knot-space.com/docs/project-management" />
	<meta name="twitter:title" content="Project Management - Knot CLI" />
	<meta name="twitter:description" content="Learn how to manage your Knot projects, from initialization to configuration and status checks." />
	<link rel="canonical" href="https://knot-space.com/docs/project-management" />
</svelte:head>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight mb-4" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
			Project Management
		</h1>
		<p class="text-lg text-muted-foreground">
			Efficiently manage your monorepo with Knot's project management commands.
		</p>
	</div>

	<!-- Initializing a Project -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Initializing a Project</h2>
		<p class="text-muted-foreground mb-6 leading-relaxed">
			Start a new monorepo with a single command. Knot will set up the basic structure for you, including the root <code>knot.yml</code> configuration file.
		</p>
		
		<div>
			<h3 class="text-lg font-semibold mb-3">Create a new project</h3>
			<div class="bg-black/90 text-green-400 font-mono text-sm rounded-lg relative group">
				<div class="overflow-x-auto p-4 pr-12">
					<code class="whitespace-nowrap block">knot init &lt;project-name&gt;</code>
				</div>
				<button 
					class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100 z-10"
					on:click={() => copyToClipboard('knot init <project-name>')}
				>
					{#if showCopied && copyText === 'knot init <project-name>'}
						<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
					{:else}
						<Icon icon="solar:copy-bold" class="w-4 h-4" />
					{/if}
				</button>
			</div>
		</div>
	</section>

	<!-- Project Configuration -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Project Configuration</h2>
		<p class="text-muted-foreground mb-6 leading-relaxed">
			The <code>knot.yml</code> file is the heart of your Knot project. It defines the project's name, description, and any scripts that can be run from the root.
		</p>
		
		<div>
			<h3 class="text-lg font-semibold mb-3">knot.yml structure</h3>
			<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
				<pre><code>{@html `<span class="text-gray-400"># knot.yml</span>
<span class="text-blue-400">name:</span> <span class="text-green-400">my-awesome-project</span>
<span class="text-blue-400">description:</span> <span class="text-green-400">"A description of my awesome project."</span>
<span class="text-blue-400">scripts:</span>
  <span class="text-blue-400">test:</span> <span class="text-green-400">"echo 'Error: no test specified' && exit 1"</span>
  <span class="text-blue-400">lint:</span> <span class="text-green-400">"eslint ."</span>`}</code></pre>
			</div>
		</div>
	</section>

	<!-- Checking Project Status -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Checking Project Status</h2>
		<p class="text-muted-foreground mb-6 leading-relaxed">
			Get a quick overview of your project's status, including a list of all apps and packages and their dependencies.
		</p>
		
		<div>
			<h3 class="text-lg font-semibold mb-3">Project status command</h3>
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
		</div>
	</section>

	<!-- Next Steps -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Next Steps</h2>
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<a href="/docs/quick-start" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:play-bold" class="w-6 h-6 text-green-600" />
					<h3 class="font-semibold">Quick Start Guide</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Get up and running with your first project in under 5 minutes.
				</p>
			</a>

			<a href="/docs/cli-commands" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:terminal-bold" class="w-6 h-6 text-blue-600" />
					<h3 class="font-semibold">CLI Commands</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Complete command reference for all Knot CLI operations.
				</p>
			</a>
		</div>
	</section>
</div>