<script lang="ts">
	import { page } from '$app/stores';
	import Icon from '@iconify/svelte';

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
			title: 'Package Management',
			icon: 'solar:box-bold',
			items: [
				{ title: 'Project Management', href: '/docs/project-management' },
				{ title: 'Package Development', href: '/docs/package-development' },
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
				{ title: 'Docker Integration', href: '/docs/docker' },
				{ title: 'Production Deployment', href: '/docs/production' }
			]
		},
		{
			title: 'Advanced',
			icon: 'solar:settings-bold',
			items: [
				{ title: 'TypeScript Path Aliases', href: '/docs/aliases' },
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

	// Get current page info for breadcrumbs
	$: currentPageInfo = (() => {
		const currentPath = $page.url.pathname;
		for (const section of sections) {
			for (const item of section.items) {
				if (isActive(item.href, item.exact)) {
					return { section: section.title, page: item.title };
				}
			}
		}
		return { section: '', page: '' };
	})();

	let sidebarOpen = false;

	// Close sidebar on escape key
	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Escape' && sidebarOpen) {
			sidebarOpen = false;
		}
	}
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
	<meta property="og:url" content="https://knot.klysium.com/docs" />
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
	<link rel="canonical" href="https://knot.klysium.com/docs" />
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
				"url": "https://knot.klysium.com/images/logo.png"
			}
		},
		"datePublished": "2025-01-01",
		"dateModified": "2025-01-15",
		"mainEntityOfPage": {
			"@type": "WebPage",
			"@id": "https://knot.klysium.com/docs"
		},
		"image": "https://knot.klysium.com/images/og/docs-main.png",
		"articleSection": "Documentation",
		"keywords": ["monorepo", "package manager", "typescript", "cli tools", "build tools"]
	}
	</script>
</svelte:head>

<svelte:window on:keydown={handleKeydown} />

