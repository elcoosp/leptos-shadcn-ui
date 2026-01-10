// Leptos ShadCN UI - Main Go Server
//
// This is a simple Go web server that can be used for development
// and testing alongside the Leptos frontend application.

package main

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"time"
)

// Response represents a standard API response structure
type Response struct {
	Status    string      `json:"status"`
	Timestamp string      `json:"timestamp"`
	Data      interface{} `json:"data,omitempty"`
	Error     string      `json:"error,omitempty"`
}

// ComponentInfo represents metadata about a component
type ComponentInfo struct {
	Name        string   `json:"name"`
	Version     string   `json:"version"`
	Category    string   `json:"category"`
	Description string   `json:"description"`
	Tags        []string `json:"tags"`
}

var components = []ComponentInfo{
	{Name: "button", Version: "0.4.0", Category: "form", Description: "Button component with variants", Tags: []string{"form", "input"}},
	{Name: "input", Version: "0.4.0", Category: "form", Description: "Input component for text entry", Tags: []string{"form", "input"}},
	{Name: "card", Version: "0.4.0", Category: "layout", Description: "Card container component", Tags: []string{"layout"}},
	{Name: "dialog", Version: "0.4.0", Category: "overlay", Description: "Dialog/modal component", Tags: []string{"overlay"}},
}

func main() {
	mux := http.NewServeMux()

	// Health check endpoint
	mux.HandleFunc("/health", healthHandler)

	// API endpoints
	mux.HandleFunc("/api/components", componentsHandler)
	mux.HandleFunc("/api/components/", componentDetailHandler)

	// Serve static files (for the Leptos frontend)
	mux.HandleFunc("/", staticFileHandler)

	addr := ":8080"
	server := &http.Server{
		Addr:         addr,
		Handler:      mux,
		ReadTimeout:  15 * time.Second,
		WriteTimeout: 15 * time.Second,
		IdleTimeout:  60 * time.Second,
	}

	log.Printf("Server starting on %s", addr)
	log.Printf("Leptos ShadCN UI Development Server")
	log.Printf("Available endpoints:")
	log.Printf("  - GET  /health                  Health check")
	log.Printf("  - GET  /api/components          List all components")
	log.Printf("  - GET  /api/components/{name}   Get component details")
	log.Printf("  - GET  /                        Static files")

	if err := server.ListenAndServe(); err != nil && err != http.ErrServerClosed {
		log.Fatalf("Server failed to start: %v", err)
	}
}

func healthHandler(w http.ResponseWriter, r *http.Request) {
	sendJSON(w, Response{
		Status:    "ok",
		Timestamp: time.Now().Format(time.RFC3339),
		Data:      map[string]string{"service": "leptos-shadcn-ui"},
	})
}

func componentsHandler(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		sendError(w, http.StatusMethodNotAllowed, "Method not allowed")
		return
	}

	sendJSON(w, Response{
		Status:    "ok",
		Timestamp: time.Now().Format(time.RFC3339),
		Data:      components,
	})
}

func componentDetailHandler(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		sendError(w, http.StatusMethodNotAllowed, "Method not allowed")
		return
	}

	// Extract component name from URL path
	// URL format: /api/components/{name}
	name := r.URL.Path[len("/api/components/"):]
	if name == "" {
		sendError(w, http.StatusBadRequest, "Component name required")
		return
	}

	for _, comp := range components {
		if comp.Name == name {
			sendJSON(w, Response{
				Status:    "ok",
				Timestamp: time.Now().Format(time.RFC3339),
				Data:      comp,
			})
			return
		}
	}

	sendError(w, http.StatusNotFound, fmt.Sprintf("Component '%s' not found", name))
}

func staticFileHandler(w http.ResponseWriter, r *http.Request) {
	if r.URL.Path != "/" {
		http.NotFound(w, r)
		return
	}

	w.Header().Set("Content-Type", "text/html; charset=utf-8")
	fmt.Fprintln(w, `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Leptos ShadCN UI</title>
    <style>
        body { font-family: system-ui, sans-serif; max-width: 800px; margin: 50px auto; padding: 20px; line-height: 1.6; }
        h1 { color: #333; }
        .badge { background: #e0e7ff; color: #3730a3; padding: 4px 8px; border-radius: 4px; font-size: 0.875em; }
        pre { background: #f5f5f5; padding: 16px; border-radius: 8px; overflow-x: auto; }
        code { background: #f5f5f5; padding: 2px 6px; border-radius: 4px; }
    </style>
</head>
<body>
    <h1>🚀 Leptos ShadCN UI</h1>
    <p><span class="badge">Development Server</span></p>
    <p>Welcome to the Leptos ShadCN UI development server. This Go server provides API endpoints for component information and serves as a backend for the Leptos frontend application.</p>

    <h2>Available Endpoints</h2>
    <ul>
        <li><code>GET /health</code> - Health check</li>
        <li><code>GET /api/components</code> - List all components</li>
        <li><code>GET /api/components/{name}</code> - Get component details</li>
    </ul>

    <h2>Quick Test</h2>
    <pre>curl http://localhost:8080/api/components</pre>

    <h2>Project Info</h2>
    <p>Leptos ShadCN UI is a comprehensive component library that brings beautiful, accessible, and customizable ShadCN UI components to the Leptos ecosystem.</p>
    <p>For more information, visit the <a href="https://github.com/cloud-shuttle/leptos-shadcn-ui">GitHub repository</a>.</p>
</body>
</html>`)
}

func sendJSON(w http.ResponseWriter, data interface{}) {
	w.Header().Set("Content-Type", "application/json")
	if err := json.NewEncoder(w).Encode(data); err != nil {
		log.Printf("Failed to encode JSON: %v", err)
	}
}

func sendError(w http.ResponseWriter, status int, message string) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	json.NewEncoder(w).Encode(Response{
		Status:    "error",
		Timestamp: time.Now().Format(time.RFC3339),
		Error:     message,
	})
}
