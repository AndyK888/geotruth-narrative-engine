"""
GeoTruth API - Database Connection

Async PostgreSQL connection with PostGIS support.
"""

import logging
from contextlib import asynccontextmanager
from typing import AsyncGenerator

from sqlalchemy import text
from sqlalchemy.ext.asyncio import AsyncSession, create_async_engine, async_sessionmaker
from sqlalchemy.orm import declarative_base

from ..config import settings

logger = logging.getLogger(__name__)

# Create async engine
engine = create_async_engine(
    settings.database_url,
    echo=settings.LOG_LEVEL == "DEBUG",
    pool_size=10,
    max_overflow=20,
    pool_pre_ping=True,
)

# Session factory
async_session_factory = async_sessionmaker(
    engine,
    class_=AsyncSession,
    expire_on_commit=False,
)

# Base class for ORM models
Base = declarative_base()


@asynccontextmanager
async def get_db_session() -> AsyncGenerator[AsyncSession, None]:
    """Get database session as async context manager."""
    session = async_session_factory()
    try:
        yield session
        await session.commit()
    except Exception:
        await session.rollback()
        raise
    finally:
        await session.close()


async def check_database_connection() -> bool:
    """Check if database is reachable."""
    try:
        async with engine.connect() as conn:
            await conn.execute(text("SELECT 1"))
            return True
    except Exception as e:
        logger.warning(f"Database connection check failed: {e}")
        return False


async def init_database() -> None:
    """Initialize database tables."""
    # Tables are created by init-scripts in Docker
    # This is for future migrations
    logger.info("Database initialization complete")


async def close_database() -> None:
    """Close database connections."""
    await engine.dispose()
    logger.info("Database connections closed")