<div class="min-h-screen bg-gradient-to-br from-slate-50 via-white to-slate-100">
	<!-- Mobile sidebar overlay -->
	{#if sidebarOpen}
		<div class="fixed inset-0 z-40 lg:hidden">
			<div
				class="fixed inset-0 bg-black/60 backdrop-blur-sm"
				role="button"
				tabindex="0"
				on:click={() => sidebarOpen = false}
				on:keydown={(e) => { if (e.key === 'Escape') sidebarOpen = false; }}
				aria-label="Close sidebar"
			></div>
		</div>
	{/if}

	<!-- Sidebar -->
	<div class="fixed inset-y-0 left-0 z-50 w-80 bg-gradient-to-b from-slate-900 via-slate-800 to-slate-900 backdrop-blur-xl border-r border-slate-700/50 shadow-2xl transform {sidebarOpen ? 'translate-x-0' : '-translate-x-full'} transition-all duration-300 ease-out lg:translate-x-0">
		<div class="flex h-full flex-col relative">
			<!-- Decorative gradient overlay -->
			<div class="absolute inset-0 bg-gradient-to-br from-blue-600/10 via-purple-600/10 to-indigo-600/10 pointer-events-none"></div>
			
			<!-- Header -->
			<div class="relative flex h-20 items-center justify-between px-8 border-b border-slate-700/50 bg-slate-800/50 backdrop-blur-sm">
				<div class="flex items-center space-x-4">
					<a href="/docs" class="flex items-center space-x-3 group">
						<div class="w-8 h-8 bg-gradient-to-br from-blue-500 to-purple-600 rounded-lg flex items-center justify-center shadow-lg group-hover:shadow-xl transition-all duration-300">
							<Icon icon="solar:book-bold" class="w-4 h-4 text-white" />
						</div>
						<h1 class="text-xl font-bold tracking-tight text-white group-hover:text-blue-300 transition-colors duration-300" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
							Knot Docs
						</h1>
					</a>
				</div>
				<div class="flex items-center space-x-3">
					<a href="/" class="flex items-center text-sm text-slate-400 hover:text-white transition-all duration-300 px-3 py-1.5 rounded-md hover:bg-slate-700/50">
						<Icon icon="solar:arrow-left-bold" class="w-3 h-3 mr-1.5" />
						Back to Knot Space
					</a>
					<button
						on:click={() => sidebarOpen = false}
						class="lg:hidden p-2 rounded-lg hover:bg-slate-700/50 text-slate-400 hover:text-white transition-all duration-300"
					>
						<Icon icon="solar:close-circle-bold" class="w-5 h-5" />
					</button>
				</div>
			</div>

			<!-- Navigation -->
			<nav class="flex-1 overflow-y-auto py-8 px-6 scrollbar-thin scrollbar-thumb-slate-600 scrollbar-track-transparent">
				<div class="space-y-10">
					{#each sections as section}
						<div class="relative">
							<div class="flex items-center space-x-3 mb-4 px-3">
								<div class="w-6 h-6 rounded-md bg-gradient-to-br from-slate-600 to-slate-700 flex items-center justify-center shadow-sm">
									<Icon icon={section.icon} class="w-3.5 h-3.5 text-slate-300" />
								</div>
								<h3 class="text-sm font-bold text-slate-300 uppercase tracking-widest">
									{section.title}
								</h3>
							</div>
							<ul class="space-y-2">
								{#each section.items as item}
									<li>
										<a
											href={item.href}
											class="group relative block px-4 py-3 text-sm rounded-xl transition-all duration-300 {isActive(item.href, item.exact)
												? 'bg-gradient-to-r from-blue-600 to-purple-600 text-white shadow-lg shadow-blue-500/20 font-semibold'
												: 'text-slate-300 hover:text-white hover:bg-slate-700/50 hover:shadow-md hover:shadow-slate-900/20'}"
											on:click={() => sidebarOpen = false}
										>
											<div class="flex items-center">
												{#if isActive(item.href, item.exact)}
													<div class="w-2 h-2 bg-white rounded-full mr-3 animate-pulse"></div>
												{:else}
													<div class="w-2 h-2 bg-slate-500 rounded-full mr-3 group-hover:bg-slate-300 transition-colors duration-300"></div>
												{/if}
												{item.title}
											</div>
										</a>
									</li>
								{/each}
							</ul>
						</div>
					{/each}
				</div>
			</nav>

			<!-- Footer -->
			<div class="relative border-t border-slate-700/50 p-6 bg-slate-800/30 backdrop-blur-sm">
				<div class="flex items-center justify-between text-xs">
					<div class="flex items-center space-x-2">
						<div class="w-2 h-2 bg-green-400 rounded-full animate-pulse"></div>
						<span class="text-slate-400 font-medium">Knot v1.0.0</span>
					</div>
					<a href="https://github.com/saravenpi/knot" class="flex items-center space-x-2 text-slate-400 hover:text-white transition-all duration-300 px-3 py-1.5 rounded-md hover:bg-slate-700/50 group">
						<Icon icon="mdi:github" class="w-4 h-4 group-hover:rotate-12 transition-transform duration-300" />
						<span class="font-medium">GitHub</span>
					</a>
				</div>
			</div>
		</div>
	</div>

	<!-- Main content -->
	<div class="lg:pl-80">
		<!-- Top bar for mobile -->
		<div class="sticky top-0 z-30 lg:hidden bg-white/80 backdrop-blur-md border-b border-slate-200/50 shadow-sm">
			<div class="h-16 flex items-center justify-between px-4">
				<button
					on:click={() => sidebarOpen = true}
					class="p-2.5 rounded-xl hover:bg-slate-100 transition-all duration-300 hover:shadow-md"
					aria-label="Open navigation menu"
				>
					<Icon icon="solar:hamburger-menu-bold" class="w-5 h-5 text-slate-700" />
				</button>
				<div class="flex items-center space-x-2">
					<div class="w-6 h-6 bg-gradient-to-br from-blue-500 to-purple-600 rounded-md flex items-center justify-center shadow-sm">
						<Icon icon="solar:book-bold" class="w-3 h-3 text-white" />
					</div>
					<div class="flex flex-col items-center">
						<h1 class="text-lg font-bold tracking-tight text-slate-900" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
							Knot Docs
						</h1>
						{#if currentPageInfo.section && $page.url.pathname !== '/docs'}
							<div class="text-xs text-slate-500 font-medium">
								{currentPageInfo.section}
							</div>
						{/if}
					</div>
				</div>
				<div class="w-9"></div> <!-- Spacer for centering -->
			</div>
			
			{#if currentPageInfo.page && $page.url.pathname !== '/docs'}
				<div class="px-4 pb-3 border-t border-slate-100">
					<div class="flex items-center space-x-1 text-sm mt-2">
						<a href="/docs" class="text-slate-500 hover:text-slate-900 transition-colors font-medium">Docs</a>
						<Icon icon="solar:arrow-right-bold" class="w-3 h-3 text-slate-400" />
						<span class="text-slate-900 font-semibold">{currentPageInfo.page}</span>
					</div>
				</div>
			{/if}
		</div>

		<!-- Page content with enhanced background -->
		<main class="min-h-screen relative">
			<!-- Decorative background elements -->
			<div class="absolute inset-0 overflow-hidden pointer-events-none">
				<div class="absolute -top-40 -right-40 w-80 h-80 bg-gradient-to-br from-blue-400/10 to-purple-400/10 rounded-full blur-3xl"></div>
				<div class="absolute top-1/2 -left-40 w-96 h-96 bg-gradient-to-br from-indigo-400/10 to-blue-400/10 rounded-full blur-3xl"></div>
				<div class="absolute bottom-0 right-1/3 w-64 h-64 bg-gradient-to-br from-purple-400/10 to-pink-400/10 rounded-full blur-3xl"></div>
			</div>
			
			<!-- Content overlay -->
			<div class="relative">
				<slot />
			</div>
		</main>
	</div>
</div>
