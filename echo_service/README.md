# Echo Service
Service that listens to a unix socket and returns the received data.

## Localhost
```
RUST_LOG=DEBUG cargo run
```

## Exec echo service
```
curl --unix-socket /tmp/echo.sock http:/localhost -d '{"msg": "test"}'
```

## Prometheus
```
curl http://127.0.0.1:6150
```
