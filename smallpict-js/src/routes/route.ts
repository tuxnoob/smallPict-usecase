import { Router, Request, Response } from 'express';
import multer from 'multer';
import { config } from '../config/config.js';
import { checkDB } from '../database/db.js';
import { processUpload } from '../helper/smallpict.js';

const router = Router();
const upload = multer({ storage: multer.memoryStorage() });

const health = async (req: Request, res: Response): Promise<void> => {
  try {
    const dbOk = await checkDB();
    const statusCode = dbOk ? 200 : 503;

    res.status(statusCode).json({
      status: dbOk ? 'ok' : 'unhealthy',
      database: dbOk ? 'connected' : 'disconnected',
      timestamp: new Date().toISOString(),
    });
  } catch (err: any) {
    res.status(503).json({
      status: 'unhealthy',
      database: 'disconnected',
      error: err.message,
      timestamp: new Date().toISOString(),
    });
  }
}

const uploadHandler = async (req: Request, res: Response): Promise<void> => {
  try {
    const contentType = req.headers['content-type'] || '';

    if (contentType.includes('application/json')) {
      const base64Str = req.body?.image || req.body?.base64 || req.body?.data;
      const customFilename = req.body?.name || req.body?.filename;

      if (!base64Str) {
        res.status(400).json({
          status: 'error',
          message: "Field 'image' or 'base64' is required in JSON payload",
        });
        return;
      }

      const record = await processUpload(undefined, base64Str, customFilename);
      res.json({
        status: 'success',
        message: 'Image uploaded successfully',
        data: record,
      });
      return;
    }

    const files = req.files as Express.Multer.File[] | undefined;
    const file = (files && files.length > 0) ? files[0] : req.file;
    const customFilename = req.body?.name || req.body?.filename;
    const base64Form = req.body?.base64 || req.body?.image;

    if (!file && !base64Form) {
      res.status(400).json({
        status: 'error',
        message: 'No file or base64 image provided in request',
      });
      return;
    }

    const record = await processUpload(
      file?.buffer,
      base64Form,
      customFilename || file?.originalname,
      file?.mimetype
    );

    res.json({
      status: 'success',
      message: 'Image uploaded successfully',
      data: record,
    });
  } catch (err: any) {
    res.status(400).json({
      status: 'error',
      message: err.message || 'Upload failed',
    });
  }
};

router.get('/', (req, res) => {
  res.json({
    status: 'ok',
    service: config.appName,
    version: '1.0.0',
  });
});
router.get('/health', health);
router.post('/upload', upload.any(), uploadHandler);

export default router;
