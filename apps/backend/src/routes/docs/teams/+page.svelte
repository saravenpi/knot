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
	<title>Team Management - Knot CLI Documentation</title>
	<meta name="description" content="Learn how to create and manage teams for collaborative package development with Knot CLI." />
	<meta property="og:title" content="Team Management - Knot CLI" />
	<meta property="og:description" content="Learn how to create and manage teams for collaborative package development with Knot CLI." />
	<meta property="og:image" content="/images/og/teams.png" />
	<meta property="og:url" content="https://knot.klysium.com/docs/teams" />
	<meta name="twitter:title" content="Team Management - Knot CLI" />
	<meta name="twitter:description" content="Learn how to create and manage teams for collaborative package development with Knot CLI." />
	<meta name="twitter:image" content="/images/og/teams.png" />
	<link rel="canonical" href="https://knot.klysium.com/docs/teams" />
</svelte:head>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight mb-4" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
			Team Management
		</h1>
		<p class="text-lg text-muted-foreground">
			Create and manage teams for collaborative package development and private package sharing
		</p>
	</div>

	<!-- Overview -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Overview</h2>
		<div class="bg-blue-50 border border-blue-200 rounded-lg p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="solar:users-group-two-rounded-bold" class="w-6 h-6 text-blue-600 mt-1 flex-shrink-0" />
				<div>
					<h3 class="font-semibold text-blue-900 mb-2">What are Teams?</h3>
					<p class="text-sm text-blue-700 mb-4">
						Teams in Knot enable collaborative package development and private package sharing. When you create a package with a team, it becomes namespaced under that team (e.g., <code>@myteam/package-name</code>) and is only accessible to team members.
					</p>
					<div class="space-y-2 text-sm text-blue-700">
						<div class="flex items-center space-x-2">
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600" />
							<span>Private package sharing within your organization</span>
						</div>
						<div class="flex items-center space-x-2">
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600" />
							<span>Role-based access control (Owner, Admin, Member)</span>
						</div>
						<div class="flex items-center space-x-2">
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600" />
							<span>Namespace protection for package names</span>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Creating Teams -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Creating Teams</h2>

		<div class="space-y-8">
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">Create a New Team</h3>
				<p class="text-muted-foreground mb-4">
					Create a new team to start collaborating on private packages.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Basic Usage</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot team create myteam</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot team create myteam')}
							>
								{#if showCopied && copyText === 'knot team create myteam'}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">With Description</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot team create myteam --description "My awesome development team"</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot team create myteam --description "My awesome development team"')}
							>
								{#if showCopied && copyText.includes('awesome development')}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div class="bg-yellow-50 border border-yellow-200 rounded-lg p-4">
						<div class="flex items-start space-x-3">
							<Icon icon="solar:info-circle-bold" class="w-5 h-5 text-yellow-600 mt-0.5 flex-shrink-0" />
							<div class="text-sm">
								<p class="font-medium text-yellow-800 mb-1">Team Name Requirements</p>
								<ul class="text-yellow-700 space-y-1">
									<li>• Must be unique across all teams</li>
									<li>• Only lowercase letters, numbers, and hyphens</li>
									<li>• Between 3-50 characters</li>
									<li>• Cannot start or end with a hyphen</li>
								</ul>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Managing Teams -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Managing Teams</h2>

		<div class="space-y-8">
			<!-- List Teams -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">List Your Teams</h3>
				<p class="text-muted-foreground mb-4">
					View all teams you're a member of and their roles.
				</p>
				<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
					<code>knot team list</code>
					<button
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
						on:click={() => copyToClipboard('knot team list')}
					>
						{#if showCopied && copyText === 'knot team list'}
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="solar:copy-bold" class="w-4 h-4" />
						{/if}
					</button>
				</div>
			</div>

			<!-- Team Info -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">Get Team Information</h3>
				<p class="text-muted-foreground mb-4">
					View detailed information about a specific team.
				</p>
				<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
					<code>knot team info myteam</code>
					<button
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
						on:click={() => copyToClipboard('knot team info myteam')}
					>
						{#if showCopied && copyText === 'knot team info myteam'}
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="solar:copy-bold" class="w-4 h-4" />
						{/if}
					</button>
				</div>
			</div>
		</div>
	</section>

	<!-- Team Members -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Team Members</h2>

		<div class="space-y-8">
			<!-- Add Members -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">Add Team Members</h3>
				<p class="text-muted-foreground mb-4">
					Invite users to join your team with specific roles.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Add Member</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot team add-member myteam username --role member</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot team add-member myteam username --role member')}
							>
								{#if showCopied && copyText.includes('add-member')}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Add Admin</h4>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
							<code>knot team add-member myteam username --role admin</code>
							<button
								class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
								on:click={() => copyToClipboard('knot team add-member myteam username --role admin')}
							>
								{#if showCopied && copyText.includes('role admin')}
									<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
								{:else}
									<Icon icon="solar:copy-bold" class="w-4 h-4" />
								{/if}
							</button>
						</div>
					</div>
				</div>
			</div>

			<!-- Remove Members -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">Remove Team Members</h3>
				<p class="text-muted-foreground mb-4">
					Remove users from your team. Only owners and admins can remove members.
				</p>
				<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
					<code>knot team remove-member myteam username</code>
					<button
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
						on:click={() => copyToClipboard('knot team remove-member myteam username')}
					>
						{#if showCopied && copyText.includes('remove-member')}
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="solar:copy-bold" class="w-4 h-4" />
						{/if}
					</button>
				</div>
			</div>
		</div>
	</section>

	<!-- Roles and Permissions -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Roles and Permissions</h2>

		<div class="grid grid-cols-1 md:grid-cols-3 gap-6">
			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-red-100 rounded-lg flex items-center justify-center mb-4">
					<Icon icon="solar:crown-bold" class="w-6 h-6 text-red-600" />
				</div>
				<h3 class="font-semibold mb-3">Owner</h3>
				<ul class="text-sm text-muted-foreground space-y-2">
					<li>• Full team management</li>
					<li>• Add/remove members</li>
					<li>• Change member roles</li>
					<li>• Publish packages</li>
					<li>• Delete team packages</li>
					<li>• Delete team</li>
				</ul>
			</div>

			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-orange-100 rounded-lg flex items-center justify-center mb-4">
					<Icon icon="solar:shield-user-bold" class="w-6 h-6 text-orange-600" />
				</div>
				<h3 class="font-semibold mb-3">Admin</h3>
				<ul class="text-sm text-muted-foreground space-y-2">
					<li>• Add/remove members</li>
					<li>• Change member roles</li>
					<li>• Publish packages</li>
					<li>• Delete team packages</li>
					<li>• View team information</li>
				</ul>
			</div>

			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-green-100 rounded-lg flex items-center justify-center mb-4">
					<Icon icon="solar:user-bold" class="w-6 h-6 text-green-600" />
				</div>
				<h3 class="font-semibold mb-3">Member</h3>
				<ul class="text-sm text-muted-foreground space-y-2">
					<li>• View team packages</li>
					<li>• Download team packages</li>
					<li>• Publish packages</li>
					<li>• View team information</li>
				</ul>
			</div>
		</div>
	</section>

	<!-- Team Packages -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Team Packages</h2>

		<div class="space-y-8">
			<!-- Creating Team Packages -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">Creating Team Packages</h3>
				<p class="text-muted-foreground mb-4">
					Create packages that belong to your team namespace.
				</p>
				<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
					<code>knot init:package shared-utils --team myteam</code>
					<button
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
						on:click={() => copyToClipboard('knot init:package shared-utils --team myteam')}
					>
						{#if showCopied && copyText.includes('shared-utils')}
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="solar:copy-bold" class="w-4 h-4" />
						{/if}
					</button>
				</div>
				<div class="mt-2 text-sm text-muted-foreground">
					This creates a package named <code>@myteam/shared-utils</code>
				</div>
			</div>

			<!-- Publishing Team Packages -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">Publishing Team Packages</h3>
				<p class="text-muted-foreground mb-4">
					Publish packages under your team namespace.
				</p>
				<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
					<code>knot publish --team myteam</code>
					<button
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
						on:click={() => copyToClipboard('knot publish --team myteam')}
					>
						{#if showCopied && copyText.includes('publish --team')}
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="solar:copy-bold" class="w-4 h-4" />
						{/if}
					</button>
				</div>
			</div>

			<!-- Using Team Packages -->
			<div class="border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">Using Team Packages</h3>
				<p class="text-muted-foreground mb-4">
					Reference team packages in your applications with the full namespace.
				</p>
				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">In knot.yml or app.yml</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-blue-400">packages:</span>
  - <span class="text-green-400">types</span>            <span class="text-gray-500"># Local package</span>
  - <span class="text-green-400">@myteam/shared-utils</span> <span class="text-gray-500"># Team package</span></code></pre>
						</div>
					</div>

					<div>
						<h4 class="font-medium mb-2">Example Usage</h4>
						<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
							<pre><code><span class="text-purple-400">import</span> {"{ "}<span class="text-yellow-400">helpers</span> {"} "}<span class="text-purple-400">from</span> <span class="text-green-400">&apos;@myteam/shared-utils&apos;</span>;</code></pre>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Best Practices -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Best Practices</h2>

		<div class="space-y-6">
			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="solar:lightbulb-bold" class="w-5 h-5 mr-2 text-yellow-500" />
					Team Organization
				</h3>
				<ul class="text-sm text-muted-foreground space-y-2">
					<li>• Use descriptive team names that reflect your organization structure</li>
					<li>• Create separate teams for different projects or departments</li>
					<li>• Use consistent naming conventions for team packages</li>
					<li>• Document team packages well for other team members</li>
				</ul>
			</div>

			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="solar:shield-check-bold" class="w-5 h-5 mr-2 text-green-500" />
					Security
				</h3>
				<ul class="text-sm text-muted-foreground space-y-2">
					<li>• Regularly review team membership and remove inactive users</li>
					<li>• Use the principle of least privilege when assigning roles</li>
					<li>• Monitor package publications and changes</li>
					<li>• Keep sensitive information out of package descriptions</li>
				</ul>
			</div>

			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="solar:users-group-rounded-bold" class="w-5 h-5 mr-2 text-blue-500" />
					Collaboration
				</h3>
				<ul class="text-sm text-muted-foreground space-y-2">
					<li>• Use semantic versioning for team packages</li>
					<li>• Maintain changelog files for significant updates</li>
					<li>• Test packages thoroughly before publishing</li>
					<li>• Communicate breaking changes to team members</li>
				</ul>
			</div>
		</div>
	</section>

	<!-- Next Steps -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Next Steps</h2>
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<a href="/docs/authentication" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:key-bold" class="w-6 h-6 text-blue-600" />
					<h3 class="font-semibold">Authentication</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Set up authentication to start creating and managing teams.
				</p>
			</a>

			<a href="/docs/publishing" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:upload-bold" class="w-6 h-6 text-red-600" />
					<h3 class="font-semibold">Publishing Packages</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Learn how to publish and manage your team packages.
				</p>
			</a>

			<a href="/docs/permissions" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:shield-user-bold" class="w-6 h-6 text-green-600" />
					<h3 class="font-semibold">Permissions</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Understand detailed permission models for teams and packages.
				</p>
			</a>

			<a href="/docs/troubleshooting" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:help-bold" class="w-6 h-6 text-purple-600" />
					<h3 class="font-semibold">Troubleshooting</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Get help with common team management issues and solutions.
				</p>
			</a>
		</div>
	</section>
</div>