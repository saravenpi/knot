<script lang="ts">
	import Icon from '@iconify/svelte';
</script>

<svelte:head>
	<title>Terms of Service - Knot Space</title>
	<meta name="description" content="Knot Space Terms of Service - Terms and conditions for using our package registry service" />
</svelte:head>

<div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
	<div class="mb-8">
		<div class="flex items-center gap-3 mb-4">
			<Icon icon="solar:document-text-bold" class="w-8 h-8 text-primary" />
			<h1 class="text-4xl font-bold" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">Terms of Service</h1>
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
						These Terms of Service govern your use of Knot Space. By using our service, you agree to these terms.
					</p>
				</div>
			</div>
		</div>

		<section class="mb-12">
			<h2 class="text-2xl font-semibold mb-4">Acceptance of Terms</h2>
			<p class="mb-4">
				By accessing or using Knot Space, you agree to be bound by these Terms of Service and our Privacy Policy. 
				If you disagree with any part of these terms, you may not access the service.
			</p>
		</section>

		<section class="mb-12">
			<h2 class="text-2xl font-semibold mb-4">Description of Service</h2>
			<p class="mb-4">
				Knot Space is a package registry service for TypeScript and JavaScript packages built with the Knot monorepo manager. 
				We provide:
			</p>
			<ul class="list-disc list-inside mb-6 space-y-2">
				<li>Package hosting and distribution</li>
				<li>Team collaboration tools</li>
				<li>CLI integration for publishing and consuming packages</li>
				<li>Download statistics and analytics</li>
			</ul>
		</section>

		<section class="mb-12">
			<h2 class="text-2xl font-semibold mb-4">User Accounts</h2>
			
			<h3 class="text-xl font-medium mb-3">Registration</h3>
			<p class="mb-4">To use certain features, you must create an account. You are responsible for:</p>
			<ul class="list-disc list-inside mb-6 space-y-2">
				<li>Providing accurate and complete information</li>
				<li>Maintaining the security of your account credentials</li>
				<li>All activities that occur under your account</li>
				<li>Notifying us immediately of any unauthorized use</li>
			</ul>

			<h3 class="text-xl font-medium mb-3">Account Termination</h3>
			<p class="mb-6">
				We may suspend or terminate your account at any time for violations of these terms or for any other reason 
				at our sole discretion.
			</p>
		</section>

		<section class="mb-12">
			<h2 class="text-2xl font-semibold mb-4">Package Content and Conduct</h2>
			
			<h3 class="text-xl font-medium mb-3">Acceptable Use</h3>
			<p class="mb-4">You may not upload or publish content that:</p>
			<ul class="list-disc list-inside mb-6 space-y-2">
				<li>Contains malicious code, viruses, or security vulnerabilities</li>
				<li>Infringes on intellectual property rights</li>
				<li>Violates any applicable laws or regulations</li>
				<li>Contains hate speech, harassment, or harmful content</li>
				<li>Impersonates another person or entity</li>
				<li>Attempts to compromise or interfere with our service</li>
			</ul>

			<h3 class="text-xl font-medium mb-3">Content Ownership</h3>
			<p class="mb-6">
				You retain ownership of the packages you publish. However, by publishing to Knot Space, you grant us 
				a license to store, distribute, and display your content as necessary to provide the service.
			</p>
		</section>

		<section class="mb-12">
			<h2 class="text-2xl font-semibold mb-4">Intellectual Property</h2>
			
			<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
				<div class="border rounded-lg p-6">
					<h3 class="text-lg font-medium mb-3">Your Content</h3>
					<p class="text-sm mb-3">You are responsible for ensuring you have the right to publish your packages.</p>
					<ul class="text-sm space-y-1">
						<li>• Own all rights to your code</li>
						<li>• Comply with open source licenses</li>
						<li>• Respect third-party copyrights</li>
					</ul>
				</div>

				<div class="border rounded-lg p-6">
					<h3 class="text-lg font-medium mb-3">Our Service</h3>
					<p class="text-sm mb-3">Knot Space and its features are protected by intellectual property laws.</p>
					<ul class="text-sm space-y-1">
						<li>• Service design and functionality</li>
						<li>• Trademarks and branding</li>
						<li>• Proprietary software and systems</li>
					</ul>
				</div>
			</div>
		</section>

		<section class="mb-12">
			<h2 class="text-2xl font-semibold mb-4">Service Availability</h2>
			
			<div class="bg-yellow-50 border border-yellow-200 rounded-lg p-6 mb-6">
				<h3 class="text-lg font-medium text-yellow-800 mb-2">Service Level</h3>
				<p class="text-yellow-700 text-sm">
					We strive to maintain high availability but do not guarantee uninterrupted service. 
					Maintenance, updates, and unforeseen issues may cause temporary outages.
				</p>
			</div>

			<h3 class="text-xl font-medium mb-3">Service Modifications</h3>
			<p class="mb-6">
				We reserve the right to modify, suspend, or discontinue any part of the service at any time. 
				We will provide reasonable notice for significant changes when possible.
			</p>
		</section>

		<section class="mb-12">
			<h2 class="text-2xl font-semibold mb-4">Payment and Billing</h2>
			
			<h3 class="text-xl font-medium mb-3">Free Tier</h3>
			<p class="mb-4">
				Basic use of Knot Space is currently free. This includes package publishing, downloading, and basic team features.
			</p>

			<h3 class="text-xl font-medium mb-3">Future Premium Features</h3>
			<p class="mb-6">
				We may introduce premium features in the future. Any paid services will have clear pricing and terms 
				communicated before billing.
			</p>
		</section>

		<section class="mb-12">
			<h2 class="text-2xl font-semibold mb-4">Disclaimers and Limitation of Liability</h2>
			
			<div class="bg-red-50 border border-red-200 rounded-lg p-6 mb-6">
				<h3 class="text-lg font-medium text-red-800 mb-2">Service Warranty</h3>
				<p class="text-red-700 text-sm mb-3">
					The service is provided "as is" without warranties of any kind, either express or implied.
				</p>
				<p class="text-red-700 text-sm">
					We do not warrant that the service will be error-free, secure, or available at all times.
				</p>
			</div>

			<h3 class="text-xl font-medium mb-3">Limitation of Liability</h3>
			<p class="mb-6">
				To the maximum extent permitted by law, we shall not be liable for any indirect, incidental, special, 
				consequential, or punitive damages arising from your use of the service.
			</p>
		</section>

		<section class="mb-12">
			<h2 class="text-2xl font-semibold mb-4">Indemnification</h2>
			<p class="mb-6">
				You agree to indemnify and hold us harmless from any claims, damages, or expenses arising from your use of 
				the service, your packages, or your violation of these terms.
			</p>
		</section>

		<section class="mb-12">
			<h2 class="text-2xl font-semibold mb-4">Governing Law</h2>
			<p class="mb-6">
				These terms are governed by and construed in accordance with applicable law. Any disputes shall be resolved 
				through binding arbitration or in the appropriate courts.
			</p>
		</section>

		<section class="mb-12">
			<h2 class="text-2xl font-semibold mb-4">Changes to Terms</h2>
			<p class="mb-4">
				We may update these terms from time to time. When we make changes, we will:
			</p>
			<ul class="list-disc list-inside mb-6 space-y-2">
				<li>Update the "Last updated" date</li>
				<li>Notify users of significant changes</li>
				<li>Provide a reasonable notice period before changes take effect</li>
			</ul>
			<p class="mb-6">
				Continued use of the service after changes constitutes acceptance of the new terms.
			</p>
		</section>

		<section class="mb-12 bg-gray-50 rounded-lg p-6">
			<h2 class="text-2xl font-semibold mb-4">Contact Information</h2>
			
			<p class="mb-4">
				For questions about these terms or to report violations, please contact us:
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