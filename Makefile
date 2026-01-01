# =============================================================================
# GeoTruth Makefile - Development Commands
# =============================================================================

.PHONY: help dev dev-full dev-backend dev-desktop stop clean logs test lint format

# Default target
help:
	@echo "GeoTruth Narrative Engine - Development Commands"
	@echo ""
	@echo "Usage: make [target]"
	@echo ""
	@echo "Development:"
	@echo "  dev          Start full-stack development (API + Desktop)"
	@echo "  dev-full     Start full-stack with Valhalla (requires 8GB+ RAM)"
	@echo "  dev-backend  Start backend services only"
	@echo "  dev-desktop  Start desktop development only (requires backend)"
	@echo "  stop         Stop all services"
	@echo "  logs         View logs from all services"
	@echo ""
	@echo "Testing:"
	@echo "  test         Run all tests"
	@echo "  test-api     Run API tests"
	@echo "  test-desktop Run desktop tests"
	@echo ""
	@echo "Code Quality:"
	@echo "  lint         Run linters on all code"
	@echo "  format       Format all code"
	@echo ""
	@echo "Maintenance:"
	@echo "  clean        Remove all containers and volumes"
	@echo "  rebuild      Rebuild all Docker images"

# =============================================================================
# Development
# =============================================================================

dev:
	docker compose -f docker-compose.dev.yml up -d
	@echo ""
	@echo "✅ GeoTruth development environment started!"
	@echo ""
	@echo "  Desktop:    http://localhost:5173"
	@echo "  API Docs:   http://localhost:8000/docs"
	@echo "  API Health: http://localhost:8000/v1/health"
	@echo ""
	@echo "Run 'make logs' to view logs"

dev-full:
	docker compose -f docker-compose.dev.yml --profile full up -d
	@echo ""
	@echo "✅ Full development environment started (including Valhalla)"

dev-backend:
	cd backend && docker compose -f docker-compose.yml -f docker-compose.dev.yml up -d
	@echo ""
	@echo "✅ Backend services started!"

dev-desktop:
	cd desktop && pnpm dev

stop:
	docker compose -f docker-compose.dev.yml down
	cd backend && docker compose down || true
	@echo "✅ All services stopped"

logs:
	docker compose -f docker-compose.dev.yml logs -f

logs-api:
	docker compose -f docker-compose.dev.yml logs -f api

# =============================================================================
# Testing
# =============================================================================

test: test-api test-desktop

test-api:
	docker compose -f docker-compose.dev.yml exec api pytest tests/ -v

test-desktop:
	cd desktop && pnpm test

# =============================================================================
# Code Quality
# =============================================================================

lint: lint-api lint-desktop

lint-api:
	docker compose -f docker-compose.dev.yml exec api ruff check app/
	docker compose -f docker-compose.dev.yml exec api ruff format --check app/

lint-desktop:
	cd desktop && pnpm lint

format: format-api format-desktop

format-api:
	docker compose -f docker-compose.dev.yml exec api ruff format app/

format-desktop:
	cd desktop && pnpm format

# =============================================================================
# Maintenance  
# =============================================================================

clean:
	docker compose -f docker-compose.dev.yml down -v --remove-orphans
	cd backend && docker compose down -v --remove-orphans || true
	@echo "✅ All containers and volumes removed"

rebuild:
	docker compose -f docker-compose.dev.yml build --no-cache
	@echo "✅ All images rebuilt"

# =============================================================================
# Database
# =============================================================================

db-shell:
	docker compose -f docker-compose.dev.yml exec geo-db psql -U geotruth -d geotruth

db-migrate:
	@echo "Database migrations will be added in Week 2"
