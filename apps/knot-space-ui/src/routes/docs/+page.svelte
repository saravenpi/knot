<script lang="ts">
	import Icon from '@iconify/svelte';
</script>

<svelte:head>
	<title>Documentation - Knot Space</title>
	<meta name="description" content="Complete guide to using Knot CLI for monorepo package management" />
</svelte:head>

<div class="max-w-4xl mx-auto py-8">
	<div class="mb-12">
		<h1 class="text-4xl font-bold tracking-tight mb-4" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
			Knot Documentation
		</h1>
		<p class="text-xl text-muted-foreground">
			Everything you need to know about building monorepos with Knot
		</p>
	</div>

	<!-- Table of Contents -->
	<div class="bg-muted/30 rounded-lg p-6 mb-12">
		<h2 class="text-lg font-semibold mb-4 flex items-center">
			<Icon icon="solar:list-bold" class="w-5 h-5 mr-2" />
			Table of Contents
		</h2>
		<div class="grid grid-cols-1 md:grid-cols-2 gap-2">
			<a href="#getting-started" class="text-sm hover:text-primary transition-colors">→ Getting Started</a>
			<a href="#project-structure" class="text-sm hover:text-primary transition-colors">→ Project Structure</a>
			<a href="#configuration" class="text-sm hover:text-primary transition-colors">→ Configuration Files</a>
			<a href="#cli-commands" class="text-sm hover:text-primary transition-colors">→ CLI Commands</a>
			<a href="#package-linking" class="text-sm hover:text-primary transition-colors">→ Package Linking</a>
			<a href="#publishing" class="text-sm hover:text-primary transition-colors">→ Publishing Packages</a>
			<a href="#team-management" class="text-sm hover:text-primary transition-colors">→ Team Management</a>
			<a href="#self-hosting" class="text-sm hover:text-primary transition-colors">→ Self-Hosting</a>
		</div>
	</div>

	<!-- Getting Started -->
	<section id="getting-started" class="mb-16">
		<div class="flex items-center space-x-3 mb-6">
			<div class="w-8 h-8 bg-green-100 rounded-lg flex items-center justify-center">
				<Icon icon="solar:rocket-2-bold" class="w-5 h-5 text-green-600" />
			</div>
			<h2 class="text-2xl font-bold">Getting Started</h2>
		</div>

		<div class="space-y-6">
			<div>
				<h3 class="text-lg font-semibold mb-3">Installation</h3>
				<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded overflow-x-auto relative">
					<code>curl -fsSL https://raw.githubusercontent.com/saravenpi/knot/main/install.sh | bash</code>
					<button 
						class="absolute top-2 right-2 p-1.5 rounded bg-white/10 hover:bg-white/20 transition-colors"
						onclick="navigator.clipboard.writeText('curl -fsSL https://raw.githubusercontent.com/saravenpi/knot/main/install.sh | bash')"
						title="Copy to clipboard"
					>
						<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"></path>
						</svg>
					</button>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Authentication</h3>
				<p class="text-muted-foreground mb-3">
					Knot uses token-based authentication. Get your token from the web interface:
				</p>
				<div class="space-y-3">
					<div class="bg-muted/30 rounded-lg p-4">
						<div class="text-sm text-muted-foreground mb-2">1. Visit Knot Space and go to Settings</div>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-3 rounded">
							<code>https://knot-space-production.up.railway.app/settings</code>
						</div>
					</div>
					<div class="bg-muted/30 rounded-lg p-4">
						<div class="text-sm text-muted-foreground mb-2">2. Set your authentication token</div>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-3 rounded">
							<code>export KNOT_TOKEN=your-token-here</code>
						</div>
					</div>
					<div class="bg-muted/30 rounded-lg p-4">
						<div class="text-sm text-muted-foreground mb-2">3. Verify authentication</div>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-3 rounded">
							<code>knot auth</code>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Project Structure -->
	<section id="project-structure" class="mb-16">
		<div class="flex items-center space-x-3 mb-6">
			<div class="w-8 h-8 bg-blue-100 rounded-lg flex items-center justify-center">
				<Icon icon="solar:folder-bold" class="w-5 h-5 text-blue-600" />
			</div>
			<h2 class="text-2xl font-bold">Project Structure</h2>
		</div>

		<div class="bg-muted/30 rounded-lg p-6 mb-6">
			<pre class="text-sm font-mono"><code>MyProject/
