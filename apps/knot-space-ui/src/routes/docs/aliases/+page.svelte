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
	<title>TypeScript Path Aliases - Knot CLI Documentation</title>
	<meta name="description" content="Learn how to configure and use TypeScript path aliases with Knot CLI for cleaner imports and better developer experience." />
	<meta property="og:title" content="TypeScript Path Aliases - Knot CLI" />
	<meta property="og:description" content="Learn how to configure and use TypeScript path aliases with Knot CLI for cleaner imports and better developer experience." />
	<meta property="og:image" content="/images/og/typescript.png" />
	<meta property="og:url" content="https://knot-space.com/docs/aliases" />
	<meta name="twitter:title" content="TypeScript Path Aliases - Knot CLI" />
	<meta name="twitter:description" content="Learn how to configure and use TypeScript path aliases with Knot CLI for cleaner imports and better developer experience." />
	<meta name="twitter:image" content="/images/og/typescript.png" />
	<link rel="canonical" href="https://knot-space.com/docs/aliases" />
</svelte:head>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight mb-4" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
			TypeScript Path Aliases
		</h1>
		<p class="text-lg text-muted-foreground">
			Configure TypeScript path aliases for cleaner imports and better developer experience
		</p>
	</div>

	<!-- Overview -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Overview</h2>
		<div class="bg-gradient-to-r from-blue-50 to-indigo-50 border border-blue-200 rounded-lg p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="solar:code-bold" class="w-6 h-6 text-blue-600 mt-1 flex-shrink-0" />
				<div>
					<h3 class="font-semibold text-blue-900 mb-2">What are Path Aliases?</h3>
					<p class="text-sm text-blue-700 mb-4">
						Path aliases allow you to use shortcut imports instead of relative paths. Instead of writing <code>../../packages/utils</code>, you can use <code>@/utils</code> or <code>#/utils</code>. Knot automatically configures TypeScript path mappings for your packages.
					</p>
					<div class="space-y-2 text-sm text-blue-700">
						<div class="flex items-center space-x-2">
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600" />
							<span>Cleaner, more readable imports</span>
						</div>
						<div class="flex items-center space-x-2">
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600" />
							<span>Automatic TypeScript configuration</span>
						</div>
						<div class="flex items-center space-x-2">
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600" />
							<span>IDE auto-completion and navigation</span>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Configuration -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Configuration</h2>

		<div class="space-y-8">
			<!-- Project Level -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">Project-Level Configuration</h3>
				<p class="text-muted-foreground mb-4">
					Set a default TypeScript alias prefix for all apps in your project.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">In knot.yml</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-blue-400">name:</span> my-project
<span class="text-blue-400">ts_alias:</span> <span class="text-green-400">"@"</span>  <span class="text-gray-500"># Default alias prefix</span>

<span class="text-blue-400">apps:</span>
  <span class="text-blue-400">frontend:</span>
    <span class="text-blue-400">packages:</span> [<span class="text-green-400">types</span>, <span class="text-green-400">utils</span>]</code></pre>
						</div>
					</div>

					<div class="bg-yellow-50 border border-yellow-200 rounded-lg p-4">
						<div class="flex items-start space-x-3">
							<Icon icon="solar:info-circle-bold" class="w-5 h-5 text-yellow-600 mt-0.5 flex-shrink-0" />
							<div class="text-sm">
								<p class="font-medium text-yellow-800 mb-1">Supported Alias Prefixes</p>
								<ul class="text-yellow-700 space-y-1">
									<li>• <code>@</code> - Most common, widely used (e.g., <code>@/utils</code>)</li>
									<li>• <code>#</code> - Alternative prefix (e.g., <code>#/types</code>)</li>
									<li>• <code>~</code> - Legacy support (e.g., <code>~/components</code>)</li>
								</ul>
							</div>
						</div>
					</div>
				</div>
			</div>

			<!-- App Level -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">App-Level Configuration</h3>
				<p class="text-muted-foreground mb-4">
					Override the alias prefix for specific apps, or disable aliases entirely.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">In app.yml</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-blue-400">name:</span> frontend
