
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

	<!-- Initializing a Package -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Initializing a Package</h2>
		<p class="text-muted-foreground leading-relaxed mb-6">
			Create a new package in your project's `packages` directory. Knot will generate a `package.yml` file for you.
		</p>
		<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
			<code>knot init:package &lt;package-name&gt;</code>
		</div>
	</section>

	<!-- Package Configuration -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Package Configuration</h2>
		<p class="text-muted-foreground leading-relaxed mb-6">
			The `package.yml` file contains metadata about your package, including its name, version, and any scripts.
		</p>
		<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
			<pre><code><span class="text-gray-400"># packages/my-package/package.yml</span>
<span class="text-blue-400">name:</span> <span class="text-green-400">my-package</span>
<span class="text-blue-400">version:</span> <span class="text-green-400">0.1.0</span>
<span class="text-blue-400">scripts:</span>
  <span class="text-blue-400">build:</span> <span class="text-green-400">tsc</span>
</code></pre>
		</div>
	</section>

	<!-- Adding Dependencies -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Adding Dependencies</h2>
		<p class="text-muted-foreground leading-relaxed mb-6">
			You can add other packages as dependencies to your package. This is done by adding a `packages` key to your `package.yml` file.
		</p>
		<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
			<pre><code><span class="text-gray-400"># packages/my-package/package.yml</span>
<span class="text-blue-400">name:</span> <span class="text-green-400">my-package</span>
<span class="text-blue-400">version:</span> <span class="text-green-400">0.1.0</span>
<span class="text-blue-400">packages:</span>
  <span class="text-green-400">- other-package</span>
  <span class="text-green-400">- @some-team/another-package</span>
</code></pre>
		</div>
	</section>
</div>
