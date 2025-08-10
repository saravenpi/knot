// Advanced Logger Package
// High-performance logging with multiple outputs and filtering

export enum LogLevel {
  TRACE = 0,
  DEBUG = 1,
  INFO = 2,
  WARN = 3,
  ERROR = 4,
  FATAL = 5,
}

export interface LogEntry {
  timestamp: Date;
  level: LogLevel;
  message: string;
  metadata?: Record<string, any>;
  source?: string;
  traceId?: string;
}

export interface LoggerOptions {
  level: LogLevel;
  outputs: LogOutput[];
  format?: LogFormatter;
  includeMetadata?: boolean;
  enableColors?: boolean;
}

export interface LogOutput {
  write(entry: LogEntry): void | Promise<void>;
}

export interface LogFormatter {
  format(entry: LogEntry): string;
}

export class Logger {
  private options: LoggerOptions;
  private metadata: Record<string, any> = {};

  constructor(options: LoggerOptions) {
    this.options = { includeMetadata: true, enableColors: true, ...options };
  }

  trace(message: string, metadata?: Record<string, any>): void {
    this.log(LogLevel.TRACE, message, metadata);
  }

  debug(message: string, metadata?: Record<string, any>): void {
    this.log(LogLevel.DEBUG, message, metadata);
  }

  info(message: string, metadata?: Record<string, any>): void {
    this.log(LogLevel.INFO, message, metadata);
  }

  warn(message: string, metadata?: Record<string, any>): void {
    this.log(LogLevel.WARN, message, metadata);
  }

  error(message: string, metadata?: Record<string, any>): void {
    this.log(LogLevel.ERROR, message, metadata);
  }

  fatal(message: string, metadata?: Record<string, any>): void {
    this.log(LogLevel.FATAL, message, metadata);
  }

  child(metadata: Record<string, any>): Logger {
    const childLogger = new Logger(this.options);
    childLogger.metadata = { ...this.metadata, ...metadata };
    return childLogger;
  }

  private log(level: LogLevel, message: string, metadata?: Record<string, any>): void {
    if (level < this.options.level) return;

    const entry: LogEntry = {
      timestamp: new Date(),
      level,
      message,
      metadata: this.options.includeMetadata ? { ...this.metadata, ...metadata } : undefined,
    };

    this.options.outputs.forEach(async (output) => {
      await output.write(entry);
    });
  }
}

// Built-in outputs
export class ConsoleOutput implements LogOutput {
  write(entry: LogEntry): void {
    const levelName = LogLevel[entry.level];
    const timestamp = entry.timestamp.toISOString();
    console.log(`[${timestamp}] ${levelName}: ${entry.message}`, entry.metadata || '');
  }
}

export class FileOutput implements LogOutput {
  constructor(private filename: string) {}

  async write(entry: LogEntry): Promise<void> {
    // File writing implementation would go here
    const levelName = LogLevel[entry.level];
    const timestamp = entry.timestamp.toISOString();
    const line = `[${timestamp}] ${levelName}: ${entry.message}\n`;
    // In real implementation: await fs.appendFile(this.filename, line);
    console.log(`Writing to ${this.filename}: ${line.trim()}`);
  }
}

// Default logger instance
export const logger = new Logger({
  level: LogLevel.INFO,
  outputs: [new ConsoleOutput()],
});

// Example usage:
// logger.info('Application started', { version: '1.0.0', env: 'production' });
// logger.error('Database connection failed', { host: 'localhost', port: 5432 });
