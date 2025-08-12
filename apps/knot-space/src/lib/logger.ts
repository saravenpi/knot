interface LogLevel {
  ERROR: 0;
  WARN: 1;
  INFO: 2;
  DEBUG: 3;
}

const LOG_LEVELS: LogLevel = {
  ERROR: 0,
  WARN: 1,
  INFO: 2,
  DEBUG: 3,
};

class Logger {
  private level: number;

  constructor() {
    const envLevel = process.env.LOG_LEVEL?.toUpperCase() || 'INFO';
    this.level = LOG_LEVELS[envLevel as keyof LogLevel] ?? LOG_LEVELS.INFO;
  }

  private log(level: keyof LogLevel, message: string, meta?: any) {
    if (LOG_LEVELS[level] > this.level) return;

    const timestamp = new Date().toISOString();
    const logEntry = {
      timestamp,
      level,
      message,
      ...(meta && { meta }),
    };

    if (process.env.NODE_ENV === 'production') {
      // Structured logging for production
      console.log(JSON.stringify(logEntry));
    } else {
      // Pretty logging for development
      const colorMap = {
        ERROR: '\x1b[31m', // Red
        WARN: '\x1b[33m',  // Yellow
        INFO: '\x1b[36m',  // Cyan
        DEBUG: '\x1b[37m', // White
      };
      
      const resetColor = '\x1b[0m';
      const color = colorMap[level];
      
      console.log(
        `${color}[${timestamp}] ${level}:${resetColor} ${message}`,
        meta ? JSON.stringify(meta, null, 2) : ''
      );
    }
  }

  error(message: string, meta?: any) {
    this.log('ERROR', message, meta);
  }

  warn(message: string, meta?: any) {
    this.log('WARN', message, meta);
  }

  info(message: string, meta?: any) {
    this.log('INFO', message, meta);
  }

  debug(message: string, meta?: any) {
    this.log('DEBUG', message, meta);
  }
}

export const logger = new Logger();