<script lang="ts">
	import Icon from '@iconify/svelte';
</script>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight mb-4" style="font-family: 'Gambarino', 'Satoshi', sans-serif;">
			Troubleshooting
		</h1>
		<p class="text-lg text-muted-foreground">
			Common issues and solutions for Knot CLI and monorepo management
		</p>
	</div>

	<!-- Installation Issues -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Installation Issues</h2>

		<div class="space-y-6">
			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="solar:danger-triangle-bold" class="w-5 h-5 mr-2 text-red-600" />
					Command not found
				</h3>
				<div class="space-y-3 text-sm">
					<p class="text-muted-foreground">If you get "command not found" after installation:</p>
					<ul class="space-y-1 ml-4 text-muted-foreground">
						<li>• Check if <code>/usr/local/bin</code> is in your PATH</li>
						<li>• Try restarting your terminal</li>
						<li>• On macOS: <code>sudo chown -R $(whoami) /usr/local/bin</code></li>
						<li>• Manually add to PATH: <code>export PATH=$PATH:/usr/local/bin</code></li>
					</ul>
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="solar:lock-bold" class="w-5 h-5 mr-2 text-yellow-600" />
					Permission denied
				</h3>
				<div class="space-y-3 text-sm">
					<p class="text-muted-foreground">If you get permission errors:</p>
					<ul class="space-y-1 ml-4 text-muted-foreground">
						<li>• Make sure the binary is executable: <code>chmod +x knot</code></li>
						<li>• On macOS, allow the app in System Preferences > Security</li>
						<li>• Use <code>sudo</code> for system-wide installation</li>
					</ul>
				</div>
			</div>
		</div>
	</section>

	<!-- Linking Issues -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Package Linking Issues</h2>

		<div class="space-y-6">
			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="solar:close-circle-bold" class="w-5 h-5 mr-2 text-red-600" />
					Package not found
				</h3>
				<div class="space-y-3 text-sm">
					<p class="text-muted-foreground">When packages can't be found:</p>
					<ul class="space-y-1 ml-4 text-muted-foreground">
						<li>• Check package name spelling in configuration</li>
						<li>• Verify package exists in <code>packages/</code> directory</li>
						<li>• For remote packages, check authentication</li>
						<li>• Run <code>knot status</code> to see available packages</li>
					</ul>
					<div class="bg-black/90 text-green-400 font-mono text-xs p-3 rounded mt-3">
						<code>knot link --verbose</code>
					</div>
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="solar:code-bold" class="w-5 h-5 mr-2 text-orange-600" />
					TypeScript import errors
				</h3>
				<div class="space-y-3 text-sm">
					<p class="text-muted-foreground">When imports don't work:</p>
					<ul class="space-y-1 ml-4 text-muted-foreground">
						<li>• Verify tsAlias configuration in app.yml or knot.yml</li>
						<li>• Check if tsconfig.json was updated correctly</li>
						<li>• Run <code>knot link --force</code> to refresh all links</li>
						<li>• Restart your IDE/TypeScript service</li>
					</ul>
					<div class="bg-black/90 text-green-400 font-mono text-xs p-3 rounded mt-3">
						<code>knot link --force</code>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Authentication Issues -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Authentication Issues</h2>

		<div class="space-y-6">
			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="solar:shield-warning-bold" class="w-5 h-5 mr-2 text-red-600" />
					Authentication failed
				</h3>
				<div class="space-y-3 text-sm">
					<p class="text-muted-foreground">When authentication fails:</p>
					<ul class="space-y-1 ml-4 text-muted-foreground">
						<li>• Check that your token is correct and not expired</li>
						<li>• Ensure there are no extra spaces in the token</li>
						<li>• Verify the environment variable is set correctly</li>
						<li>• Test with <code>knot auth</code> command</li>
					</ul>
					<div class="bg-black/90 text-green-400 font-mono text-xs p-3 rounded mt-3">
						<code>knot auth</code>
					</div>
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="solar:wifi-router-bold" class="w-5 h-5 mr-2 text-orange-600" />
					Connection failed
				</h3>
				<div class="space-y-3 text-sm">
					<p class="text-muted-foreground">When can't connect to registry:</p>
					<ul class="space-y-1 ml-4 text-muted-foreground">
						<li>• Check your internet connection</li>
						<li>• Verify the KNOT_SPACE_URL is correct</li>
						<li>• Check if you're behind a corporate firewall</li>
						<li>• Try with <code>--verbose</code> flag for more details</li>
					</ul>
				</div>
			</div>
		</div>
	</section>

	<!-- Build Issues -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Build Issues</h2>

		<div class="space-y-6">
			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="solar:bug-bold" class="w-5 h-5 mr-2 text-purple-600" />
					Build failures
				</h3>
				<div class="space-y-3 text-sm">
					<p class="text-muted-foreground">When builds fail:</p>
					<ul class="space-y-1 ml-4 text-muted-foreground">
						<li>• Check build script configuration in package.yml or app.yml</li>
						<li>• Verify all dependencies are installed</li>
						<li>• Run build command manually to see detailed error</li>
						<li>• Check for missing TypeScript types</li>
					</ul>
					<div class="bg-black/90 text-green-400 font-mono text-xs p-3 rounded mt-3">
						<code>knot run build --verbose</code>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Publishing Issues -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Publishing Issues</h2>

		<div class="space-y-6">
			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="solar:upload-bold" class="w-5 h-5 mr-2 text-red-600" />
					Publish failed
				</h3>
				<div class="space-y-3 text-sm">
					<p class="text-muted-foreground">When publishing fails:</p>
					<ul class="space-y-1 ml-4 text-muted-foreground">
						<li>• Check if you have publish permissions for the package</li>
						<li>• Verify version number follows semantic versioning</li>
						<li>• Ensure package name is unique (or team namespaced)</li>
						<li>• Check if version already exists</li>
						<li>• Use <code>--dry-run</code> to test before publishing</li>
					</ul>
					<div class="bg-black/90 text-green-400 font-mono text-xs p-3 rounded mt-3">
						<code>knot publish --dry-run</code>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Debug Commands -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Debug Commands</h2>

		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="solar:adhesive-plaster-bold-duotone" class="w-5 h-5 mr-2 text-blue-600" />
					Diagnostic Commands
				</h3>
				<div class="space-y-3 text-sm">
					<div>
						<div class="font-medium"><code>knot status</code></div>
						<div class="text-muted-foreground">Show project and linking status</div>
					</div>
					<div>
						<div class="font-medium"><code>knot info</code></div>
						<div class="text-muted-foreground">Display version and system info</div>
					</div>
					<div>
						<div class="font-medium"><code>knot --version</code></div>
						<div class="text-muted-foreground">Show CLI version</div>
					</div>
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="solar:document-text-bold" class="w-5 h-5 mr-2 text-green-600" />
					Verbose Logging
				</h3>
				<div class="space-y-3 text-sm">
					<div>
						<div class="font-medium"><code>knot link --verbose</code></div>
						<div class="text-muted-foreground">Detailed linking output</div>
					</div>
					<div>
						<div class="font-medium"><code>knot publish --verbose</code></div>
						<div class="text-muted-foreground">Detailed publishing output</div>
					</div>
					<div>
						<div class="font-medium"><code>knot build --verbose</code></div>
						<div class="text-muted-foreground">Detailed build output</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Getting Help -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Getting Help</h2>

		<div class="space-y-6">
			<div class="bg-blue-50 border border-blue-200 rounded-lg p-6">
				<div class="flex items-start space-x-3">
					<Icon icon="solar:question-circle-bold" class="w-6 h-6 text-blue-600 mt-1 flex-shrink-0" />
					<div>
						<h3 class="font-semibold text-blue-900 mb-2">Still Need Help?</h3>
						<p class="text-sm text-blue-700 mb-4">
							If you're still experiencing issues, here are additional resources:
						</p>
						<ul class="text-sm text-blue-700 space-y-1">
							<li>• Check the <a href="https://github.com/saravenpi/knot/issues" class="underline">GitHub issues</a></li>
							<li>• Join our community discussions</li>
							<li>• Review the complete documentation</li>
							<li>• Contact support team</li>
						</ul>
					</div>
				</div>
			</div>
		</div>
	</section>
</div>
