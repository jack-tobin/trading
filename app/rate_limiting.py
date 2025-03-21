from functools import wraps

from typing import Callable
import redis
from flask import request, jsonify


class RateLimiting:
    def __init__(self, url: str = "redis://localhost:6379/0"):
        self.redis = redis.from_url(url)

    def limit_backtests(self, max_requests: int = 10, period: int = 3600):
        def decorator(func: Callable) -> Callable:
            @wraps(func)
            def wrapped(*args, **kwargs):
                client_id = request.remote_addr
                key = f"rate_limit:{client_id}"

                # current count
                count = self.redis.get(key)
                count = int(count) if count else 0

                if count >= max_requests:
                    return jsonify({"error": "Rate limit exceeded. Try again later."}, 429)

                pipe = self.redis.pipeline()
                pipe.incr(key)
                if count == 0:
                    pipe.expire(key, period)
                pipe.execute()

                return func(*args, **kwargs)
            return wrapped
        return decorator



