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

/**
 * Format time ago in a human-readable way
 */
export function formatTimeAgo(dateString: string): string {
  const now = new Date();
  const date = new Date(dateString);
  const diffInMs = now.getTime() - date.getTime();
  const diffInMinutes = Math.floor(diffInMs / (1000 * 60));
  const diffInHours = Math.floor(diffInMs / (1000 * 60 * 60));
  const diffInDays = Math.floor(diffInMs / (1000 * 60 * 60 * 24));
  const diffInMonths = Math.floor(diffInDays / 30);
  const diffInYears = Math.floor(diffInDays / 365);

  if (diffInMinutes < 1) return 'just now';
  if (diffInMinutes < 60) return `${diffInMinutes} minute${diffInMinutes === 1 ? '' : 's'} ago`;
  if (diffInHours < 24) return `${diffInHours} hour${diffInHours === 1 ? '' : 's'} ago`;
  if (diffInDays < 30) return `${diffInDays} day${diffInDays === 1 ? '' : 's'} ago`;
  if (diffInMonths < 12) return `${diffInMonths} month${diffInMonths === 1 ? '' : 's'} ago`;
  return `${diffInYears} year${diffInYears === 1 ? '' : 's'} ago`;
}