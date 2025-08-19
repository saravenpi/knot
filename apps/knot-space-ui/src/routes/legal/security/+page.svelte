<script lang="ts">
	import Icon from '@iconify/svelte';
</script>

<svelte:head>
	<title>Security - Knot Space</title>
	<meta name="description" content="Knot Space Security - How we protect your data and our commitment to security" />
</svelte:head>

<div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
	<div class="mb-8">
		<div class="flex items-center gap-3 mb-4">
			<Icon icon="solar:shield-check-bold" class="w-8 h-8 text-primary" />
			<h1 class="text-4xl font-bold" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">Security</h1>
		</div>
		<p class="text-muted-foreground text-lg">
			Last updated: January 19, 2025
		</p>
	</div>

	<div class="prose prose-lg max-w-none">
		<div class="bg-green-50 border-l-4 border-green-400 p-6 mb-8">
			<div class="flex">
				<div class="flex-shrink-0">
					<Icon icon="solar:shield-minimalistic-bold" class="w-5 h-5 text-green-400" />
				</div>
				<div class="ml-3">
					<p class="text-sm text-green-700">
						Security is at the core of Knot Space. We implement industry-standard practices to protect your packages, data, and privacy.
					</p>
				</div>
			</div>
		</div>

		<section class="mb-12">
			<h2 class="text-2xl font-semibold mb-4">Our Security Approach</h2>
			
			<div class="grid grid-cols-1 md:grid-cols-3 gap-6">
				<div class="text-center">
					<div class="w-16 h-16 bg-blue-100 rounded-lg flex items-center justify-center mx-auto mb-4">
						<Icon icon="solar:security-safe-bold" class="w-8 h-8 text-blue-600" />
					</div>
					<h3 class="font-semibold text-lg mb-2">Defense in Depth</h3>
					<p class="text-muted-foreground text-sm">
						Multiple layers of security controls protect against various types of threats and attacks.
					</p>
				</div>

				<div class="text-center">
					<div class="w-16 h-16 bg-green-100 rounded-lg flex items-center justify-center mx-auto mb-4">
						<Icon icon="solar:eye-bold" class="w-8 h-8 text-green-600" />
					</div>
					<h3 class="font-semibold text-lg mb-2">Continuous Monitoring</h3>
					<p class="text-muted-foreground text-sm">
						24/7 monitoring and alerting systems detect and respond to security incidents quickly.
					</p>
				</div>

				<div class="text-center">
					<div class="w-16 h-16 bg-purple-100 rounded-lg flex items-center justify-center mx-auto mb-4">
						<Icon icon="solar:refresh-bold" class="w-8 h-8 text-purple-600" />
					</div>
					<h3 class="font-semibold text-lg mb-2">Regular Updates</h3>
					<p class="text-muted-foreground text-sm">
						Frequent security updates and patches keep our systems protected against new threats.
					</p>
				</div>
			</div>
		</section>

		<section class="mb-12">
			<h2 class="text-2xl font-semibold mb-4">Data Protection</h2>
			
			<div class="space-y-6">
				<div class="border rounded-lg p-6">
					<div class="flex items-center gap-3 mb-4">
						<Icon icon="solar:lock-bold" class="w-6 h-6 text-blue-600" />
						<h3 class="text-lg font-medium">Encryption</h3>
					</div>
					<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
						<div>
							<h4 class="font-medium mb-2">Data in Transit</h4>
							<ul class="text-sm text-muted-foreground space-y-1">
								<li>• TLS 1.3 encryption for all connections</li>
								<li>• HTTPS enforced across all services</li>
								<li>• Certificate transparency monitoring</li>
							</ul>
						</div>
						<div>
							<h4 class="font-medium mb-2">Data at Rest</h4>
							<ul class="text-sm text-muted-foreground space-y-1">
								<li>• AES-256 encryption for stored data</li>
								<li>• Encrypted database storage</li>
								<li>• Secure key management systems</li>
							</ul>
						</div>
					</div>
				</div>

				<div class="border rounded-lg p-6">
					<div class="flex items-center gap-3 mb-4">
						<Icon icon="solar:key-bold" class="w-6 h-6 text-green-600" />
						<h3 class="text-lg font-medium">Access Control</h3>
					</div>
					<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
						<div>
							<h4 class="font-medium mb-2">Authentication</h4>
							<ul class="text-sm text-muted-foreground space-y-1">
								<li>• JWT-based authentication</li>
								<li>• Secure password requirements</li>
								<li>• Rate limiting on login attempts</li>
							</ul>
						</div>
						<div>
							<h4 class="font-medium mb-2">Authorization</h4>
							<ul class="text-sm text-muted-foreground space-y-1">
								<li>• Role-based access control (RBAC)</li>
								<li>• Principle of least privilege</li>
								<li>• Team-based permissions</li>
							</ul>
						</div>
					</div>
				</div>
			</div>
		</section>

		<section class="mb-12">
			<h2 class="text-2xl font-semibold mb-4">Infrastructure Security</h2>
			
			<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
				<div class="border rounded-lg p-6">
					<div class="flex items-center gap-3 mb-3">
						<Icon icon="solar:server-bold" class="w-6 h-6 text-indigo-600" />
						<h3 class="text-lg font-medium">Secure Hosting</h3>
					</div>
					<ul class="space-y-2 text-sm">
						<li>• Cloud infrastructure with security certifications</li>
						<li>• Network isolation and firewalls</li>
						<li>• DDoS protection and mitigation</li>
						<li>• Regular security assessments</li>
					</ul>
				</div>

				<div class="border rounded-lg p-6">
					<div class="flex items-center gap-3 mb-3">
						<Icon icon="solar:database-bold" class="w-6 h-6 text-orange-600" />
						<h3 class="text-lg font-medium">Data Storage</h3>
					</div>
					<ul class="space-y-2 text-sm">
						<li>• Encrypted database storage</li>
						<li>• Regular automated backups</li>
						<li>• Geographically distributed storage</li>
						<li>• Data retention policies</li>
					</ul>
				</div>
			</div>
		</section>

		<section class="mb-12">
			<h2 class="text-2xl font-semibold mb-4">Package Security</h2>
			
			<div class="bg-blue-50 border border-blue-200 rounded-lg p-6 mb-6">
				<h3 class="text-lg font-medium text-blue-800 mb-3">Package Integrity</h3>
				<p class="text-blue-700 text-sm mb-4">
					We ensure the integrity and authenticity of packages published to our registry.
				</p>
				<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
					<div>
						<h4 class="font-medium mb-2">Verification</h4>
						<ul class="text-sm space-y-1">
							<li>• Cryptographic checksums</li>
							<li>• Package signature validation</li>
							<li>• Content scanning for malware</li>
						</ul>
					</div>
					<div>
						<h4 class="font-medium mb-2">Monitoring</h4>
						<ul class="text-sm space-y-1">
							<li>• Automated security scanning</li>
							<li>• Vulnerability detection</li>
							<li>• Suspicious activity alerts</li>
						</ul>
					</div>
				</div>
			</div>
		</section>

		<section class="mb-12">
			<h2 class="text-2xl font-semibold mb-4">Incident Response</h2>
			
			<div class="space-y-6">
				<div class="border-l-4 border-red-400 pl-6">
					<h3 class="text-lg font-medium mb-2">Detection</h3>
					<p class="text-sm text-muted-foreground">
						Automated monitoring systems and manual reviews help detect security incidents quickly.
					</p>
				</div>

				<div class="border-l-4 border-orange-400 pl-6">
					<h3 class="text-lg font-medium mb-2">Response</h3>
					<p class="text-sm text-muted-foreground">
						Our incident response team follows established procedures to contain and resolve security issues.
					</p>
				</div>

				<div class="border-l-4 border-yellow-400 pl-6">
					<h3 class="text-lg font-medium mb-2">Communication</h3>
					<p class="text-sm text-muted-foreground">
						We provide timely updates to affected users and the community about security incidents.
					</p>
				</div>

				<div class="border-l-4 border-green-400 pl-6">
					<h3 class="text-lg font-medium mb-2">Recovery</h3>
					<p class="text-sm text-muted-foreground">
						Post-incident analysis helps improve our security measures and prevent future occurrences.
					</p>
				</div>
			</div>
		</section>

		<section class="mb-12">
			<h2 class="text-2xl font-semibold mb-4">Best Practices for Users</h2>
			
			<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
				<div class="border rounded-lg p-6">
					<h3 class="text-lg font-medium mb-3">Account Security</h3>
					<ul class="space-y-2 text-sm">
						<li>• Use strong, unique passwords</li>
						<li>• Keep your account information up to date</li>
						<li>• Report suspicious activity immediately</li>
						<li>• Review team member permissions regularly</li>
					</ul>
				</div>

				<div class="border rounded-lg p-6">
					<h3 class="text-lg font-medium mb-3">Package Publishing</h3>
					<ul class="space-y-2 text-sm">
						<li>• Scan your code for vulnerabilities</li>
						<li>• Use secure development practices</li>
						<li>• Keep dependencies up to date</li>
						<li>• Review package contents before publishing</li>
					</ul>
				</div>
			</div>
		</section>

		<section class="mb-12">
			<h2 class="text-2xl font-semibold mb-4">Compliance and Certifications</h2>
			
			<div class="bg-green-50 border border-green-200 rounded-lg p-6">
				<h3 class="text-lg font-medium text-green-800 mb-3">Standards Compliance</h3>
				<p class="text-green-700 text-sm mb-4">
					We follow industry-standard security frameworks and best practices to ensure the highest level of protection.
				</p>
				<div class="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm">
					<div>
						<h4 class="font-medium mb-1">Security Standards</h4>
						<ul class="space-y-1">
							<li>• OWASP Guidelines</li>
							<li>• NIST Framework</li>
							<li>• ISO 27001 Principles</li>
						</ul>
					</div>
					<div>
						<h4 class="font-medium mb-1">Privacy Compliance</h4>
						<ul class="space-y-1">
							<li>• GDPR Compliance</li>
							<li>• CCPA Compliance</li>
							<li>• Privacy by Design</li>
						</ul>
					</div>
					<div>
						<h4 class="font-medium mb-1">Regular Audits</h4>
						<ul class="space-y-1">
							<li>• Security assessments</li>
							<li>• Penetration testing</li>
							<li>• Code reviews</li>
						</ul>
					</div>
				</div>
			</div>
		</section>

		<section class="mb-12">
			<h2 class="text-2xl font-semibold mb-4">Responsible Disclosure</h2>
			
			<div class="bg-yellow-50 border border-yellow-200 rounded-lg p-6">
				<h3 class="text-lg font-medium text-yellow-800 mb-3">Security Research</h3>
				<p class="text-yellow-700 text-sm mb-4">
					We welcome security researchers to help us improve our security posture through responsible disclosure.
				</p>
				
				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">What to Report</h4>
						<ul class="text-sm space-y-1">
							<li>• Security vulnerabilities in our services</li>
							<li>• Data exposure or unauthorized access</li>
							<li>• Authentication bypass issues</li>
							<li>• Any security-related concerns</li>
						</ul>
					</div>

					<div>
						<h4 class="font-medium mb-2">How to Report</h4>
						<ul class="text-sm space-y-1">
							<li>• Email: <a href="mailto:contact@klysium.com" class="text-blue-600 hover:text-blue-800">contact@klysium.com</a></li>
							<li>• Include detailed steps to reproduce</li>
							<li>• Provide impact assessment</li>
							<li>• Allow reasonable time for response</li>
						</ul>
					</div>
				</div>
			</div>
		</section>

		<section class="mb-12 bg-gray-50 rounded-lg p-6">
			<h2 class="text-2xl font-semibold mb-4">Contact Our Security Team</h2>
			
			<p class="mb-4">
				For security-related questions, concerns, or to report vulnerabilities:
			</p>
			
			<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
				<div>
					<h3 class="font-medium mb-2">Security Team</h3>
					<a href="mailto:contact@klysium.com" class="text-blue-600 hover:text-blue-800">contact@klysium.com</a>
					<p class="text-sm text-muted-foreground mt-1">For urgent security issues</p>
				</div>
				
				<div>
					<h3 class="font-medium mb-2">General Security Questions</h3>
					<a href="https://github.com/saravenpi/knot/issues" class="text-blue-600 hover:text-blue-800">GitHub Issues</a>
					<p class="text-sm text-muted-foreground mt-1">For non-sensitive security discussions</p>
				</div>
			</div>
		</section>
	</div>
</div>