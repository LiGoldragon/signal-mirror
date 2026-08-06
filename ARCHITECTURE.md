# signal-mirror architecture

`signal-mirror` is the ordinary Interface between a versioned component store
and a payload-blind mirror. Many stores may use one mirror; every operation
names the store whose history it concerns. The mirror never needs the component
schema to preserve, validate, and return that history.

## Semantic center

A mirrored history is an ordered chain of content-addressed opaque entries plus
optional checkpoint artifacts. The component owns the meaning of every payload.
The mirror owns continuity and durable availability: it can reject a sequence
gap, a fork from the expected head, an invalid digest relation, an empty suffix,
or an unknown store without decoding a payload.

The request surface is:

| Request | Relation |
| --- | --- |
| `Append` | Add a nonempty suffix at an optional expected head. |
| `PublishCheckpoint` | Store an opaque checkpoint covering a commit sequence. |
| `NotifyObject` | Announce a store head and an optional source mirror endpoint. |
| `Restore` | Return a checkpoint and the suffix following it. |
| `ObserveHeads` | Return one store head or the mirror's complete head listing. |

Replies explicitly distinguish accepted state, domain rejection, and internal
fault. No dropped connection is required to communicate a storage fault.

## Payload blindness

`EntryEnvelope` carries commit sequence, previous content address, current
content address, and opaque payload octets. `CheckpointArtifact` carries store,
checkpoint sequence, covered commit sequence, content address, and opaque
artifact octets. Nothing in the Interface names a component record type.

Octets are expressed as `Vector<Integer>` inside the semantic `PayloadBytes`
and `ArtifactBytes` declarations. Current behavior provides validating
conversions to and from octets; no Rust byte array, word size, compiler ABI, or
fixed emitter shape is part of the textual authority.

## Shared vocabulary

The Interface imports two identities from `signal-standard`:

- `ObjectDigest` is the shared content-address identity used for entry and
  artifact coordinates. The mirror does not mint a second binary digest type.
- `StandardSocket` is the shared network-or-local endpoint identity used by an
  object notice. The mirror does not retain an opaque address string.

`build.rs` resolves the exact Cargo-published standard Ethos directory, proves
it is the source compiled by the pinned dependency, imports the producer's
authority seats, and projects explicit encoded Rust paths. The import is not a
copied declaration or readable alias.

## Authority and projection

`ethos/interface.ethos` is a role-free `Interface.{1 0 0}` and the only schema
source. `MirrorRequest` and `MirrorReply` are ordinary declarations; request and
reply seating remains behavior until the bootstrap language expresses that
relation directly.

`src/bootstrap_manifest.rs` contains the explicit authority, grammar,
declaration, variant, and canonical-order seats. `build.rs` assembles exactly
that authorized transition, revalidates it through Core Ethos/Nomos, and asks
Rust Logos for the encoded projection in `src/schema/lib/generated.rs`.

The generated projection contains no readable schema types.
`src/schema/lib/behavior.rs` owns only present machine behavior:

- structural conversion through the standard producer's recursive wire value;
- Dotos encoding and decoding;
- rkyv behavior for encoded declarations;
- ordinary request/reply routes;
- Signal framing at contract binding 9, wire revision 2.

## Boundaries

This repository contains no mirror actor, listener, authentication, durable
store, chain-validation policy, retention policy, transport loop, or component
decoder. Runtime decisions live in `mirror`; authority and retention changes
live in `meta-signal-mirror`.

The durable schema assumes no permanent compiler, host language, database,
transport process, or operating system. Rust, rkyv, and the current Signal
envelope are projections around the mirror relation, not its definition.

## Verification

The witnesses prove all five requests and all ten replies across the bound
Signal frame, every reply through rkyv, and every root through Dotos. Canonical
Dotos examples cover the complete surface, including imported object digests
and socket endpoints. Boundary tests prove the standard pin, the corrected
bootstrap train, the absence of bootstrap crates from the runtime graph, and
the death of the legacy schema source, emitter, Nota, fixed-byte, and copied
address/digest shapes.

After changing the Interface, update the explicit manifest first and regenerate
with `SIGNAL_MIRROR_UPDATE_INTERFACE_ARTIFACTS=1 cargo build --all-features`.
An ordinary build must then prove the checked projection is fresh.
