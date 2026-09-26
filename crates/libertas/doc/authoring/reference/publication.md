# Deployment targets and Plaza publication

Deploying installs an App on one Hub. Publishing makes verified deployed
artifacts publicly discoverable and downloadable for selected targets. Studio
publishes the existing artifacts; it does not build, redeploy, or invent an App
version. Follow the [Plaza guide](../guides/plaza.md) for the wizard and account
controls. This page owns the release and artifact contract.

## Account and publication authorization

These rules apply when the updated Plaza service and Studio builds are deployed
together; they do not establish the version running at a hosted service.

This security upgrade preserves existing accounts and publication history but
revokes older Plaza API tokens. Sign in again in Studio, refresh GitHub
repository authorization, and recreate any additional API tokens. Browser
sessions remain valid.

Private account reads require `account:read`; account changes, token creation,
revocation, and repository authorization require `account:write`. Publication
and withdrawal require `store:publish` on an active Plaza API token. Browser
sessions cannot publish or withdraw, and account disabling prevents access.

A token created using another token has only scopes held by its parent and
cannot expire later than its parent. Omitting an expiry still inherits a
parent's limit. Revoking or expiring any ancestor invalidates its descendants;
delegation supports at most eight generations. New delegated tokens require
their own publishing reauthentication.

Publication and withdrawal require GitHub authentication for the exact calling
Plaza token within the preceding 15 minutes. Signing into the website or using
a different Plaza token does not renew that authority. Publication additionally
requires public-repository write permission verified for that same token and
repository within 15 minutes. Repository authorization must use the GitHub
identity belonging to the signed-in Plaza account. Revocation or expiry of the
initiating Plaza token, or disabling its account, invalidates an unfinished
repository authorization and its pending credential handoff.

Studio refreshes GitHub repository authorization before publication and
withdrawal, preserving the explicit repository selection. Plaza rechecks a
publishing repository's stable identity and public visibility when verifying
release assets. External write-permission removal takes effect at the next
authorization refresh or when the existing 15-minute proof expires; it is not
an instantaneous GitHub revocation notification. Revoking a Plaza token stops
its Plaza authority; GitHub authorization is a separate credential and must be
revoked at GitHub if that access should also end.

## Package and target identity

A package is identified by `developer-package`. A target is one exact
`distribution`/`architecture` pair. One publication can contain several targets,
each retaining its own Hub-assigned exact `appVersion`. `TypesOnly=true`
publishes as `types_only`, with no executable; other packages publish as `app`.
There is no separate agent/automaton publication choice.

The publication project chooser lists local workspace packages that already
have a current App version on an accessible registered Hub, matching both
package identity and source language. This includes TypesOnly packages. A local
manifest alone does not make a package publishable. Studio checks deployed
targets and dependencies after selection and rechecks them before publication.
If no deployment can be confirmed because a Hub is unreachable, restore its
connection and retry; otherwise deploy the App first.

Logical App dependencies belong to the schema's exact
[`AppDependencies` lock](deployment.md). Native OS packages belong to the target
manifest. A language package constraint is neither of those identities.

An Endpoint client's `Default` names a recommended service function. It does
not add that service App to `AppDependencies` or to the caller's publication
batch. Deploy and publish the recommended service App separately, with a target
for each supported Hub platform. Publishing a TypesOnly protocol package makes
its types available; it does not publish a service that implements the protocol.
An Endpoint client's `Fixed` restricts the owning function and includes its
package in the dependency graph.

Installing the service App makes its functions available. A configured service
Task must then create a compatible Endpoint before the calling Task can use it;
publishing or installing an App does not create that Task.

## Preflight and document convergence

Studio reads the complete local dependency graph and all selected targets before
changing any Hub or publication. Every local package must belong to the signed-in
Plaza developer; remote dependencies without local source remain unchanged.
Duplicate targets, undeployed current source, cycles, conflicting exact pins,
unavailable retained dependencies, different canonical schemas/locks, or a
binary failing its recorded SHA-256 stop preflight. Targets may have different
App versions while sharing the same schema and exact dependency lock.

