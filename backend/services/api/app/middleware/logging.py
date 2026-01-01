"""
GeoTruth API - Logging Middleware

Request/response logging with correlation IDs for distributed tracing.
"""

import time
import uuid
import logging
from fastapi import Request
from starlette.middleware.base import BaseHTTPMiddleware

from ..logging_config import correlation_id_var

logger = logging.getLogger(__name__)


class LoggingMiddleware(BaseHTTPMiddleware):
    """Middleware for request/response logging with correlation IDs."""
    
    async def dispatch(self, request: Request, call_next):
        # Generate or extract correlation ID
        correlation_id = request.headers.get("X-Correlation-ID", str(uuid.uuid4()))
        correlation_id_var.set(correlation_id)
        
        # Log request
        start_time = time.perf_counter()
        logger.info(
            "Request started",
            extra={
                "context": {
                    "method": request.method,
                    "path": request.url.path,
                    "client_ip": request.client.host if request.client else None,
                    "user_agent": request.headers.get("user-agent"),
                }
            }
        )
        
        try:
            response = await call_next(request)
            duration_ms = (time.perf_counter() - start_time) * 1000
            
            # Log response
            logger.info(
                "Request completed",
                extra={
                    "context": {
                        "method": request.method,
                        "path": request.url.path,
                        "status_code": response.status_code,
                        "duration_ms": round(duration_ms, 2),
                    }
                }
            )
            
            # Add correlation ID to response headers
            response.headers["X-Correlation-ID"] = correlation_id
            return response
            
        except Exception as e:
            duration_ms = (time.perf_counter() - start_time) * 1000
            logger.exception(
                "Request failed",
                extra={
                    "context": {
                        "method": request.method,
                        "path": request.url.path,
                        "duration_ms": round(duration_ms, 2),
                        "error": str(e),
                    }
                }
            )
            raise