├── <span class="text-blue-400">knot.yml</span>          # Project configuration
├── <span class="text-yellow-400">packages/</span>        # Reusable packages
│   ├── types/
│   │   └── <span class="text-blue-400">package.yml</span>
│   └── utils/
│       └── <span class="text-blue-400">package.yml</span>
└── <span class="text-green-400">apps/</span>             # Applications
    └── frontend/
        └── <span class="text-blue-400">app.yml</span></code></pre>
		</div>

		<div class="grid grid-cols-1 md:grid-cols-3 gap-6">
			<div class="border rounded-lg p-4">
				<div class="flex items-center space-x-2 mb-2">
					<Icon icon="solar:settings-bold" class="w-4 h-4 text-blue-600" />
					<span class="font-semibold">Project Root</span>
				</div>
				<p class="text-sm text-muted-foreground">Contains <code>knot.yml</code> with project configuration and global scripts</p>
			</div>
			<div class="border rounded-lg p-4">
				<div class="flex items-center space-x-2 mb-2">
					<Icon icon="solar:box-bold" class="w-4 h-4 text-yellow-600" />
					<span class="font-semibold">Packages</span>
				</div>
				<p class="text-sm text-muted-foreground">Reusable libraries with <code>package.yml</code> configuration</p>
			</div>
			<div class="border rounded-lg p-4">
				<div class="flex items-center space-x-2 mb-2">
					<Icon icon="solar:widget-bold" class="w-4 h-4 text-green-600" />
					<span class="font-semibold">Apps</span>
				</div>
				<p class="text-sm text-muted-foreground">Applications with <code>app.yml</code> that consume packages</p>
			</div>
		</div>
	</section>

	<!-- Configuration -->
	<section id="configuration" class="mb-16">
		<div class="flex items-center space-x-3 mb-6">
			<div class="w-8 h-8 bg-purple-100 rounded-lg flex items-center justify-center">
				<Icon icon="solar:document-text-bold" class="w-5 h-5 text-purple-600" />
			</div>
			<h2 class="text-2xl font-bold">Configuration Files</h2>
		</div>

		<div class="space-y-8">
			<div>
				<h3 class="text-lg font-semibold mb-3">knot.yml (Project)</h3>
				<pre class="text-sm font-mono bg-black/90 text-white p-4 rounded overflow-x-auto"><code><span class="text-blue-400">name:</span> <span class="text-green-400">MyProject</span>
<span class="text-blue-400">description:</span> <span class="text-green-400">A modern TypeScript monorepo</span>
<span class="text-blue-400">apps:</span>
  <span class="text-blue-400">frontend:</span>
    <span class="text-blue-400">tsAlias:</span> <span class="text-green-400">"#"</span>
    <span class="text-blue-400">packages:</span> <span class="text-yellow-400">[types, utils, "@jwt"]</span>
<span class="text-blue-400">scripts:</span>
  <span class="text-blue-400">setup:</span> <span class="text-green-400">"npm install"</span>
  <span class="text-blue-400">test-all:</span> <span class="text-green-400">"knot run test --all"</span></code></pre>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">package.yml (Package)</h3>
				<pre class="text-sm font-mono bg-black/90 text-white p-4 rounded overflow-x-auto"><code><span class="text-blue-400">name:</span> <span class="text-green-400">utils</span>
