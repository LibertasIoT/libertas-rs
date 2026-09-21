# Matter device type attribute encodings

This document specifies **the schema attribute encoding** generated for
`LibertasDevice` and `LibertasVirtualDevice` fields. Specifically, it defines
the string stored in each schema node's `Attributes[].Value`:

- `DeviceType` on a `LibertasDevice` field; and
- `VirtualDevice` on a `LibertasVirtualDevice` field.

The normative attribute-value rule is:

```text
Attribute.Value = Base64(uvarint32(formatVersion) || descriptorBody)
```

The descriptor is metadata used by the Hub and clients to filter a physical
device or construct a task-owned virtual device. This encoding does **not**
describe the field's runtime object ID; that value is separate and is never
part of the attribute payload.

## Schema mapping

The TypeScript and Rust parsers produce the following language-neutral schema
types:

| Source type | Schema `Type` | Matter descriptor attribute |
| --- | --- | --- |
| `LibertasDevice` | `Device` | `DeviceType` |
| `LibertasVirtualDevice` | `VirtualDevice` | `VirtualDevice` |

The schema node remains built-in and has no child fields. The parser stores the
decorator or attribute argument verbatim as the attribute value.

```json
{
  "NativeName": "device",
  "Type": "Device",
  "BuiltinType": true,
  "Children": [],
  "Attributes": [
    { "Name": "DeviceType", "Value": "BQEBAYACAQYD" }
  ]
}
```

```json
{
  "NativeName": "virtualDevice",
  "Type": "VirtualDevice",
  "BuiltinType": true,
  "Children": [],
  "Attributes": [
    { "Name": "VirtualDevice", "Value": "BQEBgAIBAQYBBAAAAQAAAwABAgA=" }
  ]
}
```

`DeviceType` is valid only on an effective `Device` node. `VirtualDevice` is
valid only on an effective `VirtualDevice` node. Lists do not inherit either
attribute from their item type.

The source forms are:

```typescript
interface Configuration {
    /** @LibertasDeviceType("BQEBAYACAQYD") */
    device: LibertasDevice;

    /** @LibertasVirtualDeviceType("BQEBgAIBAQYBBAAAAQAAAwABAgA=") */
    virtualDevice: LibertasVirtualDevice;
}
```

```rust
#[derive(LibertasExport, LibertasAvroDecode)]
pub struct Configuration {
    #[libertas_device_type("BQEBAYACAQYD")]
    pub device: LibertasDevice,

    #[libertas_virtual_device_type("BQEBgAIBAQYBBAAAAQAAAwABAgA=")]
    pub virtual_device: LibertasVirtualDevice,
}
```

Studio's Device Type Editor and Virtual Device Type Editor generate these
strings. Application code should normally use those editors instead of
assembling binary descriptors manually.

## Attribute encoding envelope and primitives

The complete schema attribute value is standard padded Base64 over this binary
envelope:

```text
descriptor = uvarint32(formatVersion) || descriptorBody
```

The current Matter binary-schema format version is `5`. Generators must write
the format version from the loaded `BinarySchema`; `5` is the fallback when no
schema is loaded. The format version is distinct from Matter device-type and
cluster revisions.

All structures are positional. There are no field tags, field names, alignment
padding, or per-field byte lengths.

| Notation | Encoding |
| --- | --- |
| `u8` | One unsigned byte. |
| `uvarint32` | Unsigned 32-bit base-128 varint, least-significant group first. Bytes with more groups set bit 7. |
| `svarint64` | Signed 64-bit ZigZag value followed by an unsigned base-128 varint. |
| `array<T>` | `uvarint32` element count followed immediately by that many encoded `T` elements. |
| `A || B` | Byte concatenation with no separator. |

The common `loadType` byte is:

| Name | Value |
| --- | ---: |
| `LOAD_NON` | `0` |
| `LOAD` | `1` |
| `LOAD_ANY` | `2` |

Studio currently exposes `LOAD` and `LOAD_NON` in both editors. Decoders retain
the protocol's `LOAD_ANY` value.

`LOAD` indicates that the device is action as a server, otherwise it is a client.
Currently on the GUI only "LOAD" is enabled and it can't be changed, because the
task itself can be a client, there is no good reason to emulate a client device.

Cluster access is a bit mask:

| Bit | Physical-device meaning | Virtual-device meaning |
| ---: | --- | --- |
| `0x01` | Read | Server |
| `0x02` | Write | Client |

The value may be `1`, `2`, or `3` when both capabilities are required.

Matter IDs are serialized numerically, not by localized name. Studio reserves
`0xffff` as the `Any` device-type and cluster ID in selectors.

## `DeviceType` attribute for `LibertasDevice`

