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
	<title>Permissions - Knot CLI Documentation</title>
	<meta name="description" content="Learn how to manage permissions for your Knot projects and packages." />
	<meta property="og:title" content="Permissions - Knot CLI" />
	<meta property="og:description" content="Learn how to manage permissions for your Knot projects and packages." />
	<meta property="og:image" content="/images/og/permissions.png" />
	<meta property="og:url" content="https://knot-space.com/docs/permissions" />
	<meta name="twitter:title" content="Permissions - Knot CLI" />
	<meta name="twitter:description" content="Learn how to manage permissions for your Knot projects and packages." />
	<meta name="twitter:image" content="/images/og/permissions.png" />
	<link rel="canonical" href="https://knot-space.com/docs/permissions" />
</svelte:head>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight mb-4" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
			Permissions
		</h1>
		<p class="text-lg text-muted-foreground">
			Manage access to your projects and packages
		</p>
	</div>

	<!-- Roles -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Roles</h2>
		<div class="bg-blue-50 border border-blue-200 rounded-lg p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="solar:user-bold" class="w-6 h-6 text-blue-600 mt-1 flex-shrink-0" />
				<div class="flex-1 min-w-0">
					<h3 class="font-semibold text-blue-900 mb-2">User Roles</h3>
					<p class="text-sm text-blue-700 mb-4">
						Knot uses a role-based access control system to manage permissions. The following roles are available:
					</p>
					<ul class="text-sm text-blue-700 space-y-1">
						<li><b>Owner:</b> Can manage all aspects of the project, including billing and team members.</li>
						<li><b>Admin:</b> Can manage packages and team members.</li>
						<li><b>Developer:</b> Can publish and manage packages.</li>
						<li><b>Guest:</b> Can view packages.</li>
					</ul>
				</div>
			</div>
		</div>
	</section>

	<!-- Managing Permissions -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Managing Permissions</h2>
		<div class="bg-purple-50 border border-purple-200 rounded-lg p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="solar:shield-user-bold" class="w-6 h-6 text-purple-600 mt-1 flex-shrink-0" />
				<div class="flex-1 min-w-0">
					<h3 class="font-semibold text-purple-900 mb-2">Team Management</h3>
					<p class="text-sm text-purple-700 mb-4">
						You can manage your team members and their roles from the Knot Space dashboard.
					</p>
					<div class="space-y-3">
						<div>
							<div class="text-sm font-medium text-purple-900 mb-1">Invite a team member</div>
							<div class="bg-black/90 text-green-400 font-mono text-sm rounded-lg relative group w-full max-w-full">
								<div class="overflow-x-auto overflow-y-hidden p-4 pr-12 max-w-full">
									<pre class="whitespace-nowrap m-0 min-w-max"><code>knot team:invite user@example.com --role developer</code></pre>
								</div>
								<button
									class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100 z-10"
									on:click={() => copyToClipboard('knot team:invite user@example.com --role developer')}
								>
									{#if showCopied && copyText === 'knot team:invite user@example.com --role developer'}
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

	<!-- Next Steps -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Next Steps</h2>
		<div class="bg-green-50 border border-green-200 rounded-lg p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="solar:check-circle-bold" class="w-6 h-6 text-green-600 mt-1 flex-shrink-0" />
				<div>
					<h3 class="font-semibold text-green-900 mb-2">Ready to collaborate?</h3>
					<p class="text-sm text-green-700 mb-4">
						Now that you understand how to manage permissions, you can start collaborating with your team.
					</p>
					<div class="flex flex-wrap gap-3">
						<a href="/docs/teams" class="inline-flex items-center px-4 py-2 bg-green-600 text-white text-sm font-medium rounded-md hover:bg-green-700 transition-colors">
							<Icon icon="solar:users-group-two-rounded-bold" class="w-4 h-4 mr-2" />
							Team Management
						</a>
					</div>
				</div>
			</div>
		</div>
	</section>
</div>