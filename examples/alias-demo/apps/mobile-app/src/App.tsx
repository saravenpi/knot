import React from 'react';
import { View, Text, StyleSheet, ScrollView, Alert } from 'react-native';

// Using # alias (global) to import from knot_packages
import { Button, Input } from '#/ui-components';
import { formatDate, isToday } from '#/utils/dates';
import { isEmail, isEmpty } from '#/utils/validation';
import { debounce, uuid } from '#/utils';

interface Todo {
  id: string;
  title: string;
  completed: boolean;
  createdAt: Date;
}

const App: React.FC = () => {
  const [newTodo, setNewTodo] = React.useState('');
  const [todos, setTodos] = React.useState<Todo[]>([
    {
      id: '1',
      title: 'Learn Knot alias system',
      completed: false,
      createdAt: new Date(),
    },
    {
      id: '2',
      title: 'Build mobile app',
      completed: true,
      createdAt: new Date(),
    },
  ]);

  // Using debounce from utils via # alias
  const debouncedValidation = React.useMemo(() => 
    debounce((value: string) => {
      if (isEmpty(value.trim())) {
        console.log('Todo cannot be empty');
      }
    }, 500), []
  );

  const handleAddTodo = () => {
    if (isEmpty(newTodo.trim())) {
      Alert.alert('Validation Error', 'Todo title cannot be empty');
      return;
    }

    const todo: Todo = {
      id: uuid(),
      title: newTodo.trim(),
      completed: false,
      createdAt: new Date(),
    };

    setTodos(prev => [...prev, todo]);
    setNewTodo('');
  };

  const toggleTodo = (id: string) => {
    setTodos(prev => 
      prev.map(todo => 
        todo.id === id 
          ? { ...todo, completed: !todo.completed }
          : todo
      )
    );
  };

  const handleInputChange = (value: string) => {
    setNewTodo(value);
    debouncedValidation(value);
  };

  return (
    <ScrollView style={styles.container}>
      <View style={styles.content}>
        <Text style={styles.title}>Mobile App with # Alias</Text>
        
        <View style={styles.infoBox}>
          <Text style={styles.infoTitle}>Alias Configuration</Text>
          <Text style={styles.infoText}>
            This app inherits the global alias "#" from knot.yml.
            All imports use # prefix (e.g., #/utils, #/ui-components).
          </Text>
        </View>

        <View style={styles.section}>
          <Text style={styles.sectionTitle}>Add Todo</Text>
          <Input
            placeholder="Enter todo title"
            value={newTodo}
            onChange={(e) => handleInputChange(e.target.value)}
            style={styles.input}
          />
          <Button 
            onPress={handleAddTodo}
            variant="primary"
            style={styles.addButton}
          >
            Add Todo
          </Button>
        </View>

        <View style={styles.section}>
          <Text style={styles.sectionTitle}>Todos</Text>
          {todos.map(todo => (
            <View key={todo.id} style={styles.todoItem}>
              <View style={styles.todoContent}>
                <Text style={[
                  styles.todoTitle,
                  todo.completed && styles.completed
                ]}>
                  {todo.title}
                </Text>
                <Text style={styles.todoDate}>
                  {isToday(todo.createdAt) 
                    ? 'Today' 
                    : formatDate(todo.createdAt, 'MM/DD/YYYY')
                  }
                </Text>
              </View>
              <Button
                onPress={() => toggleTodo(todo.id)}
                variant={todo.completed ? "secondary" : "primary"}
                size="sm"
              >
                {todo.completed ? '✓' : '○'}
              </Button>
            </View>
          ))}
        </View>

        <View style={styles.demoSection}>
          <Text style={styles.demoTitle}>💡 This example demonstrates:</Text>
          <Text style={styles.demoItem}>• Using #/ui-components for Button, Input</Text>
          <Text style={styles.demoItem}>• Using #/utils/dates for date formatting</Text>
          <Text style={styles.demoItem}>• Using #/utils/validation for input validation</Text>
          <Text style={styles.demoItem}>• Using #/utils for debounce and uuid utilities</Text>
          <Text style={styles.demoItem}>• Inheriting global alias configuration</Text>
        </View>
      </View>
    </ScrollView>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#f5f5f5',
  },
  content: {
    padding: 16,
    maxWidth: 600,
    alignSelf: 'center',
    width: '100%',
  },
  title: {
    fontSize: 24,
    fontWeight: 'bold',
    marginBottom: 16,
    textAlign: 'center',
  },
  infoBox: {
    backgroundColor: '#e3f2fd',
    padding: 16,
    borderRadius: 8,
    marginBottom: 24,
  },
  infoTitle: {
    fontSize: 16,
    fontWeight: '600',
    marginBottom: 8,
  },
  infoText: {
    fontSize: 14,
    color: '#666',
    lineHeight: 20,
  },
  section: {
    marginBottom: 24,
  },
  sectionTitle: {
    fontSize: 18,
    fontWeight: '600',
    marginBottom: 12,
  },
  input: {
    marginBottom: 12,
  },
  addButton: {
    alignSelf: 'flex-start',
  },
  todoItem: {
    flexDirection: 'row',
    alignItems: 'center',
    backgroundColor: 'white',
    padding: 12,
    borderRadius: 8,
    marginBottom: 8,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 1 },
    shadowOpacity: 0.1,
    shadowRadius: 2,
    elevation: 2,
  },
  todoContent: {
    flex: 1,
  },
  todoTitle: {
    fontSize: 16,
    marginBottom: 4,
  },
  completed: {
    textDecorationLine: 'line-through',
    color: '#666',
  },
  todoDate: {
    fontSize: 12,
    color: '#999',
  },
  demoSection: {
    backgroundColor: '#f8f9fa',
    padding: 16,
    borderRadius: 8,
    borderLeftWidth: 4,
    borderLeftColor: '#007bff',
  },
  demoTitle: {
    fontSize: 14,
    fontWeight: '600',
    marginBottom: 8,
  },
  demoItem: {
    fontSize: 12,
    color: '#666',
    lineHeight: 18,
    marginBottom: 2,
  },
});

export default App;