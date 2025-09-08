<script lang="ts">
	import Icon from '@iconify/svelte';
</script>

<svelte:head>
	<title>Workflow Best Practices - Knot CLI Documentation</title>
	<meta name="description" content="Learn the best practices for working with Knot CLI to improve your development workflow." />
	<meta property="og:title" content="Workflow Best Practices - Knot CLI" />
	<meta property="og:description" content="Learn the best practices for working with Knot CLI to improve your development workflow." />
	<meta property="og:image" content="/images/og/workflows.png" />
	<meta property="og:url" content="https://knot.klysium.com/docs/workflows" />
	<meta name="twitter:title" content="Workflow Best Practices - Knot CLI" />
	<meta name="twitter:description" content="Learn the best practices for working with Knot CLI to improve your development workflow." />
	<meta name="twitter:image" content="/images/og/workflows.png" />
	<link rel="canonical" href="https://knot.klysium.com/docs/workflows" />
</svelte:head>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight mb-4" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
			Workflow Best Practices
		</h1>
		<p class="text-lg text-muted-foreground">
			Improve your development workflow with these best practices
		</p>
	</div>

	<!-- Version Control -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Version Control</h2>
		<div class="bg-blue-50 border border-blue-200 rounded-lg p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="lucide:branch-bold" class="w-6 h-6 text-blue-600 mt-1 flex-shrink-0" />
				<div class="flex-1 min-w-0">
					<h3 class="font-semibold text-blue-900 mb-2">Commit your `knot.yml`</h3>
					<p class="text-sm text-blue-700 mb-4">
						The `knot.yml` file contains the configuration for your project. It should be committed to version control to ensure that all team members are using the same settings.
					</p>
				</div>
			</div>
		</div>
	</section>

	<!-- Package Development -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Package Development</h2>
		<div class="bg-purple-50 border border-purple-200 rounded-lg p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="lucide:box-bold" class="w-6 h-6 text-purple-600 mt-1 flex-shrink-0" />
				<div class="flex-1 min-w-0">
					<h3 class="font-semibold text-purple-900 mb-2">Keep packages small and focused</h3>
					<p class="text-sm text-purple-700 mb-4">
						Each package should have a single responsibility. This makes them easier to maintain and reuse across different applications.
					</p>
				</div>
			</div>
		</div>
	</section>

	<!-- Linking -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Linking</h2>
		<div class="bg-green-50 border border-green-200 rounded-lg p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="lucide:link" class="w-6 h-6 text-green-600 mt-1 flex-shrink-0" />
				<div class="flex-1 min-w-0">
					<h3 class="font-semibold text-green-900 mb-2">Link early, link often</h3>
					<p class="text-sm text-green-700 mb-4">
						Run `knot link` whenever you make changes to your packages to ensure that your applications are using the latest version.
					</p>
				</div>
			</div>
		</div>
	</section>

	<!-- Next Steps -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Next Steps</h2>
		<div class="bg-gray-50 border border-gray-200 rounded-lg p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="lucide:check-circle" class="w-6 h-6 text-gray-600 mt-1 flex-shrink-0" />
				<div>
					<h3 class="font-semibold text-gray-900 mb-2">Ready to optimize your workflow?</h3>
					<p class="text-sm text-gray-700 mb-4">
						Now that you know the best practices, let's dive into more advanced topics.
					</p>
					<div class="flex flex-wrap gap-3">
						<a href="/docs/package-development" class="inline-flex items-center px-4 py-2 bg-gray-600 text-white text-sm font-medium rounded-md hover:bg-gray-700 transition-colors">
							<Icon icon="lucide:box-bold" class="w-4 h-4 mr-2" />
							Package Development
						</a>
						<a href="/docs/project-management" class="inline-flex items-center px-4 py-2 border border-gray-600 text-gray-600 text-sm font-medium rounded-md hover:bg-gray-50 transition-colors">
							<Icon icon="lucide:folder-with-files-bold" class="w-4 h-4 mr-2" />
							Project Management
						</a>
					</div>
				</div>
			</div>
		</div>
	</section>
</div>