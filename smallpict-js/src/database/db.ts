import { Pool } from 'pg';
import { config } from '../config/config.js';

export const pool = new Pool(config.db);

export async function initDB() {
  try {
    const client = await pool.connect();
    try {
      await client.query(`
        CREATE TABLE IF NOT EXISTS public.images (
          id VARCHAR(50) PRIMARY KEY,
          filename VARCHAR(255),
          url TEXT,
          size BIGINT,
          size_origin BIGINT,
          mime_type VARCHAR(100),
          created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
      `);
      console.log('✅ Database connected and public.images initialized');
    } finally {
      client.release();
    }
  } catch (err: any) {
    console.warn('⚠️ Warning: Database connection failed:', err.message);
  }
}

export async function checkDB(): Promise<boolean> {
  try {
    const res = await pool.query('SELECT 1');
    return res.rowCount !== null && res.rowCount > 0;
  } catch {
    return false;
  }
}
