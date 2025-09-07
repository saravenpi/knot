/**
 * Dashboard package demonstrating complex inter-package dependency chains
 * Depends on: utils → data → ui-components → api-client
 */
export { default as Dashboard } from './Dashboard/Dashboard';
export { default as UserDashboard } from './UserDashboard/UserDashboard';
export { default as PostDashboard } from './PostDashboard/PostDashboard';
export { default as AnalyticsDashboard } from './AnalyticsDashboard/AnalyticsDashboard';
export { default as KPICard } from './KPICard/KPICard';
export { default as ChartWidget } from './ChartWidget/ChartWidget';
export { default as DataTable } from './DataTable/DataTable';

// Export types
export type { DashboardProps, DashboardConfig } from './Dashboard/Dashboard';
export type { UserDashboardProps } from './UserDashboard/UserDashboard';
export type { PostDashboardProps } from './PostDashboard/PostDashboard';
export type { AnalyticsDashboardProps } from './AnalyticsDashboard/AnalyticsDashboard';
export type { KPICardProps, KPIData } from './KPICard/KPICard';
export type { ChartWidgetProps, ChartData } from './ChartWidget/ChartWidget';