After successful preflight, Studio converges documents across selected Hubs in
dependency-first order. For each locale, and for the
[Display Template](#display-template-identity), the numerically greatest
four-part content revision wins; missing locales are filled. Equal content revisions with
different canonical content are an error. `DocumentVersion` is the JSON format
version and never chooses a winner. Each write is refetched and verified. An
unchanged Hub performs no write. A changed Hub advances its target-manifest
revision while preserving runtime identity and original deployment provenance.
If any write or verification fails, GitHub/Plaza publication does not start;
earlier successful document updates on Hubs remain.

Only after the entire graph converges are packages published dependency-first.
A failure or cancellation stops later packages; earlier published dependencies
remain published. Review the complete plan before confirming.

## Hub target manifest


After every successful deploy, or a document-metadata synchronization that
actually changes schema/document metadata, including the Display Template, the Hub writes a new immutable,
separately versioned target manifest and atomically advances the current pointer
for that language. An idempotent metadata request writes nothing. A
document-only revision
advances `targetVersion` but preserves `appVersion`, schema, artifact, language,
TypesOnly state, and distribution dependencies. The Hub retains historical
manifests, schemas, App manifests, localized
document mappings and files, and executable artifacts by exact App version. A
Hub stores only its own platform target. Studio retrieves an exact manifest and
installed artifact; publishing never rebuilds an App or allocates a replacement
version.

The JSON contract is:

```json
{
  "formatVersion": 1,
  "developerPackage": "developer-package",
  "developer": "developer",
  "package": "package",
  "language": "rs",
  "sourceRevision": "sha256:...",
  "targetVersion": "20260820.143025.417.0",
  "appVersion": "20260820.142959.003.0",
  "target": {
    "distribution": "resolute",
    "architecture": "arm64"
  },
  "typesOnly": false,
  "artifact": {
    "name": "libdeveloper__package.20260820.142959.003.0.so",
    "size": 123456,
    "sha256": "sha256:..."
  },
  "DistroDependencies": {
    "manager": "apt",
    "packages": ["libc6", "libgcc-s1"]
  },
  "schema": { "size": 1234, "sha256": "sha256:..." },
  "documents": { "count": 2, "sha256": "sha256:..." },
  "builtAtUnixMilliseconds": 1787241025417
}
```

`artifact` is `null` for TypesOnly. `documents.sha256` is the exact digest of
the content-addressed documents manifest referenced by that target revision, and
`documents.count` is its locale count. Because that manifest names the
[Display Template](#display-template-identity), its digest covers the template
too; the target manifest keeps this shape and the template is not a locale. The target manifest's APT dependency
names are deduplicated and sorted. They are per-target OS installation data and never
appear in the Libertas schema. This is distinct from the schema's logical
`AppDependencies` array of Libertas App packages. For native Rust, the compiling
Hub resolves linked shared libraries to the owning APT packages after its local
build.

Studio publishes each target-manifest revision under a content-addressed name,
`{distribution}-{architecture}.target.{sha256}.json`. It never overwrites the
prior asset. For one platform and App version, the greatest numeric
`targetVersion` is current; older manifest revisions remain available for
in-flight downloads and exact history.

`sourceRevision` in every target-manifest revision is immutable Hub deployment
provenance for that exact App/runtime binding. Document synchronization copies
it from the prior target manifest. It must not replace it with Studio's current
project or publication source revision. `builtAtUnixMilliseconds` is likewise
the original target build time. Document revisions and redeployments that reuse
the same source revision, schema, artifact, and OS dependencies preserve it.
A deployment with a changed source revision receives a new App version, even
when the compiled artifact is identical or the package is TypesOnly. Installing
an existing Plaza release preserves its published build time.

The manifest's exact `appVersion` is its install identity. Its schema digest
covers the canonical schema including the complete exact
`AppDependencies` lock. Consequently changing any dependency pin changes schema
identity, requires a new App version for every affected target carrying that
schema, and is an upgrade even when the target artifact bytes happen to be
identical. It is never classified as document-only synchronization.


## App publication and version identity

A runtime binding consists of the exact App version, canonical schema digest,
target pair, language, TypesOnly state, artifact digest, and normalized runtime
OS dependencies. Target version, source revision, build time, and documents do
not replace that identity. Each admitted binding is immutable; availability
and current metadata pointers may change.

The one public Plaza version starts as the numeric, component-wise maximum exact
`appVersion` in the reconciled creation target set. Thereafter it is the
greatest exact target App version ever admitted to that publication and never
decreases when a target is withdrawn. It is not an independently allocated
timestamp and it does not replace any target's exact version. For example,
creation bindings `20260820.142959.003.0` and `20260821.090000.010.0` produce
public version `20260821.090000.010.0`, while each installation retains its own
value. If a Hub later deploys content-equivalent bytes under a different
`appVersion`, that exact version remains a new immutable binding and follows the
normal new-publication and maximum-version rules.

Public versions are unique within a package and Plaza retains every version.
If the immutable schema or runtime content of an existing target changes but
the numeric target maximum does not advance beyond that package's greatest
published public version, Studio fails preflight before any Hub, GitHub, or
Plaza publication mutation. The affected target must be redeployed with an
exact version that makes the new maximum greater; other unchanged targets do
not need to be redeployed. A changed exact dependency lock is an immutable
schema change and follows this forced-upgrade rule.

A previously absent target whose exact App version is less than or equal to the
publication's greatest-ever version may be appended to that same publication as
a new immutable binding without changing the public version, marker, or tag. A
new target above that version creates a new publication whose public version is
the new maximum. Withdrawing any target, including the one that established the
maximum, changes only current availability and never lowers or reassigns the
public version. Re-adding a withdrawn target reuses its retained exact binding;
different content or version is not an in-place re-addition.

Before asking the developer to confirm, Studio reads the matching GitHub Release
with the newest publication time and Plaza history, then downloads and verifies
its marker, schema, current document manifest, locale assets, and Display Template, and complete immutable
target binding map for a current-format release. A recognized legacy release
(a format-1 marker or the older asset layout without a marker) is historical
input: Studio skips its obsolete asset verification and publishes a new
current-format release. The selected targets must still produce a public
version greater than every retained Plaza version, and existing exact target
bindings must remain unchanged. Studio preserves the legacy release and its
assets; it cannot synchronize documents into that release. With retained Plaza
history, a missing GitHub release still stops publication. Unrecognized or
malformed markers, failed current-format integrity checks, and GitHub access
errors also stop publication.

Reconciliation has these outcomes:

- identical immutable schema/runtime content reuses the publication, exact
  target bindings, public version, marker, and tag;
- a selected target with a different Hub `appVersion` never reuses or is
  rewritten to an older published binding, even when other content is equal;
- any schema change, including any exact `AppDependencies` pin change, or any
  real code/runtime change to an existing target creates a new publication
  after the strict public-version check above;
- a previously absent lower/equal-version target appends one immutable binding
  to the same publication, while a higher-version target creates the next
  publication; withdrawal changes latest availability but retains its binding,
  assets, and greatest-ever public version;
- localized document, source-comment, function-comment, and Display Template
  changes update only
  the current content-addressed document locator, search presentation, and one
  new content-addressed target-manifest revision per affected target for the
  same publication; each `targetVersion` advances, while no target App version,
  public version, tag, marker, schema, runtime binding, or artifact changes; and
- an identical complete state is idempotent.

The content-addressed documents manifest is the canonical current locale
inventory. Removing a bad translation means omitting its entry from a newly
addressed manifest; it does not delete or overwrite target assets. At least the
declared default locale must remain. GitHub is the immutable asset authority;
Plaza independently rejects an in-place schema, dependency-lock, target
runtime binding, existing target-manifest asset, artifact, or public-version
change; advancing to a new content-addressed target-manifest revision is the
allowed document-only operation.


### Display Template identity

Every package has one compiled [Display Template](presentation.md#display-template),
which is not part of the schema. It is a locale-independent member of the
content-addressed documents manifest with its own four-part content revision
and a SHA-256 digest of its canonical JSON; the empty template has both. It is
never counted as a locale, and removing a locale never removes it.

- **Document-only change.** A template that differs from the Hub's is a
  document-only change, never a schema change. It advances `targetVersion`
  and preserves `appVersion`, schema, artifact, language, TypesOnly state, and
  distribution dependencies, with no rebuild. Publication follows the
  document-only reconciliation outcome above: no new App version, public
  version, tag, or marker.
- **Convergence.** The document synchronization rules apply unchanged: the
  numerically greatest content revision wins, equal revisions with different
  content are an error, each write is refetched and verified, and an unchanged
  Hub writes nothing. `DisplayVersion` is the JSON format version and never
  chooses a winner.
- **Hub storage.** The Hub retains templates by digest with the target
  manifests it already keeps and rejects a template that Studio did not compile
  from source. It never compiles or edits one.
- **Delivery.** The Hub sends a package's template with its schema through the
  existing conditional delivery of cached resources, keyed by digest: an
  unchanged template costs nothing, and a changed one arrives without
  reinstalling the App. A client never receives a schema without its template,
  including for every exact dependency it fetches.

## Public release assets

The following hash and JSON examples describe generated artifacts. Placeholder
digests and sample sizes are schematic, not installable assets. Studio owns
these files; App authors must not hand-edit them.

At creation Studio hashes the canonical target-key-sorted list of
`{target,appVersion,binarySha256}` as `targetCodeSha256`; a TypesOnly target uses
the SHA-256 of its empty code payload. It deliberately excludes target version,
source revision, build time, and documents. It then hashes
`{publicationVersion,schemaSha256,targetCodeSha256}` as
`releaseIdentitySha256`. The tag is
`libertas-{developer}-{package}-{release-identity-sha256}`. Source provenance,
function comments, and localized documents are absent from all identity hashes.
The GitHub Release title is `{developer}-{package} · {publicationVersion}` so
the public App version is visible in GitHub's release list. Studio also updates
the title when synchronizing an existing release; its immutable tag stays the same.

Each GitHub Release contains `libertas.release.json`, immutable `schema.json`,
every immutable content-addressed target-manifest revision and executable
artifact, the current content-addressed documents manifest, and its
content-addressed locale assets. Published runtime bindings, schema, dependency
pins, artifacts, and individual target-manifest revisions are never overwritten
or removed by document synchronization; current manifest pointers may advance
to a newly published revision.

The documents manifest contains metadata only, never embedded locale document
or template bodies. Its asset name includes its own lowercase SHA-256 as
`documents.<sha256>.json`; locale assets follow
`document.<locale>.<sha256>.json` and the compiled Display Template follows
`display.<sha256>.json`:

```json
{
  "defaultLocale": "en-US",
  "display": {
    "name": "display.<sha256>.json",
    "sha256": "<64 lowercase hex digits>",
    "size": 48
  },
  "documents": [
    {
      "locale": "en-US",
      "name": "document.en-US.<sha256>.json",
      "sha256": "<64 lowercase hex digits>",
      "size": 1234
    }
  ],
  "formatVersion": 2
}
```

Format 2 requires the `display` member, including for the empty template.
Entries are sorted by locale and own the exact GitHub asset name, byte size,
and digest needed for a consumer to choose before downloading. Plaza atomically
advances a publication's document-manifest locator after all newly referenced
assets exist. Each new target-manifest revision records that manifest's digest
and locale count. Old content-addressed documents and target manifests remain
and are never replaced in place, so a failed upload or retry cannot create a
missing-asset window.

```json
{
  "formatVersion": 2,
  "developerPackage": "developer-package",
  "publicationVersion": "20260820.143025.417.0",
  "schemaSha256": "sha256:<64 lowercase hex digits>",
  "targetCodeSha256": "sha256:<64 lowercase hex digits>"
}
```

The marker is immutable creation identity, including the public version and
creation target set. It is not a mutable inventory: later lower/equal-version
target additions are discovered from their retained immutable binding assets,
and withdrawals change only current availability. The tag and marker remain
stable during those operations and document-only synchronization, including
new `targetVersion` values and content-addressed target-manifest revisions.
Older marker formats are historical read-only input; new publication requests
use this format-2 contract.


## Publication completion and recovery

Plaza exposes a complete target set only after referenced public assets verify.
It has no user-visible staged or partially admitted target publication. GitHub
asset upload and Plaza admission are separate operations: a failed admission
can leave already uploaded public assets. Retry reuses the same content-addressed
snapshot; an upload failure does not change the Plaza catalog. Older referenced
assets remain available during metadata updates.

Withdrawing a target changes current availability, not retained identity or
assets. Latest discovery omits it; an exact historical lookup can still identify
it as unavailable. Installation cannot substitute a different target or latest
version for an unavailable exact dependency. Re-adding the same retained binding
restores availability; changed runtime content follows the version rules above.
