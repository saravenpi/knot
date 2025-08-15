<script lang="ts">
	import { page } from '$app/stores';
	import Icon from '@iconify/svelte';
	import { authStore } from '../../lib/stores';

	const sections = [
		{
			title: 'Getting Started',
			icon: 'solar:rocket-2-bold',
			items: [
				{ title: 'Introduction', href: '/docs', exact: true },
				{ title: 'Installation', href: '/docs/installation' },
				{ title: 'Quick Start', href: '/docs/quick-start' },
				{ title: 'Authentication', href: '/docs/authentication' }
			]
		},
		{
			title: 'Core Concepts',
			icon: 'solar:book-bold',
			items: [
				{ title: 'Project Structure', href: '/docs/project-structure' },
				{ title: 'Configuration Files', href: '/docs/configuration' },
				{ title: 'Package Linking', href: '/docs/package-linking' },
				{ title: 'TypeScript Integration', href: '/docs/typescript' }
			]
		},
		{
			title: 'CLI Reference',
			icon: 'solar:terminal-bold',
			items: [
				{ title: 'Commands Overview', href: '/docs/cli-commands' },
				{ title: 'Project Management', href: '/docs/cli-project' },
				{ title: 'Package Development', href: '/docs/cli-packages' },
				{ title: 'Publishing', href: '/docs/publishing' }
			]
		},
		{
			title: 'Team Collaboration',
			icon: 'solar:users-group-two-rounded-bold',
			items: [
				{ title: 'Team Management', href: '/docs/teams' },
				{ title: 'Permissions', href: '/docs/permissions' },
				{ title: 'Workflow Best Practices', href: '/docs/workflows' }
			]
		},
		{
			title: 'Deployment',
			icon: 'solar:server-bold',
			items: [
				{ title: 'Self-Hosting', href: '/docs/self-hosting' },
				{ title: 'Docker Setup', href: '/docs/docker' },
				{ title: 'Production Deployment', href: '/docs/production' }
			]
		},
		{
			title: 'Advanced',
			icon: 'solar:settings-bold',
			items: [
				{ title: 'Custom Aliases', href: '/docs/aliases' },
				{ title: 'Build Optimization', href: '/docs/build-optimization' },
				{ title: 'Troubleshooting', href: '/docs/troubleshooting' }
			]
		}
	];

	function isActive(href: string, exact = false): boolean {
		if (exact) {
			return $page.url.pathname === href;
		}
		return $page.url.pathname.startsWith(href) && $page.url.pathname !== '/docs';
	}

	let sidebarOpen = false;
	
	$: isLoggedIn = $authStore.isAuthenticated;
</script>

<svelte:head>
	<title>Documentation - Knot Space</title>
	<meta name="description" content="Complete guide to using Knot CLI for monorepo package management, TypeScript integration, and team collaboration." />
	
	<!-- Essential Meta Tags -->
	<meta name="keywords" content="knot cli, monorepo, package manager, typescript, javascript, package registry, team collaboration, build tools" />
	<meta name="author" content="Knot Space" />
	<meta name="robots" content="index, follow" />
	<meta name="language" content="en" />
	<meta name="theme-color" content="#000000" />
	
	<!-- Open Graph Meta Tags -->
	<meta property="og:type" content="website" />
	<meta property="og:title" content="Knot CLI Documentation - Modern Monorepo Package Manager" />
	<meta property="og:description" content="Complete guide to using Knot CLI for monorepo package management, TypeScript integration, and team collaboration." />
	<meta property="og:image" content="/images/og/docs-main.png" />
	<meta property="og:image:width" content="1200" />
	<meta property="og:image:height" content="630" />
	<meta property="og:image:alt" content="Knot CLI Documentation - Modern Monorepo Package Manager" />
	<meta property="og:url" content="https://knot-space.com/docs" />
	<meta property="og:site_name" content="Knot Space" />
	<meta property="og:locale" content="en_US" />
	
	<!-- Twitter Card Meta Tags -->
	<meta name="twitter:card" content="summary_large_image" />
	<meta name="twitter:site" content="@knotspace" />
	<meta name="twitter:creator" content="@knotspace" />
	<meta name="twitter:title" content="Knot CLI Documentation - Modern Monorepo Package Manager" />
	<meta name="twitter:description" content="Complete guide to using Knot CLI for monorepo package management, TypeScript integration, and team collaboration." />
	<meta name="twitter:image" content="/images/og/docs-main.png" />
	<meta name="twitter:image:alt" content="Knot CLI Documentation" />
	
	<!-- Additional SEO Meta Tags -->
	<link rel="canonical" href="https://knot-space.com/docs" />
	<meta name="format-detection" content="telephone=no" />
	<meta name="HandheldFriendly" content="true" />
	
	<!-- Structured Data -->
	<script type="application/ld+json">
	{
		"@context": "https://schema.org",
		"@type": "TechArticle",
		"headline": "Knot CLI Documentation",
		"description": "Complete guide to using Knot CLI for monorepo package management, TypeScript integration, and team collaboration.",
		"author": {
			"@type": "Organization",
			"name": "Knot Space"
		},
		"publisher": {
			"@type": "Organization",
			"name": "Knot Space",
			"logo": {
				"@type": "ImageObject",
				"url": "https://knot-space.com/images/logo.png"
			}
		},
		"datePublished": "2024-01-01",
		"dateModified": "2024-01-15",
		"mainEntityOfPage": {
			"@type": "WebPage",
			"@id": "https://knot-space.com/docs"
		},
		"image": "https://knot-space.com/images/og/docs-main.png",
		"articleSection": "Documentation",
		"keywords": ["monorepo", "package manager", "typescript", "cli tools", "build tools"]
	}
	</script>
