import React from 'react';
import { isEmail, isEmpty } from 'utils/validation'; // Using alias with subpath

export interface InputProps {
  label?: string;
  placeholder?: string;
  value?: string;
  defaultValue?: string;
  type?: 'text' | 'email' | 'password' | 'number' | 'tel' | 'url' | 'search';
  disabled?: boolean;
  required?: boolean;
  error?: string;
  helperText?: string;
  className?: string;
  validate?: boolean;
  onChange?: (event: React.ChangeEvent<HTMLInputElement>) => void;
  onBlur?: (event: React.FocusEvent<HTMLInputElement>) => void;
  onFocus?: (event: React.FocusEvent<HTMLInputElement>) => void;
}

export const Input: React.FC<InputProps> = ({
  label,
  placeholder,
  value,
  defaultValue,
  type = 'text',
  disabled = false,
  required = false,
  error,
  helperText,
  className = '',
  validate = false,
  onChange,
  onBlur,
  onFocus,
  ...props
}) => {
  const [internalValue, setInternalValue] = React.useState(defaultValue || '');
  const [validationError, setValidationError] = React.useState<string>('');
  
  const currentValue = value !== undefined ? value : internalValue;

  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const newValue = event.target.value;
    
    if (value === undefined) {
      setInternalValue(newValue);
    }
    
    // Clear validation error when user starts typing
    if (validationError) {
      setValidationError('');
    }
    
    onChange?.(event);
  };

  const handleBlur = (event: React.FocusEvent<HTMLInputElement>) => {
    if (validate) {
      validateInput(event.target.value);
    }
    onBlur?.(event);
  };

  const validateInput = (inputValue: string) => {
    if (required && isEmpty(inputValue)) {
      setValidationError('This field is required');
      return;
    }

    if (type === 'email' && !isEmpty(inputValue) && !isEmail(inputValue)) {
      setValidationError('Please enter a valid email address');
      return;
    }

    setValidationError('');
  };

  const displayError = error || validationError;
  const hasError = Boolean(displayError);

  const inputClasses = [
    'flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background',
    'file:border-0 file:bg-transparent file:text-sm file:font-medium',
    'placeholder:text-muted-foreground',
    'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2',
    'disabled:cursor-not-allowed disabled:opacity-50',
    hasError ? 'border-red-500 focus-visible:ring-red-500' : '',
    className,
  ].join(' ');

  return (
    <div className="space-y-2">
      {label && (
        <label className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
          {label}
          {required && <span className="text-red-500 ml-1">*</span>}
        </label>
      )}
      <input
        type={type}
        placeholder={placeholder}
        value={currentValue}
        disabled={disabled}
        required={required}
        className={inputClasses}
        onChange={handleChange}
        onBlur={handleBlur}
        onFocus={onFocus}
        {...props}
      />
      {(displayError || helperText) && (
        <p className={`text-sm ${hasError ? 'text-red-500' : 'text-muted-foreground'}`}>
          {displayError || helperText}
        </p>
      )}
    </div>
  );
};