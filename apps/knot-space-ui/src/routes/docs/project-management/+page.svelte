<script lang="ts">
	import Icon from '@iconify/svelte';
</script>

<svelte:head>
	<title>Project Management - Knot CLI Documentation</title>
	<meta name="description" content="Learn how to manage your Knot projects, from initialization to configuration and status checks." />
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
		<p class="text-muted-foreground leading-relaxed mb-6">
			Start a new monorepo with a single command. Knot will set up the basic structure for you, including the root `knot.yml` configuration file.
		</p>
		<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
			<code>knot init &lt;project-name&gt;</code>
		</div>
	</section>

	<!-- Project Configuration -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Project Configuration</h2>
		<p class="text-muted-foreground leading-relaxed mb-6">
			The `knot.yml` file is the heart of your Knot project. It defines the project's name, description, and any scripts that can be run from the root.
		</p>
		<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
			<pre><code><span class="text-gray-400"># knot.yml</span>
<span class="text-blue-400">name:</span> <span class="text-green-400">my-awesome-project</span>
<span class="text-blue-400">description:</span> <span class="text-green-400">A description of my awesome project.</span>
<span class="text-blue-400">scripts:</span>
  <span class="text-blue-400">test:</span> <span class="text-green-400">echo "Error: no test specified" && exit 1</span>
  <span class="text-blue-400">lint:</span> <span class="text-green-400">eslint .</span>
</code></pre>
		</div>
	</section>

	<!-- Checking Project Status -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Checking Project Status</h2>
		<p class="text-muted-foreground leading-relaxed mb-6">
			Get a quick overview of your project's status, including a list of all apps and packages and their dependencies.
		</p>
		<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
			<code>knot status</code>
		</div>
	</section>
</div>