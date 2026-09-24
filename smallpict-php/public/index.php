<?php

require_once __DIR__ . '/../vendor/autoload.php';

use Bramus\Router\Router;
use SmallPict\Config\Config;
use SmallPict\Controller\BaseController;

$config = Config::getInstance();

$uri = parse_url($_SERVER['REQUEST_URI'] ?? '', PHP_URL_PATH);
if (str_starts_with($uri, '/uploads/')) {
    $filePath = rtrim($config->uploadPath, '/') . '/' . basename($uri);
    if (file_exists($filePath) && is_file($filePath)) {
        $mime = mime_content_type($filePath) ?: 'application/octet-stream';
        header("Content-Type: {$mime}");
        readfile($filePath);
        exit;
    }
}

$router = new Router();
$baseCtrl = new BaseController();

$router->get('/', [$baseCtrl, 'getStatus']);
$router->get('/health', [$baseCtrl, 'checkHealth']);
$router->post('/upload', [$baseCtrl, 'handleUpload']);

$router->run();
