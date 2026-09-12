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

The contract imports two identities from `signal`:

- `ObjectDigest` is the shared content-address identity used for entry and
  artifact coordinates. The mirror does not mint a second binary digest type.
- `StandardSocket` is the shared network-or-local endpoint identity used by an
  object notice. The mirror does not retain an opaque address string.

`build.rs` resolves the exact Cargo-published standard Ethos directory, proves
it is the source compiled by the pinned dependency, imports the producer's
authority seats, and projects explicit encoded Rust paths. The import is not a
copied declaration or readable alias.

## Authority and projection

`ethos/signal.ethos` is a `Signal` root and the only schema source. It declares
the five request roots and the ten reply roots directly; request and reply
seating is expressed by the Signal root itself, not by behavior.

`build.rs` reads that source through `ethos-zero`, generates the Rust, and
asserts the result equals the checked-in `src/generated/signal.rs`. The
generated projection is the whole contract surface: `Query`, `Response`, and
the declared payload types, each deriving the rkyv archive kinds together with
`Eq` and `Hash`, and, under the `datom` feature, `Datomizable` and `Composing`.

The portable rkyv frame — `Signal`, `Signalizable`, `ByteViewable`,
`Restorable` — comes from `signal` and is re-exported from `src/lib.rs`. This
crate holds no frame of its own, so a mirror frame is the same type as every
other contract's frame.

## Contract identity

`Query` bears `signal`'s `Contracted`, naming `MIRROR_SIGNAL_SOURCE` as the
authored source this contract was generated from. That makes the contract's
identity the digest of that source, so a connection is greeted once and two
peers agree exactly when their sources agree; a mismatch is a typed refusal and
never a negotiation. The digest is settled at compile time and this crate gains
no dependency to carry it. `tests/accord.rs` proves the digest against a value
computed outside this crate from the published FNV-1a algorithm.

## Boundaries

This repository contains no mirror actor, listener, authentication, durable
store, chain-validation policy, retention policy, transport loop, or component
decoder. Runtime decisions live in `mirror`; authority and retention changes
live in `meta-signal-mirror`.

The durable schema assumes no permanent compiler, host language, database,
transport process, or operating system. Rust, rkyv, and the current Signal
frame are projections around the mirror relation, not its definition.

## Verification

`examples/canonical.datom` carries every one of the five request roots and ten
reply roots as encoded Datom text. `tests/canonical.rs` actualizes each line
through the codec, renders the value back, and requires the rendering to
reproduce the authored line exactly; its root coverage is checked against
exhaustive matches over `Query` and `Response`, so a root added to the Ethos
source cannot compile until it is given a canonical line.

`tests/round_trip.rs` carries a request and a reply across fresh peer bytes
through the shared frame and proves malformed bytes are refused.
`tests/dependency_boundary.rs` proves the default graph pulls no retired codec
or generator and that the `datom` feature resolves the pinned producer
revisions. `tests/accord.rs` proves the contract digest and both arms of the
greeting receipt.

After changing `ethos/signal.ethos`, regenerate `src/generated/signal.rs` with
`ethos-zero` and update `examples/canonical.datom`. An ordinary build then
proves the checked projection is fresh.
