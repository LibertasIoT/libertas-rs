# Libertas App endpoint signaling and lifecycle

> **Status: non-normative usage guide.** The sole ground truth for endpoint SDK
> behavior is the public SDK documentation in
> [`libertas-ts/index.d.ts`](https://www.npmjs.com/package/@libertas-ai/types) for TypeScript and
> [`crates/libertas/src/lib.rs`](https://github.com/LibertasIoT/libertas-rs/blob/main/crates/libertas/src/lib.rs)
> for Rust. If this guide, a sample, a test, or any other prose disagrees with
> those SDK comments, follow the SDK comments.

This is an orientation over the public SDK, which owns exact endpoint
membership, routing, response correlation, lifecycle, and callback ordering.
Use its declarations when implementing a server or client.

## Mental model

Endpoint callbacks carry application data and standardized endpoint signals on
the same channel. Application actions carry values from the endpoint schema.
Signals such as `PeerDown`, `PeerUp`, and `PeerAlive` do not. Dispatch on the
action before asking a schema decoder or GUI to process `data`.

`PeerAlive` (opcode 24) lets a server satisfy an application-chosen silence
interval without resending unchanged application data or inventing a schema
value meaning "no change." It is a liveness observation, not application data
and not the inverse of `PeerDown`.

## Quick usage reminder

On a server, send liveness through the dedicated API:

- TypeScript: `libertasEndpointPeerAlive(endpoint, peer?)`
- Rust: `libertas_endpoint_peer_alive(endpoint, peer: Option<u32>)`

Omitting the peer asks the host to fan out through its subscriber records. A
targeted send passes back an opaque callback-provided peer handle unchanged;
the numeric value is not an App, task, endpoint, device, or client identity.

After a Hub restart, establish a fresh subscription. After the server App
restarts, do not resume its stream until the replacement receives a fresh
subscription callback. Never infer a subscriber's identity from the opaque
peer handle. A peer whose removal has completed cannot receive Data or Alive.
Each user's subscription remains subject to that user's access privileges.
The public SDK comments own exact callback ordering, timeout, resubscription,
and removal guarantees; Apps have no additional acknowledgement API.

On a client with an active subscription, handle `PeerAlive` before data decode,
rearm the applicable application watchdog, and return. It is not a cache or UI
update and does not replace sequence, revision, cursor, snapshot, or recovery
state carried by real application messages.

The exact rules for `PeerDown`, `PeerUp`, local `PeerTimeout`, removal,
resubscription, failures, retries, and host enforcement are not
restated here. Read the public SDK comments linked above before implementing an
endpoint server or client.
