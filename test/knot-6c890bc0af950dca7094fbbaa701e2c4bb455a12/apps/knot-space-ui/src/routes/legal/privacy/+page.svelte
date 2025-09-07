<script lang="ts">
	import Icon from '@iconify/svelte';
</script>

<svelte:head>
	<title>Privacy Policy - Knot Space</title>
	<meta name="description" content="Knot Space Privacy Policy - How we collect, use, and protect your data" />
</svelte:head>

<div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
	<div class="mb-8">
		<div class="flex items-center gap-3 mb-4">
			<Icon icon="solar:shield-check-bold" class="w-8 h-8 text-primary" />
			<h1 class="text-4xl font-bold" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">Privacy Policy</h1>
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
						At Knot Space, your privacy is important to us. This policy explains how we collect, use, and protect your personal information when you use our package registry service.
					</p>
				</div>
			</div>
		</div>

		<section class="mb-12">
			<h2 class="text-2xl font-semibold mb-4">Information We Collect</h2>
			
			<h3 class="text-xl font-medium mb-3">Account Information</h3>
			<p class="mb-4">When you create an account, we collect:</p>
			<ul class="list-disc list-inside mb-6 space-y-2">
				<li>Username and email address</li>
				<li>Profile information you choose to provide</li>
				<li>Team memberships and roles</li>
			</ul>

			<h3 class="text-xl font-medium mb-3">Package Data</h3>
			<p class="mb-4">When you publish packages, we store:</p>
			<ul class="list-disc list-inside mb-6 space-y-2">
				<li>Package content and metadata</li>
				<li>Version history and release notes</li>
				<li>Download statistics (anonymized)</li>
				<li>Package descriptions and tags</li>
			</ul>

			<h3 class="text-xl font-medium mb-3">Usage Information</h3>
			<p class="mb-4">We automatically collect:</p>
			<ul class="list-disc list-inside mb-6 space-y-2">
				<li>IP addresses and browser information</li>
				<li>API usage patterns and performance metrics</li>
				<li>Error logs and debugging information</li>
			</ul>
		</section>

		<section class="mb-12">
			<h2 class="text-2xl font-semibold mb-4">How We Use Your Information</h2>
			
			<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
				<div class="border rounded-lg p-6">
					<div class="flex items-center gap-3 mb-3">
						<Icon icon="solar:server-bold" class="w-6 h-6 text-green-600" />
						<h3 class="text-lg font-medium">Service Operation</h3>
					</div>
					<ul class="space-y-2 text-sm">
						<li>• Provide package registry services</li>
						<li>• Manage user accounts and authentication</li>
						<li>• Process package uploads and downloads</li>
						<li>• Handle team management and permissions</li>
					</ul>
				</div>

				<div class="border rounded-lg p-6">
					<div class="flex items-center gap-3 mb-3">
						<Icon icon="solar:chart-bold" class="w-6 h-6 text-blue-600" />
						<h3 class="text-lg font-medium">Analytics & Improvement</h3>
					</div>
					<ul class="space-y-2 text-sm">
						<li>• Monitor service performance and reliability</li>
						<li>• Generate usage statistics and insights</li>
						<li>• Improve user experience and features</li>
						<li>• Detect and prevent abuse</li>
					</ul>
				</div>

				<div class="border rounded-lg p-6">
					<div class="flex items-center gap-3 mb-3">
						<Icon icon="solar:mail-bold" class="w-6 h-6 text-purple-600" />
						<h3 class="text-lg font-medium">Communication</h3>
					</div>
					<ul class="space-y-2 text-sm">
						<li>• Send important service notifications</li>
						<li>• Respond to support requests</li>
						<li>• Share updates about new features</li>
						<li>• Deliver security alerts</li>
					</ul>
				</div>

				<div class="border rounded-lg p-6">
					<div class="flex items-center gap-3 mb-3">
						<Icon icon="solar:shield-minimalistic-bold" class="w-6 h-6 text-red-600" />
						<h3 class="text-lg font-medium">Security & Compliance</h3>
					</div>
					<ul class="space-y-2 text-sm">
						<li>• Prevent fraud and abuse</li>
						<li>• Ensure data integrity and security</li>
						<li>• Comply with legal requirements</li>
						<li>• Investigate security incidents</li>
					</ul>
				</div>
			</div>
		</section>

		<section class="mb-12">
			<h2 class="text-2xl font-semibold mb-4">Data Sharing and Disclosure</h2>
			
			<div class="bg-green-50 border border-green-200 rounded-lg p-6 mb-6">
				<h3 class="text-lg font-medium text-green-800 mb-2">What We Don't Do</h3>
				<p class="text-green-700">
					We do not sell, rent, or trade your personal information to third parties for marketing purposes.
				</p>
			</div>

			<h3 class="text-xl font-medium mb-3">Public Information</h3>
			<p class="mb-4">The following information is publicly visible:</p>
			<ul class="list-disc list-inside mb-6 space-y-2">
				<li>Your username and profile information</li>
				<li>Published packages and their metadata</li>
				<li>Package download statistics</li>
				<li>Team memberships (if made public)</li>
			</ul>

			<h3 class="text-xl font-medium mb-3">Limited Sharing</h3>
			<p class="mb-4">We may share information in these specific circumstances:</p>
			<ul class="list-disc list-inside mb-6 space-y-2">
				<li>With your explicit consent</li>
				<li>To comply with legal obligations</li>
				<li>To protect our rights and prevent abuse</li>
				<li>With service providers who help operate our platform</li>
			</ul>
		</section>

		<section class="mb-12">
			<h2 class="text-2xl font-semibold mb-4">Data Security</h2>
			
			<div class="grid grid-cols-1 md:grid-cols-3 gap-6">
				<div class="text-center">
					<div class="w-12 h-12 bg-blue-100 rounded-lg flex items-center justify-center mx-auto mb-3">
						<Icon icon="solar:lock-bold" class="w-6 h-6 text-blue-600" />
					</div>
					<h3 class="font-medium mb-2">Encryption</h3>
					<p class="text-sm text-muted-foreground">All data is encrypted in transit and at rest using industry-standard protocols.</p>
				</div>

				<div class="text-center">
					<div class="w-12 h-12 bg-green-100 rounded-lg flex items-center justify-center mx-auto mb-3">
						<Icon icon="solar:key-bold" class="w-6 h-6 text-green-600" />
					</div>
					<h3 class="font-medium mb-2">Access Control</h3>
					<p class="text-sm text-muted-foreground">Strict access controls and authentication mechanisms protect your data.</p>
				</div>

				<div class="text-center">
					<div class="w-12 h-12 bg-purple-100 rounded-lg flex items-center justify-center mx-auto mb-3">
						<Icon icon="solar:monitor-bold" class="w-6 h-6 text-purple-600" />
					</div>
					<h3 class="font-medium mb-2">Monitoring</h3>
					<p class="text-sm text-muted-foreground">Continuous monitoring and incident response to protect against threats.</p>
				</div>
			</div>
		</section>

		<section class="mb-12">
			<h2 class="text-2xl font-semibold mb-4">Your Rights and Choices</h2>
			
			<div class="space-y-6">
				<div class="border-l-4 border-blue-400 pl-6">
					<h3 class="text-lg font-medium mb-2">Access and Portability</h3>
					<p>You can access and export your data through your account settings or by contacting us.</p>
				</div>

				<div class="border-l-4 border-green-400 pl-6">
					<h3 class="text-lg font-medium mb-2">Correction and Updates</h3>
					<p>You can update your profile information and package metadata at any time.</p>
				</div>

				<div class="border-l-4 border-orange-400 pl-6">
					<h3 class="text-lg font-medium mb-2">Deletion</h3>
					<p>You can delete your account and associated data, though published packages may remain for dependency reasons.</p>
				</div>

				<div class="border-l-4 border-red-400 pl-6">
					<h3 class="text-lg font-medium mb-2">Objection</h3>
					<p>You can object to certain processing activities, subject to service functionality requirements.</p>
				</div>
			</div>
		</section>

		<section class="mb-12">
			<h2 class="text-2xl font-semibold mb-4">Cookies and Tracking</h2>
			
			<p class="mb-4">We use cookies and similar technologies to:</p>
			<ul class="list-disc list-inside mb-6 space-y-2">
				<li>Keep you logged in to your account</li>
				<li>Remember your preferences and settings</li>
				<li>Analyze how our service is used</li>
				<li>Provide security protections</li>
			</ul>

			<p class="mb-4">
				You can manage cookie preferences in your browser settings. Note that disabling certain cookies may affect service functionality.
			</p>
		</section>

		<section class="mb-12">
			<h2 class="text-2xl font-semibold mb-4">Changes to This Policy</h2>
			
			<p class="mb-4">
				We may update this privacy policy from time to time. When we make changes, we will:
			</p>
			<ul class="list-disc list-inside mb-6 space-y-2">
				<li>Update the "Last updated" date at the top of this page</li>
				<li>Notify you of significant changes via email or service notifications</li>
				<li>Give you the opportunity to review changes before they take effect</li>
			</ul>
		</section>

		<section class="mb-12 bg-gray-50 rounded-lg p-6">
			<h2 class="text-2xl font-semibold mb-4">Contact Us</h2>
			
			<p class="mb-4">
				If you have questions about this privacy policy or how we handle your data, please contact us:
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