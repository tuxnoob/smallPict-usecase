package com.smallpict.upload.controller;

import com.smallpict.upload.model.Image;
import com.smallpict.upload.model.UploadRequest;
import com.smallpict.upload.service.SmallPictService;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;
import org.springframework.web.multipart.MultipartFile;

import java.util.Map;

@RestController
public class UploadController {

    private final SmallPictService smallPictService;

    public UploadController(SmallPictService smallPictService) {
        this.smallPictService = smallPictService;
    }

    @PostMapping(value = {"/upload"}, consumes = {"multipart/form-data"})
    public ResponseEntity<?> handleMultipartUpload(
            @RequestParam(value = "file", required = false) MultipartFile file,
            @RequestParam(value = "image", required = false) MultipartFile image,
            @RequestParam(value = "name", required = false) String name,
            @RequestParam(value = "filename", required = false) String filename,
            @RequestParam(value = "base64", required = false) String base64
    ) {
        try {
            MultipartFile targetFile = file != null ? file : image;
            String targetFilename = name != null ? name : filename;

            Image record = smallPictService.processUpload(targetFile, base64, targetFilename);
            return ResponseEntity.ok(Map.of(
                    "status", "success",
                    "message", "Image uploaded successfully",
                    "data", record
            ));
        } catch (Exception e) {
            return ResponseEntity.badRequest().body(Map.of(
                    "status", "error",
                    "message", e.getMessage()
            ));
        }
    }

    @PostMapping(value = {"/upload"}, consumes = {"application/json"})
    public ResponseEntity<?> handleJsonUpload(@RequestBody UploadRequest request) {
        try {
            String base64Str = request.getImage() != null ? request.getImage()
                    : (request.getBase64() != null ? request.getBase64() : request.getData());
            String targetFilename = request.getName() != null ? request.getName() : request.getFilename();

            Image record = smallPictService.processUpload(null, base64Str, targetFilename);
            return ResponseEntity.ok(Map.of(
                    "status", "success",
                    "message", "Image uploaded successfully",
                    "data", record
            ));
        } catch (Exception e) {
            return ResponseEntity.badRequest().body(Map.of(
                    "status", "error",
                    "message", e.getMessage()
            ));
        }
    }
}