<span class="text-blue-400">team:</span> <span class="text-green-400">myteam</span>
<span class="text-blue-400">version:</span> <span class="text-green-400">1.0.0</span>
<span class="text-blue-400">description:</span> <span class="text-green-400">Shared utility functions</span>
<span class="text-blue-400">tags:</span> <span class="text-yellow-400">[utilities, helpers, typescript]</span>
<span class="text-blue-400">scripts:</span>
  <span class="text-blue-400">build:</span> <span class="text-green-400">"tsc && rollup -c"</span>
  <span class="text-blue-400">test:</span> <span class="text-green-400">"vitest run"</span></code></pre>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">app.yml (Application)</h3>
				<pre class="text-sm font-mono bg-black/90 text-white p-4 rounded overflow-x-auto"><code><span class="text-blue-400">name:</span> <span class="text-green-400">frontend</span>
<span class="text-blue-400">description:</span> <span class="text-green-400">React frontend application</span>
<span class="text-blue-400">build:</span> <span class="text-green-400">"npm run build"</span>
<span class="text-blue-400">packages:</span> <span class="text-yellow-400">[types, utils]</span>
<span class="text-blue-400">scripts:</span>
  <span class="text-blue-400">dev:</span> <span class="text-green-400">"vite dev --port 3000"</span>
  <span class="text-blue-400">test:</span> <span class="text-green-400">"vitest run"</span></code></pre>
			</div>
		</div>
	</section>

	<!-- CLI Commands -->
	<section id="cli-commands" class="mb-16">
		<div class="flex items-center space-x-3 mb-6">
			<div class="w-8 h-8 bg-gray-100 rounded-lg flex items-center justify-center">
				<Icon icon="solar:terminal-bold" class="w-5 h-5 text-gray-600" />
			</div>
			<h2 class="text-2xl font-bold">CLI Commands</h2>
		</div>

		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<div class="space-y-4">
				<h3 class="text-lg font-semibold">Project Management</h3>
				<div class="space-y-3">
					<div class="bg-muted/30 rounded-lg p-4">
						<div class="font-mono text-sm mb-2"><code>knot init &lt;name&gt;</code></div>
						<div class="text-sm text-muted-foreground">Initialize a new Knot project</div>
					</div>
					<div class="bg-muted/30 rounded-lg p-4">
						<div class="font-mono text-sm mb-2"><code>knot status</code></div>
						<div class="text-sm text-muted-foreground">Show project status and dependencies</div>
					</div>
					<div class="bg-muted/30 rounded-lg p-4">
						<div class="font-mono text-sm mb-2"><code>knot auth</code></div>
						<div class="text-sm text-muted-foreground">Check authentication status</div>
					</div>
				</div>
			</div>

			<div class="space-y-4">
				<h3 class="text-lg font-semibold">Package & App Creation</h3>
				<div class="space-y-3">
					<div class="bg-muted/30 rounded-lg p-4">
						<div class="font-mono text-sm mb-2"><code>knot init:package &lt;name&gt;</code></div>
						<div class="text-sm text-muted-foreground">Create a new package</div>
					</div>
					<div class="bg-muted/30 rounded-lg p-4">
						<div class="font-mono text-sm mb-2"><code>knot init:app &lt;name&gt;</code></div>
						<div class="text-sm text-muted-foreground">Create a new application</div>
					</div>
				</div>
			</div>

			<div class="space-y-4">
				<h3 class="text-lg font-semibold">Development</h3>
				<div class="space-y-3">
					<div class="bg-muted/30 rounded-lg p-4">
						<div class="font-mono text-sm mb-2"><code>knot link [--symlink]</code></div>
						<div class="text-sm text-muted-foreground">Link packages to apps and setup TypeScript aliases</div>
					</div>
					<div class="bg-muted/30 rounded-lg p-4">
						<div class="font-mono text-sm mb-2"><code>knot build</code></div>
						<div class="text-sm text-muted-foreground">Build apps using configured build commands</div>
					</div>
					<div class="bg-muted/30 rounded-lg p-4">
						<div class="font-mono text-sm mb-2"><code>knot run &lt;script&gt;</code></div>
						<div class="text-sm text-muted-foreground">Execute scripts from configuration files</div>
					</div>
				</div>
			</div>

			<div class="space-y-4">
				<h3 class="text-lg font-semibold">Publishing</h3>
				<div class="space-y-3">
					<div class="bg-muted/30 rounded-lg p-4">
						<div class="font-mono text-sm mb-2"><code>knot publish</code></div>
						<div class="text-sm text-muted-foreground">Publish package to Knot Space</div>
					</div>
					<div class="bg-muted/30 rounded-lg p-4">
						<div class="font-mono text-sm mb-2"><code>knot delete &lt;name&gt; &lt;version&gt;</code></div>
						<div class="text-sm text-muted-foreground">Delete package from registry</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Package Linking -->
	<section id="package-linking" class="mb-16">
		<div class="flex items-center space-x-3 mb-6">
			<div class="w-8 h-8 bg-orange-100 rounded-lg flex items-center justify-center">
				<Icon icon="solar:link-bold" class="w-5 h-5 text-orange-600" />
			</div>
			<h2 class="text-2xl font-bold">Package Linking</h2>
		</div>

		<div class="space-y-6">
			<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
				<div class="border rounded-lg p-6">
					<div class="flex items-center space-x-2 mb-3">
						<Icon icon="solar:copy-bold" class="w-5 h-5 text-orange-600" />
						<h3 class="text-lg font-semibold">Copy Mode (Default)</h3>
					</div>
					<div class="bg-black/90 text-green-400 font-mono text-sm p-3 rounded mb-3">
						<code>knot link</code>
					</div>
					<ul class="text-sm text-muted-foreground space-y-1">
						<li>• Packages copied to <code>knot_packages/</code></li>
						<li>• Better for Docker and deployment</li>
						<li>• No symlink compatibility issues</li>
						<li>• Files can be committed to git</li>
					</ul>
				</div>

				<div class="border rounded-lg p-6">
					<div class="flex items-center space-x-2 mb-3">
						<Icon icon="solar:link-bold" class="w-5 h-5 text-orange-600" />
						<h3 class="text-lg font-semibold">Symlink Mode</h3>
					</div>
					<div class="bg-black/90 text-green-400 font-mono text-sm p-3 rounded mb-3">
						<code>knot link --symlink</code>
					</div>
					<ul class="text-sm text-muted-foreground space-y-1">
						<li>• Real-time package updates</li>
						<li>• Great for active development</li>
						<li>• Changes reflected immediately</li>
						<li>• May not work in all environments</li>
					</ul>
				</div>
			</div>

			<div class="bg-blue-50 border border-blue-200 rounded-lg p-4">
				<div class="flex items-start space-x-2">
					<Icon icon="solar:info-circle-bold" class="w-5 h-5 text-blue-600 mt-0.5 flex-shrink-0" />
					<div>
						<div class="text-sm font-medium text-blue-900 mb-1">TypeScript Integration</div>
						<div class="text-sm text-blue-700">
							Knot automatically updates <code>tsconfig.json</code> with path mappings for your packages, 
							enabling clean imports like <code>import { utils } from '#/utils'</code>.
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Publishing -->
	<section id="publishing" class="mb-16">
		<div class="flex items-center space-x-3 mb-6">
			<div class="w-8 h-8 bg-green-100 rounded-lg flex items-center justify-center">
				<Icon icon="solar:upload-bold" class="w-5 h-5 text-green-600" />
			</div>
			<h2 class="text-2xl font-bold">Publishing Packages</h2>
		</div>

		<div class="space-y-6">
			<div>
				<h3 class="text-lg font-semibold mb-3">Basic Publishing</h3>
				<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded">
					<code>$ cd packages/utils
