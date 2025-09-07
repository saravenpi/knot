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
	<title>Package Aliases - Knot CLI Documentation</title>
	<meta name="description" content="Learn how to configure and use package aliases with Knot CLI for cleaner imports and better developer experience." />
	<meta property="og:title" content="Package Aliases - Knot CLI" />
	<meta property="og:description" content="Learn how to configure and use package aliases with Knot CLI for cleaner imports and better developer experience." />
	<meta property="og:image" content="/images/og/typescript.png" />
	<meta property="og:url" content="https://knot.klysium.com/docs/aliases" />
	<meta name="twitter:title" content="Package Aliases - Knot CLI" />
	<meta name="twitter:description" content="Learn how to configure and use package aliases with Knot CLI for cleaner imports and better developer experience." />
	<meta name="twitter:image" content="/images/og/typescript.png" />
	<link rel="canonical" href="https://knot.klysium.com/docs/aliases" />
</svelte:head>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight mb-4" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
			Package Aliases
		</h1>
		<p class="text-lg text-muted-foreground">
			Configure package aliases for cleaner imports and better developer experience
		</p>
	</div>

	<!-- Overview -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Overview</h2>
		<div class="bg-blue-50 border border-blue-200 rounded-lg p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="lucide:code" class="w-6 h-6 text-blue-600 mt-1 flex-shrink-0" />
				<div>
					<h3 class="font-semibold text-blue-900 mb-2">What are Package Aliases?</h3>
					<p class="text-sm text-blue-700 mb-4">
						Package aliases allow you to create custom names for your packages, making imports cleaner and more semantic. Instead of using long package names like <code>user-management-service</code>, you can use a simple alias like <code>users</code>. Knot automatically configures TypeScript path mappings for your aliased packages.
					</p>
					<div class="space-y-2 text-sm text-blue-700">
						<div class="flex items-center space-x-2">
							<Icon icon="lucide:check-circle" class="w-4 h-4 text-green-600" />
							<span>Custom package names for imports</span>
						</div>
						<div class="flex items-center space-x-2">
							<Icon icon="lucide:check-circle" class="w-4 h-4 text-green-600" />
							<span>Semantic and readable import paths</span>
						</div>
						<div class="flex items-center space-x-2">
							<Icon icon="lucide:check-circle" class="w-4 h-4 text-green-600" />
							<span>Automatic TypeScript configuration</span>
						</div>
						<div class="flex items-center space-x-2">
							<Icon icon="lucide:check-circle" class="w-4 h-4 text-green-600" />
							<span>IDE auto-completion and navigation</span>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Package-Specific Aliases -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Package-Specific Aliases</h2>
		<p class="text-muted-foreground mb-6">
			The most powerful feature of Knot aliases is the ability to assign custom names to individual packages, making your imports more semantic and maintainable.
		</p>

		<div class="space-y-8">
			<!-- Basic Syntax -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">Basic Alias Syntax</h3>
				<p class="text-muted-foreground mb-4">
					Assign custom aliases to packages in your configuration files.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">In knot.yml</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-blue-400">apps:</span>
  <span class="text-blue-400">my-app:</span>
    <span class="text-blue-400">packages:</span>
      - <span class="text-blue-400">name:</span> <span class="text-green-400">user-management-service</span>
        <span class="text-blue-400">as:</span> <span class="text-yellow-400">users</span>
      - <span class="text-blue-400">name:</span> <span class="text-green-400">"@company/shared-components@^2.0.0"</span>
        <span class="text-blue-400">as:</span> <span class="text-yellow-400">ui</span>
      - <span class="text-green-400">simple-package</span>  <span class="text-gray-500"># No alias</span></code></pre>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">In app.yml</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-blue-400">name:</span> frontend
<span class="text-blue-400">packages:</span>
  - <span class="text-blue-400">name:</span> <span class="text-green-400">database-models</span>
    <span class="text-blue-400">as:</span> <span class="text-yellow-400">models</span>
  - <span class="text-blue-400">name:</span> <span class="text-green-400">ui-component-library</span>
    <span class="text-blue-400">as:</span> <span class="text-yellow-400">components</span></code></pre>
						</div>
					</div>
				</div>
			</div>

			<!-- Before and After Example -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">Before vs After</h3>
				<p class="text-muted-foreground mb-4">
					See how package aliases transform your import statements.
				</p>

				<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
					<div>
						<h4 class="font-medium mb-2 flex items-center">
							<Icon icon="lucide:x-circle" class="w-4 h-4 mr-2 text-red-500" />
							Without Aliases
						</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-purple-400">import</span> {"{ "}<span class="text-yellow-400">User</span> {"} "}<span class="text-purple-400">from</span> <span class="text-green-400">'#/user-management-service'</span>;
<span class="text-purple-400">import</span> {"{ "}<span class="text-yellow-400">Button</span> {"} "}<span class="text-purple-400">from</span> <span class="text-green-400">'#/shared-components'</span>;
<span class="text-purple-400">import</span> {"{ "}<span class="text-yellow-400">ProductModel</span> {"} "}<span class="text-purple-400">from</span> <span class="text-green-400">'#/database-models'</span>;</code></pre>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2 flex items-center">
							<Icon icon="lucide:check-circle" class="w-4 h-4 mr-2 text-green-500" />
							With Aliases
						</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-purple-400">import</span> {"{ "}<span class="text-yellow-400">User</span> {"} "}<span class="text-purple-400">from</span> <span class="text-green-400">'#users'</span>;
