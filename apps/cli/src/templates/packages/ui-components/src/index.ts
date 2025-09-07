/**
 * UI Components package that demonstrates dependency on utils package
 */
export { default as Button } from './Button/Button';
export { default as Input } from './Input/Input';
export { default as Card } from './Card/Card';
export { default as Badge } from './Badge/Badge';
export { default as Modal } from './Modal/Modal';
export { default as DataTable } from './DataTable/DataTable';
export { default as Form } from './Form/Form';

// Export types for consumers
export type { ButtonProps } from './Button/Button';
export type { InputProps } from './Input/Input';
export type { CardProps } from './Card/Card';
export type { BadgeProps } from './Badge/Badge';
export type { ModalProps } from './Modal/Modal';
export type { DataTableProps, DataTableColumn } from './DataTable/DataTable';
export type { FormProps, FormFieldConfig } from './Form/Form';