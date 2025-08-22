<script lang="ts">
	import Icon from '@iconify/svelte';
</script>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight mb-4" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
			TypeScript & JavaScript
		</h1>
		<p class="text-lg text-muted-foreground">
			Build and manage TypeScript and JavaScript packages with Knot.
		</p>
	</div>

	<div class="prose prose-slate max-w-none">
		<h2 class="text-2xl font-bold mb-4">Overview</h2>
		<p class="mb-6">
			Knot provides first-class support for TypeScript and JavaScript packages, including automatic type generation, 
			module resolution, and seamless integration with popular build tools.
		</p>

		<h2 class="text-2xl font-bold mb-4 mt-8">TypeScript Configuration</h2>
		
		<div class="bg-muted rounded-lg p-4 mb-6">
			<h3 class="font-semibold mb-2">Recommended tsconfig.json</h3>
			<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`{
  "compilerOptions": {
    "target": "ES2020",
    "module": "ESNext",
    "lib": ["ES2020"],
    "moduleResolution": "bundler",
    "declaration": true,
    "declarationMap": true,
    "sourceMap": true,
    "outDir": "./dist",
    "rootDir": "./src",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules", "dist", "**/*.test.ts"]
}`}</code></pre>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Package.json Setup</h2>
		
		<div class="bg-muted rounded-lg p-4 mb-6">
			<h3 class="font-semibold mb-2">Example package.json</h3>
			<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`{
  "name": "@your-org/package-name",
  "version": "1.0.0",
  "type": "module",
  "main": "./dist/index.js",
  "module": "./dist/index.js",
  "types": "./dist/index.d.ts",
  "exports": {
    ".": {
      "types": "./dist/index.d.ts",
      "import": "./dist/index.js",
      "require": "./dist/index.cjs"
    }
  },
  "scripts": {
    "build": "knot build",
    "test": "knot test",
    "lint": "knot lint",
    "dev": "knot dev"
  },
  "devDependencies": {
    "typescript": "^5.0.0",
    "@types/node": "^20.0.0"
  }
}`}</code></pre>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Building Packages</h2>
		
		<div class="space-y-4">
			<div class="bg-blue-50 border border-blue-200 rounded-lg p-4">
				<h3 class="font-semibold text-blue-900 mb-2">Build Command</h3>
				<pre class="bg-white rounded p-3"><code class="text-sm">knot build</code></pre>
				<p class="text-sm text-blue-700 mt-2">
					Automatically detects and builds TypeScript files, generates type definitions, and bundles your package.
				</p>
			</div>

			<div class="bg-green-50 border border-green-200 rounded-lg p-4">
				<h3 class="font-semibold text-green-900 mb-2">Watch Mode</h3>
				<pre class="bg-white rounded p-3"><code class="text-sm">knot dev</code></pre>
				<p class="text-sm text-green-700 mt-2">
					Watches for file changes and rebuilds automatically during development.
				</p>
			</div>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Type Definitions</h2>
		
		<p class="mb-4">
			Knot automatically generates and includes TypeScript type definitions when you publish packages. 
			Ensure your <code>tsconfig.json</code> has <code>"declaration": true</code> enabled.
		</p>

		<div class="bg-muted rounded-lg p-4 mb-6">
			<h3 class="font-semibold mb-2">Publishing with Types</h3>
			<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`# Build and publish with type definitions
knot build
knot publish

# Types are automatically included in the package`}</code></pre>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Module Formats</h2>
		
		<div class="grid gap-4">
			<div class="border rounded-lg p-4">
				<h3 class="font-semibold mb-2">ESM (ECMAScript Modules)</h3>
				<p class="text-sm text-muted-foreground mb-2">Modern standard, recommended for new projects</p>
				<pre class="bg-muted rounded p-2"><code class="text-sm">import { myFunction } from '@org/package';</code></pre>
			</div>

			<div class="border rounded-lg p-4">
				<h3 class="font-semibold mb-2">CommonJS</h3>
				<p class="text-sm text-muted-foreground mb-2">Legacy format, supported for compatibility</p>
				<pre class="bg-muted rounded p-2"><code class="text-sm">const { myFunction } = require('@org/package');</code></pre>
			</div>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Testing</h2>
		
		<div class="bg-muted rounded-lg p-4 mb-6">
			<h3 class="font-semibold mb-2">Running Tests</h3>
			<pre class="bg-background rounded p-3 overflow-x-auto"><code class="text-sm">{`# Run all tests
knot test

# Run tests in watch mode
knot test --watch

# Run tests with coverage
knot test --coverage`}</code></pre>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Best Practices</h2>
		
		<div class="space-y-3">
			<div class="flex items-start space-x-3">
				<Icon icon="solar:check-circle-bold" class="w-5 h-5 text-green-600 mt-0.5 flex-shrink-0" />
				<div>
					<p class="font-medium">Use strict TypeScript settings</p>
					<p class="text-sm text-muted-foreground">Enable strict mode for better type safety</p>
				</div>
			</div>

			<div class="flex items-start space-x-3">
				<Icon icon="solar:check-circle-bold" class="w-5 h-5 text-green-600 mt-0.5 flex-shrink-0" />
				<div>
					<p class="font-medium">Include source maps</p>
					<p class="text-sm text-muted-foreground">Help with debugging in production</p>
				</div>
			</div>

			<div class="flex items-start space-x-3">
				<Icon icon="solar:check-circle-bold" class="w-5 h-5 text-green-600 mt-0.5 flex-shrink-0" />
				<div>
					<p class="font-medium">Export types explicitly</p>
					<p class="text-sm text-muted-foreground">Make your API surface clear and documented</p>
				</div>
			</div>

			<div class="flex items-start space-x-3">
				<Icon icon="solar:check-circle-bold" class="w-5 h-5 text-green-600 mt-0.5 flex-shrink-0" />
				<div>
					<p class="font-medium">Use semantic versioning</p>
					<p class="text-sm text-muted-foreground">Follow semver for version updates</p>
				</div>
			</div>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Next Steps</h2>
		
		<div class="grid gap-4 mt-6">
			<a href="/docs/publishing" class="group border rounded-lg p-4 hover:bg-muted transition-colors">
				<div class="flex items-center justify-between">
					<div>
						<h3 class="font-semibold group-hover:text-primary">Publishing Packages</h3>
						<p class="text-sm text-muted-foreground">Learn how to publish your TypeScript packages</p>
					</div>
					<Icon icon="solar:arrow-right-bold" class="w-5 h-5 text-muted-foreground group-hover:text-primary" />
				</div>
			</a>

			<a href="/docs/cli-packages" class="group border rounded-lg p-4 hover:bg-muted transition-colors">
				<div class="flex items-center justify-between">
					<div>
						<h3 class="font-semibold group-hover:text-primary">Package Commands</h3>
						<p class="text-sm text-muted-foreground">Explore all package-related CLI commands</p>
					</div>
					<Icon icon="solar:arrow-right-bold" class="w-5 h-5 text-muted-foreground group-hover:text-primary" />
				</div>
			</a>
		</div>
	</div>
</div>