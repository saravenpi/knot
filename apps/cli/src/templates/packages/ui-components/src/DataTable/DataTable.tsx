/**
 * DataTable component demonstrating complex usage of utils package
 */
import React, { useState, useMemo } from 'react';
import clsx from 'clsx';
import { motion } from 'framer-motion';
// Import from utils package - demonstrating multiple dependencies
import { formatters, helpers, SortConfig, FilterConfig, SortDirection } from 'utils';

export interface DataTableColumn<T = Record<string, unknown>> {
  key: keyof T;
  title: string;
  sortable?: boolean;
  filterable?: boolean;
  render?: (value: unknown, row: T) => React.ReactNode;
  format?: 'currency' | 'number' | 'percent' | 'date' | 'fileSize';
  width?: string;
}

export interface DataTableProps<T = Record<string, unknown>> {
  data: T[];
  columns: DataTableColumn<T>[];
  loading?: boolean;
  onRowClick?: (row: T) => void;
  className?: string;
  'data-testid'?: string;
}

function DataTable<T extends Record<string, unknown>>({
  data,
  columns,
  loading = false,
  onRowClick,
  className,
  'data-testid': testId,
}: DataTableProps<T>) {
  const [sortConfig, setSortConfig] = useState<SortConfig | null>(null);
  const [filters, setFilters] = useState<FilterConfig[]>([]);

  // Use utils package helpers for data processing
  const processedData = useMemo(() => {
    let result = helpers.cloneDeep(data);

    // Apply filters
    if (filters.length > 0) {
      result = result.filter(row => {
        return filters.every(filter => {
          const value = row[filter.field];
          switch (filter.operator) {
            case 'eq':
              return value === filter.value;
            case 'ne':
              return value !== filter.value;
            case 'gt':
              return value > filter.value;
            case 'lt':
              return value < filter.value;
            case 'gte':
              return value >= filter.value;
            case 'lte':
              return value <= filter.value;
            case 'in':
              return Array.isArray(filter.value) && filter.value.includes(value);
            case 'nin':
              return Array.isArray(filter.value) && !filter.value.includes(value);
            case 'like':
              return String(value).toLowerCase().includes(String(filter.value).toLowerCase());
            default:
              return true;
          }
        });
      });
    }

    // Apply sorting
    if (sortConfig) {
      result.sort((a, b) => {
        const aValue = a[sortConfig.field];
        const bValue = b[sortConfig.field];
        
        if (aValue < bValue) {
          return sortConfig.direction === 'asc' ? -1 : 1;
        }
        if (aValue > bValue) {
          return sortConfig.direction === 'asc' ? 1 : -1;
        }
        return 0;
      });
    }

    return result;
  }, [data, sortConfig, filters]);

  const handleSort = (columnKey: keyof T) => {
    let direction: SortDirection = 'asc';
    
    if (sortConfig && sortConfig.field === columnKey && sortConfig.direction === 'asc') {
      direction = 'desc';
    }
    
    setSortConfig({ field: String(columnKey), direction });
  };

  const formatCellValue = (value: unknown, column: DataTableColumn<T>) => {
    if (column.render) {
      return column.render(value, {} as T);
    }

    if (column.format) {
      try {
        switch (column.format) {
          case 'currency':
            return formatters.currency(Number(value));
          case 'number':
            return formatters.number(Number(value));
          case 'percent':
            return formatters.percent(Number(value));
          case 'date':
            return formatters.date(value);
          case 'fileSize':
            return formatters.fileSize(Number(value));
          default:
            return String(value);
        }
      } catch {
        return String(value);
      }
    }

    return String(value);
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center p-8">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
        <span className="ml-2 text-gray-600">Loading...</span>
      </div>
    );
  }

  return (
    <div className={clsx('overflow-hidden shadow ring-1 ring-black ring-opacity-5 md:rounded-lg', className)} data-testid={testId}>
      <table className="min-w-full divide-y divide-gray-300">
        <thead className="bg-gray-50">
          <tr>
            {columns.map((column) => (
              <th
                key={String(column.key)}
                className={clsx(
                  'px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider',
                  column.sortable && 'cursor-pointer hover:bg-gray-100 select-none'
                )}
                style={{ width: column.width }}
                onClick={() => column.sortable && handleSort(column.key)}
              >
                <div className="flex items-center space-x-1">
                  <span>{column.title}</span>
                  {column.sortable && sortConfig?.field === String(column.key) && (
                    <motion.svg
                      className="w-4 h-4"
                      fill="currentColor"
                      viewBox="0 0 20 20"
                      initial={{ rotate: 0 }}
                      animate={{ rotate: sortConfig.direction === 'desc' ? 180 : 0 }}
                      transition={{ duration: 0.2 }}
                    >
                      <path
                        fillRule="evenodd"
                        d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"
                        clipRule="evenodd"
                      />
                    </motion.svg>
                  )}
                </div>
              </th>
            ))}
          </tr>
        </thead>
        <tbody className="bg-white divide-y divide-gray-200">
          {processedData.map((row, index) => (
            <motion.tr
              key={index}
              className={clsx(
                'hover:bg-gray-50',
                onRowClick && 'cursor-pointer'
              )}
              onClick={() => onRowClick?.(row)}
              whileHover={{ backgroundColor: 'rgb(249 250 251)' }}
            >
              {columns.map((column) => (
                <td
                  key={String(column.key)}
                  className="px-6 py-4 whitespace-nowrap text-sm text-gray-900"
                >
                  {formatCellValue(row[column.key], column)}
                </td>
              ))}
            </motion.tr>
          ))}
        </tbody>
      </table>
      
      {processedData.length === 0 && (
        <div className="text-center py-8 text-gray-500">
          No data available
        </div>
      )}
    </div>
  );
}

export default DataTable;