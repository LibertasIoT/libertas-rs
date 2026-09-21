# Deployment dependency contract


The language parsers cannot assign dependency versions. In particular, the new
version of a local dependency in the same deployment batch is unknown until that
dependency has successfully deployed. Studio therefore constructs
`AppDependencies` just in time for each package:

1. It discovers direct package references from every parser FQN in a reference
   `Type`, heritage, `Bitflags`, `ForeignTypeSchema`, `FunctionDataSchema`, or
   `ProtocolSchema`, plus the package component of every Endpoint-client
   `Fixed=developer-package::function` constraint. Endpoint `Default` does not
   create a dependency. During queue planning—before any deployment begins—it
   recursively resolves local packages. For each remaining direct package,
   Studio locates the same package-manager source used by the language parser:
   TypeScript source or declarations in the nearest `node_modules`, or Rust
   source in Cargo's local cache resolved with offline metadata. It generates
   that package's schema as comparison evidence. A binary-only or incomplete
   cache is not sufficient and stops deployment. The deployment plan labels
   these as **cache dependencies**, distinctly from deployable **local
   dependencies** and exact **Plaza transitive dependencies**.
2. Studio reads the selected Hub's exact Plaza distribution/architecture keys,
   searches Plaza for each direct package's latest target, downloads and verifies
   its schema and selected localized document, and compares the source-generated
   schema with the Plaza schema after removing deployment-owned dependency locks
   and revision metadata. The programming-language dependency version is never
   interpreted as a Plaza pin. A mismatch stops before Hub mutation.
3. Studio follows the selected Plaza schema's exact `AppDependencies` locks and
   recursively resolves every historical target in their transitive closure.
   Direct packages select latest; transitive packages select the exact recorded
   App version. Missing or withdrawn targets, conflicting pins, invalid assets,
   or cycles stop the entire preflight. Studio installs the resulting closure on
   the Hub in dependency-first order and verifies each exact schema/version
   read-back before submitting any local package. An already installed exact
   target is verified and left unchanged.
4. For a queued local dependency, it first deploys that package, reads its schema
   and newly assigned App version back from the Hub, and verifies that the schema
   digest is the one just submitted.
5. It validates the dependent against the complete exact resource set. The new
   schema records the version actually verified for each package; it never
   predicts a version, substitutes Plaza latest for an exact transitive pin, or
   copies an unverified language-package version.
6. It validates and sorts the complete list, then computes the dependent schema's
   revision and submits the dependent package.

If any deployment or read-back fails, Studio stops before submitting its
dependents. It never predicts the Hub version, and `0.0.0.0` is not a valid
dependency entry.

## Schema availability and task requirements

At runtime, `AppDependencies` is both the client's complete package-fetch list and
the Hub's preloaded authorization and exact-version lookup table. The Hub reads
the historical schema, documents, manifest, and code belonging to the pinned
version; it never substitutes the package's current version. Clients verify that
the returned App version equals the declared pin before assembling the graph.
Every modern producer and consumer uses the required case-sensitive `Version`
field. It is an exact identity, never a minimum or latest-version request.

This closure covers the complete published schema, including optional fields
and unselected Union alternatives. Its schemas and localized documents must
already be available in the Hub's type system so editors can resolve and display
every supported choice without interrupting editing for on-demand package
installation. Listing an App dependency does not by itself require a configured
service task. Required Endpoint services are determined separately by the
[task pre-creation gate](schema.md#62-task-pre-creation-static-dependency-check).


For the human workflow, see [deploying with Studio](../guides/studio.md#deploying-to-a-hub).
For publication identities, see [Plaza publication](publication.md).
