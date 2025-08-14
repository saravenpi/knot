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
		if (!canvas || !data.length) return;

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

		// Vertical grid lines
		for (let i = 0; i < data.length; i++) {
			const x = padding.left + (chartWidth / (data.length - 1)) * i;
			ctx.beginPath();
			ctx.moveTo(x, padding.top);
			ctx.lineTo(x, padding.top + chartHeight);
			ctx.stroke();
		}

		ctx.setLineDash([]);

		// Draw line
		ctx.strokeStyle = color;
		ctx.lineWidth = 2;
		ctx.beginPath();

		data.forEach((point, index) => {
			const x = padding.left + (chartWidth / (data.length - 1)) * index;
			const y = padding.top + chartHeight - ((point.downloads - minDownloads) / (maxDownloads - minDownloads)) * chartHeight;
			
			if (index === 0) {
				ctx.moveTo(x, y);
			} else {
				ctx.lineTo(x, y);
			}
		});

		ctx.stroke();

		// Draw points
		ctx.fillStyle = color;
		data.forEach((point, index) => {
			const x = padding.left + (chartWidth / (data.length - 1)) * index;
			const y = padding.top + chartHeight - ((point.downloads - minDownloads) / (maxDownloads - minDownloads)) * chartHeight;
			
			ctx.beginPath();
			ctx.arc(x, y, 4, 0, Math.PI * 2);
			ctx.fill();
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
				const x = padding.left + (chartWidth / (data.length - 1)) * index;
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
		if (!canvas || !data.length) return;

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
			const pointX = padding.left + (chartWidth / (data.length - 1)) * index;
			const maxDownloads = Math.max(...data.map(d => d.downloads), 1);
			const pointY = padding.top + chartHeight - ((point.downloads - 0) / (maxDownloads - 0)) * chartHeight;
			
			const distance = Math.sqrt((x - pointX) ** 2 + (y - pointY) ** 2);
			
			if (distance < 15 && distance < closestDistance) {
				closestDistance = distance;
				closestPoint = { x: pointX, y: pointY, data: point };
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
		{#if data.length > 0}
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