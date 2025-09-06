<script lang="ts">
	import { onMount } from 'svelte';

	export let data: Array<{ date: string; downloads: number }> = [];
	export let title = '';
	export let color = '#8b5cf6';
	export let height = 300;

	let canvas: HTMLCanvasElement;
	let tooltip: HTMLDivElement;
	let hoveredPoint: { x: number; y: number; data: any } | null = null;

	function formatDate(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleDateString('en-US', { 
			month: 'short', 
			day: 'numeric' 
		});
	}

	function formatTooltipDate(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleDateString('en-US', { 
			weekday: 'short',
			month: 'short', 
			day: 'numeric',
			year: 'numeric'
		});
	}

	function drawChart() {
		if (!canvas || !data || !Array.isArray(data) || data.length === 0) return;

		const ctx = canvas.getContext('2d');
		if (!ctx) return;

		const dpr = window.devicePixelRatio || 1;
		const rect = canvas.getBoundingClientRect();
		
		canvas.width = rect.width * dpr;
		canvas.height = rect.height * dpr;
		ctx.scale(dpr, dpr);

		const padding = { top: 20, right: 30, bottom: 50, left: 50 };
		const chartWidth = rect.width - padding.left - padding.right;
		const chartHeight = rect.height - padding.top - padding.bottom;

		// Clear canvas
		ctx.fillStyle = 'transparent';
		ctx.fillRect(0, 0, rect.width, rect.height);

		// Get data bounds
		const maxDownloads = Math.max(...data.map(d => d.downloads), 1);
		const minDownloads = 0;

		// Draw grid lines
		ctx.strokeStyle = '#e2e8f0';
		ctx.lineWidth = 1;
		ctx.setLineDash([3, 3]);

		// Horizontal grid lines
		for (let i = 0; i <= 5; i++) {
			const y = padding.top + (chartHeight / 5) * i;
			ctx.beginPath();
			ctx.moveTo(padding.left, y);
			ctx.lineTo(padding.left + chartWidth, y);
			ctx.stroke();
		}

		// Vertical grid lines (at bar centers)
		for (let i = 0; i < data.length; i++) {
			const x = padding.left + (chartWidth / data.length) * i + (chartWidth / data.length) / 2;
			ctx.beginPath();
			ctx.moveTo(x, padding.top);
			ctx.lineTo(x, padding.top + chartHeight);
			ctx.stroke();
		}

		ctx.setLineDash([]);

		// Draw bars
		ctx.fillStyle = color;
		const barWidth = chartWidth / data.length * 0.7; // 70% of available space for bars
		const barSpacing = chartWidth / data.length * 0.3; // 30% for spacing

		data.forEach((point, index) => {
			const x = padding.left + (chartWidth / data.length) * index + barSpacing / 2;
			const barHeight = ((point.downloads - minDownloads) / (maxDownloads - minDownloads)) * chartHeight;
			const y = padding.top + chartHeight - barHeight;
			
			// Draw bar with rounded top corners
			ctx.beginPath();
			const radius = Math.min(4, barWidth / 4);
			
			// Fallback for browsers that don't support roundRect
			if (typeof ctx.roundRect === 'function') {
				ctx.roundRect(x, y, barWidth, barHeight, [radius, radius, 0, 0]);
			} else {
				// Manual rounded rectangle for top corners only
				ctx.moveTo(x + radius, y);
				ctx.lineTo(x + barWidth - radius, y);
				ctx.arcTo(x + barWidth, y, x + barWidth, y + radius, radius);
				ctx.lineTo(x + barWidth, y + barHeight);
				ctx.lineTo(x, y + barHeight);
				ctx.lineTo(x, y + radius);
				ctx.arcTo(x, y, x + radius, y, radius);
				ctx.closePath();
			}
			ctx.fill();
			
			// Add subtle border
			ctx.strokeStyle = 'rgba(0, 0, 0, 0.1)';
			ctx.lineWidth = 1;
			ctx.stroke();
		});

		// Draw axes
		ctx.strokeStyle = '#64748b';
		ctx.lineWidth = 1;
		
		// X-axis
		ctx.beginPath();
		ctx.moveTo(padding.left, padding.top + chartHeight);
		ctx.lineTo(padding.left + chartWidth, padding.top + chartHeight);
		ctx.stroke();

		// Y-axis
		ctx.beginPath();
		ctx.moveTo(padding.left, padding.top);
		ctx.lineTo(padding.left, padding.top + chartHeight);
		ctx.stroke();

		// Draw labels
		ctx.fillStyle = '#64748b';
		ctx.font = '12px system-ui';
		ctx.textAlign = 'center';

		// X-axis labels
		data.forEach((point, index) => {
			if (index % Math.ceil(data.length / 6) === 0) {
				const x = padding.left + (chartWidth / data.length) * index + (chartWidth / data.length) / 2;
				ctx.fillText(formatDate(point.date), x, padding.top + chartHeight + 20);
			}
		});

		// Y-axis labels
		ctx.textAlign = 'right';
		for (let i = 0; i <= 5; i++) {
			const value = Math.round((maxDownloads / 5) * (5 - i));
			const y = padding.top + (chartHeight / 5) * i + 4;
			ctx.fillText(value.toString(), padding.left - 10, y);
		}
	}

	function handleMouseMove(event: MouseEvent) {
		if (!canvas || !data || !Array.isArray(data) || data.length === 0) return;

		const rect = canvas.getBoundingClientRect();
		const x = event.clientX - rect.left;
		const y = event.clientY - rect.top;

		const padding = { top: 20, right: 30, bottom: 50, left: 50 };
		const chartWidth = rect.width - padding.left - padding.right;
		const chartHeight = rect.height - padding.top - padding.bottom;

		// Check if mouse is over a data point
		let closestPoint = null;
		let closestDistance = Infinity;

		data.forEach((point, index) => {
			const barWidth = chartWidth / data.length * 0.7;
			const barSpacing = chartWidth / data.length * 0.3;
			const barX = padding.left + (chartWidth / data.length) * index + barSpacing / 2;
			const maxDownloads = Math.max(...data.map(d => d.downloads), 1);
			const barHeight = ((point.downloads - 0) / (maxDownloads - 0)) * chartHeight;
			const barY = padding.top + chartHeight - barHeight;
			
			// Check if mouse is over this bar
			if (x >= barX && x <= barX + barWidth && y >= barY && y <= padding.top + chartHeight) {
				const centerX = barX + barWidth / 2;
				const distance = Math.abs(x - centerX);
				
				if (distance < closestDistance) {
					closestDistance = distance;
					closestPoint = { x: centerX, y: barY, data: point };
				}
			}
		});

		hoveredPoint = closestPoint;
	}

	function handleMouseLeave() {
		hoveredPoint = null;
	}

	onMount(() => {
		const resizeObserver = new ResizeObserver(() => {
			drawChart();
		});

		if (canvas) {
			resizeObserver.observe(canvas);
		}

		return () => {
			resizeObserver.disconnect();
		};
	});

	$: if (canvas && data) {
		drawChart();
	}
