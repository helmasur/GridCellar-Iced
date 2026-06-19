# GridCellar Iced implementation

This directory contains the Iced implementation of the product defined in the repository root.

## Governing documents

- `../blueprint.md` defines the product goal and MVP boundary.
- `../model-contract.md` defines the technology-neutral model contract.
- `../acceptance-tests.md` defines functional verification.
- `../plan.md` defines implementation order and work packages.
- `../implementation-status.md` records overall status, blockers, and functional deviations.

## Current scope

The application is a minimal executable foundation without product features, a domain model, or persistence.

## Commands

Run:

```sh
cargo run --manifest-path implementation/Cargo.toml
```

Check:

```sh
cargo check --manifest-path implementation/Cargo.toml
```

Build:

```sh
cargo build --manifest-path implementation/Cargo.toml
```

Format:

```sh
cargo fmt --manifest-path implementation/Cargo.toml --check
```

Lint:

```sh
cargo clippy --manifest-path implementation/Cargo.toml -- -D warnings
```

## Known limitations

- No product functionality is implemented.
- Persistence, export format, and storage architecture are not decided by this foundation.
- Functional deviations from `../blueprint.md` must be recorded in `../implementation-status.md` and resolved or explicitly decided before the MVP can be considered complete.
