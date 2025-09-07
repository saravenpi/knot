// String utility functions

export const capitalize = (str: string): string => 
  str.charAt(0).toUpperCase() + str.slice(1);

export const camelCase = (str: string): string =>
  str.replace(/-([a-z])/g, (_, letter) => letter.toUpperCase());

export const kebabCase = (str: string): string =>
  str.replace(/[A-Z]/g, letter => `-${letter.toLowerCase()}`);

export const snakeCase = (str: string): string =>
  str.replace(/[A-Z]/g, letter => `_${letter.toLowerCase()}`);

export const truncate = (str: string, length: number, ellipsis = '...'): string =>
  str.length <= length ? str : str.slice(0, length) + ellipsis;

export const slugify = (str: string): string =>
  str
    .toLowerCase()
    .trim()
    .replace(/[^\w\s-]/g, '')
    .replace(/[\s_-]+/g, '-')
    .replace(/^-+|-+$/g, '');

export const stripHtml = (str: string): string =>
  str.replace(/<[^>]*>?/gm, '');

export const escapeHtml = (str: string): string =>
  str
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#039;');

export const unescapeHtml = (str: string): string =>
  str
    .replace(/&amp;/g, '&')
    .replace(/&lt;/g, '<')
    .replace(/&gt;/g, '>')
    .replace(/&quot;/g, '"')
    .replace(/&#039;/g, "'");