The complete attribute value is:

```text
DeviceType.Value = Base64(
    uvarint32(formatVersion) || MatterAppDeviceType
)
```

The `MatterAppDeviceType` body is:

```text
MatterAppDeviceType =
    u8(loadType) ||
    array<MatterAppDeviceTypeMatch>(matches)

MatterAppDeviceTypeMatch =
    array<uvarint32>(deviceTypeIds) ||
    array<MatterAppDeviceCluster>(clusters)

MatterAppDeviceCluster =
    uvarint32(clusterId) ||
    uvarint32(access)
```

Within one match, device-type IDs are logically ORed and cluster requirements
are logically ANDed. Top-level matches are logically ORed. Consequently, a
physical device matches when it satisfies at least one complete match entry.

### Verified physical-device example

This example requires Matter device type `0x0100` and cluster `0x0006` with
read/write access:

```text
formatVersion = 5
loadType      = LOAD (1)
matches       = [
  {
    deviceTypeIds = [0x0100],
    clusters      = [{ clusterId = 0x0006, access = 3 }]
  }
]
```

```text
bytes  = 05 01 01 01 80 02 01 06 03
Base64 = BQEBAYACAQYD
```

## `VirtualDevice` attribute for `LibertasVirtualDevice`

The complete attribute value is:

```text
VirtualDevice.Value = Base64(
    uvarint32(formatVersion) || MatterAppVirtualDeviceDescriptor
)
```

The `MatterAppVirtualDeviceDescriptor` body is:

```text
MatterAppVirtualDeviceDescriptor =
    u8(loadType) ||
    array<MatterDeviceType>(deviceTypes) ||
    array<MatterAppVirtualDeviceCluster>(clusters)

MatterDeviceType =
    uvarint32(deviceTypeId) ||
    uvarint32(revision)

MatterAppVirtualDeviceCluster =
    uvarint32(clusterId) ||
    uvarint32(access) ||
    uvarint32(clusterRevision) ||
    uvarint32(featureMap) ||
    array<MatterAppVirtualDeviceAttributeValue>(fixedValues) ||
    array<uvarint32>(attributes) ||
    array<uvarint32>(events) ||
    array<uvarint32>(acceptedCommands) ||
    array<uvarint32>(generatedCommands)

MatterAppVirtualDeviceAttributeValue =
    uvarint32(attributeId) ||
    svarint64(value)
```

`revision` and `clusterRevision` are semantically unsigned 16-bit values even
though the binary serializer uses `uvarint32`. `featureMap` and all Matter IDs
are unsigned 32-bit values. Fixed attribute values are signed 64-bit values.

Device types are logically ORed; clusters are logically ANDed. The cluster
lists explicitly describe the virtual endpoint's supported attributes, events,
accepted commands, and generated commands. The feature map and fixed values
participate in Matter conformance evaluation performed by Studio's editor.

### Verified virtual-device example

This example describes device type `0x0100` revision `1` and a server-side
`0x0006` cluster at revision `4`. The cluster has attribute `0x0000` and accepts
commands `0x0000`, `0x0001`, and `0x0002`:

```text
bytes  = 05 01 01 80 02 01 01 06 01 04 00 00 01 00 00 03 00 01 02 00
Base64 = BQEBgAIBAQYBBAAAAQAAAwABAgA=
```

Decoding that Base64 value yields:

```json
{
  "_loadType": 1,
  "_deviceTypes": [
    { "_deviceTypeId": 256, "_revision": 1 }
  ],
  "_clusters": [
    {
      "_clusterId": 6,
      "_access": 1,
      "_clusterRevision": 4,
      "_featureMap": 0,
      "_fixedValues": [],
      "_attributes": [0],
      "_events": [],
      "_acceptedCommands": [0, 1, 2],
      "_generatedCommands": []
    }
  ]
}
```

## Canonical generation and decoding

Generation follows this sequence:

1. Construct the appropriate descriptor using numeric Matter IDs.
2. Prefix it with the loaded Matter binary-schema format version.
3. Serialize fields in the exact order defined above.
4. Base64-encode the complete byte sequence.
5. Store the string as `DeviceType` or `VirtualDevice` in the schema node.

Decoding performs the inverse operation. A current decoder reads the version
first and then decodes the descriptor matching the schema attribute. Canonical
generators append no trailing bytes. The descriptor type is determined by the
attribute name; it is not tagged inside the binary payload.

## Excluded: runtime field value

The `DeviceType` and `VirtualDevice` attributes contain only descriptor
metadata. They never contain the selected or allocated 32-bit Libertas object
ID carried by the field at runtime. Runtime value serialization is therefore
outside the attribute encoding specified by this document.