$ knot publish --description "Shared utility functions"
📦 Successfully published utils v1.0.0</code>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Team Publishing</h3>
				<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded">
					<code>$ knot publish --team myteam --description "Team utilities"
📦 Successfully published utils v1.0.0
   Team: myteam</code>
				</div>
			</div>

			<div class="bg-yellow-50 border border-yellow-200 rounded-lg p-4">
				<div class="flex items-start space-x-2">
					<Icon icon="solar:shield-warning-bold" class="w-5 h-5 text-yellow-600 mt-0.5 flex-shrink-0" />
					<div>
						<div class="text-sm font-medium text-yellow-900 mb-1">Publishing Requirements</div>
						<div class="text-sm text-yellow-700">
							Make sure you're authenticated (<code>knot auth</code>) and in a package directory 
							with a valid <code>package.yml</code> file before publishing.
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Team Management -->
	<section id="team-management" class="mb-16">
		<div class="flex items-center space-x-3 mb-6">
			<div class="w-8 h-8 bg-indigo-100 rounded-lg flex items-center justify-center">
				<Icon icon="solar:users-group-two-rounded-bold" class="w-5 h-5 text-indigo-600" />
			</div>
			<h2 class="text-2xl font-bold">Team Management</h2>
		</div>

		<div class="space-y-6">
			<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
				<div>
					<h3 class="text-lg font-semibold mb-3">Team Operations</h3>
					<div class="space-y-3">
						<div class="bg-muted/30 rounded-lg p-4">
							<div class="font-mono text-sm mb-2"><code>knot team create &lt;name&gt;</code></div>
							<div class="text-sm text-muted-foreground">Create a new team</div>
						</div>
						<div class="bg-muted/30 rounded-lg p-4">
							<div class="font-mono text-sm mb-2"><code>knot team list</code></div>
							<div class="text-sm text-muted-foreground">List all teams</div>
						</div>
						<div class="bg-muted/30 rounded-lg p-4">
							<div class="font-mono text-sm mb-2"><code>knot team info &lt;name&gt;</code></div>
							<div class="text-sm text-muted-foreground">Show team information</div>
						</div>
					</div>
				</div>

				<div>
					<h3 class="text-lg font-semibold mb-3">Member Management</h3>
					<div class="space-y-3">
						<div class="bg-muted/30 rounded-lg p-4">
							<div class="font-mono text-sm mb-2"><code>knot team add-member &lt;team&gt; &lt;user&gt;</code></div>
							<div class="text-sm text-muted-foreground">Add member to team</div>
						</div>
						<div class="bg-muted/30 rounded-lg p-4">
							<div class="font-mono text-sm mb-2"><code>knot team remove-member &lt;team&gt; &lt;user&gt;</code></div>
							<div class="text-sm text-muted-foreground">Remove member from team</div>
						</div>
					</div>
				</div>
			</div>

			<div class="bg-indigo-50 border border-indigo-200 rounded-lg p-4">
				<div class="flex items-start space-x-2">
					<Icon icon="solar:crown-bold" class="w-5 h-5 text-indigo-600 mt-0.5 flex-shrink-0" />
					<div>
						<div class="text-sm font-medium text-indigo-900 mb-1">Team Roles</div>
						<div class="text-sm text-indigo-700 space-y-1">
							<div><strong>Owner:</strong> Full team control and package management</div>
							<div><strong>Admin:</strong> Manage members and publish packages</div>
							<div><strong>Member:</strong> View team information and packages</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Self-Hosting -->
	<section id="self-hosting" class="mb-16">
		<div class="flex items-center space-x-3 mb-6">
			<div class="w-8 h-8 bg-red-100 rounded-lg flex items-center justify-center">
				<Icon icon="solar:server-bold" class="w-5 h-5 text-red-600" />
			</div>
			<h2 class="text-2xl font-bold">Self-Hosting</h2>
		</div>

		<div class="space-y-6">
			<p class="text-muted-foreground">
				You can host your own Knot Space instance for complete control over your package registry.
			</p>

			<div>
				<h3 class="text-lg font-semibold mb-3">Docker Deployment</h3>
				<div class="space-y-4">
					<div class="bg-muted/30 rounded-lg p-4">
						<div class="text-sm text-muted-foreground mb-2">1. Clone the repository</div>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-3 rounded">
							<code>git clone https://github.com/saravenpi/knot.git
