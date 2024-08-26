# Cache API
Pingora Cache API example.

## Localhost
```
RUST_LOG=DEBUG cargo run
```

## Cache operations
PUT operation:
```
curl --unix-socket /tmp/cache_api.sock http:/localhost -d '{"operation": "put", "key": "xptz", "data": "my content!"}'
```

GET operation:
```
curl --unix-socket /tmp/cache_api.sock http:/localhost -d '{"operation": "get", "key": "xptz"}'
```

## Prometheus
```
curl http://127.0.0.1:6150
```