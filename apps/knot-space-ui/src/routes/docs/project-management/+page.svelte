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

	<div class="prose prose-slate max-w-none">
		<h2 class="text-2xl font-bold mb-4">Initializing a Project</h2>
		<p class="mb-6">
			Start a new monorepo with a single command. Knot will set up the basic structure for you, including the root `knot.yml` configuration file.
		</p>
		
		<div class="bg-muted rounded-lg p-4 mb-6">
			<h3 class="font-semibold mb-2">Create a new project</h3>
			<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">knot init &lt;project-name&gt;</code></pre>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Project Configuration</h2>
		<p class="mb-6">
			The `knot.yml` file is the heart of your Knot project. It defines the project's name, description, and any scripts that can be run from the root.
		</p>
		
		<div class="bg-muted rounded-lg p-4 mb-6">
			<h3 class="font-semibold mb-2">knot.yml structure</h3>
			<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`# knot.yml
name: my-awesome-project
description: A description of my awesome project.
scripts:
  test: echo "Error: no test specified" && exit 1
  lint: eslint .`}</code></pre>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Checking Project Status</h2>
		<p class="mb-6">
			Get a quick overview of your project's status, including a list of all apps and packages and their dependencies.
		</p>
		
		<div class="bg-muted rounded-lg p-4 mb-6">
			<h3 class="font-semibold mb-2">Project status command</h3>
			<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">knot status</code></pre>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Next Steps</h2>
		
		<div class="grid gap-4 mt-6">
			<a href="/docs/quick-start" class="group border rounded-lg p-4 hover:bg-muted transition-colors">
				<div class="flex items-center justify-between">
					<div>
						<h3 class="font-semibold group-hover:text-primary">Quick Start Guide</h3>
						<p class="text-sm text-muted-foreground">Get up and running with your first project</p>
					</div>
					<Icon icon="solar:arrow-right-bold" class="w-5 h-5 text-muted-foreground group-hover:text-primary" />
				</div>
			</a>

			<a href="/docs/cli-commands" class="group border rounded-lg p-4 hover:bg-muted transition-colors">
				<div class="flex items-center justify-between">
					<div>
						<h3 class="font-semibold group-hover:text-primary">CLI Commands</h3>
						<p class="text-sm text-muted-foreground">Complete command reference</p>
					</div>
					<Icon icon="solar:arrow-right-bold" class="w-5 h-5 text-muted-foreground group-hover:text-primary" />
				</div>
			</a>
		</div>
	</div>
</div>