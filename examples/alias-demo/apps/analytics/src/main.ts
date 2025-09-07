// Analytics Dashboard using $ alias
// Using $ alias to import from knot_packages
import { formatDate, addDays, diffInDays } from '$/utils/dates';
import { average, median, standardDeviation } from '$/utils/math';
import { debounce, throttle } from '$/utils';
import { MemoryCache, globalCache } from '$/data-layer/cache';
import type { User, ApiResponse } from '$/data-layer';

interface AnalyticsData {
  date: string;
  users: number;
  sessions: number;
  pageViews: number;
  bounceRate: number;
}

interface MetricSummary {
  current: number;
  previous: number;
  change: number;
  changePercent: number;
}

class AnalyticsDashboard {
  private cache: MemoryCache<any>;
  private container: HTMLElement;

  constructor(containerId: string) {
    this.cache = new MemoryCache({
      ttl: 600000, // 10 minutes
      keyPrefix: 'analytics_',
    });
    
    this.container = document.getElementById(containerId)!;
    this.init();
  }

  private init(): void {
    this.render();
    this.loadData();
    this.setupEventListeners();
  }

  private render(): void {
    this.container.innerHTML = `
      <div class="analytics-dashboard">
        <header class="dashboard-header">
          <h1>Analytics Dashboard</h1>
          <div class="alias-info">
            <strong>Alias Configuration:</strong> 
            This app uses <code>tsAlias: "$"</code> for ultra-short imports 
            (e.g., $/utils, $/data-layer).
          </div>
        </header>

        <div class="metrics-grid">
          <div class="metric-card" id="users-metric">
            <div class="metric-title">Active Users</div>
            <div class="metric-value" id="users-value">Loading...</div>
            <div class="metric-change" id="users-change"></div>
          </div>

          <div class="metric-card" id="sessions-metric">
            <div class="metric-title">Sessions</div>
            <div class="metric-value" id="sessions-value">Loading...</div>
            <div class="metric-change" id="sessions-change"></div>
          </div>

          <div class="metric-card" id="pageviews-metric">
            <div class="metric-title">Page Views</div>
            <div class="metric-value" id="pageviews-value">Loading...</div>
            <div class="metric-change" id="pageviews-change"></div>
          </div>

          <div class="metric-card" id="bounce-metric">
            <div class="metric-title">Bounce Rate</div>
            <div class="metric-value" id="bounce-value">Loading...</div>
            <div class="metric-change" id="bounce-change"></div>
          </div>
        </div>

        <div class="charts-section">
          <div class="chart-container">
            <h3>User Trend (Last 30 Days)</h3>
            <div id="user-trend-chart" class="chart-placeholder">
              Chart would render here with $/data from alias imports
            </div>
          </div>

          <div class="stats-container">
            <h3>Statistical Summary</h3>
            <div id="stats-summary"></div>
          </div>
        </div>

        <div class="demo-section">
          <h3>💡 Alias Usage Examples in this App:</h3>
          <ul class="demo-list">
            <li><code>$/utils/dates</code> - Date formatting and calculations</li>
            <li><code>$/utils/math</code> - Statistical calculations (average, median, std dev)</li>
            <li><code>$/utils</code> - Debounce and throttle utilities</li>
            <li><code>$/data-layer/cache</code> - Caching analytics data</li>
            <li><code>$/data-layer</code> - Type definitions and interfaces</li>
          </ul>
        </div>
      </div>
    `;
  }

  private generateMockData(): AnalyticsData[] {
    const data: AnalyticsData[] = [];
    const today = new Date();

    for (let i = 30; i >= 0; i--) {
      const date = addDays(today, -i);
      data.push({
        date: formatDate(date, 'YYYY-MM-DD'),
        users: Math.floor(Math.random() * 1000) + 500,
        sessions: Math.floor(Math.random() * 1500) + 800,
        pageViews: Math.floor(Math.random() * 5000) + 2000,
        bounceRate: Math.random() * 0.4 + 0.3, // 30-70%
      });
    }

    return data;
  }

  private calculateMetricSummary(
    current: number[], 
    previous: number[]
  ): MetricSummary {
    const currentAvg = average(current);
    const previousAvg = average(previous);
    const change = currentAvg - previousAvg;
    const changePercent = previousAvg > 0 ? (change / previousAvg) * 100 : 0;

    return {
      current: Math.round(currentAvg),
      previous: Math.round(previousAvg),
      change: Math.round(change),
      changePercent: Math.round(changePercent * 100) / 100,
    };
  }

  private loadData(): void {
    const cacheKey = 'analytics_data_30d';
    let data = this.cache.get(cacheKey);

    if (!data) {
      console.log('📊 Generating fresh analytics data...');
      data = this.generateMockData();
      this.cache.set(cacheKey, data);
    } else {
      console.log('📦 Using cached analytics data');
    }

    this.updateMetrics(data);
    this.updateChart(data);
    this.updateStats(data);
  }

