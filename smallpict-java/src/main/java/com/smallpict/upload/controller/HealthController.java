package com.smallpict.upload.controller;

import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

import java.time.Instant;
import java.util.Map;

@RestController
public class HealthController {

    private final JdbcTemplate jdbcTemplate;

    public HealthController(JdbcTemplate jdbcTemplate) {
        this.jdbcTemplate = jdbcTemplate;
    }

    @GetMapping("/health")
    public ResponseEntity<?> checkHealth() {
        boolean dbOk = false;
        try {
            jdbcTemplate.execute("SELECT 1");
            dbOk = true;
        } catch (Exception ignored) {}

        HttpStatus status = dbOk ? HttpStatus.OK : HttpStatus.SERVICE_UNAVAILABLE;

        return ResponseEntity.status(status).body(Map.of(
                "status", dbOk ? "ok" : "unhealthy",
                "database", dbOk ? "connected" : "disconnected",
                "timestamp", Instant.now().toString()
        ));
    }
}
