#!/usr/bin/env bash
# =============================================================================
# GeoTruth Narrative Engine - Stop Script
# =============================================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "Stopping GeoTruth services..."

cd backend
docker compose -f docker-compose.yml -f docker-compose.dev.yml down
cd ..

echo "✓ All services stopped"
