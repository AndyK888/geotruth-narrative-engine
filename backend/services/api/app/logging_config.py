"""
GeoTruth API - Logging Configuration

Structured JSON logging with correlation IDs for request tracing.
"""

import logging
import sys
import json
from datetime import datetime, timezone
from contextvars import ContextVar

# Context variable for correlation ID
correlation_id_var: ContextVar[str] = ContextVar('correlation_id', default='')


class StructuredFormatter(logging.Formatter):
    """JSON structured log formatter."""
    
    def format(self, record: logging.LogRecord) -> str:
        log_entry = {
            "timestamp": datetime.now(timezone.utc).isoformat(),
            "level": record.levelname,
            "service": "api",
            "logger": record.name,
            "message": record.getMessage(),
            "correlation_id": correlation_id_var.get() or None,
        }
        
        # Add extra fields
        if hasattr(record, 'context'):
            log_entry["context"] = record.context
        
        # Add exception info if present
        if record.exc_info:
            log_entry["exception"] = self.formatException(record.exc_info)
        
        # Add source location for debugging
        log_entry["source"] = {
            "file": record.pathname,
            "line": record.lineno,
            "function": record.funcName
        }
        
        return json.dumps(log_entry)


class PrettyFormatter(logging.Formatter):
    """Human-readable formatter for development."""
    
    COLORS = {
        'DEBUG': '\033[36m',    # Cyan
        'INFO': '\033[32m',     # Green
        'WARNING': '\033[33m',  # Yellow
        'ERROR': '\033[31m',    # Red
        'CRITICAL': '\033[35m', # Magenta
    }
    RESET = '\033[0m'
    
    def format(self, record: logging.LogRecord) -> str:
        color = self.COLORS.get(record.levelname, self.RESET)
        correlation = correlation_id_var.get()
        corr_str = f"[{correlation[:8]}] " if correlation else ""
        
        return (
            f"{color}{record.levelname:8}{self.RESET} "
            f"{corr_str}"
            f"{record.getMessage()}"
        )


def setup_logging(log_level: str = "INFO", log_format: str = "json") -> logging.Logger:
    """Configure structured logging for the application."""
    
    # Create formatter based on environment
    if log_format == "json":
        formatter = StructuredFormatter()
    else:
        formatter = PrettyFormatter()
    
    # Configure root logger
    root_logger = logging.getLogger()
    root_logger.setLevel(getattr(logging, log_level.upper()))
    
    # Remove existing handlers
    for handler in root_logger.handlers[:]:
        root_logger.removeHandler(handler)
    
    # Add stdout handler
    handler = logging.StreamHandler(sys.stdout)
    handler.setFormatter(formatter)
    root_logger.addHandler(handler)
    
    # Reduce noise from third-party libraries
    logging.getLogger("uvicorn.access").setLevel(logging.WARNING)
    logging.getLogger("httpx").setLevel(logging.WARNING)
    logging.getLogger("httpcore").setLevel(logging.WARNING)
    
    return root_logger
