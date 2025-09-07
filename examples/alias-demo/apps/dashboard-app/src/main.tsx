import React from 'react';
import ReactDOM from 'react-dom/client';
// Using @ui alias to import from knot_packages
import { Button, Input, DataTable } from '@ui/ui-components';
import { formatDate, isEmail } from '@ui/utils/validation'; 
import { MemoryCache } from '@ui/data-layer/cache';
import { debounce } from '@ui/utils';

import './index.css';

// Example component using aliased imports
const Dashboard: React.FC = () => {
  const [email, setEmail] = React.useState('');
  const [users, setUsers] = React.useState([
    { id: 1, name: 'John Doe', email: 'john@example.com', createdAt: new Date() },
    { id: 2, name: 'Jane Smith', email: 'jane@example.com', createdAt: new Date() },
  ]);

  // Using debounce from utils via alias
  const debouncedEmailValidation = React.useMemo(() => 
    debounce((value: string) => {
      if (value && !isEmail(value)) {
        console.log('Invalid email format');
      }
    }, 300), []
  );

  // Using cache from data-layer via alias
  const cache = React.useMemo(() => new MemoryCache({ ttl: 300000 }), []);

  const handleEmailChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setEmail(e.target.value);
    debouncedEmailValidation(e.target.value);
  };

  const handleAddUser = () => {
    if (!isEmail(email)) {
      alert('Please enter a valid email');
      return;
    }

    const newUser = {
      id: users.length + 1,
      name: email.split('@')[0],
      email,
      createdAt: new Date(),
    };

    setUsers(prev => [...prev, newUser]);
    setEmail('');

    // Cache the user data
    cache.set(`user_${newUser.id}`, newUser);
  };

  const columns = [
    { key: 'name', title: 'Name' },
    { key: 'email', title: 'Email' },
    { 
      key: 'createdAt', 
      title: 'Created', 
      render: (date: Date) => formatDate(date, 'YYYY-MM-DD')
    },
  ];

  return (
    <div className="p-8 max-w-4xl mx-auto">
      <h1 className="text-3xl font-bold mb-8">Dashboard with Alias Demo</h1>
      
      <div className="mb-6 p-4 bg-blue-50 rounded-lg">
        <h2 className="text-lg font-semibold mb-2">Alias Configuration</h2>
        <p className="text-sm text-gray-600">
          This app uses <code className="bg-white px-2 py-1 rounded">tsAlias: "@ui"</code> 
          which overrides the global alias. All imports use @ui prefix.
        </p>
      </div>

      <div className="space-y-6">
        <div>
          <h2 className="text-xl font-semibold mb-4">Add User</h2>
          <div className="flex gap-4 items-end">
            <Input
              label="Email"
              type="email"
              value={email}
              onChange={handleEmailChange}
              placeholder="Enter email address"
              validate
              className="flex-1"
            />
            <Button onClick={handleAddUser} variant="primary">
              Add User
            </Button>
          </div>
        </div>

        <div>
          <h2 className="text-xl font-semibold mb-4">Users Table</h2>
          <DataTable 
            data={users} 
            columns={columns}
            className="border rounded-lg"
          />
        </div>

        <div className="text-sm text-gray-500">
          <p>💡 This example demonstrates:</p>
          <ul className="list-disc ml-6 mt-2">
            <li>Using <code>@ui/ui-components</code> to import Button, Input, DataTable</li>
            <li>Using <code>@ui/utils/validation</code> for email validation</li>
            <li>Using <code>@ui/data-layer/cache</code> for caching functionality</li>
            <li>Using <code>@ui/utils</code> for debounce utility</li>
          </ul>
        </div>
      </div>
    </div>
  );
};

// Mount the app
const root = ReactDOM.createRoot(document.getElementById('root') as HTMLElement);
root.render(<Dashboard />);