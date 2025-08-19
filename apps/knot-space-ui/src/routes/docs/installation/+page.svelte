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
	<title>Installation Guide - Knot CLI Documentation</title>
	<meta name="description" content="Install Knot CLI with our one-line installer script. Works on macOS, Linux, and Windows. Includes built-in self-update functionality." />
	<meta property="og:title" content="Installation Guide - Knot CLI" />
	<meta property="og:description" content="Install Knot CLI with our one-line installer script. Works on macOS, Linux, and Windows. Includes built-in self-update functionality." />
	<meta property="og:image" content="/images/og/installation.png" />
	<meta property="og:url" content="https://knot-space.com/docs/installation" />
	<meta name="twitter:title" content="Installation Guide - Knot CLI" />
	<meta name="twitter:description" content="Install Knot CLI with our one-line installer script. Works on macOS, Linux, and Windows. Includes built-in self-update functionality." />
	<meta name="twitter:image" content="/images/og/installation.png" />
	<link rel="canonical" href="https://knot-space.com/docs/installation" />
</svelte:head>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6 overflow-x-hidden">
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight mb-4" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
			Installation
		</h1>
		<p class="text-lg text-muted-foreground">
			Get Knot CLI up and running on your system in minutes
		</p>
	</div>

	<!-- Quick Install -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Quick Install (Recommended)</h2>
		<div class="bg-gradient-to-r from-blue-50 to-indigo-50 border border-blue-200 rounded-lg p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="solar:rocket-2-bold" class="w-6 h-6 text-blue-600 mt-1 flex-shrink-0" />
				<div class="flex-1 min-w-0">
					<h3 class="font-semibold text-blue-900 mb-2">One-line installer</h3>
					<p class="text-sm text-blue-700 mb-4">
						This script automatically detects your platform and installs the latest version of Knot CLI.
					</p>
					<div class="bg-black/90 text-green-400 font-mono text-sm rounded-lg relative group w-full max-w-full">
						<div class="overflow-x-auto overflow-y-hidden p-4 pr-12 max-w-full">
							<pre class="whitespace-nowrap m-0 min-w-max"><code>curl -fsSL https://raw.githubusercontent.com/saravenpi/knot/main/install.sh | bash</code></pre>
						</div>
						<button
							class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100 z-10"
							on:click={() => copyToClipboard('curl -fsSL https://raw.githubusercontent.com/saravenpi/knot/main/install.sh | bash')}
							title="Copy to clipboard"
						>
							{#if showCopied && copyText.includes('curl')}
								<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
							{:else}
								<Icon icon="solar:copy-bold" class="w-4 h-4" />
							{/if}
						</button>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Update Command -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Keeping Knot Updated</h2>
		<div class="bg-gradient-to-r from-purple-50 to-pink-50 border border-purple-200 rounded-lg p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="solar:refresh-circle-bold" class="w-6 h-6 text-purple-600 mt-1 flex-shrink-0" />
				<div class="flex-1 min-w-0">
					<h3 class="font-semibold text-purple-900 mb-2">Self-Update Command</h3>
					<p class="text-sm text-purple-700 mb-4">
						Knot can update itself to the latest version automatically. No need to reinstall!
					</p>
					<div class="space-y-3">
						<div>
							<div class="text-sm font-medium text-purple-900 mb-1">Update to latest version</div>
							<div class="bg-black/90 text-green-400 font-mono text-sm rounded-lg relative group w-full max-w-full">
								<div class="overflow-x-auto overflow-y-hidden p-4 pr-12 max-w-full">
									<pre class="whitespace-nowrap m-0 min-w-max"><code>knot update</code></pre>
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
						</div>
						<div>
							<div class="text-sm font-medium text-purple-900 mb-1">Force update (reinstall current version)</div>
							<div class="bg-black/90 text-green-400 font-mono text-sm rounded-lg relative group w-full max-w-full">
								<div class="overflow-x-auto overflow-y-hidden p-4 pr-12 max-w-full">
									<pre class="whitespace-nowrap m-0 min-w-max"><code>knot update --force</code></pre>
								</div>
								<button
									class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100 z-10"
									on:click={() => copyToClipboard('knot update --force')}
								>
									{#if showCopied && copyText === 'knot update --force'}
										<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
									{:else}
										<Icon icon="solar:copy-bold" class="w-4 h-4" />
									{/if}
								</button>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Verification -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Verify Installation</h2>

		<div class="space-y-6">
			<div>
				<h3 class="text-lg font-semibold mb-3">Check version</h3>
				<div class="bg-black/90 text-green-400 font-mono text-sm rounded-lg relative group w-full max-w-full">
					<div class="overflow-x-auto overflow-y-hidden p-4 pr-12 max-w-full">
						<pre class="whitespace-nowrap m-0 min-w-max"><code>knot --version</code></pre>
					</div>
					<button
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100 z-10"
						on:click={() => copyToClipboard('knot --version')}
					>
						{#if showCopied && copyText === 'knot --version'}
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="solar:copy-bold" class="w-4 h-4" />
						{/if}
					</button>
				</div>
				<div class="mt-2 text-sm text-muted-foreground">
					Should output something like: <code>knot v1.0.0</code>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Test basic functionality</h3>
				<div class="bg-black/90 text-green-400 font-mono text-sm rounded-lg relative group w-full max-w-full">
					<div class="overflow-x-auto overflow-y-hidden p-4 pr-12 max-w-full">
						<pre class="whitespace-nowrap m-0 min-w-max"><code>knot info</code></pre>
					</div>
					<button
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100 z-10"
						on:click={() => copyToClipboard('knot info')}
					>
						{#if showCopied && copyText === 'knot info'}
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="solar:copy-bold" class="w-4 h-4" />
						{/if}
					</button>
				</div>
				<div class="mt-2 text-sm text-muted-foreground">
					This should display version information and available commands.
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
					<Icon icon="solar:shield-warning-bold" class="w-5 h-5 mr-2 text-yellow-600" />
					Command not found
				</h3>
				<div class="space-y-3 text-sm text-muted-foreground">
					<p>If you get "command not found" after installation:</p>
					<ul class="space-y-1 ml-4">
						<li>• Check if <code>/usr/local/bin</code> is in your PATH</li>
						<li>• Try restarting your terminal</li>
						<li>• On macOS: <code>sudo chown -R $(whoami) /usr/local/bin</code></li>
						<li>• Manually add to PATH: <code>export PATH=$PATH:/usr/local/bin</code></li>
					</ul>
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="solar:lock-bold" class="w-5 h-5 mr-2 text-red-600" />
					Permission denied
				</h3>
				<div class="space-y-3 text-sm text-muted-foreground">
					<p>If you get permission errors:</p>
					<ul class="space-y-1 ml-4">
						<li>• Make sure the binary is executable: <code>chmod +x knot</code></li>
						<li>• On macOS, you might need to allow the app in System Preferences > Security</li>
						<li>• Use <code>sudo</code> for system-wide installation</li>
					</ul>
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="solar:bug-bold" class="w-5 h-5 mr-2 text-purple-600" />
					Other issues
				</h3>
				<div class="space-y-3 text-sm text-muted-foreground">
					<p>For other installation problems:</p>
					<ul class="space-y-1 ml-4">
						<li>• Check our <a href="/docs/troubleshooting" class="text-primary hover:underline">troubleshooting guide</a></li>
						<li>• Visit the <a href="https://github.com/saravenpi/knot/issues" class="text-primary hover:underline">GitHub issues</a></li>
						<li>• Join our community for support</li>
					</ul>
				</div>
			</div>
		</div>
	</section>

	<!-- Next Steps -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Next Steps</h2>
		<div class="bg-green-50 border border-green-200 rounded-lg p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="solar:check-circle-bold" class="w-6 h-6 text-green-600 mt-1 flex-shrink-0" />
				<div>
					<h3 class="font-semibold text-green-900 mb-2">Installation complete!</h3>
					<p class="text-sm text-green-700 mb-4">
						Now that Knot is installed, let's get you started with your first project.
					</p>
					<div class="flex flex-wrap gap-3">
						<a href="/docs/quick-start" class="inline-flex items-center px-4 py-2 bg-green-600 text-white text-sm font-medium rounded-md hover:bg-green-700 transition-colors">
							<Icon icon="solar:play-bold" class="w-4 h-4 mr-2" />
							Quick Start Guide
						</a>
						<a href="/docs/authentication" class="inline-flex items-center px-4 py-2 border border-green-600 text-green-600 text-sm font-medium rounded-md hover:bg-green-50 transition-colors">
							<Icon icon="solar:key-bold" class="w-4 h-4 mr-2" />
							Setup Authentication
						</a>
					</div>
				</div>
			</div>
		</div>
	</section>
</div>
