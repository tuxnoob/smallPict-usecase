package com.smallpict.upload.controller;

import org.springframework.beans.factory.annotation.Value;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.Map;

@RestController
public class BaseController {

    @Value("${spring.application.name:SmallPict Java Service}")
    private String appName;

    @GetMapping("/")
    public ResponseEntity<?> getBaseStatus() {
        return ResponseEntity.ok(Map.of(
                "status", "ok",
                "service", appName,
                "version", "1.0.0"
        ));
    }
}
