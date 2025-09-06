/**
 * Debug utilities to verify data consistency and formatting
 * These functions can be used in development to ensure download counts are accurate
 */

import type { Package } from "#/types";

/**
 * Validate package data structure and download counts
 */
export function validatePackageData(pkg: Package): {
  isValid: boolean;
  issues: string[];
} {
  const issues: string[] = [];

  // Check if download count exists and is valid
  if (pkg.downloadsCount === undefined || pkg.downloadsCount === null) {
    issues.push("Download count is missing");
  } else {
    const downloadCount =
      typeof pkg.downloadsCount === "string"
        ? parseInt(pkg.downloadsCount, 10)
        : pkg.downloadsCount;

    if (isNaN(downloadCount) || downloadCount < 0) {
      issues.push(`Invalid download count: ${pkg.downloadsCount}`);
    }
  }

  // Check if file size exists and is valid
  if (pkg.fileSize === undefined || pkg.fileSize === null) {
    issues.push("File size is missing");
  } else {
    const fileSize =
      typeof pkg.fileSize === "string"
        ? parseInt(pkg.fileSize, 10)
        : pkg.fileSize;

    if (isNaN(fileSize) || fileSize < 0) {
      issues.push(`Invalid file size: ${pkg.fileSize}`);
    }
  }

  // Check essential fields
  if (!pkg.name || pkg.name.trim() === "") {
    issues.push("Package name is missing or empty");
  }

  if (!pkg.version || pkg.version.trim() === "") {
    issues.push("Package version is missing or empty");
  }

  if (!pkg.owner || !pkg.owner.username) {
    issues.push("Package owner information is missing");
  }

  return {
    isValid: issues.length === 0,
    issues,
  };
}

/**
 * Log package validation issues in development
 */
export function debugPackageData(packages: Package[]): void {
  if (process.env.NODE_ENV !== "development") return;

  console.group("🔍 Package Data Validation");

  const totalPackages = packages.length;
  let invalidCount = 0;

  packages.forEach((pkg, index) => {
    const validation = validatePackageData(pkg);

    if (!validation.isValid) {
      invalidCount++;
      console.warn(`Package ${index + 1} (${pkg.name}):`, validation.issues);
    }
  });

  if (invalidCount === 0) {
    console.log(`✅ All ${totalPackages} packages have valid data`);
  } else {
    console.warn(
      `⚠️ ${invalidCount}/${totalPackages} packages have data issues`,
    );
  }

  // Summary stats
  const totalDownloads = packages.reduce((sum, pkg) => {
    const count =
      typeof pkg.downloadsCount === "string"
        ? parseInt(pkg.downloadsCount, 10)
        : pkg.downloadsCount || 0;
    return sum + (isNaN(count) ? 0 : count);
  }, 0);

  console.log(
    `📊 Total downloads across all packages: ${totalDownloads.toLocaleString()}`,
  );

  console.groupEnd();
}

/**
 * Check if API response data is properly formatted
 */
export function validateApiResponse(response: any, endpoint: string): void {
  if (process.env.NODE_ENV !== "development") return;

  console.group(`🌐 API Response Validation: ${endpoint}`);

  if (!response) {
    console.error("Response is null or undefined");
    console.groupEnd();
    return;
  }

  if (response.success === false) {
    console.warn(
      "API returned success: false",
      response.error || response.message,
    );
  }

  if (response.data && Array.isArray(response.data)) {
    console.log(`Response contains ${response.data.length} items`);

    // Check first item structure if it's a package array
    const firstItem = response.data[0];
    if (firstItem && firstItem.downloadsCount !== undefined) {
      const validation = validatePackageData(firstItem);
      if (!validation.isValid) {
        console.warn("First item has validation issues:", validation.issues);
      }
    }
  }

  console.groupEnd();
}