</svelte:head>

<div class="min-h-screen bg-background">
	<!-- Mobile sidebar overlay -->
	{#if sidebarOpen}
		<div class="fixed inset-0 z-40 lg:hidden">
			<div 
				class="fixed inset-0 bg-black/50" 
				role="button"
				tabindex="0"
				on:click={() => sidebarOpen = false}
				on:keydown={(e) => { if (e.key === 'Escape') sidebarOpen = false; }}
				aria-label="Close sidebar"
			></div>
		</div>
	{/if}

	<!-- Sidebar -->
	<div class="fixed inset-y-0 left-0 z-50 w-72 bg-background border-r border-border transform {sidebarOpen ? 'translate-x-0' : '-translate-x-full'} transition-transform lg:translate-x-0">
		<div class="flex h-full flex-col">
			<!-- Header -->
			<div class="flex h-16 items-center justify-between px-6 border-b border-border">
				<div class="flex items-center space-x-4">
					<a href="/docs" class="flex items-center space-x-2">
						<h1 class="text-xl font-bold tracking-tight" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
							Knot Docs
						</h1>
					</a>
					<div class="hidden lg:flex items-center space-x-2 text-sm">
						<span class="text-muted-foreground">•</span>
						{#if isLoggedIn}
							<a href="/packages" class="text-muted-foreground hover:text-foreground transition-colors">
								Dashboard
							</a>
						{:else}
							<a href="/" class="text-muted-foreground hover:text-foreground transition-colors">
								Home
							</a>
						{/if}
					</div>
				</div>
				<button 
					on:click={() => sidebarOpen = false}
					class="lg:hidden p-2 rounded-md hover:bg-accent"
				>
					<Icon icon="solar:close-circle-bold" class="w-5 h-5" />
				</button>
			</div>

			<!-- Navigation -->
			<nav class="flex-1 overflow-y-auto py-6 px-4">
				<div class="space-y-8">
					{#each sections as section}
						<div>
							<div class="flex items-center space-x-2 mb-3 px-2">
								<Icon icon={section.icon} class="w-4 h-4 text-muted-foreground" />
								<h3 class="text-sm font-semibold text-muted-foreground uppercase tracking-wider">
									{section.title}
								</h3>
							</div>
							<ul class="space-y-1">
								{#each section.items as item}
									<li>
										<a
											href={item.href}
											class="block px-3 py-2 text-sm rounded-md transition-colors {isActive(item.href, item.exact) 
												? 'bg-primary text-primary-foreground font-medium' 
												: 'text-muted-foreground hover:text-foreground hover:bg-accent'}"
											on:click={() => sidebarOpen = false}
										>
											{item.title}
										</a>
									</li>
								{/each}
							</ul>
						</div>
					{/each}
				</div>
			</nav>

			<!-- Footer -->
			<div class="border-t border-border p-4">
				<div class="flex items-center justify-between text-xs text-muted-foreground">
					<span>Knot v1.0.0</span>
					<a href="https://github.com/saravenpi/knot" class="hover:text-foreground transition-colors">
						<Icon icon="solar:link-bold" class="w-4 h-4" />
					</a>
				</div>
			</div>
		</div>
	</div>

	<!-- Main content -->
	<div class="lg:pl-72">
		<!-- Top bar for mobile -->
		<div class="sticky top-0 z-30 lg:hidden h-16 bg-background border-b border-border flex items-center justify-between px-4">
			<button
				on:click={() => sidebarOpen = true}
				class="p-2 rounded-md hover:bg-accent"
			>
				<Icon icon="solar:hamburger-menu-bold" class="w-5 h-5" />
			</button>
			<h1 class="text-lg font-semibold tracking-tight" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
				Knot Docs
			</h1>
			{#if isLoggedIn}
				<a href="/packages" class="p-2 rounded-md hover:bg-accent text-muted-foreground hover:text-foreground transition-colors">
					<Icon icon="solar:widget-2-bold" class="w-5 h-5" />
				</a>
			{:else}
				<a href="/" class="p-2 rounded-md hover:bg-accent text-muted-foreground hover:text-foreground transition-colors">
					<Icon icon="solar:home-2-bold" class="w-5 h-5" />
				</a>
			{/if}
		</div>

		<!-- Page content -->
		<main class="min-h-screen">
			<slot />
		</main>
	</div>
</div>