cd knot</code>
						</div>
					</div>
					<div class="bg-muted/30 rounded-lg p-4">
						<div class="text-sm text-muted-foreground mb-2">2. Configure environment variables</div>
						<div class="bg-black/90 text-white font-mono text-sm p-3 rounded">
							<code><span class="text-blue-400">DATABASE_URL</span>=<span class="text-green-400">postgresql://user:pass@localhost:5432/knot</span>
<span class="text-blue-400">JWT_SECRET</span>=<span class="text-green-400">your-super-secret-key</span>
<span class="text-blue-400">PORT</span>=<span class="text-green-400">8000</span></code>
						</div>
					</div>
					<div class="bg-muted/30 rounded-lg p-4">
						<div class="text-sm text-muted-foreground mb-2">3. Start with Docker Compose</div>
						<div class="bg-black/90 text-green-400 font-mono text-sm p-3 rounded">
							<code>docker-compose up -d</code>
						</div>
					</div>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">CLI Configuration</h3>
				<p class="text-muted-foreground mb-3">
					Point the Knot CLI to your self-hosted instance using the <code>KNOT_SPACE_URL</code> environment variable:
				</p>
				<div class="bg-muted/30 rounded-lg p-4">
					<div class="bg-black/90 text-green-400 font-mono text-sm p-3 rounded">
						<code>export KNOT_SPACE_URL=https://your-knot-instance.com
