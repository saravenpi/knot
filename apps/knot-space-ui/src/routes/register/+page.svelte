<script lang="ts">
	import { goto } from '$app/navigation';

	let username = '';
	let email = '';
	let password = '';
	let confirmPassword = '';
	let loading = false;
	let error = '';

	async function handleRegister() {
		// Validation
		if (!username || !email || !password || !confirmPassword) {
			error = 'Please fill in all fields';
			return;
		}

		if (password !== confirmPassword) {
			error = 'Passwords do not match';
			return;
		}

		if (password.length < 6) {
			error = 'Password must be at least 6 characters';
			return;
		}

		// Username validation
		if (!/^[a-zA-Z0-9][a-zA-Z0-9\-_]*$/.test(username)) {
			error = 'Username must start with alphanumeric character and contain only letters, numbers, hyphens, and underscores';
			return;
		}

		// Email validation
		if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email)) {
			error = 'Please enter a valid email address';
			return;
		}

		loading = true;
		error = '';

		try {
			const response = await fetch('/api/auth/register', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					username: username.trim(),
					email: email.trim().toLowerCase(),
					password
				})
			});

			const data = await response.json();

			if (response.ok) {
				// Store token
				localStorage.setItem('knot_token', data.token);
				// Redirect to home
				goto('/');
			} else {
				error = data.message || 'Registration failed';
			}
		} catch (err) {
			error = 'Network error. Please try again.';
			console.error('Registration error:', err);
		} finally {
			loading = false;
		}
	}

	function handleKeyPress(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			handleRegister();
		}
	}
</script>

<svelte:head>
	<title>Sign Up - Knot Space</title>
</svelte:head>

<div class="max-w-md mx-auto">
	<div class="text-center mb-8">
		<h1 class="text-3xl font-bold mb-2">Create Account</h1>
		<p class="text-muted-foreground">Join Knot Space and start publishing packages</p>
	</div>

	<div class="border rounded-lg p-6">
		{#if error}
			<div class="bg-destructive/10 text-destructive border border-destructive/20 rounded-md p-3 mb-4">
				{error}
			</div>
		{/if}

		<form on:submit|preventDefault={handleRegister}>
			<div class="space-y-4">
				<div>
					<label for="username" class="block text-sm font-medium mb-2">
						Username
					</label>
					<input
						id="username"
						type="text"
						bind:value={username}
						on:keypress={handleKeyPress}
						placeholder="Choose a username"
						class="w-full px-3 py-2 border border-input rounded-md bg-background focus:outline-none focus:ring-2 focus:ring-ring focus:border-transparent"
						disabled={loading}
						required
					/>
					<p class="text-xs text-muted-foreground mt-1">
						Letters, numbers, hyphens, and underscores only
					</p>
				</div>

				<div>
					<label for="email" class="block text-sm font-medium mb-2">
						Email
					</label>
					<input
						id="email"
						type="email"
						bind:value={email}
						on:keypress={handleKeyPress}
						placeholder="Enter your email"
						class="w-full px-3 py-2 border border-input rounded-md bg-background focus:outline-none focus:ring-2 focus:ring-ring focus:border-transparent"
						disabled={loading}
						required
					/>
				</div>

				<div>
					<label for="password" class="block text-sm font-medium mb-2">
						Password
					</label>
					<input
						id="password"
						type="password"
						bind:value={password}
						on:keypress={handleKeyPress}
						placeholder="Create a password"
						class="w-full px-3 py-2 border border-input rounded-md bg-background focus:outline-none focus:ring-2 focus:ring-ring focus:border-transparent"
						disabled={loading}
						required
					/>
					<p class="text-xs text-muted-foreground mt-1">
						At least 6 characters
					</p>
				</div>

				<div>
					<label for="confirmPassword" class="block text-sm font-medium mb-2">
						Confirm Password
					</label>
					<input
						id="confirmPassword"
						type="password"
						bind:value={confirmPassword}
						on:keypress={handleKeyPress}
						placeholder="Confirm your password"
						class="w-full px-3 py-2 border border-input rounded-md bg-background focus:outline-none focus:ring-2 focus:ring-ring focus:border-transparent"
						disabled={loading}
						required
					/>
				</div>

				<button
					type="submit"
					disabled={loading}
					class="w-full bg-primary text-primary-foreground hover:bg-primary/90 disabled:opacity-50 disabled:cursor-not-allowed px-4 py-2 rounded-md font-medium transition-colors"
				>
					{#if loading}
						<span class="flex items-center justify-center">
							<svg class="animate-spin -ml-1 mr-3 h-4 w-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
								<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
								<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
							</svg>
							Creating Account...
						</span>
					{:else}
						Create Account
					{/if}
				</button>
			</div>
		</form>
	</div>

	<div class="text-center mt-6">
		<p class="text-muted-foreground">
			Already have an account?
			<a href="/login" class="text-primary hover:text-primary/80 font-medium">
				Sign in
			</a>
		</p>
	</div>
</div>