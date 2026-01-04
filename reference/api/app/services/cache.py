"""
GeoTruth API - Redis Cache Service

Redis connection with async support.
"""

import logging
from typing import Optional, Any
import json

import redis.asyncio as redis

from ..config import settings

logger = logging.getLogger(__name__)

# Redis connection pool
_redis_pool: Optional[redis.Redis] = None


async def get_redis() -> redis.Redis:
    """Get Redis connection from pool."""
    global _redis_pool
    if _redis_pool is None:
        _redis_pool = redis.from_url(
            settings.REDIS_URL,
            encoding="utf-8",
            decode_responses=True,
        )
    return _redis_pool


async def check_redis_connection() -> bool:
    """Check if Redis is reachable."""
    try:
        client = await get_redis()
        await client.ping()
        return True
    except Exception as e:
        logger.warning(f"Redis connection check failed: {e}")
        return False


async def close_redis() -> None:
    """Close Redis connections."""
    global _redis_pool
    if _redis_pool is not None:
        await _redis_pool.close()
        _redis_pool = None
        logger.info("Redis connections closed")


class Cache:
    """Cache service with namespace support."""
    
    def __init__(self, namespace: str = "geotruth"):
        self.namespace = namespace
        self.default_ttl = 3600  # 1 hour
    
    def _key(self, key: str) -> str:
        """Generate namespaced key."""
        return f"{self.namespace}:{key}"
    
    async def get(self, key: str) -> Optional[Any]:
        """Get value from cache."""
        try:
            client = await get_redis()
            value = await client.get(self._key(key))
            if value:
                return json.loads(value)
            return None
        except Exception as e:
            logger.warning(f"Cache get failed for {key}: {e}")
            return None
    
    async def set(
        self, 
        key: str, 
        value: Any, 
        ttl: Optional[int] = None
    ) -> bool:
        """Set value in cache."""
        try:
            client = await get_redis()
            await client.set(
                self._key(key),
                json.dumps(value),
                ex=ttl or self.default_ttl
            )
            return True
        except Exception as e:
            logger.warning(f"Cache set failed for {key}: {e}")
            return False
    
    async def delete(self, key: str) -> bool:
        """Delete value from cache."""
        try:
            client = await get_redis()
            await client.delete(self._key(key))
            return True
        except Exception as e:
            logger.warning(f"Cache delete failed for {key}: {e}")
            return False
    
    async def exists(self, key: str) -> bool:
        """Check if key exists in cache."""
        try:
            client = await get_redis()
            return await client.exists(self._key(key)) > 0
        except Exception as e:
            logger.warning(f"Cache exists check failed for {key}: {e}")
            return False


# Default cache instance
cache = Cache()
