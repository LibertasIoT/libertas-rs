# Libertas Matter client status contract

Libertas exposes Matter interaction results to TypeScript and Rust applications
as one unsigned 32-bit value. This is an application-facing Libertas contract;
it is not the two-field `StatusIB` encoding used on the Matter wire.

| Bits | Meaning |
| --- | --- |
| 0-7 | Matter Interaction Model status |
| 8-15 | Optional Matter cluster status; zero when absent |
| 16-31 | Libertas Hub status |

Zero is success. A client that cannot recover differently for individual
errors should test the complete value for zero rather than decoding its fields.

## App interaction invariants

Use the installed public SDK's typed Matter requests, reports, and response
shapes. Register the device callback before issuing requests, preserve each
transaction ID, and branch on the documented action and transaction semantics
before decoding a callback. A report or the ignored transaction ID is not
automatically a response to the latest request.
An interaction may complete with a terminal error status even when no
attribute, event, or command payload is returned; status-only completion is
still a result and must not be discarded while waiting for payload data.

A virtual device returns each response through the SDK with the exact peer and
transaction ID from the request. It may return only standard Matter status bits
0-15; it never manufactures a Libertas Hub status.

## Task subscription replacement

Matter device subscriptions belong to the application-layer integration. Each
subscription update is sent in one shot for one App task: collect its complete
desired list of device subscriptions and submit that list together. A later
accepted list completely replaces that task's previous list, including removal
of omitted devices and paths. Tasks may replace their list repeatedly; separate
per-device submissions do not accumulate subscriptions. An empty list clears
that task's device subscriptions.

## Hub statuses

The Hub status code is shifted left by 16 before it is returned to an
application:

| Hub code | Encoded client value | Name | Meaning |
| --- | --- | --- | --- |
| `0x0000` | `0x00000000` | Success | The requested result was achieved. |
| `0x0001` | `0x00010000` | Superseded | A queued command was replaced, or a retained attribute write differs from the value finally written. |

`Superseded` is available as `LibertasMatterHubStatus.Superseded` in the
TypeScript SDK and `libertas_matter::frame::STATUS_SUPERSEDED` in the Rust SDK.

Only the Hub generates bits 16-31, and only Libertas Matter client APIs carry
them. Native Matter peers receive standard Matter wire statuses: the Hub maps
`Superseded` to `Busy` and an unknown nonzero Hub status to `Failure`.
A virtual Matter device is a Matter server and must return a standard Matter
status using bits 0-15 only.

## Command supersession

Pending commands are last-command-wins per logical-device endpoint and cluster
to reduce device traffic.
When later commands replace a queued command:

- the displaced command's original ID and complete payload are retained;
- only the newest request/reference for each source object is retained for
  completion, counting both the surviving command and displaced requests;
- when the surviving command carries a newer request from a source, that
  source's older displaced request receives no separate response. This keeps
  response bookkeeping bounded per source.

A burst from one peer leaves exactly its newest command queued; older queued
references are discarded without acknowledgement. Only that peer's latest
queued reference can receive completion or rejection. A command already sent
to the device remains owned by its active interaction.

The retained response is not sent when the queue entry is replaced. It follows
the replacement into the real Matter interaction and is sent only after that
interaction has a terminal result. Attribute reports follow ordinary Matter
subscription behavior independently; command completion
does not establish report-delivery ordering.

A retained displaced command always receives the nonzero `Superseded` rejection because
it did not execute, even when its ID and payload equal the surviving command's.
In an `A -> B -> A` chain, the retained first `A` was replaced and cannot receive
success for the final `A`'s execution. The surviving command alone receives its
own success or error result; that result is not assigned to displaced requests.
Applications can handle the rejection and decide whether to issue a new command.

## Attribute-write supersession

Queued writes share one merged Matter write interaction. Supersession is
therefore evaluated for each requested attribute rather than once for the whole
request. The Hub retains each newest requester's exact attribute/value set and,
after the interaction finishes, compares every requested value with the value
finally written by the pooled Matter interaction:

- a retained attribute receives success when its requested value equals the
  finally written value;
- it receives `Superseded` when the finally written value differs;
- retained attributes which were not replaced receive their
  actual Matter path status.

This final comparison is deliberately not updated incrementally, so a chain of
changes which eventually returns to the requested value is reported as success.
The Hub retains only the newest request/reference from each source object for a reply.
An older request's values remain in the merged write unless another write
replaces the same attributes, but its response correlation is discarded. A
retained entry can consequently contain a mixture of success, `Superseded`,
validation errors, and actual Matter path errors.

No retained write response is sent until the merged Matter write finishes. A
nonzero path status overrides the final value comparison for that attribute. A
failure of the whole interaction, including timeout or busy, becomes every
retained attribute result without performing a value comparison.
