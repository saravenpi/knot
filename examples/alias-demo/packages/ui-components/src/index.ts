// Main UI Components exports
export { Button, type ButtonProps } from './Button';
export { Input, type InputProps } from './Input';
export { DataTable, type DataTableProps } from './DataTable';
export { Modal, type ModalProps } from './Modal';

// Re-export common utilities that components might need
export { debounce, throttle } from 'utils'; // This will use the alias system