<span class="text-blue-400">ts_alias:</span> <span class="text-green-400">"#"</span>  <span class="text-gray-500"># Override project default</span>
<span class="text-blue-400">packages:</span> [<span class="text-green-400">types</span>, <span class="text-green-400">utils</span>, <span class="text-green-400">ui-components</span>]</code></pre>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Disable Aliases</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-blue-400">name:</span> legacy-app
<span class="text-blue-400">ts_alias:</span> <span class="text-red-400">false</span>  <span class="text-gray-500"># Disable aliases for this app</span>
<span class="text-blue-400">packages:</span> [<span class="text-green-400">types</span>]</code></pre>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Usage Examples -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Usage Examples</h2>

		<div class="space-y-8">
			<!-- Before and After -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">Before vs After</h3>
				<p class="text-muted-foreground mb-4">
					See how aliases make your imports cleaner and more maintainable.
				</p>

				<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
					<div>
						<h4 class="font-medium mb-2 flex items-center">
							<Icon icon="solar:close-circle-bold" class="w-4 h-4 mr-2 text-red-500" />
							Without Aliases
						</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-purple-400">import</span> { <span class="text-yellow-400">User</span> } <span class="text-purple-400">from</span> <span class="text-green-400">'../../packages/types'</span>;
<span class="text-purple-400">import</span> { <span class="text-yellow-400">formatDate</span> } <span class="text-purple-400">from</span> <span class="text-green-400">'../../packages/utils'</span>;
<span class="text-purple-400">import</span> { <span class="text-yellow-400">Button</span> } <span class="text-purple-400">from</span> <span class="text-green-400">'../../../packages/ui-components'</span>;</code></pre>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2 flex items-center">
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 mr-2 text-green-500" />
							With Aliases
						</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-purple-400">import</span> { <span class="text-yellow-400">User</span> } <span class="text-purple-400">from</span> <span class="text-green-400">'@/types'</span>;
<span class="text-purple-400">import</span> { <span class="text-yellow-400">formatDate</span> } <span class="text-purple-400">from</span> <span class="text-green-400">'@/utils'</span>;
<span class="text-purple-400">import</span> { <span class="text-yellow-400">Button</span> } <span class="text-purple-400">from</span> <span class="text-green-400">'@/ui-components'</span>;</code></pre>
						</div>
					</div>
				</div>
			</div>

			<!-- Different Prefixes -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">Different Prefix Examples</h3>
				<p class="text-muted-foreground mb-4">
					Examples of the same imports using different alias prefixes.
				</p>

				<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
					<div>
						<h4 class="font-medium mb-2">@ Prefix</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-purple-400">import</span> <span class="text-green-400">'@/types'</span>
