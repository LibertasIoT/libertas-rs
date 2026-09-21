# Matter App quick rules

- Use the installed SDK's typed Matter API. `LibertasDevice` selects a physical
  device; `LibertasVirtualDevice` identifies a task-owned Matter server.
- Generate `DeviceType` and `VirtualDeviceType` descriptors in Studio. Never
  hand-build or edit their encoded values.
- Register callbacks before requests, preserve transaction IDs, and branch on
  action/transaction semantics; reports are not necessarily request replies.
- Accept terminal status-only completion even when no data payload is returned.
- Submit a task's complete subscription list together; each later accepted list
  [replaces its previous list](../reference/matter/status.md#task-subscription-replacement),
  and an empty list clears it.
- Preserve the complete unsigned `u32` status. Zero is success. Use the SDK's
  public `Superseded` constant rather than copying its numeric value.
- Return virtual-device responses with the exact request peer and transaction.
  Virtual Apps return only standard Matter status and never Hub status bits.

Use the installed public SDK for exact function and tuple shapes, then consult
[device descriptor encodings](../reference/matter/descriptors.md) and the
[Matter interaction/status contract](../reference/matter/status.md).