export KNOT_TOKEN=your-auth-token

# Verify connection
knot auth</code>
					</div>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Production Setup</h3>
				<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
					<div class="border rounded-lg p-4">
						<div class="flex items-center space-x-2 mb-2">
							<Icon icon="solar:database-bold" class="w-4 h-4 text-red-600" />
							<span class="font-semibold">Database</span>
						</div>
						<ul class="text-sm text-muted-foreground space-y-1">
							<li>• PostgreSQL recommended</li>
							<li>• Automatic migrations included</li>
							<li>• Backup strategy important</li>
						</ul>
					</div>
					<div class="border rounded-lg p-4">
						<div class="flex items-center space-x-2 mb-2">
							<Icon icon="solar:shield-check-bold" class="w-4 h-4 text-red-600" />
							<span class="font-semibold">Security</span>
						</div>
						<ul class="text-sm text-muted-foreground space-y-1">
							<li>• HTTPS/TLS required</li>
							<li>• Strong JWT secret</li>
							<li>• Regular security updates</li>
						</ul>
					</div>
					<div class="border rounded-lg p-4">
						<div class="flex items-center space-x-2 mb-2">
							<Icon icon="solar:storage-bold" class="w-4 h-4 text-red-600" />
							<span class="font-semibold">Storage</span>
						</div>
						<ul class="text-sm text-muted-foreground space-y-1">
							<li>• Package files stored locally</li>
							<li>• Consider S3 for scalability</li>
							<li>• Monitor disk usage</li>
						</ul>
					</div>
					<div class="border rounded-lg p-4">
						<div class="flex items-center space-x-2 mb-2">
							<Icon icon="solar:monitor-bold" class="w-4 h-4 text-red-600" />
							<span class="font-semibold">Monitoring</span>
						</div>
						<ul class="text-sm text-muted-foreground space-y-1">
							<li>• Health check endpoints</li>
							<li>• Log aggregation</li>
							<li>• Performance monitoring</li>
						</ul>
					</div>
				</div>
			</div>

			<div class="bg-red-50 border border-red-200 rounded-lg p-4">
				<div class="flex items-start space-x-2">
					<Icon icon="solar:danger-bold" class="w-5 h-5 text-red-600 mt-0.5 flex-shrink-0" />
					<div>
						<div class="text-sm font-medium text-red-900 mb-1">Production Considerations</div>
						<div class="text-sm text-red-700">
							Self-hosting requires proper infrastructure management, security updates, backups, 
							and monitoring. Consider using the hosted version at knot-space-production.up.railway.app 
							for easier maintenance.
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Footer -->
	<div class="border-t pt-8 mt-16">
		<div class="flex flex-col sm:flex-row justify-between items-center">
			<div class="text-sm text-muted-foreground">
				Need help? Check out our <a href="https://github.com/saravenpi/knot" class="text-primary hover:underline">GitHub repository</a>
			</div>
			<div class="text-sm text-muted-foreground mt-4 sm:mt-0">
				Made with ❤️ for the TypeScript community
			</div>
		</div>
	</div>
</div>