</script>

<div class="space-y-4">
	{#if title}
		<div class="text-center">
			<h3 class="text-lg font-medium leading-none tracking-tight">{title}</h3>
		</div>
	{/if}
	
	<div class="relative w-full" style="height: {height}px;">
		{#if data && Array.isArray(data) && data.length > 0}
			<canvas
				bind:this={canvas}
				class="w-full h-full cursor-crosshair"
				on:mousemove={handleMouseMove}
				on:mouseleave={handleMouseLeave}
			></canvas>
			
			{#if hoveredPoint}
				<div
					bind:this={tooltip}
					class="absolute pointer-events-none bg-background border rounded-lg p-2 shadow-md text-sm z-10"
					style="left: {hoveredPoint.x + 10}px; top: {hoveredPoint.y - 50}px;"
				>
					<div class="font-medium">{formatTooltipDate(hoveredPoint.data.date)}</div>
					<div class="text-muted-foreground">
						Downloads: {hoveredPoint.data.downloads}
					</div>
				</div>
			{/if}
		{:else}
			<div class="flex items-center justify-center h-full text-muted-foreground">
				<div class="text-center space-y-2">
					<div class="text-lg">📊</div>
					<p class="text-sm">No download data available</p>
				</div>
			</div>
		{/if}
	</div>
</div>