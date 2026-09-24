package model

import "time"

type Image struct {
	ID         string    `gorm:"column:id;primaryKey;size:50" json:"id"`
	Filename   *string   `gorm:"column:filename;size:255" json:"filename"`
	URL        *string   `gorm:"column:url;type:text" json:"url"`
	Size       *int64    `gorm:"column:size" json:"size"`
	SizeOrigin *int64    `gorm:"column:size_origin" json:"size_origin"`
	MIMEType   *string   `gorm:"column:mime_type;size:100" json:"mime_type"`
	CreatedAt  time.Time `gorm:"column:created_at;autoCreateTime" json:"created_at"`
}

func (Image) TableName() string {
	return "public.images"
}