<span class="text-purple-400">import</span> {"{ "}<span class="text-yellow-400">Button</span> {"} "}<span class="text-purple-400">from</span> <span class="text-green-400">'#ui'</span>;
<span class="text-purple-400">import</span> {"{ "}<span class="text-yellow-400">ProductModel</span> {"} "}<span class="text-purple-400">from</span> <span class="text-green-400">'#models'</span>;</code></pre>
						</div>
					</div>
				</div>
			</div>

			<!-- TypeScript Integration -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">TypeScript Integration</h3>
				<p class="text-muted-foreground mb-4">
					Knot automatically generates TypeScript path mappings for your aliases.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Generated tsconfig.json</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code>{"{"}<br/>  <span class="text-blue-400">"compilerOptions"</span>: {"{"}<br/>    <span class="text-blue-400">"paths"</span>: {"{"}<br/>      <span class="text-green-400">"#users/*"</span>: [<span class="text-green-400">"./knot_packages/users/*"</span>],<br/>      <span class="text-green-400">"#ui/*"</span>: [<span class="text-green-400">"./knot_packages/ui/*"</span>],<br/>      <span class="text-green-400">"#models/*"</span>: [<span class="text-green-400">"./knot_packages/models/*"</span>]<br/>    {"}"}<br/>  {"}"}<br/>{"}"}</code></pre>
						</div>
					</div>

					<div class="bg-yellow-50 border border-yellow-200 rounded-lg p-4">
						<div class="flex items-start space-x-3">
							<Icon icon="lucide:lightbulb" class="w-5 h-5 text-yellow-600 mt-0.5 flex-shrink-0" />
							<div class="text-sm">
								<p class="font-medium text-yellow-800 mb-1">Directory Structure</p>
								<p class="text-yellow-700">
									Packages are linked using their alias names in the <code>knot_packages/</code> directory, so <code>user-management-service</code> becomes <code>knot_packages/users/</code>.
								</p>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Two Types of Aliases -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Two Types of Aliases</h2>
		<p class="text-muted-foreground mb-6">
			Knot supports two complementary aliasing systems that work together to provide maximum flexibility.
		</p>

		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="lucide:hash" class="w-5 h-5 mr-2 text-blue-600" />
					TypeScript Path Aliases
				</h3>
				<p class="text-sm text-muted-foreground mb-3">
					Set a global prefix for all packages (e.g., <code>@</code>, <code>#</code>, <code>~</code>).
				</p>
				<div class="bg-black/90 text-white font-mono text-sm p-3 rounded">
					<pre><code><span class="text-blue-400">ts_alias:</span> <span class="text-green-400">"@"</span>
<span class="text-gray-400"># Results in: @/package-name</span></code></pre>
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="lucide:tag" class="w-5 h-5 mr-2 text-purple-600" />
					Package-Specific Aliases
				</h3>
				<p class="text-sm text-muted-foreground mb-3">
					Assign custom names to individual packages for semantic imports.
				</p>
				<div class="bg-black/90 text-white font-mono text-sm p-3 rounded">
					<pre><code><span class="text-blue-400">name:</span> <span class="text-green-400">long-package-name</span>
<span class="text-blue-400">as:</span> <span class="text-yellow-400">short</span>
<span class="text-gray-400"># Results in: @/short</span></code></pre>
				</div>
			</div>
		</div>

		<div class="mt-6 bg-green-50 border border-green-200 rounded-lg p-4">
			<div class="flex items-start space-x-3">
				<Icon icon="lucide:check-circle" class="w-5 h-5 text-green-600 mt-0.5 flex-shrink-0" />
				<div class="text-sm">
					<h4 class="font-medium text-green-900 mb-1">Best of Both Worlds</h4>
					<p class="text-green-700">
						You can use both systems together! Set a <code>ts_alias</code> for consistency, then use package-specific aliases for semantic naming: <code>@/users</code>, <code>@/api</code>, <code>@/components</code>.
					</p>
				</div>
			</div>
		</div>
	</section>

	<!-- TypeScript Path Configuration -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">TypeScript Path Configuration</h2>

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
    <span class="text-blue-400">packages:</span>
      - <span class="text-green-400">types</span>
      - <span class="text-green-400">utils</span></code></pre>
						</div>
					</div>

					<div class="bg-yellow-50 border border-yellow-200 rounded-lg p-4">
						<div class="flex items-start space-x-3">
							<Icon icon="lucide:info" class="w-5 h-5 text-yellow-600 mt-0.5 flex-shrink-0" />
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
<span class="text-blue-400">packages:</span>
  - <span class="text-green-400">types</span>
  - <span class="text-green-400">utils</span>
  - <span class="text-green-400">ui-components</span></code></pre>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Disable Aliases</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-blue-400">name:</span> legacy-app
<span class="text-blue-400">ts_alias:</span> <span class="text-red-400">false</span>  <span class="text-gray-500"># Disable aliases for this app</span>
<span class="text-blue-400">packages:</span>
  - <span class="text-green-400">types</span></code></pre>
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
							<Icon icon="lucide:x-circle" class="w-4 h-4 mr-2 text-red-500" />
							Without Aliases
						</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-purple-400">import</span> {"{"} <span class="text-yellow-400">User</span> {"}"} <span class="text-purple-400">from</span> <span class="text-green-400">&apos;../../packages/types&apos;</span>;
<span class="text-purple-400">import</span> {"{"} <span class="text-yellow-400">formatDate</span> {"}"} <span class="text-purple-400">from</span> <span class="text-green-400">&apos;../../packages/utils&apos;</span>;
<span class="text-purple-400">import</span> {"{"} <span class="text-yellow-400">Button</span> {"}"} <span class="text-purple-400">from</span> <span class="text-green-400">&apos;../../../packages/ui-components&apos;</span>;</code></pre>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2 flex items-center">
							<Icon icon="lucide:check-circle" class="w-4 h-4 mr-2 text-green-500" />
							With Aliases
						</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-purple-400">import</span> {"{"} <span class="text-yellow-400">User</span> {"}"} <span class="text-purple-400">from</span> <span class="text-green-400">&apos;@/types&apos;</span>;
<span class="text-purple-400">import</span> {"{"} <span class="text-yellow-400">formatDate</span> {"}"} <span class="text-purple-400">from</span> <span class="text-green-400">&apos;@/utils&apos;</span>;
<span class="text-purple-400">import</span> {"{"} <span class="text-yellow-400">Button</span> {"}"} <span class="text-purple-400">from</span> <span class="text-green-400">&apos;@/ui-components&apos;</span>;</code></pre>
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
							<pre><code><span class="text-purple-400">import</span> <span class="text-green-400">&apos;@/types&apos;</span>
<span class="text-purple-400">import</span> <span class="text-green-400">&apos;@/utils&apos;</span>
<span class="text-purple-400">import</span> <span class="text-green-400">&apos;@/components&apos;</span></code></pre>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2"># Prefix</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-purple-400">import</span> <span class="text-green-400">&apos;#/types&apos;</span>
<span class="text-purple-400">import</span> <span class="text-green-400">&apos;#/utils&apos;</span>
<span class="text-purple-400">import</span> <span class="text-green-400">&apos;#/components&apos;</span></code></pre>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">~ Prefix</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-purple-400">import</span> <span class="text-green-400">&apos;~/types&apos;</span>
<span class="text-purple-400">import</span> <span class="text-green-400">&apos;~/utils&apos;</span>
<span class="text-purple-400">import</span> <span class="text-green-400">&apos;~/components&apos;</span></code></pre>
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
							<pre><code>{"{"}<br/>  <span class="text-blue-400">&quot;compilerOptions&quot;</span>: {"{"}<br/>    <span class="text-blue-400">&quot;baseUrl&quot;</span>: <span class="text-green-400">&quot;.&quot;</span>,<br/>    <span class="text-blue-400">&quot;paths&quot;</span>: {"{"}<br/>      <span class="text-green-400">&quot;@/types&quot;</span>: [<span class="text-green-400">&quot;./knot_packages/types&quot;</span>],<br/>      <span class="text-green-400">&quot;@/utils&quot;</span>: [<span class="text-green-400">&quot;./knot_packages/utils&quot;</span>],<br/>      <span class="text-green-400">&quot;@/ui-components&quot;</span>: [<span class="text-green-400">&quot;./knot_packages/ui-components&quot;</span>]<br/>    {"}"}<br/>  {"}"}<br/>{"}"}</code></pre>
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
									<Icon icon="lucide:check-circle" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="lucide:copy" class="w-4 h-4" />
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
							<Icon icon="lucide:check-circle" class="w-4 h-4 mr-2 text-green-500" />
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
							<Icon icon="lucide:code" class="w-4 h-4 mr-2 text-blue-500" />
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
<span class="text-purple-400">import</span> {"{ "}<span class="text-yellow-400">UserCard</span> {"} "}<span class="text-purple-400">from</span> <span class="text-green-400">&apos;@/ui-components&apos;</span>;
<span class="text-purple-400">import</span> {"{ "}<span class="text-yellow-400">api</span> {"} "}<span class="text-purple-400">from</span> <span class="text-green-400">&apos;@/utils&apos;</span>;</code></pre>
				</div>
			</div>

			<!-- Vite -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">Vite</h3>
				<p class="text-muted-foreground mb-4">
					For Vite projects, you may need to add alias configuration to <code>vite.config.js</code>.
				</p>
				<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
					<pre><code><span class="text-purple-400">import</span> {"{ "}<span class="text-yellow-400">defineConfig</span> {"} "}<span class="text-purple-400">from</span> <span class="text-green-400">&apos;vite&apos;</span>;
<span class="text-purple-400">import</span> <span class="text-yellow-400">path</span> <span class="text-purple-400">from</span> <span class="text-green-400">&apos;path&apos;</span>;

<span class="text-purple-400">export default</span> <span class="text-yellow-400">defineConfig</span>({"{"}<br/>  <span class="text-blue-400">resolve:</span> {"{"}<br/>    <span class="text-blue-400">alias:</span> {"{"}<br/>      <span class="text-green-400">&apos;@&apos;</span>: <span class="text-yellow-400">path.resolve</span>(__dirname, <span class="text-green-400">&apos;./knot_packages&apos;</span>)<br/>    {"}"}<br/>  {"}"}<br/>{"}"});</code></pre>
				</div>
			</div>

			<!-- SvelteKit -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">SvelteKit</h3>
				<p class="text-muted-foreground mb-4">
					Configure aliases in <code>svelte.config.js</code> for SvelteKit projects.
				</p>
				<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
					<pre><code><span class="text-purple-400">import</span> <span class="text-yellow-400">adapter</span> <span class="text-purple-400">from</span> <span class="text-green-400">&apos;@sveltejs/adapter-auto&apos;</span>;
<span class="text-purple-400">import</span> {"{ "}<span class="text-yellow-400">vitePreprocess</span> {"} "}<span class="text-purple-400">from</span> <span class="text-green-400">&apos;@sveltejs/kit/vite&apos;</span>;

<span class="text-purple-400">export default</span> {"{"}<br/>  <span class="text-blue-400">kit:</span> {"{"}<br/>    <span class="text-blue-400">adapter:</span> <span class="text-yellow-400">adapter</span>(),<br/>    <span class="text-blue-400">alias:</span> {"{"}<br/>      <span class="text-green-400">&apos;@/*&apos;</span>: <span class="text-green-400">&apos;./knot_packages/*&apos;</span><br/>    {"}"}<br/>  {"}"},<br/>  <span class="text-blue-400">preprocess:</span> <span class="text-yellow-400">vitePreprocess</span>()<br/>{"};"};</code></pre>
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
					<Icon icon="lucide:danger-triangle-bold" class="w-5 h-5 mr-2 text-red-500" />
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
					<Icon icon="lucide:refresh-bold" class="w-5 h-5 mr-2 text-orange-500" />
					Force Update Aliases
				</h3>
				<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
					<code>knot link --force</code>
					<button
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
						on:click={() => copyToClipboard('knot link --force')}
					>
						{#if showCopied && copyText === 'knot link --force'}
							<Icon icon="lucide:check-circle" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="lucide:copy" class="w-4 h-4" />
						{/if}
					</button>
				</div>
				<div class="mt-2 text-sm text-muted-foreground">
					Forces regeneration of all TypeScript path mappings
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="lucide:shield-warning-bold" class="w-5 h-5 mr-2 text-yellow-500" />
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

	<!-- Package Types and Aliases -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Package Types & Alias Examples</h2>
		
		<div class="space-y-8">
			<!-- Local Packages -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-4 flex items-center">
					<Icon icon="lucide:home" class="w-5 h-5 mr-2 text-blue-600" />
					Local Packages
				</h3>
				<p class="text-muted-foreground mb-4">
					Local packages stored in your <code>packages/</code> directory can be easily accessed with aliases.
				</p>
				
				<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
					<div>
						<h4 class="font-medium mb-2">Configuration</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-blue-400">apps:</span>
  <span class="text-blue-400">frontend:</span>
    <span class="text-blue-400">ts_alias:</span> <span class="text-green-400">"@"</span>
    <span class="text-blue-400">packages:</span>
      - <span class="text-yellow-400">types</span>
      - <span class="text-yellow-400">utils</span>
      - <span class="text-yellow-400">ui-components</span></code></pre>
						</div>
					</div>
					
					<div>
						<h4 class="font-medium mb-2">Usage in TypeScript</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-purple-400">import</span> {"{ "}<span class="text-yellow-400">User</span> {"} "}<span class="text-purple-400">from</span> <span class="text-green-400">'@/types'</span>;
<span class="text-purple-400">import</span> {"{ "}<span class="text-yellow-400">formatDate</span> {"} "}<span class="text-purple-400">from</span> <span class="text-green-400">'@/utils'</span>;
<span class="text-purple-400">import</span> {"{ "}<span class="text-yellow-400">Button</span> {"} "}<span class="text-purple-400">from</span> <span class="text-green-400">'@/ui-components'</span>;</code></pre>
						</div>
					</div>
				</div>
			</div>

			<!-- Team Packages -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-4 flex items-center">
					<Icon icon="lucide:users" class="w-5 h-5 mr-2 text-purple-600" />
					Team Packages
				</h3>
				<p class="text-muted-foreground mb-4">
					Team packages from your organization registry can be aliased for cleaner imports.
				</p>
				
				<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
					<div>
						<h4 class="font-medium mb-2">Configuration</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-blue-400">apps:</span>
  <span class="text-blue-400">api:</span>
    <span class="text-blue-400">ts_alias:</span> <span class="text-green-400">"#"</span>
    <span class="text-blue-400">packages:</span>
      - <span class="text-yellow-400">"@mycompany/shared-lib"</span>
      - <span class="text-yellow-400">"@mycompany/database"</span>
      - <span class="text-yellow-400">"@myteam/validators"</span></code></pre>
						</div>
					</div>
					
					<div>
						<h4 class="font-medium mb-2">Aliased Imports</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-purple-400">import</span> {"{ "}<span class="text-yellow-400">ApiClient</span> {"} "}<span class="text-purple-400">from</span> <span class="text-green-400">'#/shared-lib'</span>;
<span class="text-purple-400">import</span> {"{ "}<span class="text-yellow-400">Database</span> {"} "}<span class="text-purple-400">from</span> <span class="text-green-400">'#/database'</span>;
<span class="text-purple-400">import</span> {"{ "}<span class="text-yellow-400">validate</span> {"} "}<span class="text-purple-400">from</span> <span class="text-green-400">'#/validators'</span>;</code></pre>
						</div>
					</div>
				</div>
			</div>

			<!-- Online Packages -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-4 flex items-center">
					<Icon icon="lucide:cloud" class="w-5 h-5 mr-2 text-green-600" />
					Online Registry Packages
				</h3>
				<p class="text-muted-foreground mb-4">
					Published packages from the Knot registry benefit from aliases for consistent organization.
				</p>
				
				<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
					<div>
						<h4 class="font-medium mb-2">Configuration</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-blue-400">apps:</span>
  <span class="text-blue-400">webapp:</span>
    <span class="text-blue-400">ts_alias:</span> <span class="text-green-400">"~"</span>
    <span class="text-blue-400">packages:</span>
      - <span class="text-yellow-400">"@community/ui-kit"</span>
      - <span class="text-yellow-400">"@external/charts"</span>
      - <span class="text-yellow-400">"@opensource/helpers"</span></code></pre>
						</div>
					</div>
					
					<div>
						<h4 class="font-medium mb-2">Clean Imports</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-purple-400">import</span> {"{ "}<span class="text-yellow-400">Theme</span> {"} "}<span class="text-purple-400">from</span> <span class="text-green-400">'~/ui-kit'</span>;
<span class="text-purple-400">import</span> {"{ "}<span class="text-yellow-400">LineChart</span> {"} "}<span class="text-purple-400">from</span> <span class="text-green-400">'~/charts'</span>;
<span class="text-purple-400">import</span> {"{ "}<span class="text-yellow-400">debounce</span> {"} "}<span class="text-purple-400">from</span> <span class="text-green-400">'~/helpers'</span>;</code></pre>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Semantic Alias Patterns -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Semantic Alias Patterns</h2>
		<p class="text-muted-foreground mb-6">
			Create meaningful aliases that reflect functionality rather than package names for better developer experience.
		</p>
		
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="lucide:database" class="w-5 h-5 mr-2 text-blue-600" />
					Functional Aliases
				</h3>
				<div class="space-y-3">
					<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
						<pre><code><span class="text-gray-400"># Create semantic mappings</span>
<span class="text-blue-400">apps:</span>
  <span class="text-blue-400">backend:</span>
    <span class="text-blue-400">ts_alias:</span> <span class="text-green-400">"@"</span>
    <span class="text-blue-400">packages:</span>
      - <span class="text-gray-400"># Database layer</span>
      - <span class="text-yellow-400">database-models</span>  <span class="text-gray-400"># becomes @/database-models</span>
      - <span class="text-gray-400"># API utilities</span>
      - <span class="text-yellow-400">api-helpers</span>     <span class="text-gray-400"># becomes @/api-helpers</span>
      - <span class="text-gray-400"># Authentication</span>
      - <span class="text-yellow-400">auth-utils</span>      <span class="text-gray-400"># becomes @/auth-utils</span></code></pre>
					</div>
					
					<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
						<pre><code><span class="text-gray-400">// Usage reflects purpose</span>
<span class="text-purple-400">import</span> {"{ "}<span class="text-yellow-400">User</span> {"} "}<span class="text-purple-400">from</span> <span class="text-green-400">'@/database-models'</span>;
<span class="text-purple-400">import</span> {"{ "}<span class="text-yellow-400">validateRequest</span> {"} "}<span class="text-purple-400">from</span> <span class="text-green-400">'@/api-helpers'</span>;
<span class="text-purple-400">import</span> {"{ "}<span class="text-yellow-400">authenticate</span> {"} "}<span class="text-purple-400">from</span> <span class="text-green-400">'@/auth-utils'</span>;</code></pre>
					</div>
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="lucide:layers" class="w-5 h-5 mr-2 text-purple-600" />
					Architectural Aliases
				</h3>
				<div class="space-y-3">
					<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
						<pre><code><span class="text-gray-400"># Organize by architecture layers</span>
<span class="text-blue-400">apps:</span>
  <span class="text-blue-400">frontend:</span>
    <span class="text-blue-400">ts_alias:</span> <span class="text-green-400">"@"</span>
    <span class="text-blue-400">packages:</span>
      - <span class="text-yellow-400">domain-models</span>    <span class="text-gray-400"># Domain layer</span>
      - <span class="text-yellow-400">ui-components</span>    <span class="text-gray-400"># Presentation layer</span>
      - <span class="text-yellow-400">data-services</span>    <span class="text-gray-400"># Data access layer</span>
      - <span class="text-yellow-400">shared-types</span>     <span class="text-gray-400"># Cross-cutting</span></code></pre>
					</div>
					
					<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
						<pre><code><span class="text-gray-400">// Clear architectural boundaries</span>
<span class="text-purple-400">import</span> {"{ "}<span class="text-yellow-400">Product</span> {"} "}<span class="text-purple-400">from</span> <span class="text-green-400">'@/domain-models'</span>;
<span class="text-purple-400">import</span> {"{ "}<span class="text-yellow-400">Card</span> {"} "}<span class="text-purple-400">from</span> <span class="text-green-400">'@/ui-components'</span>;
<span class="text-purple-400">import</span> {"{ "}<span class="text-yellow-400">productApi</span> {"} "}<span class="text-purple-400">from</span> <span class="text-green-400">'@/data-services'</span>;</code></pre>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Real-World Scenarios -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Real-World Benefits</h2>
		<p class="text-muted-foreground mb-6">
			Discover practical scenarios where package aliases provide significant developer experience improvements.
		</p>
		
		<div class="space-y-8">
			<!-- Large Monorepo -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-4 flex items-center">
					<Icon icon="lucide:building-2" class="w-5 h-5 mr-2 text-blue-600" />
					Large Enterprise Monorepo
				</h3>
				<p class="text-muted-foreground mb-4">
					In a large monorepo with 50+ packages, aliases become essential for maintainability.
				</p>
				
				<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
					<div>
						<h4 class="font-medium mb-2 flex items-center">
							<Icon icon="lucide:x-circle" class="w-4 h-4 mr-2 text-red-500" />
							Without Aliases
						</h4>
						<div class="bg-red-50 border border-red-200 rounded p-4 text-sm font-mono">
							<pre><code><span class="text-red-700">// Deeply nested imports</span>
<span class="text-purple-400">import</span> {"{"} <span class="text-yellow-400">UserService</span> {"}"} <span class="text-purple-400">from</span> 
  <span class="text-green-400">'../../../packages/backend/user-management'</span>;
<span class="text-purple-400">import</span> {"{"} <span class="text-yellow-400">PaymentGateway</span> {"}"} <span class="text-purple-400">from</span> 
  <span class="text-green-400">'../../../../packages/shared/payment-processing'</span>;
<span class="text-purple-400">import</span> {"{"} <span class="text-yellow-400">NotificationQueue</span> {"}"} <span class="text-purple-400">from</span> 
  <span class="text-green-400">'../../../packages/infrastructure/messaging'</span>;</code></pre>
						</div>
					</div>
					
					<div>
						<h4 class="font-medium mb-2 flex items-center">
							<Icon icon="lucide:check-circle" class="w-4 h-4 mr-2 text-green-500" />
							With Aliases
						</h4>
						<div class="bg-green-50 border border-green-200 rounded p-4 text-sm font-mono">
							<pre><code><span class="text-green-700">// Clean, semantic imports</span>
<span class="text-purple-400">import</span> {"{"} <span class="text-yellow-400">UserService</span> {"}"} <span class="text-purple-400">from</span> <span class="text-green-400">'@/user-management'</span>;
<span class="text-purple-400">import</span> {"{"} <span class="text-yellow-400">PaymentGateway</span> {"}"} <span class="text-purple-400">from</span> <span class="text-green-400">'@/payment-processing'</span>;
<span class="text-purple-400">import</span> {"{"} <span class="text-yellow-400">NotificationQueue</span> {"}"} <span class="text-purple-400">from</span> <span class="text-green-400">'@/messaging'</span>;</code></pre>
						</div>
					</div>
				</div>
			</div>

			<!-- Multi-App Consistency -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-4 flex items-center">
					<Icon icon="lucide:git-branch" class="w-5 h-5 mr-2 text-purple-600" />
					Multi-App Consistency
				</h3>
				<p class="text-muted-foreground mb-4">
					Maintain consistent import patterns across different applications in your monorepo.
				</p>
				
				<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
					<pre><code><span class="text-gray-400"># Consistent aliases across apps</span>
<span class="text-blue-400">apps:</span>
  <span class="text-blue-400">web-admin:</span>
    <span class="text-blue-400">ts_alias:</span> <span class="text-green-400">"@"</span>
    <span class="text-blue-400">packages:</span> [<span class="text-yellow-400">shared-types</span>, <span class="text-yellow-400">ui-kit</span>, <span class="text-yellow-400">api-client</span>]
  
  <span class="text-blue-400">mobile-app:</span>
    <span class="text-blue-400">ts_alias:</span> <span class="text-green-400">"@"</span>
    <span class="text-blue-400">packages:</span> [<span class="text-yellow-400">shared-types</span>, <span class="text-yellow-400">mobile-components</span>, <span class="text-yellow-400">api-client</span>]
  
  <span class="text-blue-400">backend-api:</span>
    <span class="text-blue-400">ts_alias:</span> <span class="text-green-400">"@"</span>
    <span class="text-blue-400">packages:</span> [<span class="text-yellow-400">shared-types</span>, <span class="text-yellow-400">database-models</span>, <span class="text-yellow-400">validators</span>]

<span class="text-gray-400"># Same import pattern everywhere:</span>
<span class="text-gray-400"># import { User } from '@/shared-types';</span></code></pre>
				</div>
			</div>

			<!-- Refactoring Benefits -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-4 flex items-center">
					<Icon icon="lucide:refresh-cw" class="w-5 h-5 mr-2 text-green-600" />
					Easy Refactoring
				</h3>
				<p class="text-muted-foreground mb-4">
					Aliases make code refactoring and package reorganization much simpler.
				</p>
				
				<div class="space-y-4">
					<div class="bg-blue-50 border border-blue-200 rounded-lg p-4">
						<div class="flex items-start space-x-3">
							<Icon icon="lucide:lightbulb" class="w-5 h-5 text-blue-600 mt-0.5 flex-shrink-0" />
							<div class="text-sm">
								<h4 class="font-medium text-blue-900 mb-1">Package Renaming</h4>
								<p class="text-blue-700">
									When you rename a package from <code>user-helpers</code> to <code>user-utils</code>, 
									imports using aliases like <code>@/user-helpers</code> automatically update when you 
									run <code>knot link</code>, without changing any import statements.
								</p>
							</div>
						</div>
					</div>
					
					<div class="bg-green-50 border border-green-200 rounded-lg p-4">
						<div class="flex items-start space-x-3">
							<Icon icon="lucide:move" class="w-5 h-5 text-green-600 mt-0.5 flex-shrink-0" />
							<div class="text-sm">
								<h4 class="font-medium text-green-900 mb-1">Package Migration</h4>
								<p class="text-green-700">
									Moving packages between local and remote registries becomes transparent to consumers 
									when using aliases, as the import paths remain consistent.
								</p>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Advanced Configuration -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Advanced Alias Configuration</h2>
		
		<div class="space-y-8">
			<!-- Per-App Customization -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">Per-App Alias Customization</h3>
				<p class="text-muted-foreground mb-4">
					Configure different alias prefixes for different types of applications to maintain clarity.
				</p>
				
				<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
					<pre><code><span class="text-blue-400">apps:</span>
  <span class="text-blue-400">frontend:</span>
    <span class="text-blue-400">ts_alias:</span> <span class="text-green-400">"@"</span>        <span class="text-gray-400"># Frontend uses @ for UI packages</span>
    <span class="text-blue-400">packages:</span> [<span class="text-yellow-400">ui-components</span>, <span class="text-yellow-400">frontend-utils</span>]
    
  <span class="text-blue-400">backend:</span>
    <span class="text-blue-400">ts_alias:</span> <span class="text-green-400">"#"</span>        <span class="text-gray-400"># Backend uses # for server packages</span>
    <span class="text-blue-400">packages:</span> [<span class="text-yellow-400">database-models</span>, <span class="text-yellow-400">api-utils</span>]
    
  <span class="text-blue-400">mobile:</span>
    <span class="text-blue-400">ts_alias:</span> <span class="text-green-400">"~"</span>        <span class="text-gray-400"># Mobile uses ~ for platform packages</span>
    <span class="text-blue-400">packages:</span> [<span class="text-yellow-400">mobile-components</span>, <span class="text-yellow-400">native-utils</span>]
    
  <span class="text-blue-400">cli:</span>
    <span class="text-blue-400">ts_alias:</span> <span class="text-red-400">false</span>     <span class="text-gray-400"># CLI app disables aliases</span>
    <span class="text-blue-400">packages:</span> [<span class="text-yellow-400">shared-types</span>]</code></pre>
				</div>
			</div>

			<!-- Global vs Local Configuration -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">Global vs App-Level Settings</h3>
				<p class="text-muted-foreground mb-4">
					Set default aliases globally and override them per application as needed.
				</p>
				
				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">knot.yml (Global Default)</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-blue-400">name:</span> my-project
<span class="text-blue-400">ts_alias:</span> <span class="text-green-400">"@"</span>  <span class="text-gray-400"># Default alias for all apps</span>

<span class="text-blue-400">apps:</span>
  <span class="text-blue-400">web:</span>
    <span class="text-gray-400"># Inherits @ from global setting</span>
    <span class="text-blue-400">packages:</span> [<span class="text-yellow-400">ui-components</span>]
    
  <span class="text-blue-400">api:</span>
    <span class="text-blue-400">ts_alias:</span> <span class="text-green-400">"#"</span>  <span class="text-gray-400"># Override global default</span>
    <span class="text-blue-400">packages:</span> [<span class="text-yellow-400">api-utils</span>]</code></pre>
						</div>
					</div>
					
					<div>
						<h4 class="font-medium mb-2">apps/special/app.yml (App Override)</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-blue-400">name:</span> special-app
<span class="text-blue-400">ts_alias:</span> <span class="text-green-400">"$"</span>  <span class="text-gray-400"># App-specific alias prefix</span>
<span class="text-blue-400">packages:</span>
  - <span class="text-yellow-400">special-utils</span>
  - <span class="text-yellow-400">custom-types</span></code></pre>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Practical Examples -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Practical Examples</h2>
		<p class="text-muted-foreground mb-6">
			Real-world examples showing how to use package aliases effectively in different scenarios.
		</p>

		<div class="space-y-8">
			<!-- E-commerce App -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-4 flex items-center">
					<Icon icon="lucide:shopping-cart" class="w-5 h-5 mr-2 text-blue-600" />
					E-commerce Application
				</h3>
				<p class="text-muted-foreground mb-4">
					Large e-commerce app with semantic aliases for different business domains.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Configuration</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-blue-400">apps:</span>
  <span class="text-blue-400">storefront:</span>
    <span class="text-blue-400">ts_alias:</span> <span class="text-green-400">"@"</span>
    <span class="text-blue-400">packages:</span>
      - <span class="text-blue-400">name:</span> <span class="text-green-400">user-management-service</span>
        <span class="text-blue-400">as:</span> <span class="text-yellow-400">users</span>
      - <span class="text-blue-400">name:</span> <span class="text-green-400">product-catalog-engine</span>
        <span class="text-blue-400">as:</span> <span class="text-yellow-400">products</span>
      - <span class="text-blue-400">name:</span> <span class="text-green-400">shopping-cart-logic</span>
        <span class="text-blue-400">as:</span> <span class="text-yellow-400">cart</span>
      - <span class="text-blue-400">name:</span> <span class="text-green-400">payment-processing</span>
        <span class="text-blue-400">as:</span> <span class="text-yellow-400">payments</span>
      - <span class="text-blue-400">name:</span> <span class="text-green-400">"@company/design-system@^3.0.0"</span>
        <span class="text-blue-400">as:</span> <span class="text-yellow-400">ui</span></code></pre>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Usage in Components</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-gray-400">// ProductPage.tsx</span>
<span class="text-purple-400">import</span> {"{ "}<span class="text-yellow-400">User</span> {"} "}<span class="text-purple-400">from</span> <span class="text-green-400">'@users/types'</span>;
<span class="text-purple-400">import</span> {"{ "}<span class="text-yellow-400">Product, searchProducts</span> {"} "}<span class="text-purple-400">from</span> <span class="text-green-400">'@products'</span>;
<span class="text-purple-400">import</span> {"{ "}<span class="text-yellow-400">addToCart</span> {"} "}<span class="text-purple-400">from</span> <span class="text-green-400">'@cart/actions'</span>;
<span class="text-purple-400">import</span> {"{ "}<span class="text-yellow-400">processPayment</span> {"} "}<span class="text-purple-400">from</span> <span class="text-green-400">'@payments/stripe'</span>;
<span class="text-purple-400">import</span> {"{ "}<span class="text-yellow-400">Button, Card</span> {"} "}<span class="text-purple-400">from</span> <span class="text-green-400">'@ui/components'</span>;</code></pre>
						</div>
					</div>
				</div>
			</div>

			<!-- Development Tools -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-4 flex items-center">
					<Icon icon="lucide:wrench" class="w-5 h-5 mr-2 text-purple-600" />
					Development Tools Monorepo
				</h3>
				<p class="text-muted-foreground mb-4">
					Developer tools with different prefixes for different app types.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Multi-App Configuration</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-blue-400">apps:</span>
  <span class="text-blue-400">cli-tool:</span>
    <span class="text-blue-400">ts_alias:</span> <span class="text-green-400">"$"</span>  <span class="text-gray-400"># CLI tools use $ prefix</span>
    <span class="text-blue-400">packages:</span>
      - <span class="text-blue-400">name:</span> <span class="text-green-400">command-line-parser</span>
        <span class="text-blue-400">as:</span> <span class="text-yellow-400">cli</span>
      - <span class="text-blue-400">name:</span> <span class="text-green-400">file-system-utils</span>
        <span class="text-blue-400">as:</span> <span class="text-yellow-400">fs</span>

  <span class="text-blue-400">web-dashboard:</span>
    <span class="text-blue-400">ts_alias:</span> <span class="text-green-400">"@"</span>  <span class="text-gray-400"># Web apps use @ prefix</span>
    <span class="text-blue-400">packages:</span>
      - <span class="text-blue-400">name:</span> <span class="text-green-400">dashboard-components</span>
        <span class="text-blue-400">as:</span> <span class="text-yellow-400">components</span>
      - <span class="text-blue-400">name:</span> <span class="text-green-400">analytics-engine</span>
        <span class="text-blue-400">as:</span> <span class="text-yellow-400">analytics</span>

  <span class="text-blue-400">api-server:</span>
    <span class="text-blue-400">ts_alias:</span> <span class="text-green-400">"#"</span>  <span class="text-gray-400"># Server apps use # prefix</span>
    <span class="text-blue-400">packages:</span>
      - <span class="text-blue-400">name:</span> <span class="text-green-400">database-models</span>
        <span class="text-blue-400">as:</span> <span class="text-yellow-400">db</span>
      - <span class="text-blue-400">name:</span> <span class="text-green-400">authentication-middleware</span>
        <span class="text-blue-400">as:</span> <span class="text-yellow-400">auth</span></code></pre>
						</div>
					</div>

					<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
						<div>
							<h5 class="font-medium mb-2">CLI Tool Imports</h5>
							<div class="bg-black/90 text-white font-mono text-sm p-3 rounded">
								<pre><code><span class="text-purple-400">import</span> <span class="text-green-400">'$cli/commander'</span>
<span class="text-purple-400">import</span> <span class="text-green-400">'$fs/operations'</span></code></pre>
							</div>
						</div>
						<div>
							<h5 class="font-medium mb-2">Web App Imports</h5>
							<div class="bg-black/90 text-white font-mono text-sm p-3 rounded">
								<pre><code><span class="text-purple-400">import</span> <span class="text-green-400">'@components/Chart'</span>
<span class="text-purple-400">import</span> <span class="text-green-400">'@analytics/reports'</span></code></pre>
							</div>
						</div>
						<div>
							<h5 class="font-medium mb-2">Server Imports</h5>
							<div class="bg-black/90 text-white font-mono text-sm p-3 rounded">
								<pre><code><span class="text-purple-400">import</span> <span class="text-green-400">'#db/User'</span>
<span class="text-purple-400">import</span> <span class="text-green-400">'#auth/middleware'</span></code></pre>
							</div>
						</div>
					</div>
				</div>
			</div>

			<!-- Team Workflow -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-4 flex items-center">
					<Icon icon="lucide:users" class="w-5 h-5 mr-2 text-green-600" />
					Team Collaboration Workflow
				</h3>
				<p class="text-muted-foreground mb-4">
					Using aliases for better team collaboration and onboarding.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Semantic Aliases for New Team Members</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-gray-400"># Clear, self-documenting aliases</span>
<span class="text-blue-400">apps:</span>
  <span class="text-blue-400">main-app:</span>
    <span class="text-blue-400">packages:</span>
      - <span class="text-blue-400">name:</span> <span class="text-green-400">customer-relationship-management</span>
        <span class="text-blue-400">as:</span> <span class="text-yellow-400">crm</span>
      - <span class="text-blue-400">name:</span> <span class="text-green-400">inventory-management-system</span>
        <span class="text-blue-400">as:</span> <span class="text-yellow-400">inventory</span>
      - <span class="text-blue-400">name:</span> <span class="text-green-400">financial-reporting-engine</span>
        <span class="text-blue-400">as:</span> <span class="text-yellow-400">finance</span>
      - <span class="text-blue-400">name:</span> <span class="text-green-400">human-resources-portal</span>
        <span class="text-blue-400">as:</span> <span class="text-yellow-400">hr</span></code></pre>
						</div>
					</div>

					<div class="bg-blue-50 border border-blue-200 rounded-lg p-4">
						<div class="flex items-start space-x-3">
							<Icon icon="lucide:lightbulb" class="w-5 h-5 text-blue-600 mt-0.5 flex-shrink-0" />
							<div class="text-sm">
								<h4 class="font-medium text-blue-900 mb-1">Team Benefits</h4>
								<ul class="text-blue-700 space-y-1">
									<li>• New developers immediately understand what <code>@crm</code> and <code>@finance</code> contain</li>
									<li>• Code reviews become faster with semantic import names</li>
									<li>• Domain experts can easily find relevant code sections</li>
									<li>• Package refactoring doesn't break import semantics</li>
								</ul>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Best Practices -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Alias Best Practices</h2>
		
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<div class="space-y-6">
				<div class="border rounded-lg p-6">
					<h3 class="font-semibold mb-3 flex items-center">
						<Icon icon="lucide:check-circle" class="w-5 h-5 mr-2 text-green-600" />
						Naming Conventions
					</h3>
					<ul class="text-sm text-muted-foreground space-y-2">
						<li>• Use consistent alias prefixes across similar apps</li>
						<li>• Choose meaningful package names that reflect functionality</li>
						<li>• Avoid generic names like "utils" or "helpers"</li>
						<li>• Use kebab-case for package names</li>
						<li>• Group related packages with common prefixes</li>
					</ul>
				</div>

				<div class="border rounded-lg p-6">
					<h3 class="font-semibold mb-3 flex items-center">
						<Icon icon="lucide:target" class="w-5 h-5 mr-2 text-blue-600" />
						Organization
					</h3>
					<ul class="text-sm text-muted-foreground space-y-2">
						<li>• Organize packages by domain, not technical layer</li>
						<li>• Create focused packages with single responsibility</li>
						<li>• Use aliases to hide complex package structures</li>
						<li>• Document alias patterns in your project README</li>
						<li>• Consider team knowledge when choosing prefixes</li>
					</ul>
				</div>
			</div>

			<div class="space-y-6">
				<div class="border rounded-lg p-6">
					<h3 class="font-semibold mb-3 flex items-center">
						<Icon icon="lucide:users" class="w-5 h-5 mr-2 text-purple-600" />
						Team Collaboration
					</h3>
					<ul class="text-sm text-muted-foreground space-y-2">
						<li>• Establish team-wide alias conventions early</li>
						<li>• Use the same alias prefix for shared packages</li>
						<li>• Document the reasoning behind alias choices</li>
						<li>• Consider IDE auto-completion when choosing prefixes</li>
						<li>• Train team members on alias benefits and usage</li>
					</ul>
				</div>

				<div class="border rounded-lg p-6">
					<h3 class="font-semibold mb-3 flex items-center">
						<Icon icon="lucide:shield-alert" class="w-5 h-5 mr-2 text-yellow-600" />
						Common Pitfalls
					</h3>
					<ul class="text-sm text-muted-foreground space-y-2">
						<li>• Don't use conflicting alias prefixes in same app</li>
						<li>• Avoid aliases that conflict with node_modules</li>
						<li>• Don't change alias prefixes frequently</li>
						<li>• Remember to run <code>knot link</code> after changes</li>
						<li>• Test aliases work in all build environments</li>
					</ul>
				</div>
			</div>
		</div>
	</section>

	<!-- Enhanced Troubleshooting -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Troubleshooting Aliases</h2>

		<div class="space-y-6">
			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="lucide:alert-triangle" class="w-5 h-5 mr-2 text-red-500" />
					Aliases Not Working
				</h3>
				<div class="space-y-3">
					<div class="text-sm">
						<div class="font-medium mb-1">Symptoms:</div>
						<ul class="text-muted-foreground space-y-1">
							<li>• TypeScript errors: "Cannot find module '@/package-name'"</li>
							<li>• IDE doesn't provide auto-completion for aliases</li>
							<li>• Build fails with module resolution errors</li>
						</ul>
					</div>
					<div class="text-sm">
						<div class="font-medium mb-1">Solutions:</div>
						<ul class="text-muted-foreground space-y-1">
							<li>• Run <code>knot link --force</code> to regenerate TypeScript paths</li>
							<li>• Check that <code>tsconfig.json</code> exists in your app directory</li>
							<li>• Restart your TypeScript service in your IDE (Ctrl+Shift+P → "TypeScript: Restart TS Server")</li>
							<li>• Verify the alias prefix in your knot.yml or app.yml configuration</li>
							<li>• Ensure package names match exactly (case-sensitive)</li>
						</ul>
					</div>
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="lucide:settings" class="w-5 h-5 mr-2 text-blue-500" />
					Build Tool Issues
				</h3>
				<div class="space-y-3">
					<div class="text-sm">
						<div class="font-medium mb-1">Framework-Specific Solutions:</div>
						<div class="space-y-2">
							<div class="border rounded p-3">
								<div class="font-medium mb-1">Vite/SvelteKit</div>
								<div class="text-muted-foreground text-xs">
									Add alias configuration to vite.config.js or svelte.config.js
								</div>
								<div class="bg-black/90 text-white font-mono text-xs p-2 rounded mt-2">
									<code>alias: {"{"} '@': path.resolve('./knot_packages') {"}"}</code>
								</div>
							</div>
							<div class="border rounded p-3">
								<div class="font-medium mb-1">Webpack/Next.js</div>
								<div class="text-muted-foreground text-xs">
									Configure webpack alias in next.config.js or webpack.config.js
								</div>
								<div class="bg-black/90 text-white font-mono text-xs p-2 rounded mt-2">
									<code>resolve: {"{"} alias: {"{"} '@': path.resolve('./knot_packages') {"}"} {"}"}</code>
								</div>
							</div>
						</div>
					</div>
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="lucide:bug" class="w-5 h-5 mr-2 text-orange-500" />
					Debug Commands
				</h3>
				<div class="space-y-3">
					<div>
						<div class="font-medium mb-2">Check project status</div>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-3 rounded">
							<code>knot status</code>
						</div>
						<div class="text-xs text-muted-foreground mt-1">
							Shows which packages are linked and their current alias configuration
						</div>
					</div>
					<div>
						<div class="font-medium mb-2">Verbose linking output</div>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-3 rounded">
							<code>knot link --verbose</code>
						</div>
						<div class="text-xs text-muted-foreground mt-1">
							Provides detailed output about the linking process and alias setup
						</div>
					</div>
					<div>
						<div class="font-medium mb-2">Preview changes without applying</div>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-3 rounded">
							<code>knot link --dry-run</code>
						</div>
						<div class="text-xs text-muted-foreground mt-1">
							Shows what changes would be made without actually modifying files
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Next Steps -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Next Steps</h2>
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<a href="/docs/typescript" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:code" class="w-6 h-6 text-blue-600" />
					<h3 class="font-semibold">TypeScript Integration</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Learn more about TypeScript configuration and best practices with Knot.
				</p>
			</a>

			<a href="/docs/package-development" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:package" class="w-6 h-6 text-purple-600" />
					<h3 class="font-semibold">Package Development</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Understand how to use aliases effectively during package development.
				</p>
			</a>

			<a href="/docs/project-management" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:folder" class="w-6 h-6 text-green-600" />
					<h3 class="font-semibold">Project Structure</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Understand how to organize your monorepo structure effectively.
				</p>
			</a>

			<a href="/docs/workflows" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:workflow" class="w-6 h-6 text-orange-600" />
					<h3 class="font-semibold">Development Workflows</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Optimize your development workflow with aliases and linking strategies.
				</p>
			</a>
		</div>
	</section>
</div>