  private updateMetrics(data: AnalyticsData[]): void {
    const last7Days = data.slice(-7);
    const previous7Days = data.slice(-14, -7);

    const metrics = {
      users: this.calculateMetricSummary(
        last7Days.map(d => d.users),
        previous7Days.map(d => d.users)
      ),
      sessions: this.calculateMetricSummary(
        last7Days.map(d => d.sessions),
        previous7Days.map(d => d.sessions)
      ),
      pageViews: this.calculateMetricSummary(
        last7Days.map(d => d.pageViews),
        previous7Days.map(d => d.pageViews)
      ),
      bounceRate: this.calculateMetricSummary(
        last7Days.map(d => d.bounceRate * 100),
        previous7Days.map(d => d.bounceRate * 100)
      ),
    };

    // Update UI
    this.updateMetricCard('users', metrics.users);
    this.updateMetricCard('sessions', metrics.sessions);
    this.updateMetricCard('pageviews', metrics.pageViews);
    this.updateMetricCard('bounce', metrics.bounceRate, '%');
  }

  private updateMetricCard(
    metricId: string, 
    summary: MetricSummary, 
    suffix: string = ''
  ): void {
    const valueEl = document.getElementById(`${metricId}-value`);
    const changeEl = document.getElementById(`${metricId}-change`);

    if (valueEl && changeEl) {
      valueEl.textContent = `${summary.current.toLocaleString()}${suffix}`;
      
      const isPositive = summary.changePercent >= 0;
      const arrow = isPositive ? '↗' : '↘';
      const color = metricId === 'bounce' 
        ? (isPositive ? 'negative' : 'positive') // Bounce rate: lower is better
        : (isPositive ? 'positive' : 'negative');

      changeEl.innerHTML = `
        <span class="change-${color}">
          ${arrow} ${Math.abs(summary.changePercent)}% vs last week
        </span>
      `;
    }
  }

  private updateChart(data: AnalyticsData[]): void {
    const chartEl = document.getElementById('user-trend-chart');
    if (!chartEl) return;

    // Simple ASCII chart using the data
    const userCounts = data.map(d => d.users);
    const max = Math.max(...userCounts);
    const min = Math.min(...userCounts);

    chartEl.innerHTML = `
      <div class="ascii-chart">
        <div class="chart-stats">
          Max: ${max.toLocaleString()} | Min: ${min.toLocaleString()} | 
          Avg: ${average(userCounts).toFixed(0)}
        </div>
        <div class="chart-bars">
          ${data.slice(-14).map((d, i) => {
            const height = ((d.users - min) / (max - min)) * 100;
            return `
              <div class="chart-bar" style="height: ${height}%">
                <div class="bar-value">${d.users}</div>
                <div class="bar-date">${d.date.slice(-5)}</div>
              </div>
            `;
          }).join('')}
        </div>
      </div>
    `;
  }

  private updateStats(data: AnalyticsData[]): void {
    const statsEl = document.getElementById('stats-summary');
    if (!statsEl) return;

    const userCounts = data.map(d => d.users);
    const sessionCounts = data.map(d => d.sessions);
    const bounceRates = data.map(d => d.bounceRate * 100);

    const stats = {
      users: {
        avg: average(userCounts),
        median: median(userCounts),
        stdDev: standardDeviation(userCounts),
      },
      sessions: {
        avg: average(sessionCounts),
        median: median(sessionCounts),
        stdDev: standardDeviation(sessionCounts),
      },
      bounceRate: {
        avg: average(bounceRates),
        median: median(bounceRates),
        stdDev: standardDeviation(bounceRates),
      },
    };

    statsEl.innerHTML = `
      <div class="stats-grid">
        <div class="stat-group">
          <h4>Users</h4>
          <p>Average: ${Math.round(stats.users.avg).toLocaleString()}</p>
          <p>Median: ${Math.round(stats.users.median).toLocaleString()}</p>
          <p>Std Dev: ${Math.round(stats.users.stdDev).toLocaleString()}</p>
        </div>
        <div class="stat-group">
          <h4>Sessions</h4>
          <p>Average: ${Math.round(stats.sessions.avg).toLocaleString()}</p>
          <p>Median: ${Math.round(stats.sessions.median).toLocaleString()}</p>
          <p>Std Dev: ${Math.round(stats.sessions.stdDev).toLocaleString()}</p>
        </div>
        <div class="stat-group">
          <h4>Bounce Rate</h4>
          <p>Average: ${stats.bounceRate.avg.toFixed(1)}%</p>
          <p>Median: ${stats.bounceRate.median.toFixed(1)}%</p>
          <p>Std Dev: ${stats.bounceRate.stdDev.toFixed(1)}%</p>
        </div>
      </div>
    `;
  }

  private setupEventListeners(): void {
    // Throttled refresh function using utility from alias
    const throttledRefresh = throttle(() => {
      console.log('🔄 Refreshing analytics data...');
      this.cache.clear();
      this.loadData();
    }, 2000);

    // Add refresh button functionality
    const refreshButton = document.createElement('button');
    refreshButton.textContent = '🔄 Refresh Data';
    refreshButton.className = 'refresh-button';
    refreshButton.addEventListener('click', throttledRefresh);
    
    const header = document.querySelector('.dashboard-header');
    header?.appendChild(refreshButton);
  }
}

// Initialize the dashboard when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
  new AnalyticsDashboard('app');
  
  console.log('📈 Analytics Dashboard initialized');
  console.log(`💾 Cache system ready`);
  console.log(`🕒 Current time: ${formatDate(new Date(), 'YYYY-MM-DD HH:mm:ss')}`);
});