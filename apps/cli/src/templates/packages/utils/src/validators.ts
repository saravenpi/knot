/**
 * Common validation utilities
 */
import { ValidationError } from './types';

export function isEmail(email: string): boolean {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return emailRegex.test(email);
}

export function isUrl(url: string): boolean {
  try {
    new URL(url);
    return true;
  } catch {
    return false;
  }
}

export function isPhoneNumber(phone: string): boolean {
  const phoneRegex = /^\+?[\d\s\-\(\)]+$/;
  return phoneRegex.test(phone) && phone.replace(/\D/g, '').length >= 10;
}

export function isStrongPassword(password: string): boolean {
  // At least 8 characters, 1 uppercase, 1 lowercase, 1 number, 1 special character
  const strongPasswordRegex = /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$/;
  return strongPasswordRegex.test(password);
}

export function validateRequired(value: any, field: string): ValidationError | null {
  if (value === null || value === undefined || value === '') {
    return { field, message: `${field} is required` };
  }
  return null;
}

export function validateEmail(email: string, field = 'email'): ValidationError | null {
  if (!isEmail(email)) {
    return { field, message: 'Invalid email format', value: email };
  }
  return null;
}

export function validateMinLength(value: string, minLength: number, field: string): ValidationError | null {
  if (value.length < minLength) {
    return { field, message: `${field} must be at least ${minLength} characters`, value };
  }
  return null;
}

export function validateMaxLength(value: string, maxLength: number, field: string): ValidationError | null {
  if (value.length > maxLength) {
    return { field, message: `${field} must be no more than ${maxLength} characters`, value };
  }
  return null;
}

export function validateRange(value: number, min: number, max: number, field: string): ValidationError | null {
  if (value < min || value > max) {
    return { field, message: `${field} must be between ${min} and ${max}`, value };
  }
  return null;
}

const validators = {
  isEmail,
  isUrl,
  isPhoneNumber,
  isStrongPassword,
  validateRequired,
  validateEmail,
  validateMinLength,
  validateMaxLength,
  validateRange,
};

export default validators;