// Math utility functions

export const clamp = (value: number, min: number, max: number): number => 
  Math.min(Math.max(value, min), max);

export const lerp = (start: number, end: number, t: number): number => 
  start + (end - start) * t;

export const mapRange = (
  value: number, 
  inMin: number, 
  inMax: number, 
  outMin: number, 
  outMax: number
): number => {
  return ((value - inMin) * (outMax - outMin)) / (inMax - inMin) + outMin;
};

export const randomInt = (min: number, max: number): number =>
  Math.floor(Math.random() * (max - min + 1)) + min;

export const randomFloat = (min: number, max: number): number =>
  Math.random() * (max - min) + min;

export const roundToDecimal = (value: number, decimals: number): number =>
  Math.round(value * Math.pow(10, decimals)) / Math.pow(10, decimals);

export const average = (numbers: number[]): number =>
  numbers.reduce((sum, num) => sum + num, 0) / numbers.length;

export const median = (numbers: number[]): number => {
  const sorted = [...numbers].sort((a, b) => a - b);
  const mid = Math.floor(sorted.length / 2);
  return sorted.length % 2 === 0 
    ? (sorted[mid - 1] + sorted[mid]) / 2 
    : sorted[mid];
};

export const mode = (numbers: number[]): number[] => {
  const counts = new Map<number, number>();
  let maxCount = 0;
  
  for (const num of numbers) {
    const count = (counts.get(num) || 0) + 1;
    counts.set(num, count);
    maxCount = Math.max(maxCount, count);
  }
  
  return Array.from(counts.entries())
    .filter(([_, count]) => count === maxCount)
    .map(([num]) => num);
};

export const standardDeviation = (numbers: number[]): number => {
  const avg = average(numbers);
  const squaredDiffs = numbers.map(num => Math.pow(num - avg, 2));
  return Math.sqrt(average(squaredDiffs));
};

export const gcd = (a: number, b: number): number => {
  a = Math.abs(a);
  b = Math.abs(b);
  while (b !== 0) {
    [a, b] = [b, a % b];
  }
  return a;
};

export const lcm = (a: number, b: number): number => {
  return Math.abs(a * b) / gcd(a, b);
};

export const isPrime = (n: number): boolean => {
  if (n <= 1) return false;
  if (n <= 3) return true;
  if (n % 2 === 0 || n % 3 === 0) return false;
  
  for (let i = 5; i * i <= n; i += 6) {
    if (n % i === 0 || n % (i + 2) === 0) return false;
  }
  
  return true;
};