package routes

import (
	"net/http"
	"time"

	"smallpict-go/config"
	"smallpict-go/database"
	"smallpict-go/helper"

	"github.com/gin-gonic/gin"
)

type JSONUploadRequest struct {
	Image    string `json:"image"`
	Base64   string `json:"base64"`
	Data     string `json:"data"`
	Name     string `json:"name"`
	Filename string `json:"filename"`
}

func RegisterRoutes(r *gin.Engine, cfg *config.Config) {
	r.GET("/", func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{
			"status":  "ok",
			"service": cfg.AppName,
			"version": "1.0.0",
		})
	})

	r.GET("/health", func(c *gin.Context) {
		dbOk := database.CheckDB()
		status := http.StatusOK
		if !dbOk {
			status = http.StatusServiceUnavailable
		}

		dbStatus := "disconnected"
		st := "unhealthy"
		if dbOk {
			dbStatus = "connected"
			st = "ok"
		}

		c.JSON(status, gin.H{
			"status":    st,
			"database":  dbStatus,
			"timestamp": time.Now().UTC().Format(time.RFC3339),
		})
	})

	uploadHandler := func(c *gin.Context) {
		contentType := c.GetHeader("Content-Type")

		if contentType == "application/json" || c.ContentType() == "application/json" {
			var req JSONUploadRequest
			if err := c.ShouldBindJSON(&req); err != nil {
				c.JSON(http.StatusBadRequest, gin.H{
					"status":  "error",
					"message": "Invalid JSON request payload",
				})
				return
			}

			base64Str := req.Image
			if base64Str == "" {
				base64Str = req.Base64
			}
			if base64Str == "" {
				base64Str = req.Data
			}

			if base64Str == "" {
				c.JSON(http.StatusBadRequest, gin.H{
					"status":  "error",
					"message": "Field 'image' or 'base64' is required",
				})
				return
			}

			targetFilename := req.Name
			if targetFilename == "" {
				targetFilename = req.Filename
			}

			imgRecord, err := helper.ProcessUpload(nil, base64Str, targetFilename)
			if err != nil {
				c.JSON(http.StatusBadRequest, gin.H{
					"status":  "error",
					"message": err.Error(),
				})
				return
			}

			c.JSON(http.StatusOK, gin.H{
				"status":  "success",
				"message": "Image uploaded successfully",
				"data":    imgRecord,
			})
			return
		}

		file, _ := c.FormFile("file")
		if file == nil {
			file, _ = c.FormFile("image")
		}

		customName := c.PostForm("name")
		if customName == "" {
			customName = c.PostForm("filename")
		}

		base64Field := c.PostForm("base64")
		if base64Field == "" {
			base64Field = c.PostForm("image")
		}

		if file == nil && base64Field == "" {
			c.JSON(http.StatusBadRequest, gin.H{
				"status":  "error",
				"message": "No file or base64 image provided in request",
			})
			return
		}

		imgRecord, err := helper.ProcessUpload(file, base64Field, customName)
		if err != nil {
			c.JSON(http.StatusBadRequest, gin.H{
				"status":  "error",
				"message": err.Error(),
			})
			return
		}

		c.JSON(http.StatusOK, gin.H{
			"status":  "success",
			"message": "Image uploaded successfully",
			"data":    imgRecord,
		})
	}

	r.POST("/upload", uploadHandler)
}
