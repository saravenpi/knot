/**
 * User Dashboard demonstrating complex dependency chain:
 * dashboard → api-client → data → utils
 *          → ui-components → utils
 */
import React, { useState, useEffect } from 'react';
import { useQuery } from 'react-query';
// Import from utils package (base dependency)
import { formatters, formatDate, generateId } from 'utils';
// Import from data package (depends on utils)
import { User, QueryOptions } from 'data';
// Import from ui-components package (depends on utils)
import { Button, DataTable, Input, DataTableColumn } from 'ui-components';
// Import from api-client package (depends on utils + data)
import { ApiClient, UserApiOperations } from 'api-client';

export interface UserDashboardProps {
  apiClient: ApiClient;
  className?: string;
  onUserSelect?: (user: User) => void;
  onUserCreate?: () => void;
  onUserEdit?: (user: User) => void;
  onUserDelete?: (user: User) => void;
}

const UserDashboard: React.FC<UserDashboardProps> = ({
  apiClient,
  className,
  onUserSelect,
  onUserCreate,
  onUserEdit,
  onUserDelete,
}) => {
  const [searchTerm, setSearchTerm] = useState('');
  const [queryOptions, setQueryOptions] = useState<QueryOptions>({
    page: 1,
    limit: 20,
    sort: [{ field: 'createdAt', direction: 'desc' }],
    filters: [],
  });

  // Use react-query with API client
  const {
    data: usersResponse,
    isLoading,
    isError,
    error,
    refetch
  } = useQuery(
    ['users', queryOptions],
    () => apiClient.getUsers(queryOptions),
    {
      keepPreviousData: true,
      staleTime: 5 * 60 * 1000, // 5 minutes
    }
  );

  const users = usersResponse?.data || [];
  
  // Update query options when search changes
  useEffect(() => {
    setQueryOptions(prev => ({
      ...prev,
      page: 1,
      filters: searchTerm 
        ? [{ field: 'name', operator: 'like', value: searchTerm }]
        : [],
    }));
  }, [searchTerm]);

  // Define table columns with utils package formatters
  const columns: DataTableColumn<User>[] = [
    {
      key: 'name',
      title: 'Name',
      sortable: true,
      render: (value, user) => (
        <div className="flex items-center space-x-3">
          {user.avatar && (
            <img 
              className="h-8 w-8 rounded-full" 
              src={user.avatar} 
              alt={user.name}
            />
          )}
          <span className="font-medium">{value}</span>
        </div>
      ),
    },
    {
      key: 'email',
      title: 'Email',
      sortable: true,
      render: (value) => (
        <a 
          href={`mailto:${value}`}
          className="text-blue-600 hover:text-blue-800"
        >
          {value}
        </a>
      ),
    },
    {
      key: 'isActive',
      title: 'Status',
      sortable: true,
      render: (value) => (
        <span
          className={`inline-flex px-2 py-1 text-xs font-semibold rounded-full ${
            value
              ? 'bg-green-100 text-green-800'
              : 'bg-red-100 text-red-800'
          }`}
        >
          {value ? 'Active' : 'Inactive'}
        </span>
      ),
    },
    {
      key: 'createdAt',
      title: 'Created',
      sortable: true,
      format: 'date',
      render: (value) => formatDate(value, 'MMM d, yyyy'),
    },
    {
      key: 'lastLoginAt',
      title: 'Last Login',
      sortable: true,
      render: (value) => 
        value 
          ? formatDate(value, 'MMM d, yyyy HH:mm')
          : <span className="text-gray-400">Never</span>,
    },
    {
      key: 'id',
      title: 'Actions',
      render: (_, user) => (
        <div className="flex space-x-2">
          <Button
            variant="ghost"
            size="sm"
            onClick={() => onUserEdit?.(user)}
          >
            Edit
          </Button>
          <Button
            variant="danger"
            size="sm"
            onClick={() => onUserDelete?.(user)}
          >
            Delete
          </Button>
        </div>
      ),
    },
  ];

  const handleUserClick = (user: User) => {
    onUserSelect?.(user);
  };

  const handleRefresh = async () => {
    await refetch();
  };

  if (isError) {
    return (
      <div className="bg-red-50 border border-red-200 rounded-md p-4">
        <div className="flex">
          <div className="flex-shrink-0">
            <svg className="h-5 w-5 text-red-400" viewBox="0 0 20 20" fill="currentColor">
              <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clipRule="evenodd" />
            </svg>
          </div>
          <div className="ml-3">
            <h3 className="text-sm font-medium text-red-800">
              Error loading users
            </h3>
            <div className="mt-2 text-sm text-red-700">
              {(error as Error)?.message || 'An unexpected error occurred'}
            </div>
            <div className="mt-4">
              <Button variant="secondary" size="sm" onClick={handleRefresh}>
                Try Again
              </Button>
            </div>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className={`space-y-6 ${className}`}>
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Users</h1>
          <p className="text-gray-600">
            Manage and view user accounts
          </p>
        </div>
        <div className="flex space-x-3">
          <Button variant="secondary" onClick={handleRefresh}>
            Refresh
          </Button>
          <Button variant="primary" onClick={onUserCreate}>
            Create User
          </Button>
        </div>
      </div>

      {/* Search and Filters */}
      <div className="flex items-center space-x-4">
        <div className="flex-1 max-w-md">
          <Input
            value={searchTerm}
            onChange={setSearchTerm}
            placeholder="Search users by name..."
            type="text"
          />
        </div>
      </div>

      {/* Users Table */}
      <DataTable
        data={users}
        columns={columns}
        loading={isLoading}
        onRowClick={handleUserClick}
        data-testid="users-table"
      />

      {/* Pagination Info */}
      {usersResponse?.meta && (
        <div className="flex items-center justify-between text-sm text-gray-700">
          <div>
            Showing {((usersResponse.meta.page - 1) * usersResponse.meta.limit) + 1} to{' '}
            {Math.min(usersResponse.meta.page * usersResponse.meta.limit, usersResponse.meta.total)} of{' '}
            {formatters.number(usersResponse.meta.total)} users
          </div>
          <div className="flex space-x-2">
            <Button
              variant="ghost"
              size="sm"
              disabled={usersResponse.meta.page === 1}
              onClick={() => setQueryOptions(prev => ({ ...prev, page: prev.page - 1 }))}
            >
              Previous
            </Button>
            <Button
              variant="ghost"
              size="sm"
              disabled={usersResponse.meta.page === usersResponse.meta.totalPages}
              onClick={() => setQueryOptions(prev => ({ ...prev, page: prev.page + 1 }))}
            >
              Next
            </Button>
          </div>
        </div>
      )}
    </div>
  );
};

export default UserDashboard;