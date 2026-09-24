<?php

namespace SmallPict\Database;

use PDO;
use PDOException;
use SmallPict\Config\Config;

class Database
{
    private static ?PDO $pdo = null;

    public static function getConnection(): ?PDO
    {
        if (self::$pdo === null) {
            $config = Config::getInstance();
            $dsn = sprintf(
                "pgsql:host=%s;port=%s;dbname=%s;sslmode=%s",
                $config->dbHost,
                $config->dbPort,
                $config->dbName,
                $config->dbSslMode
            );

            try {
                self::$pdo = new PDO($dsn, $config->dbUser, $config->dbPass, [
                    PDO::ATTR_ERRMODE            => PDO::ERRMODE_EXCEPTION,
                    PDO::ATTR_DEFAULT_FETCH_MODE => PDO::FETCH_ASSOC,
                    PDO::ATTR_TIMEOUT            => 5,
                ]);

                // Create public.images table if not exists
                self::$pdo->exec("
                    CREATE TABLE IF NOT EXISTS public.images (
                        id VARCHAR(50) PRIMARY KEY,
                        filename VARCHAR(255),
                        url TEXT,
                        size BIGINT,
                        size_origin BIGINT,
                        mime_type VARCHAR(100),
                        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                    );
                ");
            } catch (PDOException $e) {
                error_log("Database connection failed: " . $e->getMessage());
                self::$pdo = null;
            }
        }

        return self::$pdo;
    }

    public static function checkConnection(): bool
    {
        try {
            $pdo = self::getConnection();
            if ($pdo === null) {
                return false;
            }
            $stmt = $pdo->query("SELECT 1");
            return $stmt !== false;
        } catch (\Throwable $e) {
            return false;
        }
    }
}
