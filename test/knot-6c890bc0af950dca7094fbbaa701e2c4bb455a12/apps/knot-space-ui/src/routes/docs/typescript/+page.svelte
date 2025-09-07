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
	<title>TypeScript & JavaScript - Knot CLI Documentation</title>
	<meta name="description" content="Build and manage TypeScript and JavaScript packages with Knot. Learn about type definitions, builds, and best practices." />
	<meta property="og:title" content="TypeScript & JavaScript - Knot CLI" />
	<meta property="og:description" content="Build and manage TypeScript and JavaScript packages with Knot. Learn about type definitions, builds, and best practices." />
	<meta property="og:url" content="https://knot.klysium.com/docs/typescript" />
	<meta name="twitter:title" content="TypeScript & JavaScript - Knot CLI" />
	<meta name="twitter:description" content="Build and manage TypeScript and JavaScript packages with Knot. Learn about type definitions, builds, and best practices." />
	<link rel="canonical" href="https://knot.klysium.com/docs/typescript" />
</svelte:head>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight mb-4" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
			TypeScript & JavaScript
		</h1>
		<p class="text-lg text-muted-foreground">
			Build and manage TypeScript and JavaScript packages with Knot.
		</p>
	</div>

	<!-- Overview -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Overview</h2>
		<p class="text-muted-foreground mb-6 leading-relaxed">
			Knot provides first-class support for TypeScript and JavaScript packages, including automatic type generation, 
			module resolution, and seamless integration with popular build tools.
		</p>
	</section>

	<!-- TypeScript Configuration -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">TypeScript Configuration</h2>
		
		<div>
			<h3 class="text-lg font-semibold mb-3">Recommended tsconfig.json</h3>
			<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
				<pre><code>{@html `{
  <span class="text-green-400">"compilerOptions"</span>: {
    <span class="text-green-400">"target"</span>: <span class="text-yellow-400">"ES2020"</span>,
    <span class="text-green-400">"module"</span>: <span class="text-yellow-400">"ESNext"</span>,
    <span class="text-green-400">"lib"</span>: [<span class="text-yellow-400">"ES2020"</span>],
    <span class="text-green-400">"moduleResolution"</span>: <span class="text-yellow-400">"bundler"</span>,
    <span class="text-green-400">"declaration"</span>: <span class="text-blue-400">true</span>,
    <span class="text-green-400">"declarationMap"</span>: <span class="text-blue-400">true</span>,
    <span class="text-green-400">"sourceMap"</span>: <span class="text-blue-400">true</span>,
    <span class="text-green-400">"outDir"</span>: <span class="text-yellow-400">"./dist"</span>,
    <span class="text-green-400">"rootDir"</span>: <span class="text-yellow-400">"./src"</span>,
    <span class="text-green-400">"strict"</span>: <span class="text-blue-400">true</span>,
    <span class="text-green-400">"esModuleInterop"</span>: <span class="text-blue-400">true</span>,
    <span class="text-green-400">"skipLibCheck"</span>: <span class="text-blue-400">true</span>,
    <span class="text-green-400">"forceConsistentCasingInFileNames"</span>: <span class="text-blue-400">true</span>
  },
  <span class="text-green-400">"include"</span>: [<span class="text-yellow-400">"src/**/*"</span>],
  <span class="text-green-400">"exclude"</span>: [<span class="text-yellow-400">"node_modules"</span>, <span class="text-yellow-400">"dist"</span>, <span class="text-yellow-400">"**/*.test.ts"</span>]
}`}</code></pre>
			</div>
		</div>
	</section>

	<!-- Package.json Setup -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Package.json Setup</h2>
		
		<div>
			<h3 class="text-lg font-semibold mb-3">Example package.json</h3>
			<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
				<pre><code>{@html `{
  <span class="text-green-400">"name"</span>: <span class="text-yellow-400">"@your-org/package-name"</span>,
  <span class="text-green-400">"version"</span>: <span class="text-yellow-400">"1.0.0"</span>,
  <span class="text-green-400">"type"</span>: <span class="text-yellow-400">"module"</span>,
  <span class="text-green-400">"main"</span>: <span class="text-yellow-400">"./dist/index.js"</span>,
  <span class="text-green-400">"module"</span>: <span class="text-yellow-400">"./dist/index.js"</span>,
  <span class="text-green-400">"types"</span>: <span class="text-yellow-400">"./dist/index.d.ts"</span>,
  <span class="text-green-400">"exports"</span>: {
    <span class="text-green-400">"."</span>: {
      <span class="text-green-400">"types"</span>: <span class="text-yellow-400">"./dist/index.d.ts"</span>,
      <span class="text-green-400">"import"</span>: <span class="text-yellow-400">"./dist/index.js"</span>,
      <span class="text-green-400">"require"</span>: <span class="text-yellow-400">"./dist/index.cjs"</span>
    }
  },
  <span class="text-green-400">"scripts"</span>: {
    <span class="text-green-400">"build"</span>: <span class="text-yellow-400">"knot build"</span>,
    <span class="text-green-400">"test"</span>: <span class="text-yellow-400">"knot test"</span>,
    <span class="text-green-400">"lint"</span>: <span class="text-yellow-400">"knot lint"</span>,
    <span class="text-green-400">"dev"</span>: <span class="text-yellow-400">"knot dev"</span>
  },
  <span class="text-green-400">"devDependencies"</span>: {
    <span class="text-green-400">"typescript"</span>: <span class="text-yellow-400">"^5.0.0"</span>,
    <span class="text-green-400">"@types/node"</span>: <span class="text-yellow-400">"^20.0.0"</span>
  }
}`}</code></pre>
			</div>
		</div>
	</section>

	<!-- Building Packages -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Building Packages</h2>
		
		<div class="space-y-6">
			<div class="bg-blue-50 border border-blue-200 rounded-lg p-6">
				<div class="flex items-start space-x-3">
					<Icon icon="solar:hammer-bold" class="w-6 h-6 text-blue-600 mt-1 flex-shrink-0" />
					<div class="flex-1">
						<h3 class="font-semibold text-blue-900 mb-2">Build Command</h3>
						<div class="bg-black/90 text-green-400 font-mono text-sm rounded-lg relative group mb-3">
							<div class="overflow-x-auto p-4 pr-12">
								<code class="whitespace-nowrap block">knot build</code>
							</div>
							<button 
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100 z-10"
								on:click={() => copyToClipboard('knot build')}
							>
								{#if showCopied && copyText === 'knot build'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
						<p class="text-sm text-blue-700">
							Automatically detects and builds TypeScript files, generates type definitions, and bundles your package.
						</p>
					</div>
				</div>
			</div>

			<div class="bg-green-50 border border-green-200 rounded-lg p-6">
				<div class="flex items-start space-x-3">
					<Icon icon="solar:eye-bold" class="w-6 h-6 text-green-600 mt-1 flex-shrink-0" />
					<div class="flex-1">
						<h3 class="font-semibold text-green-900 mb-2">Watch Mode</h3>
						<div class="bg-black/90 text-green-400 font-mono text-sm rounded-lg relative group mb-3">
							<div class="overflow-x-auto p-4 pr-12">
								<code class="whitespace-nowrap block">knot dev</code>
							</div>
							<button 
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100 z-10"
								on:click={() => copyToClipboard('knot dev')}
							>
								{#if showCopied && copyText === 'knot dev'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
						<p class="text-sm text-green-700">
							Watches for file changes and rebuilds automatically during development.
						</p>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Type Definitions -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Type Definitions</h2>
		
		<p class="text-muted-foreground mb-6 leading-relaxed">
			Knot automatically generates and includes TypeScript type definitions when you publish packages. 
			Ensure your <code>tsconfig.json</code> has <code>"declaration": true</code> enabled.
		</p>

		<div>
			<h3 class="text-lg font-semibold mb-3">Publishing with Types</h3>
			<div class="bg-black/90 text-green-400 font-mono text-sm rounded-lg relative group">
				<div class="overflow-x-auto p-4 pr-12">
					<pre class="whitespace-pre m-0"><code>{@html `<span class="text-gray-400"># Build and publish with type definitions</span>
knot build
knot publish

<span class="text-gray-400"># Types are automatically included in the package</span>`}</code></pre>
				</div>
				<button 
					class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100 z-10"
					on:click={() => copyToClipboard('knot build\nknot publish')}
				>
					{#if showCopied && copyText === 'knot build\nknot publish'}
						<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
					{:else}
						<Icon icon="solar:copy-bold" class="w-4 h-4" />
					{/if}
				</button>
			</div>
		</div>
	</section>

	<!-- Module Formats -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Module Formats</h2>
		
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<div class="border rounded-lg p-6">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:code-bold" class="w-6 h-6 text-blue-600" />
					<h3 class="font-semibold">ESM (ECMAScript Modules)</h3>
				</div>
				<p class="text-sm text-muted-foreground mb-3">Modern standard, recommended for new projects</p>
				<div class="bg-black/90 text-white font-mono text-sm p-3 rounded">
					<code>{@html `<span class="text-blue-400">import</span> { <span class="text-yellow-400">myFunction</span> } <span class="text-blue-400">from</span> <span class="text-green-400">'@org/package'</span>;`}</code>
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:code-2-bold" class="w-6 h-6 text-purple-600" />
					<h3 class="font-semibold">CommonJS</h3>
				</div>
				<p class="text-sm text-muted-foreground mb-3">Legacy format, supported for compatibility</p>
				<div class="bg-black/90 text-white font-mono text-sm p-3 rounded">
					<code>{@html `<span class="text-blue-400">const</span> { <span class="text-yellow-400">myFunction</span> } = <span class="text-purple-400">require</span>(<span class="text-green-400">'@org/package'</span>);`}</code>
				</div>
			</div>
		</div>
	</section>

	<!-- Testing -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Testing</h2>
		
		<div>
			<h3 class="text-lg font-semibold mb-3">Running Tests</h3>
			<div class="bg-black/90 text-green-400 font-mono text-sm rounded-lg relative group">
				<div class="overflow-x-auto p-4 pr-12">
					<pre class="whitespace-pre m-0"><code>{@html `<span class="text-gray-400"># Run all tests</span>
knot test

<span class="text-gray-400"># Run tests in watch mode</span>
knot test --watch

<span class="text-gray-400"># Run tests with coverage</span>
knot test --coverage`}</code></pre>
				</div>
				<button 
					class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100 z-10"
					on:click={() => copyToClipboard('knot test')}
				>
					{#if showCopied && copyText === 'knot test'}
						<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
					{:else}
						<Icon icon="solar:copy-bold" class="w-4 h-4" />
					{/if}
				</button>
			</div>
		</div>
	</section>

	<!-- Best Practices -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Best Practices</h2>
		
		<div class="space-y-4">
			<div class="flex items-start space-x-3">
				<Icon icon="solar:check-circle-bold" class="w-5 h-5 text-green-600 mt-0.5 flex-shrink-0" />
				<div>
					<p class="font-semibold">Use strict TypeScript settings</p>
					<p class="text-sm text-muted-foreground">Enable strict mode for better type safety and catching potential issues early.</p>
				</div>
			</div>

			<div class="flex items-start space-x-3">
				<Icon icon="solar:check-circle-bold" class="w-5 h-5 text-green-600 mt-0.5 flex-shrink-0" />
				<div>
					<p class="font-semibold">Include source maps</p>
					<p class="text-sm text-muted-foreground">Help with debugging in production and development environments.</p>
				</div>
			</div>

			<div class="flex items-start space-x-3">
				<Icon icon="solar:check-circle-bold" class="w-5 h-5 text-green-600 mt-0.5 flex-shrink-0" />
				<div>
					<p class="font-semibold">Export types explicitly</p>
					<p class="text-sm text-muted-foreground">Make your API surface clear and documented for better developer experience.</p>
				</div>
			</div>

			<div class="flex items-start space-x-3">
				<Icon icon="solar:check-circle-bold" class="w-5 h-5 text-green-600 mt-0.5 flex-shrink-0" />
				<div>
					<p class="font-semibold">Use semantic versioning</p>
					<p class="text-sm text-muted-foreground">Follow semver conventions for version updates to ensure compatibility.</p>
				</div>
			</div>
		</div>
	</section>

	<!-- Next Steps -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Next Steps</h2>
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<a href="/docs/publishing" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:upload-bold" class="w-6 h-6 text-green-600" />
					<h3 class="font-semibold">Publishing Packages</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Learn how to publish your TypeScript packages to the registry.
				</p>
			</a>

			<a href="/docs/cli-packages" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:terminal-bold" class="w-6 h-6 text-blue-600" />
					<h3 class="font-semibold">Package Commands</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Explore all package-related CLI commands and options.
				</p>
			</a>
		</div>
	</section>
</div>