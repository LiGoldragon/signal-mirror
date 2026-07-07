# signal-mirror agent notes

Read this repo's `INTENT.md` and `ARCHITECTURE.md` before editing.

`signal-mirror` is the ordinary working wire contract of the mirror triad —
the typed vocabulary components speak to the payload-blind sema
version-control mirror daemon.

Load-bearing rules for this repo:

- Wire-only: no runtime, no actors, no tokio, no interpretation of payload
  bytes. The mirror is payload-blind; this contract keeps it that way.
- Edit `schema/lib.schema` and regenerate
  (`SIGNAL_MIRROR_UPDATE_SCHEMA_ARTIFACTS=1 cargo build`); never hand-edit
  `src/schema/lib.rs`.
- Every operation keeps an rkyv frame round-trip and a NOTA text round-trip
  witness in `tests/round_trip.rs`.
- Digests are blake3, 32 bytes, fixed-width on the wire (Spirit `x0ja`).
