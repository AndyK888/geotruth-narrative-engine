# =============================================================================
# Stage 1: Builder
# =============================================================================
FROM python:3.12-slim AS builder

WORKDIR /build

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Python dependencies
COPY requirements.txt .
RUN pip wheel --no-cache-dir --wheel-dir /build/wheels -r requirements.txt

# =============================================================================
# Stage 2: Runtime
# =============================================================================
FROM python:3.12-slim AS runtime

# Labels for container identification
LABEL org.opencontainers.image.title="GeoTruth API"
LABEL org.opencontainers.image.description="Geospatial intelligence API for GeoTruth"
LABEL org.opencontainers.image.version="0.1.0"

WORKDIR /app

# Install runtime dependencies only
RUN apt-get update && apt-get install -y --no-install-recommends \
    libpq5 \
    libgdal-dev \
    libgeos-dev \
    libproj-dev \
    curl \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Install Python packages from wheels
COPY --from=builder /build/wheels /wheels
COPY --from=builder /build/requirements.txt .
RUN pip install --no-cache-dir --no-index /wheels/* \
    && rm -rf /wheels

# Copy application code
COPY ./app ./app

# Create non-root user for security
RUN groupadd -r geotruth && useradd -r -g geotruth geotruth \
    && chown -R geotruth:geotruth /app
USER geotruth

# Environment
ENV PYTHONUNBUFFERED=1 \
    PYTHONDONTWRITEBYTECODE=1 \
    PYTHONPATH=/app

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=40s --retries=3 \
    CMD curl -f http://localhost:8000/v1/health || exit 1

# Run with production server
CMD ["uvicorn", "app.main:app", "--host", "0.0.0.0", "--port", "8000", "--workers", "4"]
