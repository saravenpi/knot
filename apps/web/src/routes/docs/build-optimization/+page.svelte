<script lang="ts">
	import Icon from '@iconify/svelte';
</script>

<svelte:head>
	<title>Build Optimization - Knot CLI Documentation</title>
	<meta name="description" content="Optimize your builds for maximum performance and efficiency with Knot's build optimization features." />
</svelte:head>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight mb-4" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
			Build Optimization
		</h1>
		<p class="text-lg text-muted-foreground">
			Optimize your builds for maximum performance and efficiency.
		</p>
	</div>

	<div class="prose prose-slate max-w-none">
		<h2 class="text-2xl font-bold mb-4">Build Performance</h2>
		
		<div class="bg-muted rounded-lg p-4 mb-6">
			<h3 class="font-semibold mb-2">Parallel Building</h3>
			<p class="text-sm text-muted-foreground mb-3">Leverage multiple CPU cores for faster builds</p>
			<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`# Build with parallel processing
knot build --parallel

# Specify number of workers
knot build --parallel 4

# Build specific packages in parallel
knot build @org/package-* --parallel`}</code></pre>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Caching Strategies</h2>
		
		<div class="space-y-4">
			<div class="bg-blue-50 border border-blue-200 rounded-lg p-4">
				<h3 class="font-semibold text-blue-900 mb-2">Build Cache</h3>
				<p class="text-sm text-blue-700 mb-3">Knot automatically caches build outputs to speed up subsequent builds</p>
				<pre class="bg-white rounded p-3"><code class="text-sm">{`# View cache status
knot cache status

# Clear cache
knot cache clean

# Build with cache disabled
knot build --no-cache`}</code></pre>
			</div>

			<div class="bg-green-50 border border-green-200 rounded-lg p-4">
				<h3 class="font-semibold text-green-900 mb-2">Dependency Cache</h3>
				<p class="text-sm text-green-700 mb-3">Cache dependencies for faster installs</p>
				<pre class="bg-white rounded p-3"><code class="text-sm">{`# Cache location
~/.knot/cache/

# Prune unused cached packages
knot cache prune

# Verify cache integrity
knot cache verify`}</code></pre>
			</div>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Bundle Optimization</h2>
		
		<div class="space-y-4">
			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">Tree Shaking</h3>
				<p class="text-sm text-muted-foreground mb-3">Remove unused code from bundles</p>
				<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`// knot.config.js
export default {
  build: {
    treeShake: true,
    sideEffects: false,
    pure: ['console.log', 'console.debug']
  }
}`}</code></pre>
			</div>

			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">Code Splitting</h3>
				<p class="text-sm text-muted-foreground mb-3">Split code into smaller chunks</p>
				<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`// Dynamic imports for code splitting
const MyComponent = () => import('./MyComponent');

// knot.config.js
export default {
  build: {
    splitting: true,
    chunkSizeLimit: 500 // KB
  }
}`}</code></pre>
			</div>

			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">Minification</h3>
				<p class="text-sm text-muted-foreground mb-3">Reduce bundle size with minification</p>
				<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`# Build with minification
knot build --minify

# Advanced minification options
knot build --minify --mangle --compress`}</code></pre>
			</div>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Incremental Builds</h2>
		
		<div class="bg-yellow-50 border border-yellow-200 rounded-lg p-4 mb-6">
			<h3 class="font-semibold text-yellow-900 mb-2">Smart Rebuilds</h3>
			<p class="text-sm text-yellow-700 mb-3">Only rebuild what has changed</p>
			<pre class="bg-yellow-100 rounded p-3 overflow-x-auto"><code class="text-sm">{`# Enable incremental builds
knot build --incremental

# Watch mode with incremental builds
knot dev --incremental

# Force full rebuild
knot build --force`}</code></pre>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Build Analysis</h2>
		
		<div class="space-y-4">
			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">Bundle Analysis</h3>
				<p class="text-sm text-muted-foreground mb-3">Analyze bundle size and composition</p>
				<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`# Generate bundle analysis
knot build --analyze

# Output detailed stats
knot build --stats detailed

# Save stats to file
knot build --stats-json stats.json`}</code></pre>
			</div>

			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">Performance Metrics</h3>
				<p class="text-sm text-muted-foreground mb-3">Measure build performance</p>
				<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`# Show build timings
knot build --timings

# Profile build process
knot build --profile

# Generate performance report
knot build --perf-report`}</code></pre>
			</div>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Configuration</h2>
		
		<div class="bg-muted rounded-lg p-4 mb-6">
			<h3 class="font-semibold mb-2">knot.config.js</h3>
			<p class="text-sm text-muted-foreground mb-3">Complete build optimization configuration</p>
			<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`export default {
  build: {
    // Output configuration
    outDir: 'dist',
    sourcemap: true,
    minify: 'terser',
    
    // Optimization
    target: 'es2020',
    treeShake: true,
    splitting: true,
    
    // Performance
    parallel: true,
    workers: 4,
    incremental: true,
    cache: true,
    
    // Advanced
    rollupOptions: {
      output: {
        manualChunks: {
          vendor: ['react', 'react-dom'],
          utils: ['lodash', 'date-fns']
        }
      }
    }
  }
}`}</code></pre>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">CI/CD Optimization</h2>
		
		<div class="space-y-4">
			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">GitHub Actions</h3>
				<p class="text-sm text-muted-foreground mb-3">Optimize builds in CI</p>
				<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`name: Build
on: [push]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      
      - name: Cache Knot
        uses: actions/cache@v2
        with:
          path: ~/.knot/cache
          key: knot-\${{ runner.os }}-\${{ hashFiles('**/package-lock.json') }}
          
      - name: Install Knot
        run: curl -fsSL https://knot.space/install.sh | bash
        
      - name: Build
        run: knot build --parallel --cache`}</code></pre>
			</div>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Best Practices</h2>
		
		<div class="space-y-3">
			<div class="flex items-start space-x-3">
				<Icon icon="solar:check-circle-bold" class="w-5 h-5 text-green-600 mt-0.5 flex-shrink-0" />
				<div>
					<p class="font-medium">Use production builds for deployment</p>
					<p class="text-sm text-muted-foreground">Always use <code>knot build --production</code> for production</p>
				</div>
			</div>

			<div class="flex items-start space-x-3">
				<Icon icon="solar:check-circle-bold" class="w-5 h-5 text-green-600 mt-0.5 flex-shrink-0" />
				<div>
					<p class="font-medium">Enable caching in CI/CD</p>
					<p class="text-sm text-muted-foreground">Cache dependencies and build outputs</p>
				</div>
			</div>

			<div class="flex items-start space-x-3">
				<Icon icon="solar:check-circle-bold" class="w-5 h-5 text-green-600 mt-0.5 flex-shrink-0" />
				<div>
					<p class="font-medium">Monitor bundle sizes</p>
					<p class="text-sm text-muted-foreground">Set up size limits and track changes</p>
				</div>
			</div>

			<div class="flex items-start space-x-3">
				<Icon icon="solar:check-circle-bold" class="w-5 h-5 text-green-600 mt-0.5 flex-shrink-0" />
				<div>
					<p class="font-medium">Use incremental builds in development</p>
					<p class="text-sm text-muted-foreground">Speed up development with smart rebuilds</p>
				</div>
			</div>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Next Steps</h2>
		
		<div class="grid gap-4 mt-6">
			<a href="/docs/configuration" class="group border rounded-lg p-4 hover:bg-muted transition-colors">
				<div class="flex items-center justify-between">
					<div>
						<h3 class="font-semibold group-hover:text-primary">Configuration</h3>
						<p class="text-sm text-muted-foreground">Learn about all configuration options</p>
					</div>
					<Icon icon="solar:arrow-right-bold" class="w-5 h-5 text-muted-foreground group-hover:text-primary" />
				</div>
			</a>

			<a href="/docs/project-management" class="group border rounded-lg p-4 hover:bg-muted transition-colors">
				<div class="flex items-center justify-between">
					<div>
						<h3 class="font-semibold group-hover:text-primary">Project Commands</h3>
						<p class="text-sm text-muted-foreground">Explore build-related CLI commands</p>
					</div>
					<Icon icon="solar:arrow-right-bold" class="w-5 h-5 text-muted-foreground group-hover:text-primary" />
				</div>
			</a>
		</div>
	</div>
</div>