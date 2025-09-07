/**
 * Main entry point for the package
 * 
 * This template shows how to create packages that work with Knot's alias system.
 * When this package is linked to an app, it can be imported using the app's
 * configured alias (e.g., @ui/package-name, #/package-name, $/package-name).
 */

// Example: Import from other local packages using direct names
// In apps, these would be available via aliases like @ui/utils, #/utils, etc.
// import { someUtility } from 'utils';

export function hello(name: string): string {
    return `Hello, ${name}!`;
}

export function getPackageInfo(): { name: string; version: string; description: string } {
    return {
        name: 'typescript-template',
        version: '1.0.0',
        description: 'TypeScript package template with alias system support'
    };
}

// Export common utilities that might be used by other packages or apps
export const createId = (): string => Math.random().toString(36).substr(2, 9);

export const formatMessage = (message: string, timestamp?: Date): string => {
    const time = timestamp || new Date();
    return `[${time.toISOString()}] ${message}`;
};

// Example interface that could be shared across packages
export interface PackageConfig {
    name: string;
    version: string;
    dependencies?: string[];
    exports?: Record<string, string>;
}

// Example class demonstrating TypeScript features
export class PackageManager {
    private packages: Map<string, PackageConfig> = new Map();

    addPackage(config: PackageConfig): void {
        this.packages.set(config.name, config);
    }

    getPackage(name: string): PackageConfig | undefined {
        return this.packages.get(name);
    }

    listPackages(): string[] {
        return Array.from(this.packages.keys());
    }

    hasPackage(name: string): boolean {
        return this.packages.has(name);
    }
}

export default hello;