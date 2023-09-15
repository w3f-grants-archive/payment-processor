# PCIDSS Compliant Oracle Gateway

Workspace contains three crates:

- `pcidss-oracle` - the [oracle](./oracle/README.md) itself
- `op-api` - Oracle Provider [API](./api/README.md)
- `op-core` - Common [primitive core types](./core/README.md) and traits used by both oracle and API

Workspace is written in a decoupled way, so that it is possible to add more services in the future.

Please refer to crate level READMEs for more information.
