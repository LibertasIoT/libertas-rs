# Libertas App Rust Runtime

The core dependency of Libertas IoT App.


App authors and coding agents: start with the [CodeBot guide](crates/libertas/CODEBOT.md).
The SDK includes the complete linked public documentation for offline use.
Generate API documentation from the SDK source with `cargo doc`.

Maintainers: run `node scripts/sync-docs.cjs` before preparing a push, review and
commit the generated changes, then run it with `--check`. Set
`LIBERTAS_SOURCE_ROOT` to the canonical Libertas source checkout and run
`node scripts/sync-docs.cjs --install-hook` once per checkout to enable the
automatic pre-push check. Existing pre-push hooks are preserved.
