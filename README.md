# agartex-resource-management

Resource Management Service

## Runbook

To run locally from repository root use (This requires that postgres is running.)

```
cargo run
```

To run tests use
```
cargo test
```

To run linting use
```
cargo clippy --all-targets --all-features --fix -- -D warnings
```

## Docker

### Build
```
docker build -t agaross.azurecr.io/agar-oss/agartex-resource-management .
```
