<?php

namespace SmallPict\Controller;

use SmallPict\Config\Config;
use SmallPict\Database\Database;
use SmallPict\Helper\SmallPictHelper;

class BaseController
{
    public function getStatus(): void
    {
        $config = Config::getInstance();
        header('Content-Type: application/json');
        echo json_encode([
            'status'  => 'ok',
            'service' => $config->appName,
            'version' => '1.0.0',
        ]);
    }

    public function checkHealth(): void
    {
        $dbOk = Database::checkConnection();
        $statusCode = $dbOk ? 200 : 503;

        http_response_code($statusCode);
        header('Content-Type: application/json');

        echo json_encode([
            'status'    => $dbOk ? 'ok' : 'unhealthy',
            'database'  => $dbOk ? 'connected' : 'disconnected',
            'timestamp' => gmdate('Y-m-d\TH:i:s\Z'),
        ]);
    }

    public function handleUpload(): void
    {
        header('Content-Type: application/json');

        try {
            $contentType = strtolower($_SERVER['CONTENT_TYPE'] ?? '');

            if (str_contains($contentType, 'application/json')) {
                $rawInput = file_get_contents('php://input');
                $body = json_decode($rawInput, true) ?? [];

                $base64 = $body['image'] ?? ($body['base64'] ?? ($body['data'] ?? null));
                $filename = $body['name'] ?? ($body['filename'] ?? null);

                if (empty($base64)) {
                    http_response_code(400);
                    echo json_encode([
                        'status'  => 'error',
                        'message' => "Field 'image' or 'base64' is required in JSON payload",
                    ]);
                    return;
                }

                $record = SmallPictHelper::processUpload(null, $base64, $filename);
                http_response_code(200);
                echo json_encode([
                    'status'  => 'success',
                    'message' => 'Image uploaded successfully',
                    'data'    => $record,
                ]);
                return;
            }

            $file = $_FILES['file'] ?? ($_FILES['image'] ?? null);
            $customFilename = $_POST['name'] ?? ($_POST['filename'] ?? null);
            $base64Form = $_POST['base64'] ?? ($_POST['image'] ?? null);

            if (empty($file) && empty($base64Form)) {
                http_response_code(400);
                echo json_encode([
                    'status'  => 'error',
                    'message' => 'No file or base64 image provided in request',
                ]);
                return;
            }

            $record = SmallPictHelper::processUpload($file, $base64Form, $customFilename);
            http_response_code(200);
            echo json_encode([
                'status'  => 'success',
                'message' => 'Image uploaded successfully',
                'data'    => $record,
            ]);

        } catch (\Throwable $e) {
            http_response_code(400);
            echo json_encode([
                'status'  => 'error',
                'message' => $e->getMessage(),
            ]);
        }
    }
}