<span class="text-purple-400">import</span> <span class="text-green-400">'@/utils'</span>
<span class="text-purple-400">import</span> <span class="text-green-400">'@/components'</span></code></pre>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2"># Prefix</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-purple-400">import</span> <span class="text-green-400">'#/types'</span>
<span class="text-purple-400">import</span> <span class="text-green-400">'#/utils'</span>
<span class="text-purple-400">import</span> <span class="text-green-400">'#/components'</span></code></pre>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">~ Prefix</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-purple-400">import</span> <span class="text-green-400">'~/types'</span>
<span class="text-purple-400">import</span> <span class="text-green-400">'~/utils'</span>
<span class="text-purple-400">import</span> <span class="text-green-400">'~/components'</span></code></pre>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Automatic Configuration -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Automatic Configuration</h2>

		<div class="space-y-8">
			<!-- tsconfig.json -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">TypeScript Configuration</h3>
				<p class="text-muted-foreground mb-4">
					Knot automatically updates your <code>tsconfig.json</code> with the correct path mappings when you run <code>knot link</code>.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Generated tsconfig.json paths</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code>{
  <span class="text-blue-400">"compilerOptions"</span>: {
    <span class="text-blue-400">"baseUrl"</span>: <span class="text-green-400">"."</span>,
    <span class="text-blue-400">"paths"</span>: {
      <span class="text-green-400">"@/types"</span>: [<span class="text-green-400">"./knot_packages/types"</span>],
      <span class="text-green-400">"@/utils"</span>: [<span class="text-green-400">"./knot_packages/utils"</span>],
      <span class="text-green-400">"@/ui-components"</span>: [<span class="text-green-400">"./knot_packages/ui-components"</span>]
    }
  }
}</code></pre>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Update aliases</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot link</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot link')}
							>
								{#if showCopied && copyText === 'knot link'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
						<div class="mt-2 text-sm text-muted-foreground">
							Automatically updates TypeScript path mappings
						</div>
					</div>
				</div>
			</div>

			<!-- IDE Support -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">IDE Support</h3>
				<p class="text-muted-foreground mb-4">
					Once configured, your IDE will provide full support for aliases including auto-completion, go-to-definition, and refactoring.
				</p>

				<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
					<div class="space-y-3">
						<h4 class="font-medium flex items-center">
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 mr-2 text-green-500" />
							Supported Features
						</h4>
						<ul class="text-sm text-muted-foreground space-y-1">
							<li>• Auto-completion for package names</li>
							<li>• Go-to-definition (Ctrl/Cmd + Click)</li>
							<li>• Find all references</li>
							<li>• Automatic import suggestions</li>
							<li>• Refactoring support</li>
						</ul>
					</div>

					<div class="space-y-3">
						<h4 class="font-medium flex items-center">
							<Icon icon="solar:code-bold" class="w-4 h-4 mr-2 text-blue-500" />
							Tested IDEs
						</h4>
						<ul class="text-sm text-muted-foreground space-y-1">
							<li>• Visual Studio Code</li>
							<li>• WebStorm / IntelliJ IDEA</li>
							<li>• Vim with TypeScript LSP</li>
							<li>• Neovim with TypeScript support</li>
							<li>• Any LSP-compatible editor</li>
						</ul>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Framework Integration -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Framework Integration</h2>

		<div class="space-y-8">
			<!-- React/Next.js -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">React & Next.js</h3>
				<p class="text-muted-foreground mb-4">
					React and Next.js automatically respect TypeScript path mappings from <code>tsconfig.json</code>.
				</p>
				<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
					<pre><code><span class="text-gray-500">// Works out of the box</span>
<span class="text-purple-400">import</span> { <span class="text-yellow-400">UserCard</span> } <span class="text-purple-400">from</span> <span class="text-green-400">'@/ui-components'</span>;
<span class="text-purple-400">import</span> { <span class="text-yellow-400">api</span> } <span class="text-purple-400">from</span> <span class="text-green-400">'@/utils'</span>;</code></pre>
				</div>
			</div>

			<!-- Vite -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">Vite</h3>
				<p class="text-muted-foreground mb-4">
					For Vite projects, you may need to add alias configuration to <code>vite.config.js</code>.
				</p>
				<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
					<pre><code><span class="text-purple-400">import</span> { <span class="text-yellow-400">defineConfig</span> } <span class="text-purple-400">from</span> <span class="text-green-400">'vite'</span>;
<span class="text-purple-400">import</span> <span class="text-yellow-400">path</span> <span class="text-purple-400">from</span> <span class="text-green-400">'path'</span>;

<span class="text-purple-400">export default</span> <span class="text-yellow-400">defineConfig</span>({
  <span class="text-blue-400">resolve:</span> {
    <span class="text-blue-400">alias:</span> {
      <span class="text-green-400">'@'</span>: <span class="text-yellow-400">path.resolve</span>(__dirname, <span class="text-green-400">'./knot_packages'</span>)
    }
  }
});</code></pre>
				</div>
			</div>

			<!-- SvelteKit -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">SvelteKit</h3>
				<p class="text-muted-foreground mb-4">
					Configure aliases in <code>svelte.config.js</code> for SvelteKit projects.
				</p>
				<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
					<pre><code><span class="text-purple-400">import</span> <span class="text-yellow-400">adapter</span> <span class="text-purple-400">from</span> <span class="text-green-400">'@sveltejs/adapter-auto'</span>;
<span class="text-purple-400">import</span> { <span class="text-yellow-400">vitePreprocess</span> } <span class="text-purple-400">from</span> <span class="text-green-400">'@sveltejs/kit/vite'</span>;

<span class="text-purple-400">export default</span> {
  <span class="text-blue-400">kit:</span> {
    <span class="text-blue-400">adapter:</span> <span class="text-yellow-400">adapter</span>(),
    <span class="text-blue-400">alias:</span> {
      <span class="text-green-400">'@/*'</span>: <span class="text-green-400">'./knot_packages/*'</span>
    }
  },
  <span class="text-blue-400">preprocess:</span> <span class="text-yellow-400">vitePreprocess</span>()
};</code></pre>
				</div>
			</div>
		</div>
	</section>

	<!-- Troubleshooting -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Troubleshooting</h2>

		<div class="space-y-6">
			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="solar:danger-triangle-bold" class="w-5 h-5 mr-2 text-red-500" />
					Aliases Not Working
				</h3>
				<ul class="text-sm text-muted-foreground space-y-2">
					<li>• Run <code>knot link</code> to regenerate TypeScript paths</li>
					<li>• Check that <code>tsconfig.json</code> exists in your app directory</li>
					<li>• Restart your TypeScript service in your IDE</li>
					<li>• Verify the alias prefix in your configuration</li>
				</ul>
			</div>

			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="solar:refresh-bold" class="w-5 h-5 mr-2 text-orange-500" />
					Force Update Aliases
				</h3>
				<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
					<code>knot link --force</code>
					<button
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
						on:click={() => copyToClipboard('knot link --force')}
					>
						{#if showCopied && copyText === 'knot link --force'}
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="solar:copy-bold" class="w-4 h-4" />
						{/if}
					</button>
				</div>
				<div class="mt-2 text-sm text-muted-foreground">
					Forces regeneration of all TypeScript path mappings
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="solar:shield-warning-bold" class="w-5 h-5 mr-2 text-yellow-500" />
					Build Tool Configuration
				</h3>
				<ul class="text-sm text-muted-foreground space-y-2">
					<li>• Some build tools require additional alias configuration</li>
					<li>• Check your bundler's documentation for path alias setup</li>
					<li>• Ensure build tool aliases match TypeScript paths</li>
					<li>• Test aliases work in both development and production builds</li>
				</ul>
			</div>
		</div>
	</section>

	<!-- Next Steps -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Next Steps</h2>
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<a href="/docs/typescript" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:code-bold" class="w-6 h-6 text-blue-600" />
					<h3 class="font-semibold">TypeScript Integration</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Learn more about TypeScript configuration and best practices with Knot.
				</p>
			</a>

			<a href="/docs/cli-project" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:folder-bold" class="w-6 h-6 text-green-600" />
					<h3 class="font-semibold">Project Structure</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Understand how to organize your monorepo structure effectively.
				</p>
			</a>

			<a href="/docs/workflows" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:programming-bold" class="w-6 h-6 text-purple-600" />
					<h3 class="font-semibold">Development Workflows</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Optimize your development workflow with aliases and linking strategies.
				</p>
			</a>

			<a href="/docs/troubleshooting" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:help-bold" class="w-6 h-6 text-red-600" />
					<h3 class="font-semibold">Troubleshooting</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Get help with common TypeScript and alias configuration issues.
				</p>
			</a>
		</div>
	</section>
</div>