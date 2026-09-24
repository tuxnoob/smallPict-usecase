package com.smallpict.upload.model;

import jakarta.persistence.*;
import org.hibernate.annotations.CreationTimestamp;

import java.time.LocalDateTime;

@Entity
@Table(name = "images", schema = "public")
public class Image {

    @Id
    @Column(name = "id", length = 50, nullable = false)
    private String id;

    @Column(name = "filename", length = 255)
    private String filename;

    @Column(name = "url", columnDefinition = "TEXT")
    private String url;

    @Column(name = "size")
    private Long size;

    @Column(name = "size_origin")
    private Long sizeOrigin;

    @Column(name = "mime_type", length = 100)
    private String mimeType;

    @CreationTimestamp
    @Column(name = "created_at", updatable = false)
    private LocalDateTime createdAt;

    public Image() {
    }

    public Image(String id, String filename, String url, Long size, Long sizeOrigin, String mimeType, LocalDateTime createdAt) {
        this.id = id;
        this.filename = filename;
        this.url = url;
        this.size = size;
        this.sizeOrigin = sizeOrigin;
        this.mimeType = mimeType;
        this.createdAt = createdAt;
    }

    public static ImageBuilder builder() {
        return new ImageBuilder();
    }

    public String getId() {
        return id;
    }

    public void setId(String id) {
        this.id = id;
    }

    public String getFilename() {
        return filename;
    }

    public void setFilename(String filename) {
        this.filename = filename;
    }

    public String getUrl() {
        return url;
    }

    public void setUrl(String url) {
        this.url = url;
    }

    public Long getSize() {
        return size;
    }

    public void setSize(Long size) {
        this.size = size;
    }

    public Long getSizeOrigin() {
        return sizeOrigin;
    }

    public void setSizeOrigin(Long sizeOrigin) {
        this.sizeOrigin = sizeOrigin;
    }

    public String getMimeType() {
        return mimeType;
    }

    public void setMimeType(String mimeType) {
        this.mimeType = mimeType;
    }

    public LocalDateTime getCreatedAt() {
        return createdAt;
    }

    public void setCreatedAt(LocalDateTime createdAt) {
        this.createdAt = createdAt;
    }

    public static class ImageBuilder {
        private String id;
        private String filename;
        private String url;
        private Long size;
        private Long sizeOrigin;
        private String mimeType;
        private LocalDateTime createdAt;

        public ImageBuilder id(String id) {
            this.id = id;
            return this;
        }

        public ImageBuilder filename(String filename) {
            this.filename = filename;
            return this;
        }

        public ImageBuilder url(String url) {
            this.url = url;
            return this;
        }

        public ImageBuilder size(Long size) {
            this.size = size;
            return this;
        }

        public ImageBuilder sizeOrigin(Long sizeOrigin) {
            this.sizeOrigin = sizeOrigin;
            return this;
        }

        public ImageBuilder mimeType(String mimeType) {
            this.mimeType = mimeType;
            return this;
        }

        public ImageBuilder createdAt(LocalDateTime createdAt) {
            this.createdAt = createdAt;
            return this;
        }

        public Image build() {
            return new Image(id, filename, url, size, sizeOrigin, mimeType, createdAt);
        }
    }
}
