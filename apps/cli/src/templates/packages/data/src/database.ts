/**
 * Database connection and management using Prisma and utils package
 */
import { PrismaClient } from '@prisma/client';
import { Client as PgClient } from 'pg';
import { createConnection, Connection } from 'mysql2/promise';
import { MongoClient, Db } from 'mongodb';
// Import from utils package
import { retry, generateId, ValidationError } from 'utils';

export interface DatabaseConfig {
  provider: 'postgresql' | 'mysql' | 'mongodb' | 'sqlite';
  url: string;
  poolSize?: number;
  timeout?: number;
  retryAttempts?: number;
  ssl?: boolean;
}

export interface ConnectionOptions {
  maxRetries: number;
  retryDelay: number;
  timeout: number;
}

class Database {
  private prisma?: PrismaClient;
  private pgClient?: PgClient;
  private mysqlConnection?: Connection;
  private mongoClient?: MongoClient;
  private mongoDb?: Db;
  private config: DatabaseConfig;
  private connectionId: string;

  constructor(config: DatabaseConfig) {
    this.config = config;
    // Use utils package to generate unique connection ID
    this.connectionId = generateId('db_conn');
  }

  async connect(options: Partial<ConnectionOptions> = {}): Promise<void> {
    const defaultOptions: ConnectionOptions = {
      maxRetries: 3,
      retryDelay: 1000,
      timeout: 10000,
    };

    const finalOptions = { ...defaultOptions, ...options };

    try {
      // Use utils package retry helper for robust connection
      await retry(async () => {
        switch (this.config.provider) {
          case 'postgresql':
            await this.connectPostgres();
            break;
          case 'mysql':
            await this.connectMySQL();
            break;
          case 'mongodb':
            await this.connectMongoDB();
            break;
          case 'sqlite':
            await this.connectSQLite();
            break;
          default:
            throw new Error(`Unsupported database provider: ${this.config.provider}`);
        }
      }, finalOptions.maxRetries, finalOptions.retryDelay);

      console.log(`Database connected successfully (${this.connectionId})`);
    } catch (error) {
      console.error('Failed to connect to database:', error);
      throw error;
    }
  }

  private async connectPostgres(): Promise<void> {
    this.prisma = new PrismaClient({
      datasources: {
        db: {
          url: this.config.url,
        },
      },
    });

    // Test connection
    await this.prisma.$connect();

    this.pgClient = new PgClient({
      connectionString: this.config.url,
    });

    await this.pgClient.connect();
  }

  private async connectMySQL(): Promise<void> {
    this.mysqlConnection = await createConnection({
      uri: this.config.url,
      acquireTimeout: this.config.timeout || 10000,
      timeout: this.config.timeout || 10000,
    });

    // Test connection
    await this.mysqlConnection.ping();
  }

  private async connectMongoDB(): Promise<void> {
    this.mongoClient = new MongoClient(this.config.url, {
      maxPoolSize: this.config.poolSize || 10,
      serverSelectionTimeoutMS: this.config.timeout || 10000,
    });

    await this.mongoClient.connect();
    
    const dbName = new URL(this.config.url).pathname.slice(1);
    this.mongoDb = this.mongoClient.db(dbName);
  }

  private async connectSQLite(): Promise<void> {
    this.prisma = new PrismaClient({
      datasources: {
        db: {
          url: this.config.url,
        },
      },
    });

    await this.prisma.$connect();
  }

  async disconnect(): Promise<void> {
    try {
      if (this.prisma) {
        await this.prisma.$disconnect();
      }

      if (this.pgClient) {
        await this.pgClient.end();
      }

      if (this.mysqlConnection) {
        await this.mysqlConnection.end();
      }

      if (this.mongoClient) {
        await this.mongoClient.close();
      }

      console.log(`Database disconnected (${this.connectionId})`);
    } catch (error) {
      console.error('Error disconnecting from database:', error);
      throw error;
    }
  }

  getPrismaClient(): PrismaClient {
    if (!this.prisma) {
      throw new Error('Prisma client not initialized. Call connect() first.');
    }
    return this.prisma;
  }

  getPostgresClient(): PgClient {
    if (!this.pgClient) {
      throw new Error('PostgreSQL client not initialized. Call connect() first.');
    }
    return this.pgClient;
  }

  getMySQLConnection(): Connection {
    if (!this.mysqlConnection) {
      throw new Error('MySQL connection not initialized. Call connect() first.');
    }
    return this.mysqlConnection;
  }

  getMongoDb(): Db {
    if (!this.mongoDb) {
      throw new Error('MongoDB connection not initialized. Call connect() first.');
    }
    return this.mongoDb;
  }

  async healthCheck(): Promise<boolean> {
    try {
      switch (this.config.provider) {
        case 'postgresql':
        case 'sqlite':
          await this.prisma?.$queryRaw`SELECT 1`;
          return true;
        case 'mysql':
          await this.mysqlConnection?.ping();
          return true;
        case 'mongodb':
          await this.mongoDb?.admin().ping();
          return true;
        default:
          return false;
      }
    } catch {
      return false;
    }
  }
}

export default Database;