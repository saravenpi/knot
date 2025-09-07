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

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight mb-4" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
			Docker Integration
		</h1>
		<p class="text-lg text-muted-foreground">
			Learn how to containerize your Knot monorepo projects and deploy them with Docker
		</p>
	</div>

	<!-- Overview -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Why Docker with Knot?</h2>
		
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-blue-100 rounded-lg flex items-center justify-center mb-4">
					<Icon icon="solar:box-bold" class="w-6 h-6 text-blue-600" />
				</div>
				<h3 class="font-semibold mb-2">Consistent Environments</h3>
				<p class="text-sm text-muted-foreground">
					Ensure your monorepo runs the same way across development, staging, and production.
				</p>
			</div>
			
			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-green-100 rounded-lg flex items-center justify-center mb-4">
					<Icon icon="solar:copy-bold" class="w-6 h-6 text-green-600" />
				</div>
				<h3 class="font-semibold mb-2">Copy-First Benefits</h3>
				<p class="text-sm text-muted-foreground">
					Knot's copy-first package linking works perfectly with Docker's layered architecture.
				</p>
			</div>
		</div>

		<div class="bg-amber-50 border border-amber-200 rounded-lg p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="solar:info-circle-bold" class="w-6 h-6 text-amber-600 mt-1 flex-shrink-0" />
				<div>
					<h3 class="font-semibold text-amber-900 mb-2">Important Note</h3>
					<p class="text-sm text-amber-700">
						Always use <code>knot link</code> (copy mode) instead of <code>knot link --symlink</code> when building Docker images. 
						Symlinks don't work reliably across Docker layer boundaries.
					</p>
				</div>
			</div>
		</div>
	</section>

	<!-- Basic Dockerfile -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Basic Dockerfile Setup</h2>
		
		<div class="space-y-6">
			<div>
				<h3 class="text-lg font-semibold mb-3">Single App Dockerfile</h3>
				<p class="text-muted-foreground mb-4">
					For containerizing a single application from your monorepo:
				</p>
				
				<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
					<button
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
						on:click={() => copyToClipboard(`# apps/frontend/Dockerfile
FROM node:18-alpine

WORKDIR /app

# Copy package files
COPY package*.json ./
COPY ../../knot.yml ./
COPY ../../packages ./packages

# Install dependencies
RUN npm install

# Copy app source
COPY . .

# Link packages (copy mode)
RUN npx knot link

# Build application
RUN npm run build

EXPOSE 3000
CMD ["npm", "start"]`)}
					>
						{#if showCopied && copyText.includes('Single app')}
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="solar:copy-bold" class="w-4 h-4" />
						{/if}
					</button>
					<pre><code># apps/frontend/Dockerfile
FROM node:18-alpine

WORKDIR /app

# Copy package files
COPY package*.json ./
COPY ../../knot.yml ./
COPY ../../packages ./packages

# Install dependencies
RUN npm install

# Copy app source
COPY . .

# Link packages (copy mode)
RUN npx knot link

# Build application
RUN npm run build

EXPOSE 3000
CMD ["npm", "start"]</code></pre>
				</div>
			</div>
			
			<div>
				<h3 class="text-lg font-semibold mb-3">Multi-Stage Build</h3>
				<p class="text-muted-foreground mb-4">
					Optimized Dockerfile with multi-stage builds for smaller production images:
				</p>
				
				<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
					<button
						class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
						on:click={() => copyToClipboard(`# Multi-stage Dockerfile
FROM node:18-alpine as builder

WORKDIR /app

# Copy package files
COPY package*.json ./
COPY knot.yml ./
COPY packages ./packages

# Install all dependencies (including devDependencies)
RUN npm install

# Copy app source
COPY apps/frontend ./apps/frontend

# Link packages and build
WORKDIR /app/apps/frontend
RUN npx knot link
RUN npm run build

# Production stage
FROM node:18-alpine as production

WORKDIR /app

# Copy only production dependencies
COPY --from=builder /app/package*.json ./
RUN npm ci --only=production

# Copy built application
COPY --from=builder /app/apps/frontend/dist ./dist
COPY --from=builder /app/apps/frontend/package.json ./

EXPOSE 3000
CMD ["npm", "start"]`)}
					>
						{#if showCopied && copyText.includes('Multi-stage')}
							<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
						{:else}
							<Icon icon="solar:copy-bold" class="w-4 h-4" />
						{/if}
					</button>
					<pre><code># Multi-stage Dockerfile
FROM node:18-alpine as builder

WORKDIR /app

# Copy package files
COPY package*.json ./
COPY knot.yml ./
COPY packages ./packages

# Install all dependencies (including devDependencies)
RUN npm install

# Copy app source
COPY apps/frontend ./apps/frontend

# Link packages and build
WORKDIR /app/apps/frontend
RUN npx knot link
RUN npm run build

# Production stage
FROM node:18-alpine as production

WORKDIR /app

# Copy only production dependencies
COPY --from=builder /app/package*.json ./
RUN npm ci --only=production

# Copy built application
COPY --from=builder /app/apps/frontend/dist ./dist
COPY --from=builder /app/apps/frontend/package.json ./

EXPOSE 3000
CMD ["npm", "start"]</code></pre>
				</div>
			</div>
		</div>
	</section>

	<!-- Docker Compose -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Docker Compose for Monorepo</h2>
		
		<p class="text-muted-foreground mb-6">
			Use Docker Compose to orchestrate multiple applications and services from your monorepo:
		</p>
		
		<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
			<button
				class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
				on:click={() => copyToClipboard(`# docker-compose.yml
version: '3.8'

services:
  frontend:
    build:
      context: .
      dockerfile: apps/frontend/Dockerfile
    ports:
      - "3000:3000"
    environment:
      - NODE_ENV=production
      - API_URL=http://api:4000
    depends_on:
      - api

  api:
    build:
      context: .
      dockerfile: apps/api/Dockerfile
    ports:
      - "4000:4000"
    environment:
      - NODE_ENV=production
      - DATABASE_URL=postgresql://user:pass@db:5432/myapp
    depends_on:
      - db

  db:
    image: postgres:15-alpine
    environment:
      - POSTGRES_USER=user
      - POSTGRES_PASSWORD=pass
      - POSTGRES_DB=myapp
    volumes:
      - postgres_data:/var/lib/postgresql/data
    ports:
      - "5432:5432"

volumes:
  postgres_data:`)}
			>
				{#if showCopied && copyText.includes('docker-compose')}
					<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
				{:else}
					<Icon icon="solar:copy-bold" class="w-4 h-4" />
				{/if}
			</button>
			<pre><code># docker-compose.yml
version: '3.8'

services:
  frontend:
    build:
      context: .
      dockerfile: apps/frontend/Dockerfile
    ports:
      - "3000:3000"
    environment:
      - NODE_ENV=production
      - API_URL=http://api:4000
    depends_on:
      - api

  api:
    build:
      context: .
      dockerfile: apps/api/Dockerfile
    ports:
      - "4000:4000"
    environment:
      - NODE_ENV=production
      - DATABASE_URL=postgresql://user:pass@db:5432/myapp
    depends_on:
      - db

  db:
    image: postgres:15-alpine
    environment:
      - POSTGRES_USER=user
      - POSTGRES_PASSWORD=pass
      - POSTGRES_DB=myapp
    volumes:
      - postgres_data:/var/lib/postgresql/data
    ports:
      - "5432:5432"

volumes:
  postgres_data:</code></pre>
		</div>
	</section>

	<!-- Best Practices -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Best Practices</h2>
		
		<div class="space-y-6">
			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="solar:layers-bold" class="w-5 h-5 mr-2 text-blue-600" />
					Layer Optimization
				</h3>
				<div class="space-y-3 text-sm">
					<div class="flex items-start space-x-3">
						<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600 mt-0.5 flex-shrink-0" />
						<span>Copy package.json files first to leverage Docker layer caching</span>
					</div>
					<div class="flex items-start space-x-3">
						<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600 mt-0.5 flex-shrink-0" />
						<span>Run <code>npm install</code> before copying source code</span>
					</div>
					<div class="flex items-start space-x-3">
						<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600 mt-0.5 flex-shrink-0" />
						<span>Use <code>.dockerignore</code> to exclude unnecessary files</span>
					</div>
				</div>
			</div>
			
			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="solar:link-bold" class="w-5 h-5 mr-2 text-purple-600" />
					Package Linking
				</h3>
				<div class="space-y-3 text-sm">
					<div class="flex items-start space-x-3">
						<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600 mt-0.5 flex-shrink-0" />
						<span>Always use <code>knot link</code> (copy mode) in Docker builds</span>
					</div>
					<div class="flex items-start space-x-3">
						<Icon icon="solar:close-circle-bold" class="w-4 h-4 text-red-600 mt-0.5 flex-shrink-0" />
						<span>Never use <code>knot link --symlink</code> in Docker containers</span>
					</div>
					<div class="flex items-start space-x-3">
						<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600 mt-0.5 flex-shrink-0" />
						<span>Run linking after copying all packages and app source</span>
					</div>
				</div>
			</div>
			
			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="solar:shield-check-bold" class="w-5 h-5 mr-2 text-green-600" />
					Security
				</h3>
				<div class="space-y-3 text-sm">
					<div class="flex items-start space-x-3">
						<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600 mt-0.5 flex-shrink-0" />
						<span>Use non-root user in production containers</span>
					</div>
					<div class="flex items-start space-x-3">
						<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600 mt-0.5 flex-shrink-0" />
						<span>Remove development dependencies in production stage</span>
					</div>
					<div class="flex items-start space-x-3">
						<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-600 mt-0.5 flex-shrink-0" />
						<span>Use specific base image tags instead of 'latest'</span>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Dockerignore -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">.dockerignore File</h2>
		
		<p class="text-muted-foreground mb-4">
			Create a <code>.dockerignore</code> file in your project root to exclude unnecessary files:
		</p>
		
		<div class="bg-black/90 text-green-400 font-mono text-sm p-4 rounded-lg relative group">
			<button
				class="absolute top-2 right-2 p-2 rounded-md bg-white/10 hover:bg-white/20 transition-colors opacity-0 group-hover:opacity-100"
				on:click={() => copyToClipboard(`# .dockerignore
node_modules
npm-debug.log*
.env
.env.local
.env.development.local
.env.test.local
.env.production.local
.git
.gitignore
README.md
.DS_Store
.vscode
.idea
*.log
dist
build
coverage
.nyc_output
.cache
*.tgz
*.tar.gz
*/knot_packages
*/.next
*/out`)}
			>
				{#if showCopied && copyText.includes('dockerignore')}
					<Icon icon="solar:check-circle-bold" class="w-4 h-4 text-green-400" />
				{:else}
					<Icon icon="solar:copy-bold" class="w-4 h-4" />
				{/if}
			</button>
			<pre><code># .dockerignore
node_modules
npm-debug.log*
.env
.env.local
.env.development.local
.env.test.local
.env.production.local
.git
.gitignore
README.md
.DS_Store
.vscode
.idea
*.log
dist
build
coverage
.nyc_output
.cache
*.tgz
*.tar.gz
*/knot_packages
*/.next
*/out</code></pre>
		</div>
	</section>

	<!-- Common Commands -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Common Docker Commands</h2>
		
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<div class="space-y-4">
				<h3 class="font-semibold">Building Images</h3>
				<div class="space-y-3">
					<div class="bg-muted p-3 rounded-lg">
						<code class="text-sm">docker build -t my-app .</code>
						<p class="text-xs text-muted-foreground mt-1">Build single app</p>
					</div>
					<div class="bg-muted p-3 rounded-lg">
						<code class="text-sm">docker-compose build</code>
						<p class="text-xs text-muted-foreground mt-1">Build all services</p>
					</div>
				</div>
			</div>
			
			<div class="space-y-4">
				<h3 class="font-semibold">Running Containers</h3>
				<div class="space-y-3">
					<div class="bg-muted p-3 rounded-lg">
						<code class="text-sm">docker run -p 3000:3000 my-app</code>
						<p class="text-xs text-muted-foreground mt-1">Run single container</p>
					</div>
					<div class="bg-muted p-3 rounded-lg">
						<code class="text-sm">docker-compose up -d</code>
						<p class="text-xs text-muted-foreground mt-1">Start all services</p>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Troubleshooting -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Troubleshooting</h2>
		
		<div class="space-y-6">
			<div class="border border-red-200 rounded-lg p-6">
				<h3 class="font-semibold text-red-900 mb-3">Package linking fails in container</h3>
				<p class="text-sm text-red-700 mb-3">
					<strong>Problem:</strong> Knot can't find packages or linking fails during Docker build.
				</p>
				<div class="text-sm space-y-2">
					<p><strong>Solution:</strong></p>
					<ul class="list-disc list-inside space-y-1 text-red-700">
						<li>Ensure you copy the project root <code>knot.yml</code> file</li>
						<li>Copy all local packages before running <code>knot link</code></li>
						<li>Use copy mode: <code>knot link</code> (not <code>--symlink</code>)</li>
						<li>Check that package paths are relative to the working directory</li>
					</ul>
				</div>
			</div>
			
			<div class="border border-amber-200 rounded-lg p-6">
				<h3 class="font-semibold text-amber-900 mb-3">Large image sizes</h3>
				<p class="text-sm text-amber-700 mb-3">
					<strong>Problem:</strong> Docker images are larger than expected.
				</p>
				<div class="text-sm space-y-2">
					<p><strong>Solutions:</strong></p>
					<ul class="list-disc list-inside space-y-1 text-amber-700">
						<li>Use multi-stage builds to exclude development dependencies</li>
						<li>Add comprehensive <code>.dockerignore</code> file</li>
						<li>Use Alpine Linux base images for smaller footprint</li>
						<li>Remove build tools and caches in the same RUN command</li>
					</ul>
				</div>
			</div>
		</div>
	</section>

	<!-- Next Steps -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Next Steps</h2>
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<a href="/docs/production" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:rocket-bold" class="w-6 h-6 text-green-600" />
					<h3 class="font-semibold">Production Deployment</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Learn how to deploy your containerized Knot applications to production environments.
				</p>
			</a>

			<a href="/docs/troubleshooting" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="solar:help-bold" class="w-6 h-6 text-purple-600" />
					<h3 class="font-semibold">Troubleshooting</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Common issues and solutions for Docker deployment and container management.
				</p>
			</a>
		</div>
	</section>
</div>