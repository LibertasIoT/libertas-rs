# Delivery quick checklist

Before deployment:

- run `cargo check --lib` or the project's TypeScript typecheck/build;
- save source, resolve local dependencies, and run Studio schema validation;
- review every schema-visible node, permission, access host, and localized
  reason; and
- generate Matter descriptors with Studio rather than encoding them manually.

For App authoring, never hand-edit generated schema/document JSON, `AppDependencies`,
`DistroDependencies`, target manifests, or release versions. Studio resolves
the effective dependency graph, deploys changed local dependencies before
dependents, and records deployed versions. Declare Rust native build packages
only through the supported
[Cargo metadata](../reference/languages/rust.md#declaring-native-build-packages).

For a translation task, write only the designated handoff output and preserve
all structure and message arguments required by the
[translation contract](../reference/translation.md). The
[workflow guide](../guides/translation.md) owns locale choices, immediate import
persistence, review, and removal; do not defer a Hub-connected import until OK.

Deployment installs a target on a Hub. Plaza publication synchronizes the
package's desired multi-target state: changing an existing target binary
or App version, or changing the common schema/dependency lock, creates a new
release. A missing target at or below the publication's greatest-ever exact version
can be added to that release; a higher target version creates a new release.
Withdrawal and document-only updates preserve immutable runtime bindings. App source chooses neither tag nor
public version. Follow the current [Studio manual](../guides/studio.md) and
[Plaza publication contract](../reference/publication.md#app-publication-and-version-identity),
then report which checks, deployments, or publication remain.
