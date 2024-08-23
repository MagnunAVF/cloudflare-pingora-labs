# Load Balancer
Simple load balancer implementation.

## Localhost
Run in terminal:
```
RUST_BACKTRACE=1 RUST_LOG=DEBUG cargo run
```

Then:
```
curl 127.0.0.1:6188 -svo /dev/null
```