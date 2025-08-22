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
	<title>Production Deployment - Knot CLI Documentation</title>
	<meta name="description" content="Learn how to deploy your Knot projects to production. Covers building for production, containerizing with Docker, and deploying to common platforms." />
	<meta property="og:title" content="Production Deployment - Knot CLI" />
	<meta property="og:description" content="Learn how to deploy your Knot projects to production. Covers building for production, containerizing with Docker, and deploying to common platforms." />
	<meta property="og:image" content="/images/og/production.png" />
	<meta property="og:url" content="https://knot-space.com/docs/production" />
	<meta name="twitter:title" content="Production Deployment - Knot CLI" />
	<meta name="twitter:description" content="Learn how to deploy your Knot projects to production. Covers building for production, containerizing with Docker, and deploying to common platforms." />
	<meta name="twitter:image" content="/images/og/production.png" />
	<link rel="canonical" href="https://knot-space.com/docs/production" />
</svelte:head>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight mb-4" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
			Production Deployment
		</h1>
		<p class="text-lg text-muted-foreground">
			Deploy your Knot projects to production with confidence
		</p>
	</div>

	<!-- Building for Production -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Building for Production</h2>
		<div class="bg-blue-50 border border-blue-200 rounded-lg p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="solar:box-bold" class="w-6 h-6 text-blue-600 mt-1 flex-shrink-0" />
				<div class="flex-1 min-w-0">
					<h3 class="font-semibold text-blue-900 mb-2">Create a production-ready build</h3>
					<p class="text-sm text-blue-700 mb-4">
						The `knot build` command will create optimized builds for all your apps.
					</p>
					<div class="bg-black/90 text-green-400 font-mono text-sm rounded-lg relative group w-full max-w-full">
						<div class="overflow-x-auto overflow-y-hidden p-4 pr-12 max-w-full">
							<pre class="whitespace-nowrap m-0 min-w-max"><code>knot build</code></pre>
						</div>
						<button
							class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100 z-10"
							on:click={() => copyToClipboard('knot build')}
							title="Copy to clipboard"
						>
							{#if showCopied && copyText === 'knot build'}
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

	<!-- Containerizing with Docker -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Containerizing with Docker</h2>
		<div class="bg-purple-50 border border-purple-200 rounded-lg p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="solar:server-bold" class="w-6 h-6 text-purple-600 mt-1 flex-shrink-0" />
				<div class="flex-1 min-w-0">
					<h3 class="font-semibold text-purple-900 mb-2">Dockerize your apps</h3>
					<p class="text-sm text-purple-700 mb-4">
						A `Dockerfile` is provided in the `knot-space` app to containerize your application.
					</p>
					<div class="bg-black/90 text-green-400 font-mono text-sm rounded-lg relative group w-full max-w-full">
						<div class="overflow-x-auto overflow-y-hidden p-4 pr-12 max-w-full">
							<pre class="whitespace-nowrap m-0 min-w-max"><code>docker build -t my-app ./apps/knot-space</code></pre>
						</div>
						<button
							class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100 z-10"
							on:click={() => copyToClipboard('docker build -t my-app ./apps/knot-space')}
						>
							{#if showCopied && copyText === 'docker build -t my-app ./apps/knot-space'}
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

	<!-- Deployment Platforms -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Deployment Platforms</h2>
		<div class="space-y-6">
			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="simple-icons:vercel" class="w-5 h-5 mr-2" />
					Deploying to Vercel
				</h3>
				<div class="space-y-3 text-sm text-muted-foreground">
					<p>
						Vercel is a great platform for deploying frontend applications.
					</p>
					<ul class="space-y-1 ml-4">
						<li>• Connect your Git repository to Vercel.</li>
						<li>• Configure the build command to be `knot build`.</li>
						<li>• Set the output directory to `apps/knot-space-ui/build`.</li>
					</ul>
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="simple-icons:amazonaws" class="w-5 h-5 mr-2" />
					Deploying to AWS
				</h3>
				<div class="space-y-3 text-sm text-muted-foreground">
					<p>
						Use Docker to deploy your apps to AWS services like ECS or EKS.
					</p>
					<ul class="space-y-1 ml-4">
						<li>• Build your Docker image and push it to ECR.</li>
						<li>• Create a new task definition in ECS.</li>
						<li>• Launch a new service with your task definition.</li>
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
					<h3 class="font-semibold text-green-900 mb-2">Ready for production!</h3>
					<p class="text-sm text-green-700 mb-4">
						Now that you know how to deploy your project, let's explore more advanced topics.
					</p>
					<div class="flex flex-wrap gap-3">
						<a href="/docs/self-hosting" class="inline-flex items-center px-4 py-2 bg-green-600 text-white text-sm font-medium rounded-md hover:bg-green-700 transition-colors">
							<Icon icon="solar:server-bold" class="w-4 h-4 mr-2" />
							Self-Hosting Guide
						</a>
						<a href="/docs/workflows" class="inline-flex items-center px-4 py-2 border border-green-600 text-green-600 text-sm font-medium rounded-md hover:bg-green-50 transition-colors">
							<Icon icon="solar:settings-bold" class="w-4 h-4 mr-2" />
							Workflow Best Practices
						</a>
					</div>
				</div>
			</div>
		</div>
	</section>
</div>