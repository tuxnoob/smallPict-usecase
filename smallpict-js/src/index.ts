import express from 'express';
import cors from 'cors';
import path from 'path';
import fs from 'fs';
import { config } from './config/config.js';
import { initDB } from './database/db.js';
import router from './routes/route.js';

const app = express();

app.use(cors());
app.use(express.json({ limit: '50mb' }));
app.use(express.urlencoded({ extended: true, limit: '50mb' }));

const uploadDir = path.resolve(config.uploadPath);
if (!fs.existsSync(uploadDir)) {
  fs.mkdirSync(uploadDir, { recursive: true });
}
app.use('/uploads', express.static(uploadDir));

app.use(router);

async function bootstrap() {
  await initDB();

  app.listen(config.port, () => {
    console.log(`🚀 ${config.appName} running on port ${config.port} (http://localhost:${config.port})`);
  });
}

bootstrap();
