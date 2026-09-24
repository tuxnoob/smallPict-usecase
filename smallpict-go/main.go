package main

import (
	"fmt"
	"log"
	"os"

	"smallpict-go/config"
	"smallpict-go/database"
	"smallpict-go/routes"

	"github.com/gin-gonic/gin"
)

func main() {
	cfg := config.LoadConfig()

	_, err := database.InitDB(cfg)
	if err != nil {
		log.Printf("Starting service without active DB connection: %v\n", err)
	}

	_ = os.MkdirAll(cfg.UploadPath, 0755)

	gin.SetMode(gin.ReleaseMode)
	r := gin.Default()

	r.Static("/uploads", cfg.UploadPath)

	routes.RegisterRoutes(r, cfg)

	addr := fmt.Sprintf(":%s", cfg.Port)
	log.Printf("🚀 %s running on port %s (http://localhost%s)\n", cfg.AppName, cfg.Port, addr)
	if err := r.Run(addr); err != nil {
		log.Fatalf("Failed to run server: %v", err)
	}
}
