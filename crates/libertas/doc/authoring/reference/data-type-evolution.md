# Libertas App data-type design guidelines

For initial declarations, follow the App contract's
[user-facing and non-user-facing type guidance](app-contract.md#user-facing-and-non-user-facing-types).
This page owns compatibility when those declarations have been published.

Libertas protocol schemas and persistent-data schemas are published contracts.
Every published version of an App must remain backward compatible with the
earlier versions of those contracts.

## Stable Union identity

Use one invariant Union type for each protocol or persistent-data family for
the entire lifetime of the App. The Union's native type name is part of its
schema identity and must not contain an App, schema, or release version.

Good stable container names include:

- `WeatherProtocol`;
- `DeviceControlProtocol`; and
- `StoredData`.

Do not publish container names such as `WeatherProtocolV1`,
`WeatherProtocolV2`, `StoredDataV3`, or `Protocol2026`. A later App version
must continue to reference the original Union; it must not replace that Union
with a newly versioned container.

This restriction applies to the Union container. A newly appended payload type
or variant may have a version-specific name when that is the clearest way to
distinguish a new wire shape, although a semantic name is normally easier to
maintain.

## Append-only evolution

Evolve a published protocol or persistent-data Union only by appending new
variants at the end of the existing declaration. A newer App version may add:

- a new protocol request, response, subscription request, or subscription-data
  payload type; or
- a new persistent-data payload type.

The new type becomes a new variant of the same stable Union. For every
previously published variant:

- preserve its declaration position and discriminator or fixed value;
- preserve its native variant name;
- preserve its payload's wire shape and meaning; and
- preserve its protocol roles and other compatibility-relevant contract.

Never insert a new variant before an existing one, remove or reorder a
variant, reuse an existing discriminator, or silently change an existing
payload type. If a payload needs an incompatible shape or meaning, define a
new payload/variant and append it.

```text
WeatherProtocol                    WeatherProtocol
  0 ReadCurrent                      0 ReadCurrent       unchanged
  1 CurrentReading    version N      1 CurrentReading    unchanged
                                 ->  2 SetUnits          appended
                                     3 UnitsChanged      appended
```

Do not evolve it by replacing the container:

```text
WeatherProtocolV1 -> WeatherProtocolV2    invalid design
```

## Why both rules are required

Union declaration order determines its runtime variant index. Keeping the
container's schema identity stable and appending only at the end preserves the
identity and index of every existing variant.

For endpoint protocols, configuration-time conformance checks establish only
the mandatory client/server contract. Optional protocol variants are checked
by the client at runtime against the server declaration before use. Stable
append-only indices make that runtime check reliable across App versions.

For persistent data, a newer App version must continue to recognize values
written under every older version of the same data schema. Appending a new
payload case preserves the older cases while allowing the newer version to
write a new data shape.

Backward compatibility is directional: the newer App must understand all
older published variants. An older App is not expected to understand a variant
first published later. Protocol code therefore treats later optional variants
as capabilities and checks support before sending them.

## Declaration-order schematics

The language syntax differs, but the schema rule is identical. The following
blocks are deliberately non-copyable schematics: they omit payload
declarations, required user documentation, schema roles, discriminators,
derives, and export annotations so that only stable identity and declaration
order are visible. Do not paste them into App source.

```typescript
// Keep this name and the existing declaration order in every release.
export type WeatherProtocol =
    | ReadCurrent
    | CurrentReading
    | SetUnits; // appended by a later release

// Keep this name and append new persistent payload types at the end.
export type StoredData =
    | RecordedReading
    | UnitPreference; // appended by a later release
```

```rust
// Keep this name and the existing declaration order in every release.
pub enum WeatherProtocol {
    ReadCurrent,
    CurrentReading(CurrentReading),
    SetUnits(SetUnits), // appended by a later release
}

// Keep this name and append new persistent payload types at the end.
pub enum StoredData {
    RecordedReading(RecordedReading),
    UnitPreference(UnitPreference), // appended by a later release
}
```

The TypeScript variants must retain their stable numeric discriminator values.
The Rust declaration order supplies the corresponding variant indices. Follow
the [TypeScript application manual](languages/typescript.md) or
[Rust application manual](languages/rust.md) for the exact protocol-role,
data-schema, discriminator, derive, and export annotations. Every real package,
type, field, member, variant, List side, and public function must also follow
the [schema documentation contract](documentation.md).

## Review checklist

Before publishing a new App version, compare each protocol and persistent-data
Union with the last published version:

- The Union has the same versionless native name and schema identity.
- Every old variant is present at the same position with the same
  discriminator, payload, meaning, and compatibility contract.
- All new payload types appear only as variants appended after the old ones.
- Existing code still reads every persistent-data variant published earlier.
- Clients check server support before using newly appended optional protocol
  variants.
