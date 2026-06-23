# signal-mirror — Architecture

`signal-mirror` is the ordinary working wire contract of the mirror triad
(`mirror` runtime, `signal-mirror` ordinary contract, `meta-signal-mirror`
meta policy contract). It is schema-derived: `schema/lib.schema` is the
source, `build.rs` drives `schema_rust::build::ContractCrateBuild`
(`WireContract` target), and the generated module is checked in at
`src/schema/lib.rs`. It cites `primary/skills/component-triad.md` and
`primary/skills/contract-repo.md`; only contract-specific shape is stated
here.

## The relation

One relation: **component store ↔ mirror daemon**, over Unix socket
(same-host) or tailnet TCP (cross-host), as length-prefixed signal frames.

- **Endpoints.** A component-side shipper/restorer sends requests; the mirror
  daemon replies. The mirror never initiates.
- **Cardinality.** Many components to one mirror; each request names its
  store.
- **Direction.** `Append` and `PublishCheckpoint` push history; `Restore` and
  `ObserveHeads` read it back. `NotifyObject` is the router-carried
  object/head notice: it names the store, the announced head, and optionally
  the source mirror endpoint a receiver can fetch from. Every operation is
  request/reply in this cut.
- **Authority.** The component mints commit sequences and digests (its
  sema-engine versioned log already did); the mirror only validates
  continuity and echoes heads. Store registration authority lives in
  `meta-signal-mirror`, not here.
- **Lifecycle vectors.** Appended / AppendRejected (gap, fork, unknown store,
  digest mismatch, empty suffix), CheckpointPublished / PublishRejected,
  Restored / RestoreRejected, HeadsObserved.

## Payload blindness

`EntryEnvelope` is the wire projection of one
`sema_engine::VersionedCommitLogEntry`: the envelope repeats the entry's
commit sequence, previous digest, and digest beside opaque payload bytes, so
the mirror can validate the hash chain without decoding component types.
The payload bytes are the component's own rkyv encoding of the full entry;
only the owning component ever decodes them. `CheckpointArtifact` is the
same shape for checkpoints: chain metadata beside opaque artifact bytes.

## Code map

| Path | What |
|---|---|
| `schema/lib.schema` | the authored contract source |
| `build.rs` | `ContractCrateBuild` — regenerate with `SIGNAL_MIRROR_UPDATE_SCHEMA_ARTIFACTS=1 cargo build` |
| `src/schema/lib.rs` | generated wire types + signal-frame codec (never hand-edited) |
| `src/lib.rs` | re-exports + small hand-written accessors on generated nouns |
| `tests/round_trip.rs` | rkyv frame + NOTA text round-trips per operation |

## Not owned

No runtime, no actors, no tokio, no validation logic — the append decision
(expected head, dedup, gap/fork) is the mirror daemon's Nexus plane. NOTA is
the optional text surface (`nota-text` feature, on by default); the wire is
rkyv frames.
