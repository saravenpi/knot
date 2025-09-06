
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
	<title>Self-Hosting - Knot CLI Documentation</title>
	<meta name="description" content="Learn how to self-host your own Knot Space instance for complete control over your package registry." />
	<meta property="og:title" content="Self-Hosting - Knot CLI" />
	<meta property="og:description" content="Learn how to self-host your own Knot Space instance for complete control over your package registry." />
	<meta property="og:image" content="/images/og/self-hosting.png" />
	<meta property="og:url" content="https://knot.klysium.com/docs/self-hosting" />
	<meta name="twitter:title" content="Self-Hosting - Knot CLI" />
	<meta name="twitter:description" content="Learn how to self-host your own Knot Space instance for complete control over your package registry." />
	<meta name="twitter:image" content="/images/og/self-hosting.png" />
	<link rel="canonical" href="https://knot.klysium.com/docs/self-hosting" />
</svelte:head>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight mb-4" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
			Self-Hosting
		</h1>
		<p class="text-lg text-muted-foreground">
			Host your own private Knot Space instance
		</p>
	</div>

	<!-- Prerequisites -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Prerequisites</h2>
		<div class="bg-blue-50 border border-blue-200 rounded-lg p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="solar:info-circle-bold" class="w-6 h-6 text-blue-600 mt-1 flex-shrink-0" />
				<div>
					<h3 class="font-semibold text-blue-900 mb-2">Before you start</h3>
					<ul class="text-sm text-blue-700 space-y-1">
						<li>• A server with Docker installed.</li>
						<li>• A domain name pointing to your server's IP address.</li>
						<li>• A running PostgreSQL database.</li>
						<li>• Basic knowledge of the command line and Docker.</li>
					</ul>
				</div>
			</div>
		</div>
	</section>

	<!-- Step 1: Clone the repository -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<div class="w-8 h-8 bg-green-100 rounded-full flex items-center justify-center">
				<span class="text-green-600 font-bold text-sm">1</span>
			</div>
			<h2 class="text-2xl font-bold">Clone the Repository</h2>
		</div>
		<div class="bg-black/90 text-green-400 font-mono text-sm rounded-lg relative group">
			<div class="overflow-x-auto p-4 pr-12">
				<code class="whitespace-nowrap block">git clone https://github.com/saravenpi/knot.git</code>
			</div>
			<button 
				class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100 z-10"
				on:click={() => copyToClipboard('git clone https://github.com/saravenpi/knot.git')}
			>
				{#if showCopied && copyText === 'git clone https://github.com/saravenpi/knot.git'}
					<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
				{:else}
					<Icon icon="solar:copy-bold" class="w-4 h-4" />
				{/if}
			</button>
		</div>
	</section>

	<!-- Step 2: Configure your environment -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<div class="w-8 h-8 bg-blue-100 rounded-full flex items-center justify-center">
				<span class="text-blue-600 font-bold text-sm">2</span>
			</div>
			<h2 class="text-2xl font-bold">Configure Your Environment</h2>
		</div>
		<p class="text-muted-foreground mb-3">
			Navigate to the `apps/web` directory and create a `.env` file from the example.
		</p>
		<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
			<code>cp .env.example .env</code>
			<button 
				class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
				on:click={() => copyToClipboard('cp .env.example .env')}
			>
				{#if showCopied && copyText === 'cp .env.example .env'}
					<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
				{:else}
					<Icon icon="solar:copy-bold" class="w-4 h-4" />
				{/if}
			</button>
		</div>
		<p class="text-muted-foreground mt-3">
			Edit the `.env` file and set the `DATABASE_URL` to your PostgreSQL database, and `JWT_SECRET` to a secret of your choice.
		</p>
	</section>

	<!-- Step 3: Build and Run the Docker Image -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<div class="w-8 h-8 bg-purple-100 rounded-full flex items-center justify-center">
				<span class="text-purple-600 font-bold text-sm">3</span>
			</div>
			<h2 class="text-2xl font-bold">Build and Run the Docker Image</h2>
		</div>
		<div class="space-y-6">
			<div>
				<h3 class="text-lg font-semibold mb-3">Build the image</h3>
				<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
					<code>docker build -t knot-web .</code>
					<button 
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
						on:click={() => copyToClipboard('docker build -t knot-web .')}
					>
						{#if showCopied && copyText === 'docker build -t knot-web .'}
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="solar:copy-bold" class="w-4 h-4" />
						{/if}
					</button>
				</div>
			</div>
			<div>
				<h3 class="text-lg font-semibold mb-3">Run the container</h3>
				<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
					<code>docker run -d -p 3000:3000 --env-file .env knot-web</code>
					<button 
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
						on:click={() => copyToClipboard('docker run -d -p 3000:3000 --env-file .env knot-web')}
					>
						{#if showCopied && copyText === 'docker run -d -p 3000:3000 --env-file .env knot-web'}
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="solar:copy-bold" class="w-4 h-4" />
						{/if}
					</button>
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
					<h3 class="font-semibold text-green-900 mb-2">Your Knot Space is ready!</h3>
					<p class="text-sm text-green-700 mb-4">
						You can now access your private Knot Space instance at your domain.
					</p>
					<div class="flex flex-wrap gap-3">
						<a href="/docs/production" class="inline-flex items-center px-4 py-2 bg-green-600 text-white text-sm font-medium rounded-md hover:bg-green-700 transition-colors">
							<Icon icon="solar:box-bold" class="w-4 h-4 mr-2" />
							Production Deployment
						</a>
						<a href="/docs/permissions" class="inline-flex items-center px-4 py-2 border border-green-600 text-green-600 text-sm font-medium rounded-md hover:bg-green-50 transition-colors">
							<Icon icon="solar:shield-user-bold" class="w-4 h-4 mr-2" />
							Permissions Guide
						</a>
					</div>
				</div>
			</div>
		</div>
	</section>
</div>
