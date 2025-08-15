/**
 * Utility functions for formatting data consistently across the application
 */

/**
 * Safely parse and format download counts
 * Handles both string and number types that may come from the API
 */
export function formatDownloadCount(count: number | string | undefined): string {
  if (count === undefined || count === null) {
    return '0';
  }
  
  // Handle string representation of numbers (including BigInt serialization)
  const numCount = typeof count === 'string' ? parseInt(count, 10) : count;
  
  // Handle invalid numbers
  if (isNaN(numCount) || numCount < 0) {
    return '0';
  }
  
  return numCount.toLocaleString();
}

/**
 * Get the raw download count as a number for calculations
 */
export function getDownloadCount(count: number | string | undefined): number {
  if (count === undefined || count === null) {
    return 0;
  }
  
  const numCount = typeof count === 'string' ? parseInt(count, 10) : count;
  
  // Handle invalid numbers
  if (isNaN(numCount) || numCount < 0) {
    return 0;
  }
  
  return numCount;
}

/**
 * Format file sizes from bytes to human readable format
 */
export function formatFileSize(bytes: number | string | undefined): string {
  const numBytes = parseInt(bytes?.toString() || '0');
  if (numBytes < 1024) return numBytes + ' B';
  if (numBytes < 1024 * 1024) return (numBytes / 1024).toFixed(1) + ' KB';
  return (numBytes / (1024 * 1024)).toFixed(1) + ' MB';
}

/**
 * Format dates consistently
 */
export function formatDate(dateString: string): string {
  return new Date(dateString).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric'
  });
}

/**
 * Format dates with time for detailed views
 */
export function formatDateTime(dateString: string): string {
  return new Date(dateString).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  });
}

/**
 * Format large numbers with appropriate suffixes (K, M, B)
 */
export function formatLargeNumber(num: number): string {
  if (num < 1000) return num.toString();
  if (num < 1000000) return (num / 1000).toFixed(1) + 'K';
  if (num < 1000000000) return (num / 1000000).toFixed(1) + 'M';
  return (num / 1000000000).toFixed(1) + 'B';
}