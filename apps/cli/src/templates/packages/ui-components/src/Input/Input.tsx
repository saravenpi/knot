/**
 * Input component demonstrating usage of utils package for validation
 */
import React, { useState, useCallback } from 'react';
import clsx from 'clsx';
// Import from utils package - demonstrating inter-package dependency
import { validators, generateId, ValidationError } from 'utils';

export interface InputProps {
  label?: string;
  value: string;
  onChange: (value: string) => void;
  type?: 'text' | 'email' | 'password' | 'number' | 'tel' | 'url';
  placeholder?: string;
  disabled?: boolean;
  required?: boolean;
  validation?: 'email' | 'phone' | 'url' | 'strongPassword';
  minLength?: number;
  maxLength?: number;
  error?: string;
  helperText?: string;
  id?: string;
  className?: string;
  'data-testid'?: string;
}

const Input: React.FC<InputProps> = ({
  label,
  value,
  onChange,
  type = 'text',
  placeholder,
  disabled = false,
  required = false,
  validation,
  minLength,
  maxLength,
  error: externalError,
  helperText,
  id,
  className,
  'data-testid': testId,
}) => {
  const [internalError, setInternalError] = useState<string>('');
  const [touched, setTouched] = useState(false);
  
  // Use utils package function to generate unique ID if not provided
  const inputId = id || generateId('input');
  const errorId = `${inputId}-error`;
  const helperTextId = `${inputId}-helper`;

  // Validate input using utils package validators
  const validateInput = useCallback((inputValue: string): ValidationError | null => {
    if (required) {
      const requiredError = validators.validateRequired(inputValue, label || 'Field');
      if (requiredError) return requiredError;
    }

    if (inputValue && validation) {
      switch (validation) {
        case 'email':
          return validators.validateEmail(inputValue, label || 'Email');
        case 'phone':
          if (!validators.isPhoneNumber(inputValue)) {
            return { field: label || 'Phone', message: 'Invalid phone number format', value: inputValue };
          }
          break;
        case 'url':
          if (!validators.isUrl(inputValue)) {
            return { field: label || 'URL', message: 'Invalid URL format', value: inputValue };
          }
          break;
        case 'strongPassword':
          if (!validators.isStrongPassword(inputValue)) {
            return {
              field: label || 'Password',
              message: 'Password must be at least 8 characters with uppercase, lowercase, number, and special character',
              value: inputValue
            };
          }
          break;
      }
    }

    if (minLength && inputValue.length < minLength) {
      return validators.validateMinLength(inputValue, minLength, label || 'Field');
    }

    if (maxLength && inputValue.length > maxLength) {
      return validators.validateMaxLength(inputValue, maxLength, label || 'Field');
    }

    return null;
  }, [required, validation, minLength, maxLength, label]);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const newValue = e.target.value;
    onChange(newValue);
    
    if (touched) {
      const validationError = validateInput(newValue);
      setInternalError(validationError?.message || '');
    }
  };

  const handleBlur = () => {
    setTouched(true);
    const validationError = validateInput(value);
    setInternalError(validationError?.message || '');
  };

  const displayError = externalError || internalError;
  const hasError = Boolean(displayError);

  const inputClasses = clsx(
    'block w-full px-3 py-2 border rounded-md shadow-sm placeholder-gray-400 focus:outline-none focus:ring-1',
    {
      'border-gray-300 focus:border-blue-500 focus:ring-blue-500': !hasError,
      'border-red-300 focus:border-red-500 focus:ring-red-500': hasError,
      'bg-gray-50 cursor-not-allowed': disabled,
    },
    className
  );

  return (
    <div className="space-y-1">
      {label && (
        <label
          htmlFor={inputId}
          className={clsx(
            'block text-sm font-medium',
            hasError ? 'text-red-700' : 'text-gray-700'
          )}
        >
          {label}
          {required && <span className="text-red-500 ml-1">*</span>}
        </label>
      )}
      
      <input
        id={inputId}
        type={type}
        value={value}
        onChange={handleChange}
        onBlur={handleBlur}
        placeholder={placeholder}
        disabled={disabled}
        className={inputClasses}
        aria-invalid={hasError}
        aria-describedby={clsx({
          [errorId]: hasError,
          [helperTextId]: helperText,
        })}
        data-testid={testId}
      />
      
      {displayError && (
        <p id={errorId} className="text-sm text-red-600" role="alert">
          {displayError}
        </p>
      )}
      
      {helperText && !displayError && (
        <p id={helperTextId} className="text-sm text-gray-500">
          {helperText}
        </p>
      )}
    </div>
  );
};

export default Input;