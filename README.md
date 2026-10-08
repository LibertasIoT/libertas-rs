# Libertas App Rust Runtime

The core dependency of Libertas IoT App.


App authors and coding agents: start with the
[CodeBot guide](https://github.com/LibertasIoT/libertas-doc/blob/main/CODEBOT.md).
Public guides and contracts live only in
[libertas-doc](https://github.com/LibertasIoT/libertas-doc); clone that repository
for offline documentation or select a specific documentation commit.
Generate API documentation from the SDK source with `cargo doc`.

Maintainers: run `node scripts/sync-docs.cjs` to update the three documentation
links, review and commit any changes, then run it with `--check`. Run it with
`--install-hook` once per checkout to check outgoing commits. The check needs
Node.js and Git; it does not need the documentation authoring checkout.
Existing pre-push hooks are preserved. SDK code retains its MIT license;
the canonical documentation states its CC BY 4.0 license.
