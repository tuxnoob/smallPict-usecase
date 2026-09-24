package config

import (
	"os"
	"strconv"

	"github.com/joho/godotenv"
)

type Config struct {
	AppName           string
	Port              string
	DBHost            string
	DBPort            string
	DBName            string
	DBUser            string
	DBPass            string
	DBSSLMode         string
	SmallPictMode     string
	SmallPictBaseURL  string
	SmallPictAPIKey   string
	SmallPictSecretKey string
	SmallPictFormat   string
	SmallPictQuality  int
	UploadPath        string
}

var Cfg *Config

func LoadConfig() *Config {
	_ = godotenv.Load()

	quality, _ := strconv.Atoi(getEnv("SMALLPICT_QUALITY", "80"))
	if quality <= 0 {
		quality = 80
	}

	Cfg = &Config{
		AppName:           getEnv("APP_NAME", "SmallPict Go Service"),
		Port:              getEnv("SERVER_PORT", "8004"),
		DBHost:            getEnv("DB_HOST", "localhost"),
		DBPort:            getEnv("DB_PORT", "5432"),
		DBName:            getEnv("DB_NAME", "smallpict"),
		DBUser:            getEnv("DB_USER", "postgres"),
		DBPass:            getEnv("DB_PASS", "postgres"),
		DBSSLMode:         getEnv("DB_SSLMODE", "disable"),
		SmallPictMode:     getEnv("SMALLPICT_MODE", "locale"),
		SmallPictBaseURL:  getEnv("SMALLPICT_BASE_URL", "https://api.smallpict.app"),
		SmallPictAPIKey:   getEnv("SMALLPICT_API_KEY", ""),
		SmallPictSecretKey: getEnv("SMALLPICT_SECRET_KEY", ""),
		SmallPictFormat:   getEnv("SMALLPICT_FORMAT", "auto"),
		SmallPictQuality:  quality,
		UploadPath:        getEnv("UPLOAD_PATH", "./uploads"),
	}

	return Cfg
}

func getEnv(key, fallback string) string {
	if val, ok := os.LookupEnv(key); ok && val != "" {
		return val
	}
	return fallback
}
