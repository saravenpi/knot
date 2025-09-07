<script lang="ts">
	import { page } from '$app/stores';
	import Icon from '@iconify/svelte';

	const sections = [
		{
			title: 'Getting Started',
			icon: 'lucide:rocket',
			items: [
				{ title: 'Introduction', href: '/docs', exact: true },
				{ title: 'Installation', href: '/docs/installation' },
				{ title: 'Quick Start', href: '/docs/quick-start' },
				{ title: 'Authentication', href: '/docs/authentication' }
			]
		},
		{
			title: 'Core Concepts',
			icon: 'lucide:book-open',
			items: [
				{ title: 'Project Structure', href: '/docs/project-structure' },
				{ title: 'Configuration Files', href: '/docs/configuration' },
				{ title: 'Package Linking', href: '/docs/package-linking' },
				{ title: 'TypeScript Integration', href: '/docs/typescript' }
			]
		},
		{
			title: 'Package Management',
			icon: 'lucide:package',
			items: [
				{ title: 'Project Management', href: '/docs/project-management' },
				{ title: 'Package Development', href: '/docs/package-development' },
				{ title: 'Publishing', href: '/docs/publishing' }
			]
		},
		{
			title: 'Team Collaboration',
			icon: 'lucide:users',
			items: [
				{ title: 'Team Management', href: '/docs/teams' },
				{ title: 'Permissions', href: '/docs/permissions' },
				{ title: 'Workflow Best Practices', href: '/docs/workflows' }
			]
		},
		{
			title: 'Deployment',
			icon: 'lucide:server',
			items: [
				{ title: 'Self-Hosting', href: '/docs/self-hosting' },
				{ title: 'Docker Integration', href: '/docs/docker' },
				{ title: 'Production Deployment', href: '/docs/production' }
			]
		},
		{
			title: 'Advanced',
			icon: 'lucide:settings',
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
	<meta name="theme-color" content="#ffffff" />

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

<div class="min-h-screen bg-white">
	<!-- Mobile sidebar overlay -->
	{#if sidebarOpen}
		<div class="fixed inset-0 z-40 lg:hidden">
			<div
				class="fixed inset-0 bg-black/30"
				role="button"
				tabindex="0"
				on:click={() => sidebarOpen = false}
				on:keydown={(e) => { if (e.key === 'Escape') sidebarOpen = false; }}
				aria-label="Close sidebar"
			></div>
		</div>
	{/if}

	<!-- Sidebar -->
	<div class="fixed inset-y-0 left-0 z-50 w-80 bg-white border-r border-gray-200 transform {sidebarOpen ? 'translate-x-0' : '-translate-x-full'} transition-transform duration-200 ease-out lg:translate-x-0">
		<div class="flex h-full flex-col">
			<!-- Header -->
			<div class="flex h-20 items-center justify-between px-8 border-b border-gray-200">
				<div class="flex items-center space-x-4">
					<a href="/docs" class="flex items-center space-x-3 group">
						<div class="w-8 h-8 bg-gray-100 rounded-lg flex items-center justify-center group-hover:bg-gray-200 transition-colors duration-200">
							<Icon icon="lucide:book-open" class="w-4 h-4 text-gray-700" />
						</div>
						<h1 class="text-xl font-semibold text-gray-900 group-hover:text-black transition-colors duration-200" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
							Knot Docs
						</h1>
					</a>
				</div>
				<div class="flex items-center space-x-3">
					<a href="/" class="flex items-center text-sm text-gray-600 hover:text-gray-900 transition-colors duration-200 px-3 py-1.5 rounded-md hover:bg-gray-100">
						<Icon icon="lucide:arrow-left" class="w-3 h-3 mr-1.5" />
						Back to Knot Space
					</a>
					<button
						on:click={() => sidebarOpen = false}
						class="lg:hidden p-2 rounded-lg hover:bg-gray-100 text-gray-600 hover:text-gray-900 transition-colors duration-200"
					>
						<Icon icon="lucide:x" class="w-5 h-5" />
					</button>
				</div>
			</div>

			<!-- Navigation -->
			<nav class="flex-1 overflow-y-auto py-6 px-4">
				<div class="space-y-6">
					{#each sections as section}
						<div>
							<div class="flex items-center space-x-2 mb-3 px-2">
								<Icon icon={section.icon} class="w-4 h-4 text-gray-500" />
								<h3 class="text-xs font-semibold text-gray-600 uppercase tracking-wide">
									{section.title}
								</h3>
							</div>
							<ul class="space-y-1">
								{#each section.items as item}
									<li>
										<a
											href={item.href}
											class="block px-3 py-2 text-sm rounded-md transition-colors {isActive(item.href, item.exact)
												? 'bg-blue-50 text-blue-700 font-medium'
												: 'text-gray-700 hover:text-gray-900 hover:bg-gray-50'}"
											on:click={() => sidebarOpen = false}
										>
											<div class="flex items-center">
												<div class="w-1.5 h-1.5 {isActive(item.href, item.exact) ? 'bg-blue-600' : 'bg-gray-400'} rounded-full mr-3"></div>
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
			<div class="border-t border-gray-200 p-6 bg-gray-50">
				<div class="flex items-center justify-between text-xs">
					<div class="flex items-center space-x-2">
						<div class="w-2 h-2 bg-green-500 rounded-full"></div>
						<span class="text-gray-600 font-medium">Knot v1.0.0</span>
					</div>
					<a href="https://github.com/saravenpi/knot" class="flex items-center space-x-2 text-gray-600 hover:text-gray-900 transition-colors duration-200 px-3 py-1.5 rounded-md hover:bg-gray-100 group">
						<Icon icon="lucide:github" class="w-4 h-4" />
						<span class="font-medium">GitHub</span>
					</a>
				</div>
			</div>
		</div>
	</div>

	<!-- Main content -->
	<div class="lg:pl-80">
		<!-- Top bar for mobile -->
		<div class="sticky top-0 z-30 lg:hidden bg-white border-b border-gray-200">
			<div class="h-16 flex items-center justify-between px-4">
				<button
					on:click={() => sidebarOpen = true}
					class="p-2.5 rounded-lg hover:bg-gray-100 transition-colors duration-200"
					aria-label="Open navigation menu"
				>
					<Icon icon="lucide:menu" class="w-5 h-5 text-gray-700" />
				</button>
				<div class="flex items-center space-x-2">
					<div class="w-6 h-6 bg-gray-100 rounded-md flex items-center justify-center">
						<Icon icon="lucide:book-open" class="w-3 h-3 text-gray-700" />
					</div>
					<div class="flex flex-col items-center">
						<h1 class="text-lg font-semibold text-gray-900" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
							Knot Docs
						</h1>
						{#if currentPageInfo.section && $page.url.pathname !== '/docs'}
							<div class="text-xs text-gray-600 font-medium">
								{currentPageInfo.section}
							</div>
						{/if}
					</div>
				</div>
				<div class="w-9"></div> <!-- Spacer for centering -->
			</div>
			
			{#if currentPageInfo.page && $page.url.pathname !== '/docs'}
				<div class="px-4 pb-3 border-t border-gray-100">
					<div class="flex items-center space-x-1 text-sm mt-2">
						<a href="/docs" class="text-gray-600 hover:text-gray-900 transition-colors font-medium">Docs</a>
						<Icon icon="lucide:chevron-right" class="w-3 h-3 text-gray-400" />
						<span class="text-gray-900 font-semibold">{currentPageInfo.page}</span>
					</div>
				</div>
			{/if}
		</div>

		<!-- Page content -->
		<main class="min-h-screen bg-white">
			<slot />
		</main>
	</div>
</div>
