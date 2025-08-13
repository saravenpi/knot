<script lang="ts">
	import { onMount } from 'svelte';
	import { teamsStore } from '../../../lib/stores';

	$: teams = $teamsStore.teams;
	$: loading = $teamsStore.loading;

	let showCreateForm = false;
	let teamName = '';
	let teamDescription = '';
	let createError = '';

	onMount(async () => {
		await teamsStore.loadTeams();
	});

	async function handleCreateTeam() {
		if (!teamName.trim()) {
			createError = 'Team name is required';
			return;
		}

		createError = '';

		try {
			await teamsStore.createTeam({
				name: teamName.trim(),
				description: teamDescription.trim() || undefined
			});
			
			// Reset form
			teamName = '';
			teamDescription = '';
			showCreateForm = false;
		} catch (error) {
			createError = error instanceof Error ? error.message : 'Failed to create team';
		}
	}

	function formatDate(dateString: string | Date) {
		const date = new Date(dateString);
		return date.toLocaleDateString();
	}
</script>

<svelte:head>
	<title>Teams - Knot Space</title>
</svelte:head>

<div class="space-y-6">
	<div class="flex justify-between items-center">
		<div>
			<h1 class="text-3xl font-bold tracking-tight">Teams</h1>
			<p class="text-muted-foreground mt-2">Manage your teams and collaborations</p>
		</div>
		
		<button
			on:click={() => showCreateForm = !showCreateForm}
			class="bg-black text-white hover:bg-black/90 px-4 py-2 rounded-md text-sm font-medium transition-colors"
		>
			{showCreateForm ? 'Cancel' : 'Create Team'}
		</button>
	</div>

	{#if showCreateForm}
		<div class="bg-card border border-border rounded-lg p-6">
			<h2 class="text-xl font-semibold mb-4">Create New Team</h2>
			
			{#if createError}
				<div class="bg-destructive/10 text-destructive border border-destructive/20 rounded-md p-3 mb-4">
					{createError}
				</div>
			{/if}

			<form on:submit|preventDefault={handleCreateTeam} class="space-y-4">
				<div>
					<label for="teamName" class="block text-sm font-medium mb-2">
						Team Name
					</label>
					<input
						id="teamName"
						type="text"
						bind:value={teamName}
						placeholder="Enter team name"
						class="w-full px-3 py-2 border border-input rounded-md bg-background focus:outline-none focus:ring-2 focus:ring-ring focus:border-transparent"
						required
					/>
					<p class="text-xs text-muted-foreground mt-1">
						Lowercase letters, numbers, and hyphens only
					</p>
				</div>

				<div>
					<label for="teamDescription" class="block text-sm font-medium mb-2">
						Description (Optional)
					</label>
					<textarea
						id="teamDescription"
						bind:value={teamDescription}
						placeholder="Describe your team"
						rows="3"
						class="w-full px-3 py-2 border border-input rounded-md bg-background focus:outline-none focus:ring-2 focus:ring-ring focus:border-transparent"
					></textarea>
				</div>

				<div class="flex space-x-3">
					<button
						type="submit"
						class="bg-black text-white hover:bg-black/90 px-4 py-2 rounded-md text-sm font-medium transition-colors"
					>
						Create Team
					</button>
					<button
						type="button"
						on:click={() => showCreateForm = false}
						class="border border-input bg-background hover:bg-accent hover:text-accent-foreground px-4 py-2 rounded-md text-sm font-medium transition-colors"
					>
						Cancel
					</button>
				</div>
			</form>
		</div>
	{/if}

	{#if loading}
		<div class="flex items-center justify-center py-16">
			<div class="text-center">
				<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-black mx-auto mb-4"></div>
				<p class="text-muted-foreground">Loading teams...</p>
			</div>
		</div>
	{:else if teams.length === 0}
		<div class="text-center py-16">
			<div class="mb-4">
				<svg class="w-16 h-16 text-muted-foreground mx-auto" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
				</svg>
			</div>
			<h3 class="text-lg font-semibold mb-2">No teams yet</h3>
			<p class="text-muted-foreground mb-6">Create your first team to start collaborating with others.</p>
			<button
				on:click={() => showCreateForm = true}
				class="bg-black text-white hover:bg-black/90 px-4 py-2 rounded-md text-sm font-medium transition-colors"
			>
				Create Your First Team
			</button>
		</div>
	{:else}
		<div class="grid gap-6">
			{#each teams as team}
				<div class="bg-card border border-border rounded-lg p-6">
					<div class="flex justify-between items-start mb-4">
						<div>
							<h3 class="text-lg font-semibold">{team.name}</h3>
							{#if team.description}
								<p class="text-muted-foreground mt-1">{team.description}</p>
							{/if}
						</div>
						<div class="flex items-center space-x-2">
							<span class="text-sm text-muted-foreground">
								{team.members.length} member{team.members.length !== 1 ? 's' : ''}
							</span>
						</div>
					</div>

					<div class="space-y-3">
						<h4 class="text-sm font-medium">Members</h4>
						<div class="space-y-2">
							{#each team.members as member}
								<div class="flex justify-between items-center py-2">
									<div class="flex items-center space-x-3">
										<div class="w-8 h-8 bg-muted rounded-full flex items-center justify-center">
											<span class="text-xs font-medium">
												{member.user.username.charAt(0).toUpperCase()}
											</span>
										</div>
										<div>
											<div class="font-medium text-sm">{member.user.username}</div>
											<div class="text-xs text-muted-foreground">{member.user.email}</div>
										</div>
									</div>
									<span class="text-xs px-2 py-1 bg-muted rounded-md font-medium capitalize">
										{member.role}
									</span>
								</div>
							{/each}
						</div>
					</div>

					<div class="mt-4 pt-4 border-t border-border">
						<div class="flex justify-between items-center text-sm text-muted-foreground">
							<span>Created {formatDate(team.createdAt || new Date())}</span>
							<button class="text-black hover:underline font-medium">
								Manage Team
							</button>
						</div>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>