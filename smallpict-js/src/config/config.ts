import dotenv from 'dotenv';
dotenv.config();

export const config = {
  appName: process.env.APP_NAME || 'SmallPict Node.js Service',
  port: parseInt(process.env.SERVER_PORT || '8006', 10),
  db: {
    host: process.env.DB_HOST || 'localhost',
    port: parseInt(process.env.DB_PORT || '5432', 10),
    database: process.env.DB_NAME || 'smallpict',
    user: process.env.DB_USER || 'postgres',
    password: process.env.DB_PASS || 'postgres',
    ssl: process.env.DB_SSLMODE === 'require' ? { rejectUnauthorized: false } : false,
  },
  smallpict: {
    mode: (process.env.SMALLPICT_MODE || 'locale').toLowerCase(),
    baseUrl: (process.env.SMALLPICT_BASE_URL || 'https://api.smallpict.app').replace(/\/$/, ''),
    apiKey: process.env.SMALLPICT_API_KEY || '',
    secretKey: process.env.SMALLPICT_SECRET_KEY || '',
    format: process.env.SMALLPICT_FORMAT || 'auto',
    quality: parseInt(process.env.SMALLPICT_QUALITY || '80', 10),
  },
  uploadPath: process.env.UPLOAD_PATH || './uploads',
};
