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

<div class="max-w-4xl mx-auto py-8 px-6">
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight mb-4" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
			Authentication
		</h1>
		<p class="text-lg text-muted-foreground">
			Set up authentication to publish and manage packages with Knot CLI
		</p>
	</div>

	<!-- Overview -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">How Authentication Works</h2>
		<div class="space-y-6">
			<p class="text-muted-foreground leading-relaxed">
				Knot uses JWT (JSON Web Token) based authentication for secure package publishing and management. 
				You'll need to authenticate with Knot Space to publish packages, manage teams, and access private packages.
			</p>
			
			<div class="bg-blue-50 border border-blue-200 rounded-lg p-6">
				<div class="flex items-start space-x-3">
					<Icon icon="solar:shield-check-bold" class="w-6 h-6 text-blue-600 mt-1 flex-shrink-0" />
					<div>
						<h3 class="font-semibold text-blue-900 mb-2">Secure by Default</h3>
						<p class="text-sm text-blue-700">
							Authentication tokens are stored securely and expire automatically. 
							You can revoke access at any time from your account settings.
						</p>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Getting Your Token -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Getting Your Authentication Token</h2>
		
		<div class="space-y-8">
			<!-- Step 1 -->
			<div>
				<div class="flex items-center space-x-3 mb-4">
					<div class="w-6 h-6 bg-green-100 rounded-full flex items-center justify-center">
						<span class="text-green-600 font-bold text-xs">1</span>
					</div>
					<h3 class="text-lg font-semibold">Create or Sign In to Your Account</h3>
				</div>
				<div class="ml-9 space-y-3">
					<p class="text-muted-foreground">
						First, you'll need a Knot Space account. You can create one or sign in if you already have an account.
					</p>
					<div class="flex flex-wrap gap-3">
						<a href="/register" class="inline-flex items-center px-4 py-2 bg-primary text-primary-foreground text-sm font-medium rounded-md hover:bg-primary/90 transition-colors">
							<Icon icon="solar:user-plus-bold" class="w-4 h-4 mr-2" />
							Create Account
						</a>
						<a href="/login" class="inline-flex items-center px-4 py-2 border border-input text-foreground text-sm font-medium rounded-md hover:bg-accent transition-colors">
							<Icon icon="solar:login-3-bold" class="w-4 h-4 mr-2" />
							Sign In
						</a>
					</div>
				</div>
			</div>

			<!-- Step 2 -->
			<div>
				<div class="flex items-center space-x-3 mb-4">
					<div class="w-6 h-6 bg-blue-100 rounded-full flex items-center justify-center">
						<span class="text-blue-600 font-bold text-xs">2</span>
					</div>
					<h3 class="text-lg font-semibold">Navigate to Settings</h3>
				</div>
				<div class="ml-9 space-y-3">
					<p class="text-muted-foreground">
						Once logged in, go to your account settings to access your authentication token.
					</p>
					<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
						<code>https://knot-space-production.up.railway.app/settings</code>
						<button 
							class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
							on:click={() => copyToClipboard('https://knot-space-production.up.railway.app/settings')}
						>
							{#if showCopied && copyText.includes('settings')}
								<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
							{:else}
								<Icon icon="solar:copy-bold" class="w-4 h-4" />
							{/if}
						</button>
					</div>
					<p class="text-sm text-muted-foreground">
						Or click the "Settings" link in your user menu after logging in.
					</p>
				</div>
			</div>

			<!-- Step 3 -->
			<div>
				<div class="flex items-center space-x-3 mb-4">
					<div class="w-6 h-6 bg-purple-100 rounded-full flex items-center justify-center">
						<span class="text-purple-600 font-bold text-xs">3</span>
					</div>
					<h3 class="text-lg font-semibold">Copy Your Authentication Token</h3>
				</div>
				<div class="ml-9 space-y-3">
					<p class="text-muted-foreground">
						In the settings page, you'll find your personal authentication token. 
						Click the copy button to copy it to your clipboard.
					</p>
					<div class="bg-yellow-50 border border-yellow-200 rounded-lg p-4">
						<div class="flex items-start space-x-2">
							<Icon icon="solar:shield-warning-bold" class="w-5 h-5 text-yellow-600 mt-0.5 flex-shrink-0" />
							<div>
								<div class="text-sm font-medium text-yellow-900 mb-1">Keep Your Token Secure</div>
								<div class="text-sm text-yellow-700">
									Your authentication token is like a password. Don't share it publicly 
									or commit it to version control. Store it securely in your environment.
								</div>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Setting Up CLI -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Configure Knot CLI</h2>
		
		<div class="space-y-6">
			<div>
				<h3 class="text-lg font-semibold mb-3">Method 1: Environment Variable (Recommended)</h3>
				<div class="space-y-3">
					<p class="text-muted-foreground">
						Set your token as an environment variable. This keeps it secure and makes it available to the CLI.
					</p>
					
					<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
						<!-- macOS/Linux -->
						<div>
							<h4 class="font-medium mb-2">macOS / Linux</h4>
							<div class="bg-black/90 text-green-400 font-mono text-sm p-3 rounded-lg relative group">
								<code>export KNOT_TOKEN=your-token-here</code>
								<button 
									class="absolute top-1 right-1 p-1.5 rounded bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
									on:click={() => copyToClipboard('export KNOT_TOKEN=your-token-here')}
								>
									{#if showCopied && copyText.includes('export')}
										<Icon icon="solar:check-circle-bold" class="w-3 h-3 text-green-400" />
									{:else}
										<Icon icon="solar:copy-bold" class="w-3 h-3" />
									{/if}
								</button>
							</div>
							<div class="text-xs text-muted-foreground mt-1">
								Add to <code>~/.bashrc</code> or <code>~/.zshrc</code> for persistence
							</div>
						</div>

						<!-- Windows -->
						<div>
							<h4 class="font-medium mb-2">Windows</h4>
							<div class="bg-blue-900 text-blue-100 font-mono text-sm p-3 rounded-lg relative group">
								<code>set KNOT_TOKEN=your-token-here</code>
								<button 
									class="absolute top-1 right-1 p-1.5 rounded bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
									on:click={() => copyToClipboard('set KNOT_TOKEN=your-token-here')}
								>
									{#if showCopied && copyText.includes('set KNOT_TOKEN')}
										<Icon icon="solar:check-circle-bold" class="w-3 h-3 text-green-400" />
									{:else}
										<Icon icon="solar:copy-bold" class="w-3 h-3" />
									{/if}
								</button>
							</div>
							<div class="text-xs text-muted-foreground mt-1">
								Or use <code>setx</code> for system-wide persistence
							</div>
						</div>
					</div>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Method 2: Configuration File</h3>
				<div class="space-y-3">
					<p class="text-muted-foreground">
						Alternatively, you can store your token in a configuration file.
					</p>
					<div class="bg-black/90 text-white font-mono text-sm p-4 rounded-lg">
						<code><span class="text-gray-400"># ~/.knotrc</span>
<span class="text-blue-400">KNOT_TOKEN</span>=<span class="text-green-400">your-token-here</span>
<span class="text-blue-400">KNOT_SPACE_URL</span>=<span class="text-green-400">https://knot-space-production.up.railway.app</span></code>
					</div>
					<div class="text-sm text-muted-foreground">
						The CLI will automatically read from <code>~/.knotrc</code> if it exists.
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Verification -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Verify Authentication</h2>
		
		<div class="space-y-6">
			<div>
				<h3 class="text-lg font-semibold mb-3">Test your authentication</h3>
				<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
					<code>knot auth</code>
					<button 
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
						on:click={() => copyToClipboard('knot auth')}
					>
						{#if showCopied && copyText === 'knot auth'}
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="solar:copy-bold" class="w-4 h-4" />
						{/if}
					</button>
				</div>
				<div class="mt-2 space-y-2">
					<div class="text-sm text-muted-foreground">
						<strong>Success:</strong> You should see your username and authentication status.
					</div>
					<div class="bg-green-50 border border-green-200 rounded p-3 text-sm">
						<code class="text-green-800">✓ Authenticated as username</code>
					</div>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Common issues</h3>
				<div class="space-y-4">
					<div class="border rounded-lg p-4">
						<h4 class="font-medium mb-2 flex items-center">
							<Icon icon="solar:danger-triangle-bold" class="w-4 h-4 mr-2 text-red-600" />
							"Authentication failed"
						</h4>
						<ul class="text-sm text-muted-foreground space-y-1 ml-6">
							<li>• Check that your token is correct and not expired</li>
							<li>• Ensure there are no extra spaces in the token</li>
							<li>• Verify the environment variable is set correctly</li>
						</ul>
					</div>

					<div class="border rounded-lg p-4">
						<h4 class="font-medium mb-2 flex items-center">
							<Icon icon="solar:wifi-router-bold" class="w-4 h-4 mr-2 text-orange-600" />
							"Connection failed"
						</h4>
						<ul class="text-sm text-muted-foreground space-y-1 ml-6">
							<li>• Check your internet connection</li>
							<li>• Verify the KNOT_SPACE_URL is correct</li>
							<li>• Check if you're behind a corporate firewall</li>
						</ul>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Self-Hosted -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Self-Hosted Instances</h2>
		
		<div class="space-y-6">
			<p class="text-muted-foreground">
				If you're using a self-hosted Knot Space instance, you'll need to configure the CLI 
				to point to your instance URL.
			</p>

			<div>
				<h3 class="text-lg font-semibold mb-3">Configure custom URL</h3>
				<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
					<code>export KNOT_SPACE_URL=https://your-knot-instance.com
export KNOT_TOKEN=your-auth-token</code>
					<button 
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
						on:click={() => copyToClipboard('export KNOT_SPACE_URL=https://your-knot-instance.com\nexport KNOT_TOKEN=your-auth-token')}
					>
						{#if showCopied && copyText.includes('your-knot-instance')}
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="solar:copy-bold" class="w-4 h-4" />
						{/if}
					</button>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Verify connection</h3>
				<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
					<code>knot auth</code>
					<button 
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
						on:click={() => copyToClipboard('knot auth')}
					>
						{#if showCopied && copyText === 'knot auth'}
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="solar:copy-bold" class="w-4 h-4" />
						{/if}
					</button>
				</div>
				<div class="mt-2 text-sm text-muted-foreground">
					This should connect to your self-hosted instance and verify authentication.
				</div>
			</div>
		</div>
	</section>

	<!-- Security -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Security Best Practices</h2>
		
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<div class="border rounded-lg p-6">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:eye-closed-bold" class="w-6 h-6 text-purple-600" />
					<h3 class="font-semibold">Keep Tokens Private</h3>
				</div>
				<ul class="text-sm text-muted-foreground space-y-1">
					<li>• Never commit tokens to version control</li>
					<li>• Don't share tokens in chat or email</li>
					<li>• Use environment variables or secure config files</li>
					<li>• Rotate tokens regularly</li>
				</ul>
			</div>

			<div class="border rounded-lg p-6">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:shield-network-bold" class="w-6 h-6 text-green-600" />
					<h3 class="font-semibold">Team Access</h3>
				</div>
				<ul class="text-sm text-muted-foreground space-y-1">
					<li>• Each team member should have their own token</li>
					<li>• Use team permissions to control access</li>
					<li>• Revoke access when team members leave</li>
					<li>• Monitor authentication logs</li>
				</ul>
			</div>

			<div class="border rounded-lg p-6">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:refresh-bold" class="w-6 h-6 text-blue-600" />
					<h3 class="font-semibold">Token Management</h3>
				</div>
				<ul class="text-sm text-muted-foreground space-y-1">
					<li>• Tokens automatically expire for security</li>
					<li>• Generate new tokens from your settings</li>
					<li>• Old tokens are invalidated when new ones are created</li>
					<li>• Check token status with <code>knot auth</code></li>
				</ul>
			</div>

			<div class="border rounded-lg p-6">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:server-path-bold" class="w-6 h-6 text-red-600" />
					<h3 class="font-semibold">CI/CD Integration</h3>
				</div>
				<ul class="text-sm text-muted-foreground space-y-1">
					<li>• Store tokens as encrypted secrets</li>
					<li>• Use dedicated service accounts for automation</li>
					<li>• Limit permissions for automated publishing</li>
					<li>• Monitor and audit automated access</li>
				</ul>
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
					<h3 class="font-semibold text-green-900 mb-2">Authentication Complete!</h3>
					<p class="text-sm text-green-700 mb-4">
						Now that you're authenticated, you can start publishing packages and collaborating with teams.
					</p>
					<div class="flex flex-wrap gap-3">
						<a href="/docs/quick-start" class="inline-flex items-center px-4 py-2 bg-green-600 text-white text-sm font-medium rounded-md hover:bg-green-700 transition-colors">
							<Icon icon="solar:play-bold" class="w-4 h-4 mr-2" />
							Quick Start Guide
						</a>
						<a href="/docs/publishing" class="inline-flex items-center px-4 py-2 border border-green-600 text-green-600 text-sm font-medium rounded-md hover:bg-green-50 transition-colors">
							<Icon icon="solar:upload-bold" class="w-4 h-4 mr-2" />
							Publishing Packages
						</a>
					</div>
				</div>
			</div>
		</div>
	</section>
</div>