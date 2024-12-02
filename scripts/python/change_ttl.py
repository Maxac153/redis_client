import sys

from src.redis_client import RedisClient


if __name__ == "__main__":
    if len(sys.argv) > 1:
        app_host, app_port, ttl = sys.argv[1:]
        dump = RedisClient(app_host, app_port)
        dump.change_ttl(ttl)
        dump.close()
        