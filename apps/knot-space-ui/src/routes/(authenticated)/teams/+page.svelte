<script lang="ts">
	import { onMount } from 'svelte';
	import { teamsStore, authStore } from '../../../lib/stores';
	import { usersApi } from '../../../lib/api';
	import Icon from '@iconify/svelte';
	import Drawer from '../../../lib/components/ui/drawer.svelte';
	import Input from '../../../lib/components/ui/input.svelte';
	import Button from '../../../lib/components/ui/button.svelte';
	import Textarea from '../../../lib/components/ui/textarea.svelte';
	import type { User } from '#/types';

	$: teams = $teamsStore.teams;
	$: loading = $teamsStore.loading;
	$: currentUser = $authStore.user;

	let showCreateForm = false;
	let teamName = '';
	let teamDescription = '';
	let createError = '';

	// Team management state
	let managingTeam = null;
	let showDeleteConfirm = false;
	let deleteTeamId = '';
	let deleteError = '';
	let showManageModal = false;
	let manageTeamId = '';

	// Member management state
	let showAddMember = false;
	let addMemberTeamId = '';
	let newMemberUsername = '';
	let newMemberRole = 'member';
	let memberError = '';
	
	// User search/autocomplete state
	let allUsers: User[] = [];
	let filteredUsers: User[] = [];
	let userSearchTerm = '';
	let showUserSuggestions = false;

	onMount(async () => {
		await teamsStore.fetchAll();
		// Load all users for autocomplete
		try {
			allUsers = await usersApi.getAllUsers();
		} catch (error) {
			console.error('Failed to load users:', error);
		}
	});

	// Filter users based on search term
	$: {
		if (userSearchTerm.trim()) {
			filteredUsers = allUsers
				.filter(user => 
					user.username.toLowerCase().includes(userSearchTerm.toLowerCase()) ||
					(user.email && user.email.toLowerCase().includes(userSearchTerm.toLowerCase()))
				)
				.slice(0, 5); // Limit to 5 suggestions
			showUserSuggestions = filteredUsers.length > 0;
		} else {
			filteredUsers = [];
			showUserSuggestions = false;
		}
	}

	function selectUser(user: User) {
		newMemberUsername = user.username;
		userSearchTerm = user.username;
		showUserSuggestions = false;
	}

	function getRoleIcon(role: string) {
		switch (role) {
			case 'owner':
				return 'solar:crown-bold';
			case 'admin':
				return 'solar:shield-check-bold';
			case 'member':
				return 'solar:user-bold';
			default:
				return 'solar:user-bold';
		}
	}

	function getRoleColor(role: string) {
		switch (role) {
			case 'owner':
				return 'text-amber-500';
			case 'admin':
				return 'text-blue-500';
			case 'member':
				return 'text-gray-500';
			default:
				return 'text-gray-500';
		}
	}

	function getUserInitials(username: string) {
		return username.charAt(0).toUpperCase();
	}

	async function handleCreateTeam() {
		if (!teamName.trim()) {
			createError = 'Team name is required';
			return;
		}

		createError = '';

		try {
			await teamsStore.create({
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

	function isTeamOwnerOrAdmin(team: any): boolean {
		if (!currentUser || !team.members) return false;
		const member = team.members.find(m => m.user.id === currentUser.id);
		return member && (member.role === 'owner' || member.role === 'admin');
	}

	async function handleDeleteTeam(teamId: string) {
		deleteError = '';
		try {
			await teamsStore.delete(teamId);
			showDeleteConfirm = false;
			deleteTeamId = '';
		} catch (error) {
			deleteError = error instanceof Error ? error.message : 'Failed to delete team';
		}
	}

	async function handleAddMember() {
		if (!newMemberUsername.trim()) {
			memberError = 'Username is required';
			return;
		}

		memberError = '';
		try {
			await teamsStore.addMember(addMemberTeamId, newMemberUsername.trim(), newMemberRole);
			
			// Reset form
			newMemberUsername = '';
			userSearchTerm = '';
			newMemberRole = 'member';
			showAddMember = false;
			addMemberTeamId = '';
			showUserSuggestions = false;
		} catch (error) {
			memberError = error instanceof Error ? error.message : 'Failed to add member';
		}
	}

	function openManageTeam(teamId: string) {
		manageTeamId = teamId;
		showManageModal = true;
	}

	async function handleRemoveMember(teamId: string, userId: string) {
		try {
			await teamsStore.removeMember(teamId, userId);
		} catch (error) {
			console.error('Failed to remove member:', error);
		}
	}

	async function handleUpdateMemberRole(teamId: string, userId: string, newRole: string) {
		try {
			await teamsStore.updateMemberRole(teamId, userId, newRole);
		} catch (error) {
			console.error('Failed to update member role:', error);
		}
	}
</script>

<svelte:head>
	<title>Teams - Knot Space</title>
</svelte:head>

<div class="space-y-6">
	<div class="flex flex-col sm:flex-row sm:justify-between sm:items-center gap-4">
		<div>
			<h1 class="text-3xl font-bold tracking-tight" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">Teams</h1>
			<p class="text-muted-foreground mt-2">Manage your teams and collaborations</p>
		</div>
		
		<Button
			on:click={() => showCreateForm = !showCreateForm}
			class="w-full sm:w-auto"
		>
			<Icon icon={showCreateForm ? "solar:close-circle-bold" : "solar:add-circle-bold"} class="w-4 h-4 mr-2" />
			{showCreateForm ? 'Cancel' : 'Create Team'}
		</Button>
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
					<Input
						id="teamName"
						type="text"
						bind:value={teamName}
						placeholder="Enter team name"
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
					<Textarea
						id="teamDescription"
						bind:value={teamDescription}
						placeholder="Describe your team"
						rows="3"
					/>
				</div>

				<div class="flex flex-col sm:flex-row gap-3">
					<Button
						type="submit"
						class="w-full sm:w-auto"
					>
						<Icon icon="solar:check-circle-bold" class="w-4 h-4 mr-2" />
						Create Team
					</Button>
					<Button
						type="button"
						on:click={() => showCreateForm = false}
						variant="outline"
						class="w-full sm:w-auto"
					>
						<Icon icon="solar:close-circle-bold" class="w-4 h-4 mr-2" />
						Cancel
					</Button>
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
				<Icon icon="solar:users-group-two-rounded-bold" class="w-16 h-16 text-muted-foreground mx-auto" />
			</div>
			<h3 class="text-lg font-semibold mb-2">No teams yet</h3>
			<p class="text-muted-foreground mb-6">Create your first team to start collaborating with others.</p>
			<button
				on:click={() => showCreateForm = true}
				class="bg-black text-white hover:bg-black/90 px-4 py-2 rounded-md text-sm font-medium transition-colors flex items-center justify-center space-x-2 mx-auto w-full sm:w-auto max-w-xs"
			>
				<Icon icon="solar:add-circle-bold" class="w-4 h-4" />
				<span>Create Your First Team</span>
			</button>
		</div>
	{:else}
		<div class="grid gap-6">
			{#each teams as team}
				<div class="bg-card border border-border rounded-lg p-4 sm:p-6">
					<div class="flex flex-col sm:flex-row sm:justify-between sm:items-start gap-4 mb-4">
						<div class="flex-1">
							<h3 class="text-lg font-semibold">{team.name}</h3>
							{#if team.description}
								<p class="text-muted-foreground mt-1">{team.description}</p>
							{/if}
							<span class="text-sm text-muted-foreground mt-2 sm:hidden block">
								{team.members?.length || 0} member{(team.members?.length || 0) !== 1 ? 's' : ''}
							</span>
						</div>
						<div class="flex flex-col sm:flex-row sm:items-center gap-3 sm:gap-2">
							<span class="text-sm text-muted-foreground hidden sm:block">
								{team.members?.length || 0} member{(team.members?.length || 0) !== 1 ? 's' : ''}
							</span>
							{#if isTeamOwnerOrAdmin(team)}
								<div class="flex gap-2 w-full sm:w-auto">
									<button
										on:click={() => {
											addMemberTeamId = team.id;
											showAddMember = true;
										}}
										class="text-xs bg-secondary hover:bg-secondary/80 text-secondary-foreground px-3 py-2 rounded transition-colors flex items-center gap-1 flex-1 sm:flex-initial justify-center sm:justify-start"
										title="Add Member"
									>
										<Icon icon="solar:user-plus-bold" class="w-4 h-4 sm:w-3 sm:h-3" />
										<span class="hidden sm:inline">Add Member</span>
									</button>
									<button
										on:click={() => {
											deleteTeamId = team.id;
											showDeleteConfirm = true;
										}}
										class="text-xs bg-destructive/10 hover:bg-destructive/20 text-destructive px-3 py-2 rounded transition-colors flex items-center gap-1 flex-1 sm:flex-initial justify-center sm:justify-start"
										title="Delete Team"
									>
										<Icon icon="solar:trash-bin-minimalistic-bold" class="w-4 h-4 sm:w-3 sm:h-3" />
										<span class="hidden sm:inline">Delete</span>
									</button>
								</div>
							{/if}
						</div>
					</div>

					<div class="space-y-3">
						<h4 class="text-sm font-medium">Members</h4>
						<div class="space-y-2">
							{#each (team.members || []) as member}
								<div class="flex flex-col sm:flex-row sm:justify-between sm:items-center gap-3 py-2">
									<div class="flex items-center space-x-3 flex-1 min-w-0">
										<div class="w-8 h-8 bg-muted rounded-full flex items-center justify-center flex-shrink-0">
											<span class="text-xs font-medium">
												{getUserInitials(member.user.username)}
											</span>
										</div>
										<div class="flex-1 min-w-0">
											<div class="font-medium text-sm flex items-center gap-2">
												<span class="truncate">{member.user.username}</span>
												<Icon 
													icon={getRoleIcon(member.role)} 
													class="w-3 h-3 {getRoleColor(member.role)} flex-shrink-0"
													title={member.role}
												/>
											</div>
											<div class="text-xs text-muted-foreground truncate">{member.user.email || 'No email'}</div>
										</div>
									</div>
									<div class="flex items-center gap-2 sm:flex-shrink-0">
										{#if isTeamOwnerOrAdmin(team) && member.role !== 'owner'}
											<select
												value={member.role}
												on:change={(e) => handleUpdateMemberRole(team.id, member.user.id, e.target.value)}
												class="text-xs px-2 py-1 bg-muted rounded-md font-medium border-none outline-none flex-1 sm:flex-initial"
											>
												<option value="member">Member</option>
												<option value="admin">Admin</option>
											</select>
											<button
												on:click={() => handleRemoveMember(team.id, member.user.id)}
												class="text-xs text-destructive hover:bg-destructive/10 p-2 rounded transition-colors flex-shrink-0"
											>
												<Icon icon="solar:close-circle-bold" class="w-3 h-3" />
											</button>
										{:else}
											<span class="text-xs px-2 py-1 bg-muted rounded-md font-medium capitalize">
												{member.role}
											</span>
										{/if}
									</div>
								</div>
							{/each}
						</div>
					</div>

					<div class="mt-4 pt-4 border-t border-border">
						<div class="flex flex-col sm:flex-row sm:justify-between sm:items-center gap-3 text-sm text-muted-foreground">
							<span>Created {formatDate(team.createdAt || new Date())}</span>
							<button 
								on:click={() => openManageTeam(team.id)}
								class="text-black hover:underline font-medium text-left sm:text-right"
							>
								Manage Team
							</button>
						</div>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>

<!-- Add Member Drawer -->
<Drawer
	bind:open={showAddMember}
	title="Add Team Member"
	description="Invite a user to join your team"
	side="right"
>
	<div class="space-y-6">
			{#if memberError}
				<div class="bg-destructive/10 text-destructive border border-destructive/20 rounded-md p-3">
					{memberError}
				</div>
			{/if}

			<form on:submit|preventDefault={handleAddMember} class="space-y-4">
				<div class="relative">
					<label for="memberUsername" class="block text-sm font-medium mb-2">Username</label>
					<Input
						id="memberUsername"
						type="text"
						bind:value={userSearchTerm}
						on:input={() => {
							newMemberUsername = userSearchTerm;
						}}
						placeholder="Start typing to search users..."
						required
					/>
					
					<!-- User suggestions dropdown -->
					{#if showUserSuggestions}
						<div class="absolute z-10 w-full mt-1 bg-background border border-border rounded-md shadow-lg max-h-60 overflow-y-auto">
							{#each filteredUsers as user}
								<button
									type="button"
									on:click={() => selectUser(user)}
									class="w-full flex items-center gap-3 px-3 py-2 hover:bg-muted text-left transition-colors"
								>
									<div class="w-8 h-8 bg-muted rounded-full flex items-center justify-center">
										<span class="text-xs font-medium">
											{getUserInitials(user.username)}
										</span>
									</div>
									<div class="flex-1 min-w-0">
										<div class="font-medium text-sm text-foreground">{user.username}</div>
										<div class="text-xs text-muted-foreground truncate">{user.email || 'No email'}</div>
									</div>
								</button>
							{/each}
						</div>
					{/if}
				</div>

				<div>
					<label for="memberRole" class="block text-sm font-medium mb-2">Role</label>
					<select
						id="memberRole"
						bind:value={newMemberRole}
						class="w-full px-3 py-2 border border-input rounded-md bg-background focus:outline-none focus:ring-2 focus:ring-ring focus:border-transparent"
					>
						<option value="member">Member</option>
						<option value="admin">Admin</option>
					</select>
				</div>

				<div class="flex flex-col gap-3 pt-4 border-t border-border">
					<Button
						type="submit"
					>
						<Icon icon="solar:user-plus-bold" class="mr-2 h-4 w-4" />
						Add Member
					</Button>
					<Button
						type="button"
						variant="outline"
						on:click={() => { 
							showAddMember = false; 
							addMemberTeamId = ''; 
							memberError = '';
							newMemberUsername = '';
							userSearchTerm = '';
							newMemberRole = 'member';
							showUserSuggestions = false;
						}}
					>
						Cancel
					</Button>
				</div>
			</form>
		</div>
</Drawer>

<!-- Delete Team Confirmation Drawer -->
<Drawer
	bind:open={showDeleteConfirm}
	title="Delete Team"
	description="This action cannot be undone"
	side="right"
>
	<div class="space-y-6">
			<div class="flex items-center gap-3 mb-4">
				<Icon icon="solar:danger-triangle-bold" class="w-6 h-6 text-destructive" />
				<h3 class="text-lg font-semibold">Delete Team</h3>
			</div>
			
			<p class="text-muted-foreground mb-4">
				Are you sure you want to delete this team? This action cannot be undone and will remove all team members.
			</p>

			{#if deleteError}
				<div class="bg-destructive/10 text-destructive border border-destructive/20 rounded-md p-3 mb-4">
					{deleteError}
				</div>
			{/if}

			<div class="flex flex-col sm:flex-row justify-end gap-3">
				<Button
					on:click={() => { showDeleteConfirm = false; deleteTeamId = ''; deleteError = ''; }}
					variant="outline"
					class="w-full sm:w-auto order-2 sm:order-1"
				>
					Cancel
				</Button>
				<Button
					on:click={() => handleDeleteTeam(deleteTeamId)}
					variant="destructive"
					class="w-full sm:w-auto order-1 sm:order-2"
				>
					Delete Team
				</Button>
			</div>
		</div>
{/if}

<!-- Team Management Drawer -->
<Drawer
	bind:open={showManageModal}
	title="Manage Team"
	description="View and manage team members and settings"
	side="right"
>
	{#if teams.find(t => t.id === manageTeamId)}
		{@const managedTeam = teams.find(t => t.id === manageTeamId)}
		<div class="space-y-6">
			<!-- Team Info -->
			<div>
				<h4 class="text-md font-medium mb-3">Team Information</h4>
				<div class="bg-muted/30 rounded-lg p-4 space-y-2">
					<div class="flex justify-between">
						<span class="text-sm text-muted-foreground">Name:</span>
						<span class="text-sm font-medium">{managedTeam.name}</span>
					</div>
					{#if managedTeam.description}
						<div class="flex justify-between">
							<span class="text-sm text-muted-foreground">Description:</span>
							<span class="text-sm font-medium">{managedTeam.description}</span>
						</div>
					{/if}
					<div class="flex justify-between">
						<span class="text-sm text-muted-foreground">Members:</span>
						<span class="text-sm font-medium">{managedTeam.members?.length || 0}</span>
					</div>
					<div class="flex justify-between">
						<span class="text-sm text-muted-foreground">Created:</span>
						<span class="text-sm font-medium">{formatDate(managedTeam.createdAt || new Date())}</span>
					</div>
				</div>
			</div>

			<!-- Members List -->
			<div>
				<div class="flex justify-between items-center mb-3">
					<h4 class="text-md font-medium">Team Members</h4>
					{#if isTeamOwnerOrAdmin(managedTeam)}
						<Button
							on:click={() => {
								addMemberTeamId = managedTeam.id;
								showAddMember = true;
								showManageModal = false;
							}}
							size="sm"
						>
							<Icon icon="solar:user-plus-bold" class="w-3 h-3 mr-1" />
							Add Member
						</Button>
					{/if}
				</div>
				
				<div class="space-y-3 max-h-60 overflow-y-auto">
					{#each (managedTeam.members || []) as member}
						<div class="flex items-center justify-between p-3 bg-muted/30 rounded-lg">
							<div class="flex items-center space-x-3">
								<div class="w-10 h-10 bg-muted rounded-full flex items-center justify-center">
									<span class="text-sm font-medium">
										{getUserInitials(member.user.username)}
									</span>
								</div>
								<div>
									<div class="font-medium text-sm flex items-center gap-2">
										{member.user.username}
										<Icon 
											icon={getRoleIcon(member.role)} 
											class="w-4 h-4 {getRoleColor(member.role)}"
											title={member.role}
										/>
									</div>
									<div class="text-xs text-muted-foreground">{member.user.email || 'No email'}</div>
									<div class="text-xs text-muted-foreground capitalize">{member.role}</div>
								</div>
							</div>
							
							{#if isTeamOwnerOrAdmin(managedTeam) && member.role !== 'owner'}
								<div class="flex items-center gap-2">
									<select
										value={member.role}
										on:change={(e) => handleUpdateMemberRole(managedTeam.id, member.user.id, e.target.value)}
										class="text-xs px-2 py-1 bg-background border border-border rounded-md font-medium"
									>
										<option value="member">Member</option>
										<option value="admin">Admin</option>
									</select>
									<button
										on:click={() => handleRemoveMember(managedTeam.id, member.user.id)}
										class="text-xs text-destructive hover:bg-destructive/10 p-2 rounded transition-colors"
										title="Remove member"
									>
										<Icon icon="solar:trash-bin-minimalistic-bold" class="w-4 h-4" />
									</button>
								</div>
							{:else}
								<span class="text-xs px-2 py-1 bg-muted rounded-md font-medium capitalize">
									{member.role}
								</span>
							{/if}
						</div>
					{/each}
				</div>
			</div>

			<div class="flex justify-end gap-3 pt-4 border-t border-border">
				<Button
					on:click={() => { 
						showManageModal = false; 
						manageTeamId = ''; 
					}}
					variant="outline"
				>
					Close
				</Button>
			</div>
		</div>
	{/if}
</Drawer>