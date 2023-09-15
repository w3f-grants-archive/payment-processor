# Oracle Core Primitives

This crate contains common types and traits used by both oracle and API to decouple them from each other. 

## Migrations

This crate also contains migrations and types for PostgreSQL connecton. When you run the oracle for the first time, it will create tables in the database.

## How to run tests

This crate contains some unit tests for models and helper functions:

```bash
cargo test -p op-core
```
