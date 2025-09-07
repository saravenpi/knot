<script lang="ts">
	import Icon from '@iconify/svelte';
</script>

<svelte:head>
	<title>Cookie Policy - Knot Space</title>
	<meta name="description" content="Knot Space Cookie Policy - How we use cookies and similar technologies" />
</svelte:head>

<div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
	<div class="mb-8">
		<div class="flex items-center gap-3 mb-4">
			<Icon icon="solar:cookie-bold" class="w-8 h-8 text-primary" />
			<h1 class="text-4xl font-bold" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">Cookie Policy</h1>
		</div>
		<p class="text-muted-foreground text-lg">
			Last updated: January 19, 2025
		</p>
	</div>

	<div class="prose prose-lg max-w-none">
		<div class="bg-blue-50 border-l-4 border-blue-400 p-6 mb-8">
			<div class="flex">
				<div class="flex-shrink-0">
					<Icon icon="solar:info-circle-bold" class="w-5 h-5 text-blue-400" />
				</div>
				<div class="ml-3">
					<p class="text-sm text-blue-700">
						This Cookie Policy explains how Knot Space uses cookies and similar technologies to provide and improve our service.
					</p>
				</div>
			</div>
		</div>

		<section class="mb-12">
			<h2 class="text-2xl font-semibold mb-4">What are Cookies?</h2>
			<p class="mb-6">
				Cookies are small text files that are stored on your device when you visit a website. They help websites remember 
				information about your visit, such as your preferences and login status, which can make your next visit easier 
				and the site more useful to you.
			</p>
		</section>

		<section class="mb-12">
			<h2 class="text-2xl font-semibold mb-4">How We Use Cookies</h2>
			
			<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
				<div class="border rounded-lg p-6">
					<div class="flex items-center gap-3 mb-3">
						<Icon icon="solar:key-bold" class="w-6 h-6 text-green-600" />
						<h3 class="text-lg font-medium">Authentication</h3>
					</div>
					<p class="text-sm text-muted-foreground mb-3">Keep you securely logged in</p>
					<ul class="space-y-1 text-sm">
						<li>• Session management</li>
						<li>• Security tokens</li>
						<li>• Login persistence</li>
					</ul>
				</div>

				<div class="border rounded-lg p-6">
					<div class="flex items-center gap-3 mb-3">
						<Icon icon="solar:settings-bold" class="w-6 h-6 text-blue-600" />
						<h3 class="text-lg font-medium">Preferences</h3>
					</div>
					<p class="text-sm text-muted-foreground mb-3">Remember your settings</p>
					<ul class="space-y-1 text-sm">
						<li>• Language preferences</li>
						<li>• Theme settings</li>
						<li>• Display options</li>
					</ul>
				</div>

				<div class="border rounded-lg p-6">
					<div class="flex items-center gap-3 mb-3">
						<Icon icon="solar:chart-bold" class="w-6 h-6 text-purple-600" />
						<h3 class="text-lg font-medium">Analytics</h3>
					</div>
					<p class="text-sm text-muted-foreground mb-3">Understand service usage</p>
					<ul class="space-y-1 text-sm">
						<li>• Performance metrics</li>
						<li>• Usage patterns</li>
						<li>• Error tracking</li>
					</ul>
				</div>

				<div class="border rounded-lg p-6">
					<div class="flex items-center gap-3 mb-3">
						<Icon icon="solar:shield-check-bold" class="w-6 h-6 text-red-600" />
						<h3 class="text-lg font-medium">Security</h3>
					</div>
					<p class="text-sm text-muted-foreground mb-3">Protect against threats</p>
					<ul class="space-y-1 text-sm">
						<li>• CSRF protection</li>
						<li>• Rate limiting</li>
						<li>• Fraud prevention</li>
					</ul>
				</div>
			</div>
		</section>

		<section class="mb-12">
			<h2 class="text-2xl font-semibold mb-4">Types of Cookies We Use</h2>
			
			<div class="space-y-6">
				<div class="border-l-4 border-green-400 pl-6">
					<h3 class="text-lg font-medium mb-2">Essential Cookies</h3>
					<p class="text-sm text-muted-foreground mb-2">Required for the website to function properly. Cannot be disabled.</p>
					<ul class="text-sm space-y-1">
						<li>• Authentication tokens</li>
						<li>• Session identifiers</li>
						<li>• Security cookies</li>
					</ul>
				</div>

				<div class="border-l-4 border-blue-400 pl-6">
					<h3 class="text-lg font-medium mb-2">Functional Cookies</h3>
					<p class="text-sm text-muted-foreground mb-2">Enable enhanced functionality and personalization.</p>
					<ul class="text-sm space-y-1">
						<li>• User preferences</li>
						<li>• Language settings</li>
						<li>• Interface customizations</li>
					</ul>
				</div>

				<div class="border-l-4 border-orange-400 pl-6">
					<h3 class="text-lg font-medium mb-2">Analytics Cookies</h3>
					<p class="text-sm text-muted-foreground mb-2">Help us understand how visitors interact with our website.</p>
					<ul class="text-sm space-y-1">
						<li>• Page views and navigation</li>
						<li>• Feature usage statistics</li>
						<li>• Performance monitoring</li>
					</ul>
				</div>
			</div>
		</section>

		<section class="mb-12">
			<h2 class="text-2xl font-semibold mb-4">Third-Party Cookies</h2>
			
			<p class="mb-4">We may use third-party services that set their own cookies:</p>
			
			<div class="bg-gray-50 rounded-lg p-6">
				<h3 class="text-lg font-medium mb-3">Analytics Services</h3>
				<p class="text-sm text-muted-foreground mb-4">
					We may use analytics services to understand how our service is used and to improve performance.
				</p>
				<ul class="text-sm space-y-2">
					<li>• <strong>Purpose:</strong> Website analytics and performance monitoring</li>
					<li>• <strong>Data:</strong> Anonymous usage statistics</li>
					<li>• <strong>Retention:</strong> Varies by service provider</li>
				</ul>
			</div>
		</section>

		<section class="mb-12">
			<h2 class="text-2xl font-semibold mb-4">Managing Your Cookie Preferences</h2>
			
			<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
				<div class="border rounded-lg p-6">
					<h3 class="text-lg font-medium mb-3">Browser Settings</h3>
					<p class="text-sm text-muted-foreground mb-4">
						You can control cookies through your browser settings:
					</p>
					<ul class="text-sm space-y-2">
						<li>• <button class="text-blue-600 hover:text-blue-800" aria-label="Chrome Cookie Settings">Chrome Cookie Settings</button></li>
						<li>• <button class="text-blue-600 hover:text-blue-800" aria-label="Firefox Cookie Settings">Firefox Cookie Settings</button></li>
						<li>• <button class="text-blue-600 hover:text-blue-800" aria-label="Safari Cookie Settings">Safari Cookie Settings</button></li>
						<li>• <button class="text-blue-600 hover:text-blue-800" aria-label="Edge Cookie Settings">Edge Cookie Settings</button></li>
					</ul>
				</div>

				<div class="border rounded-lg p-6">
					<h3 class="text-lg font-medium mb-3">Cookie Preferences</h3>
					<p class="text-sm text-muted-foreground mb-4">
						Manage your preferences directly:
					</p>
					<div class="space-y-3">
						<button class="w-full bg-blue-600 text-white px-4 py-2 rounded-md text-sm hover:bg-blue-700 transition-colors">
							Cookie Preferences
						</button>
						<p class="text-xs text-muted-foreground">
							Note: Disabling essential cookies may affect site functionality.
						</p>
					</div>
				</div>
			</div>
		</section>

		<section class="mb-12">
			<h2 class="text-2xl font-semibold mb-4">Cookie Retention</h2>
			
			<div class="overflow-x-auto">
				<table class="w-full border-collapse border border-gray-300">
					<thead>
						<tr class="bg-gray-50">
							<th class="border border-gray-300 px-4 py-2 text-left">Cookie Type</th>
							<th class="border border-gray-300 px-4 py-2 text-left">Duration</th>
							<th class="border border-gray-300 px-4 py-2 text-left">Purpose</th>
						</tr>
					</thead>
					<tbody>
						<tr>
							<td class="border border-gray-300 px-4 py-2">Session</td>
							<td class="border border-gray-300 px-4 py-2">Until browser closes</td>
							<td class="border border-gray-300 px-4 py-2">Authentication and security</td>
						</tr>
						<tr>
							<td class="border border-gray-300 px-4 py-2">Persistent</td>
							<td class="border border-gray-300 px-4 py-2">30 days</td>
							<td class="border border-gray-300 px-4 py-2">User preferences and settings</td>
						</tr>
						<tr>
							<td class="border border-gray-300 px-4 py-2">Analytics</td>
							<td class="border border-gray-300 px-4 py-2">2 years</td>
							<td class="border border-gray-300 px-4 py-2">Usage statistics and improvements</td>
						</tr>
					</tbody>
				</table>
			</div>
		</section>

		<section class="mb-12">
			<h2 class="text-2xl font-semibold mb-4">Updates to This Policy</h2>
			<p class="mb-6">
				We may update this Cookie Policy from time to time to reflect changes in our practices or for legal reasons. 
				When we make changes, we will update the "Last updated" date and notify you of significant changes.
			</p>
		</section>

		<section class="mb-12 bg-gray-50 rounded-lg p-6">
			<h2 class="text-2xl font-semibold mb-4">Contact Us</h2>
			
			<p class="mb-4">
				If you have questions about our use of cookies, please contact us:
			</p>
			
			<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
				<div>
					<h3 class="font-medium mb-2">Email</h3>
					<a href="mailto:contact@klysium.com" class="text-blue-600 hover:text-blue-800">contact@klysium.com</a>
				</div>
				
				<div>
					<h3 class="font-medium mb-2">GitHub</h3>
					<a href="https://github.com/saravenpi/knot/issues" class="text-blue-600 hover:text-blue-800">Report on GitHub</a>
				</div>
			</div>
		</section>
	</div>
</div>