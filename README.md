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

To run formatting use
```
rustfmt --edition 2021 $(find src/ -name "*.rs")
```
or alternatively
```
rustfmt --edition 2021 src/**/*.rs
```
This variant may require you to run
```
shopt -s globstar
```
beforehand.

## Docker

### Build
```
docker build -t agaross.azurecr.io/agar-oss/agartex-resource-management .
```

