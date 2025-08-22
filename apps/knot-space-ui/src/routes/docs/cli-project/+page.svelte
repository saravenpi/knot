<script lang="ts">
	import Icon from '@iconify/svelte';
</script>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight mb-4" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
			Project Commands
		</h1>
		<p class="text-lg text-muted-foreground">
			Manage your Knot projects with powerful CLI commands.
		</p>
	</div>

	<div class="prose prose-slate max-w-none">
		<h2 class="text-2xl font-bold mb-4">Creating Projects</h2>
		
		<div class="bg-muted rounded-lg p-4 mb-6">
			<h3 class="font-semibold mb-2">knot init</h3>
			<p class="text-sm text-muted-foreground mb-3">Initialize a new Knot project or workspace</p>
			<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`# Initialize a new project
knot init

# Initialize with specific template
knot init --template typescript

# Initialize in a specific directory
knot init my-project`}</code></pre>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Development Commands</h2>
		
		<div class="space-y-4">
			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">knot dev</h3>
				<p class="text-sm text-muted-foreground mb-3">Start development server with hot reload</p>
				<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`# Start dev server
knot dev

# Start with specific port
knot dev --port 4000

# Start with verbose output
knot dev --verbose`}</code></pre>
			</div>

			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">knot build</h3>
				<p class="text-sm text-muted-foreground mb-3">Build your project for production</p>
				<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`# Build the project
knot build

# Build with source maps
knot build --source-maps

# Build specific packages
knot build @org/package-name`}</code></pre>
			</div>

			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">knot test</h3>
				<p class="text-sm text-muted-foreground mb-3">Run tests for your project</p>
				<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`# Run all tests
knot test

# Run tests in watch mode
knot test --watch

# Run with coverage report
knot test --coverage

# Run specific test file
knot test src/utils.test.ts`}</code></pre>
			</div>

			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">knot lint</h3>
				<p class="text-sm text-muted-foreground mb-3">Lint your code for style and errors</p>
				<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`# Run linter
knot lint

# Auto-fix issues
knot lint --fix

# Lint specific files
knot lint src/**/*.ts`}</code></pre>
			</div>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Running Scripts</h2>
		
		<div class="bg-muted rounded-lg p-4 mb-6">
			<h3 class="font-semibold mb-2">knot run</h3>
			<p class="text-sm text-muted-foreground mb-3">Execute scripts from knot.yml, app.yml, or package.yml</p>
			<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`# Interactive script selection (no arguments)
knot run

# Run a specific script
knot run build

# Run with arguments
knot run test -- --watch`}</code></pre>
			<div class="mt-3 p-3 bg-blue-50 border border-blue-200 rounded-lg">
				<p class="text-sm text-blue-700">
					<strong>💡 Interactive Mode:</strong> Running <code>knot run</code> without arguments opens a beautiful interactive menu to select from all available scripts across your project, apps, and packages.
				</p>
			</div>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Workspace Management</h2>
		
		<div class="space-y-4">
			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">knot workspace</h3>
				<p class="text-sm text-muted-foreground mb-3">Manage monorepo workspaces</p>
				<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`# List all workspaces
knot workspace list

# Add a new workspace
knot workspace add packages/new-package

# Run command in all workspaces
knot workspace run build

# Run in specific workspace
knot workspace run test --workspace @org/package`}</code></pre>
			</div>

			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">knot clean</h3>
				<p class="text-sm text-muted-foreground mb-3">Clean build artifacts and caches</p>
				<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`# Clean all build artifacts
knot clean

# Clean node_modules as well
knot clean --include-node-modules

# Clean specific packages
knot clean @org/package-name`}</code></pre>
			</div>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Environment & Configuration</h2>
		
		<div class="space-y-4">
			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">knot config</h3>
				<p class="text-sm text-muted-foreground mb-3">Manage Knot configuration</p>
				<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`# View current config
knot config list

# Set a config value
knot config set registry https://knot.space

# Get specific config value
knot config get registry

# Reset to defaults
knot config reset`}</code></pre>
			</div>

			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">knot env</h3>
				<p class="text-sm text-muted-foreground mb-3">Manage environment variables</p>
				<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`# List environment variables
knot env list

# Set environment variable
knot env set NODE_ENV production

# Load from .env file
knot env load .env.local`}</code></pre>
			</div>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Debugging & Diagnostics</h2>
		
		<div class="space-y-4">
			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">knot doctor</h3>
				<p class="text-sm text-muted-foreground mb-3">Check project health and configuration</p>
				<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`# Run diagnostics
knot doctor

# Verbose output
knot doctor --verbose

# Auto-fix issues
knot doctor --fix`}</code></pre>
			</div>

			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">knot logs</h3>
				<p class="text-sm text-muted-foreground mb-3">View and manage logs</p>
				<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`# View recent logs
knot logs

# Follow logs in real-time
knot logs --follow

# Filter by level
knot logs --level error

# Clear logs
knot logs --clear`}</code></pre>
			</div>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Common Options</h2>
		
		<div class="bg-yellow-50 border border-yellow-200 rounded-lg p-4">
			<h3 class="font-semibold text-yellow-900 mb-2">Global Flags</h3>
			<div class="space-y-2 text-sm">
				<div><code class="bg-yellow-100 px-1 rounded">--help, -h</code> - Show help for any command</div>
				<div><code class="bg-yellow-100 px-1 rounded">--version, -v</code> - Show Knot version</div>
				<div><code class="bg-yellow-100 px-1 rounded">--verbose</code> - Enable verbose output</div>
				<div><code class="bg-yellow-100 px-1 rounded">--quiet, -q</code> - Suppress output</div>
				<div><code class="bg-yellow-100 px-1 rounded">--cwd</code> - Set working directory</div>
				<div><code class="bg-yellow-100 px-1 rounded">--no-color</code> - Disable colored output</div>
			</div>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Next Steps</h2>
		
		<div class="grid gap-4 mt-6">
			<a href="/docs/cli-packages" class="group border rounded-lg p-4 hover:bg-muted transition-colors">
				<div class="flex items-center justify-between">
					<div>
						<h3 class="font-semibold group-hover:text-primary">Package Commands</h3>
						<p class="text-sm text-muted-foreground">Learn about package management commands</p>
					</div>
					<Icon icon="solar:arrow-right-bold" class="w-5 h-5 text-muted-foreground group-hover:text-primary" />
				</div>
			</a>

			<a href="/docs/configuration" class="group border rounded-lg p-4 hover:bg-muted transition-colors">
				<div class="flex items-center justify-between">
					<div>
						<h3 class="font-semibold group-hover:text-primary">Configuration</h3>
						<p class="text-sm text-muted-foreground">Configure Knot for your project</p>
					</div>
					<Icon icon="solar:arrow-right-bold" class="w-5 h-5 text-muted-foreground group-hover:text-primary" />
				</div>
			</a>
		</div>
	</div>
</div>